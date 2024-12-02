use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::mem::ManuallyDrop;
use core::sync::atomic::{AtomicUsize, Ordering};
use lazy_init::LazyInit;
use scheduler::BaseScheduler;
use spinlock::{SpinNoIrq, SpinNoIrqOnly, SpinNoIrqOnlyGuard};

#[cfg(feature = "monolithic")]
use axhal::KERNEL_PROCESS_ID;

use crate::task::{new_init_task, new_task, CurrentTask, TaskState};

use crate::{AxTaskRef, Scheduler, WaitQueue};

static PROCESSORS: SpinNoIrqOnly<VecDeque<&'static Processor>> =
    SpinNoIrqOnly::new(VecDeque::new());

#[percpu::def_percpu]
static PROCESSOR: LazyInit<Processor> = LazyInit::new();

/// Processor on coresponding core
pub struct Processor {
    /// Processor SCHEDULER
    scheduler: SpinNoIrq<Scheduler>,
    /// Owned this Processor task num
    task_nr: AtomicUsize,
    /// The exited task-queue of the current processor
    exited_tasks: SpinNoIrq<VecDeque<AxTaskRef>>,
    /// GC wait or notify use
    gc_wait: WaitQueue,
    /// Pre save ctx when processor switch ctx
    prev_ctx_save: SpinNoIrq<PrevCtxSave>,
    /// The idle task of the processor
    idle_task: AxTaskRef,
    /// The gc task of the processor
    gc_task: AxTaskRef,
}

unsafe impl Sync for Processor {}
unsafe impl Send for Processor {}

impl Processor {
    /// Create a new processor
    pub fn new(idle_task: AxTaskRef) -> Self {
        let gc_task = new_task(
            gc_entry,
            "gc".into(),
            axconfig::TASK_STACK_SIZE,
            #[cfg(feature = "monolithic")]
            KERNEL_PROCESS_ID,
            #[cfg(feature = "monolithic")]
            0,
        );

        Processor {
            scheduler: SpinNoIrq::new(Scheduler::new()),
            idle_task,
            prev_ctx_save: SpinNoIrq::new(PrevCtxSave::new_empty()),
            exited_tasks: SpinNoIrq::new(VecDeque::new()),
            gc_wait: WaitQueue::new(),
            task_nr: AtomicUsize::new(0),
            gc_task,
        }
    }

    /// The idle task of the processor
    pub fn idle_task(&self) -> &AxTaskRef {
        &self.idle_task
    }

    pub(crate) fn kick_exited_task(&self, task: &AxTaskRef) {
        self.exited_tasks.lock().push_back(task.clone());
        self.task_nr.fetch_sub(1, Ordering::Acquire);
        self.gc_wait.notify_one();
    }

    pub(crate) fn clean_task_wait(&self) {
        loop {
            // Drop all exited tasks and recycle resources.
            let n = self.exited_tasks.lock().len();
            for _ in 0..n {
                // Do not do the slow drops in the critical section.
                let task = self.exited_tasks.lock().pop_front();
                if let Some(task) = task {
                    if Arc::strong_count(&task) == 1 {
                        // If I'm the last holder of the task, drop it immediately.
                        debug!("clean task :{} ", task.id().as_u64());
                        drop(task);
                    } else {
                        // Otherwise (e.g, `switch_to` is not compeleted, held by the
                        // joiner, etc), push it back and wait for them to drop first.
                        self.exited_tasks.lock().push_back(task);
                    }
                }
            }
            // gc wait other task exit
            self.gc_wait.wait();
        }
    }

    #[inline]
    /// Pick one task from processor
    pub(crate) fn pick_next_task(&self) -> AxTaskRef {
        self.scheduler
            .lock()
            .pick_next_task()
            .unwrap_or_else(|| self.idle_task.clone())
    }

    #[inline]
    /// Add curr task to Processor, it ususally add to back
    pub(crate) fn put_prev_task(&self, task: AxTaskRef, front: bool) {
        self.scheduler.lock().put_prev_task(task, front);
    }

    #[inline]
    /// Add task to processor, now just put it to own processor
    /// TODO: support task migrate on differ processor
    pub(crate) fn add_task(task: AxTaskRef) {
        task.get_processor().scheduler.lock().add_task(task);
    }

    #[inline]
    /// Processor Clean
    pub(crate) fn task_tick(&self, task: &AxTaskRef) -> bool {
        self.scheduler.lock().task_tick(task)
    }

    #[inline]
    /// Processor Clean
    pub(crate) fn set_priority(&self, task: &AxTaskRef, prio: isize) -> bool {
        self.scheduler.lock().set_priority(task, prio)
    }

    #[inline]
    /// update prev_ctx_save when ctx_switch
    pub(crate) fn set_prev_ctx_save(&self, prev_save: PrevCtxSave) {
        *self.prev_ctx_save.lock() = prev_save;
    }

    #[inline]
    /// post process prev_ctx_save
    pub(crate) fn switch_post(&self) {
        let mut prev_ctx = self.prev_ctx_save.lock();
        if let Some(prev_lock_state) = prev_ctx.get_mut().take() {
            // Note the lock sequence: prev_lock_state.lock -> prev_ctx_save.lock ->
            // prev_ctx_save.unlock -> prev_lock_state.unlock
            drop(prev_ctx);
            ManuallyDrop::into_inner(prev_lock_state);
        } else {
            panic!("no prev ctx");
        }

        #[cfg(feature = "irq")]
        {
            let curr = crate::current();
            match curr.get_irq_state() {
                true => axhal::arch::enable_irqs(),
                false => axhal::arch::disable_irqs(),
            }
        }
    }

    #[inline]
    /// Processor Clean
    fn clean(&self) {
        self.exited_tasks.lock().clear()
    }

    #[inline]
    /// Processor Clean all
    pub fn clean_all() {
        for p in PROCESSORS.lock().iter() {
            p.clean()
        }
    }

    #[inline]
    /// First add task to processor
    pub fn first_add_task(task: AxTaskRef) {
        let p = Processor::select_one_processor();
        task.init_processor(p);
        p.scheduler.lock().add_task(task);
        p.task_nr.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    /// gc init
    pub(crate) fn gc_init(&'static self) {
        self.gc_task.init_processor(&self);
        self.scheduler.lock().add_task(self.gc_task.clone());
    }

    #[inline]
    /// Add task to processor
    fn select_one_processor() -> &'static Processor {
        PROCESSORS
            .lock()
            .iter()
            .min_by_key(|p| p.task_nr.load(Ordering::Acquire))
            .unwrap()
    }
}

/// Get current processor pointer
///
/// # Safety
/// The processor pointer is a per-core global variable, so it is safe to access it.
pub fn current_processor() -> &'static Processor {
    unsafe { PROCESSOR.current_ref_raw() }
}

pub(crate) struct PrevCtxSave(Option<ManuallyDrop<SpinNoIrqOnlyGuard<'static, TaskState>>>);

impl PrevCtxSave {
    pub(crate) fn new(
        prev_lock_state: ManuallyDrop<SpinNoIrqOnlyGuard<'static, TaskState>>,
    ) -> PrevCtxSave {
        Self(Some(prev_lock_state))
    }

    const fn new_empty() -> PrevCtxSave {
        Self(None)
    }

    #[allow(unused)]
    pub(crate) fn get(&self) -> &Option<ManuallyDrop<SpinNoIrqOnlyGuard<'static, TaskState>>> {
        &self.0
    }

    pub(crate) fn get_mut(
        &mut self,
    ) -> &mut Option<ManuallyDrop<SpinNoIrqOnlyGuard<'static, TaskState>>> {
        &mut self.0
    }
}

fn gc_entry() {
    current_processor().clean_task_wait();
}

pub(crate) fn init() {
    const IDLE_TASK_STACK_SIZE: usize = 4096;

    let idle_task = new_task(
        || crate::run_idle(),
        "idle".into(), // FIXME: name 现已被用作 prctl 使用的程序名，应另选方式判断 idle 进程
        IDLE_TASK_STACK_SIZE,
        #[cfg(feature = "monolithic")]
        KERNEL_PROCESS_ID,
        #[cfg(feature = "monolithic")]
        0,
    );

    let main_task = new_init_task("main".into());
    #[cfg(feature = "monolithic")]
    main_task.set_process_id(KERNEL_PROCESS_ID);

    let processor = Processor::new(idle_task.clone());
    PROCESSOR.with_current(|i| i.init_by(processor));
    current_processor().gc_init();
    PROCESSORS.lock().push_back(current_processor());

    main_task.init_processor(current_processor());

    unsafe { CurrentTask::init_current(main_task) }
}

pub(crate) fn init_secondary() {
    // FIXME: name 现已被用作 prctl 使用的程序名，应另选方式判断 idle 进程
    let idle_task = new_init_task("idle".into());
    #[cfg(feature = "monolithic")]
    idle_task.set_process_id(KERNEL_PROCESS_ID);

    let processor = Processor::new(idle_task.clone());
    PROCESSOR.with_current(|i| i.init_by(processor));
    current_processor().gc_init();
    PROCESSORS.lock().push_back(current_processor());

    unsafe { CurrentTask::init_current(idle_task) };
}
