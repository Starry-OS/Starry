//! Task APIs for multi-task configuration.

use alloc::{string::String, sync::Arc};
#[cfg(feature = "monolithic")]
use axhal::KERNEL_PROCESS_ID;

use crate::task::{ScheduleTask, TaskState};

use crate::schedule::get_wait_for_exit_queue;
#[doc(cfg(feature = "multitask"))]
pub use crate::task::{new_task, CurrentTask, TaskId};
#[doc(cfg(feature = "multitask"))]
pub use crate::wait_queue::WaitQueue;

pub use crate::processor::{current_processor, Processor};

pub use crate::schedule::schedule;

#[cfg(feature = "irq")]
pub use crate::schedule::schedule_timeout;

/// The reference type of a task.
pub type AxTaskRef = Arc<AxTask>;

cfg_if::cfg_if! {
    if #[cfg(feature = "sched_rr")] {
        const MAX_TIME_SLICE: usize = 5;
        pub(crate) type AxTask = scheduler::RRTask<ScheduleTask, MAX_TIME_SLICE>;
        pub(crate) type Scheduler = scheduler::RRScheduler<ScheduleTask, MAX_TIME_SLICE>;
    } else if #[cfg(feature = "sched_cfs")] {
        pub(crate) type AxTask = scheduler::CFSTask<ScheduleTask>;
        pub(crate) type Scheduler = scheduler::CFScheduler<ScheduleTask>;
    } else {
        // If no scheduler features are set, use FIFO as the default.
        pub(crate) type AxTask = scheduler::FifoTask<ScheduleTask>;
        pub(crate) type Scheduler = scheduler::FifoScheduler<ScheduleTask>;
    }
}

/// Gets the current task, or returns [`None`] if the current task is not
/// initialized.
pub fn current_may_uninit() -> Option<CurrentTask> {
    CurrentTask::try_get()
}

/// Gets the current task.
///
/// # Panics
///
/// Panics if the current task is not initialized.
pub fn current() -> CurrentTask {
    CurrentTask::get()
}

/// Initializes the task scheduler (for the primary CPU).
pub fn init_scheduler() {
    info!("Initialize scheduling...");

    crate::processor::init();
    #[cfg(feature = "irq")]
    crate::timers::init();

    info!("  use {} scheduler.", Scheduler::scheduler_name());
}

/// Initializes the task scheduler for secondary CPUs.
pub fn init_scheduler_secondary() {
    crate::processor::init_secondary();
}

/// Handles periodic timer ticks for the task manager.
///
/// For example, advance scheduler states, checks timed events, etc.
#[cfg(feature = "irq")]
#[doc(cfg(feature = "irq"))]
pub fn on_timer_tick() {
    crate::timers::check_events();
    crate::schedule::scheduler_timer_tick();
}

#[cfg(feature = "preempt")]
/// Checks if the current task should be preempted.
/// This api called after handle irq,it may be on a
/// disable_preempt ctx
pub fn current_check_preempt_pending() {
    let curr = crate::current();
    // if task is already exited or blocking,
    // no need preempt, they are rescheduling
    if curr.get_preempt_pending() && curr.can_preempt() && !curr.is_exited() && !curr.is_blocking()
    {
        debug!(
            "current {} is to be preempted , allow {}",
            curr.id_name(),
            curr.can_preempt()
        );
        crate::schedule::schedule()
    }
}

/// Spawns a new task with the given parameters.
///
/// Returns the task reference.
pub fn spawn_raw<F>(f: F, name: String, stack_size: usize) -> AxTaskRef
where
    F: FnOnce() + Send + 'static,
{
    let task = new_task(
        f,
        name,
        stack_size,
        #[cfg(feature = "monolithic")]
        KERNEL_PROCESS_ID,
        #[cfg(feature = "monolithic")]
        0,
    );
    Processor::first_add_task(task.clone());
    task
}

/// Spawns a new task with the default parameters.
///
/// The default task name is an empty string. The default task stack size is
/// [`axconfig::TASK_STACK_SIZE`].
///
/// Returns the task reference.
pub fn spawn<F>(f: F) -> AxTaskRef
where
    F: FnOnce() + Send + 'static,
{
    spawn_raw(f, "".into(), axconfig::TASK_STACK_SIZE)
}

/// Set the priority for current task.
///
/// The range of the priority is dependent on the underlying scheduler. For
/// example, in the [CFS] scheduler, the priority is the nice value, ranging from
/// -20 to 19.
///
/// Returns `true` if the priority is set successfully.
///
/// [CFS]: https://en.wikipedia.org/wiki/Completely_Fair_Scheduler
pub fn set_priority(prio: isize) -> bool {
    crate::schedule::set_current_priority(prio)
}

/// Current task gives up the CPU time voluntarily, and switches to another
/// ready task.
pub fn yield_now() {
    crate::schedule::yield_current();
}

/// Current task is going to sleep for the given duration.
///
/// If the feature `irq` is not enabled, it uses busy-wait instead.
pub fn sleep(dur: core::time::Duration) {
    sleep_until(axhal::time::current_time() + dur);
}

/// Current task is going to sleep, it will be woken up at the given deadline.
///
/// If the feature `irq` is not enabled, it uses busy-wait instead.
pub fn sleep_until(deadline: axhal::time::TimeValue) {
    #[cfg(feature = "irq")]
    crate::schedule::schedule_timeout(deadline);
    #[cfg(not(feature = "irq"))]
    axhal::time::busy_wait_until(deadline);
}
/// wake up task
pub fn wakeup_task(task: AxTaskRef) {
    crate::schedule::wakeup_task(task)
}

/// Current task is going to sleep, it will be woken up when the given task exits.
///
/// If the given task is already exited, it will return immediately.
pub fn join(task: &AxTaskRef) -> Option<i32> {
    if let Some(wait_queue) = get_wait_for_exit_queue(task) {
        wait_queue.wait_until(|| task.state() == TaskState::Exited)
    }
    Some(task.get_exit_code())
}

#[cfg(feature = "monolithic")]
/// Current task is going to sleep. It will be woken up when the given task does exec syscall or exit.
pub fn vfork_suspend(task: &AxTaskRef) {
    if let Some(wait_queue) = get_wait_for_exit_queue(task) {
        wait_queue.wait_until(|| {
            // If the given task does the exec syscall, it will be the leader of the new process.
            task.is_leader() || task.state() == TaskState::Exited
        });
    }
}

#[cfg(feature = "monolithic")]
/// To wake up the task that is blocked because vfork out of current task
pub fn wake_vfork_process(task: &AxTaskRef) {
    if let Some(wait_queue) = get_wait_for_exit_queue(task) {
        wait_queue.notify_all()
    }
}

/// Exits the current task.
pub fn exit(exit_code: i32) -> ! {
    crate::schedule::exit_current(exit_code)
}

/// The idle task routine.
///
/// It runs an infinite loop that keeps calling [`yield_now()`].
pub fn run_idle() -> ! {
    loop {
        yield_now();
        //debug!("idle task: waiting for IRQs...");
        #[cfg(feature = "irq")]
        axhal::arch::wait_for_irqs();
    }
}

/// Dump the current task backtrace.
pub fn dump_curr_backtrace() {
    dump_task_backtrace(current().as_task_ref().clone());
}

/// Dump the given task backtrace.
pub fn dump_task_backtrace(task: AxTaskRef) {
    use axbacktrace::{dump_backtrace, StackInfo, Unwind, UnwindIf};

    let stack_low = task.get_kernel_stack_down().unwrap();
    let stack_high = task.get_kernel_stack_top().unwrap();
    info!(
        "dump task: {}, stack range: {:#016x}: {:#016x}",
        task.id_name(),
        stack_low,
        stack_high
    );
    let stack_info = StackInfo::new(stack_low, stack_high);

    //Init Unwind instance from current context
    let curr = crate::current();
    let mut unwind = if curr.ptr_eq(&task) {
        Unwind::new_from_cur_ctx(stack_info)
    } else {
        let (pc, fp) = task.ctx_unwind();
        Unwind::new(pc, fp, stack_info)
    };
    // dump current task trace
    dump_backtrace(&mut unwind);
}
