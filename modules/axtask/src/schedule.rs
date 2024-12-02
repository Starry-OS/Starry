use alloc::{collections::BTreeMap, sync::Arc};

use core::mem::ManuallyDrop;

use crate::processor::{current_processor, PrevCtxSave, Processor};
use crate::task::{CurrentTask, TaskState};
use crate::{AxTaskRef, WaitQueue};
use spinlock::{SpinNoIrq, SpinNoIrqOnlyGuard};

/// A map to store tasks' wait queues, which stores tasks that are waiting for this task to exit.
pub(crate) static WAIT_FOR_TASK_EXITS: SpinNoIrq<BTreeMap<u64, Arc<WaitQueue>>> =
    SpinNoIrq::new(BTreeMap::new());

pub(crate) fn add_wait_for_exit_queue(task: &AxTaskRef) {
    WAIT_FOR_TASK_EXITS
        .lock()
        .insert(task.id().as_u64(), Arc::new(WaitQueue::new()));
}

pub(crate) fn get_wait_for_exit_queue(task: &AxTaskRef) -> Option<Arc<WaitQueue>> {
    WAIT_FOR_TASK_EXITS.lock().get(&task.id().as_u64()).cloned()
}

/// When the task exits, notify all tasks that are waiting for this task to exit, and
/// then remove the wait queue of the exited task.
pub(crate) fn notify_wait_for_exit(task: &AxTaskRef) {
    if let Some(wait_queue) = WAIT_FOR_TASK_EXITS.lock().remove(&task.id().as_u64()) {
        wait_queue.notify_all();
    }
}

pub(crate) fn exit_current(exit_code: i32) -> ! {
    let curr = crate::current();
    debug!("task exit: {}, exit_code={}", curr.id_name(), exit_code);
    curr.set_state(TaskState::Exited);

    // maybe others join on this thread
    // must set state before notify wait_exit
    notify_wait_for_exit(curr.as_task_ref());

    current_processor().kick_exited_task(curr.as_task_ref());
    if curr.is_init() {
        Processor::clean_all();
        axhal::misc::terminate();
    } else {
        curr.set_exit_code(exit_code);
        schedule();
    }
    unreachable!("task exited!");
}

pub(crate) fn yield_current() {
    let curr = crate::current();
    assert!(curr.is_runable());
    trace!("task yield: {}", curr.id_name());
    schedule();
}

/// Sleep the current task until the deadline.
#[cfg(feature = "irq")]
pub fn schedule_timeout(deadline: axhal::time::TimeValue) -> bool {
    let curr = crate::current();
    debug!("task sleep: {}, deadline={:?}", curr.id_name(), deadline);
    assert!(!curr.is_idle());
    crate::timers::set_alarm_wakeup(deadline, curr.clone());
    schedule();
    let timeout = axhal::time::current_time() >= deadline;
    // may wake up by others
    crate::timers::cancel_alarm(curr.as_task_ref());
    timeout
}

/// Check whether the current task should be preempted.
#[cfg(feature = "irq")]
pub fn scheduler_timer_tick() {
    let curr = crate::current();
    if !curr.is_idle() && current_processor().task_tick(curr.as_task_ref()) {
        #[cfg(feature = "preempt")]
        curr.set_preempt_pending(true);
    }
}

/// Set the sched priority of the current task.
pub fn set_current_priority(prio: isize) -> bool {
    current_processor().set_priority(crate::current().as_task_ref(), prio)
}

/// Wake up the given task from the blocking state.
pub fn wakeup_task(task: AxTaskRef) {
    let mut state = task.state_lock_manual();
    match **state {
        TaskState::Blocking => **state = TaskState::Runable,
        TaskState::Runable => (),
        TaskState::Blocked => {
            debug!("task unblock: {}", task.id_name());
            **state = TaskState::Runable;
            ManuallyDrop::into_inner(state);
            // may be other processor wake up
            Processor::add_task(task.clone());
            return;
        }
        _ => panic!("try to wakeup {:?} unexpect state {:?}", task.id(), **state),
    }
    ManuallyDrop::into_inner(state);
}

/// Schedule the next task to run.
pub fn schedule() {
    let next_task = current_processor().pick_next_task();
    switch_to(next_task);
}

fn switch_to(mut next_task: AxTaskRef) {
    let prev_task = crate::current();

    // task in a disable_preempt context? it not allowed ctx switch
    #[cfg(feature = "preempt")]
    assert!(
        prev_task.can_preempt(),
        "task can_preempt failed {}",
        prev_task.id_name()
    );

    // When the prev_task state_lock is locked, it records the irq configuration of
    // the prev_task at that time, after swich(in switch_post) it would be unlocked,
    // and restore the irq configuration to the lock_state store(NOTE: it own the prev_task).
    //
    // so have to save the prev_task irq config here,and restore it after swich_post
    #[cfg(feature = "irq")]
    prev_task.set_irq_state(axhal::arch::irqs_enabled());

    // Here must lock curr state, and no one can change curr state
    // when excuting ctx_switch
    let mut prev_state_lock = prev_task.state_lock_manual();

    match **prev_state_lock {
        TaskState::Runable => {
            if next_task.is_idle() {
                next_task = prev_task.clone();
            } else if !prev_task.is_idle() {
                #[cfg(feature = "preempt")]
                current_processor()
                    .put_prev_task(prev_task.clone(), prev_task.get_preempt_pending());
                #[cfg(not(feature = "preempt"))]
                current_processor().put_prev_task(prev_task.clone(), false);
            }
        }
        TaskState::Blocking => {
            debug!("task block: {}", prev_task.id_name());
            **prev_state_lock = TaskState::Blocked;
        }
        TaskState::Exited => {}
        _ => {
            panic!("unexpect state when switch_to happend ");
        }
    }

    #[cfg(feature = "preempt")]
    //reset preempt pending
    next_task.set_preempt_pending(false);

    if prev_task.ptr_eq(&next_task) {
        ManuallyDrop::into_inner(prev_state_lock);
        return;
    }

    // 当任务进行切换时，更新两个任务的时间统计信息
    #[cfg(feature = "monolithic")]
    {
        let current_timestamp = axhal::time::current_time_nanos() as usize;
        next_task.time_stat_when_switch_to(current_timestamp);
        prev_task.time_stat_when_switch_from(current_timestamp);
    }

    trace!(
        "context switch: {} -> {}",
        prev_task.id_name(),
        next_task.id_name(),
    );
    unsafe {
        let prev_ctx_ptr = prev_task.ctx_mut_ptr();
        let next_ctx_ptr = next_task.ctx_mut_ptr();

        // The strong reference count of `prev_task` will be decremented by 1,
        // but won't be dropped until `gc_entry()` is called.
        assert!(
            Arc::strong_count(prev_task.as_task_ref()) > 1,
            "task id {} strong count {}",
            prev_task.id().as_u64(),
            Arc::strong_count(prev_task.as_task_ref())
        );

        assert!(Arc::strong_count(&next_task) >= 1);
        #[cfg(feature = "monolithic")]
        {
            let page_table_token = *next_task.page_table_token.get();
            if page_table_token != 0 {
                axhal::arch::write_page_table_root0(page_table_token.into());
            }
        }

        let prev_ctx = PrevCtxSave::new(core::mem::transmute::<
            ManuallyDrop<SpinNoIrqOnlyGuard<'_, TaskState>>,
            ManuallyDrop<SpinNoIrqOnlyGuard<'static, TaskState>>,
        >(prev_state_lock));

        current_processor().set_prev_ctx_save(prev_ctx);

        CurrentTask::set_current(prev_task, next_task);

        axhal::arch::task_context_switch(&mut (*prev_ctx_ptr), &(*next_ctx_ptr));

        current_processor().switch_post();
    }
}
