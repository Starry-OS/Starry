use crate::schedule::schedule;
#[cfg(feature = "irq")]
use crate::schedule::schedule_timeout;
use crate::wait_list::WaitTaskList;
use crate::wait_list::WaitTaskNode;
use crate::AxTaskRef;
use alloc::sync::Arc;
use spinlock::SpinNoIrq;
/// A queue to store sleeping tasks.
///
/// # Examples
///
/// ```
/// use axtask::WaitQueue;
/// use core::sync::atomic::{AtomicU32, Ordering};
///
/// static VALUE: AtomicU32 = AtomicU32::new(0);
/// static WQ: WaitQueue = WaitQueue::new();
///
/// axtask::init_scheduler();
/// // spawn a new task that updates `VALUE` and notifies the main task
/// axtask::spawn(|| {
///     assert_eq!(VALUE.load(Ordering::Relaxed), 0);
///     VALUE.fetch_add(1, Ordering::Relaxed);
///     WQ.notify_one(true); // wake up the main task
/// });
///
/// WQ.wait(); // block until `notify()` is called
/// assert_eq!(VALUE.load(Ordering::Relaxed), 1);
/// ```
///

#[macro_export]
macro_rules! declare_wait {
    ($name: ident) => {
        let $name = Arc::new(WaitTaskNode::new($crate::current().as_task_ref().clone()));
    };
}

/// A queue to store tasks that are waiting for some conditions.
pub struct WaitQueue {
    // Support queue lock by external caller,use SpinNoIrq
    // Arceos SpinNoirq current implementation implies irq_save,
    // so it can be nested
    // use linked list has good performance
    queue: SpinNoIrq<WaitTaskList>,
}

impl WaitQueue {
    /// Creates an empty wait queue.
    pub const fn new() -> Self {
        Self {
            queue: SpinNoIrq::new(WaitTaskList::new()),
        }
    }

    // CPU0 wait                CPU1 notify           CPU2 signal
    // q.lock()
    // task.lock()
    // task.state = blocking
    // task.unlock()
    // q.unlock()
    //                          q.lock()
    //                          task = q.get;
    //                          wakeup(task)
    //                            task == blocking
    //                              task = runable
    //                          q.unlock()
    // schedule()
    // queue.lock().remove(curr)
    //
    /// Blocks the current task and put it into the wait queue, until other task
    /// notifies it.
    pub fn wait(&self) {
        declare_wait!(waiter);
        self.queue.lock().prepare_to_wait(waiter.clone());
        schedule();

        // maybe wakeup by signal or others, try to delete again
        // 1. starry support UNINTERRUPT mask, no need to check
        // 2. starry support INTERRUPTABLE mask, still need to check
        self.queue.lock().remove(&waiter);
    }

    /// Blocks the current task and put it into the wait queue, until the given
    /// `condition` becomes true.
    ///
    /// Note that even other tasks notify this task, it will not wake up until
    /// the condition becomes true.
    pub fn wait_until<F>(&self, condition: F)
    where
        F: Fn() -> bool,
    {
        declare_wait!(waiter);
        loop {
            if condition() {
                break;
            }
            // maybe wakeup by signal or others, should check before push
            // wait_list will do check
            self.queue.lock().prepare_to_wait(waiter.clone());
            schedule();
        }

        //maybe wakeup by signal or others, try to delete again
        self.queue.lock().remove(&waiter);
    }

    /// Blocks the current task and put it into the wait queue, until other tasks
    /// notify it, or the given duration has elapsed.
    #[cfg(feature = "irq")]
    pub fn wait_timeout(&self, dur: core::time::Duration) -> bool {
        declare_wait!(waiter);
        let deadline = axhal::time::current_time() + dur;

        self.queue.lock().prepare_to_wait(waiter.clone());
        let timeout = schedule_timeout(deadline);

        //maybe wakeup by timer or signal, try to delete again
        self.queue.lock().remove(&waiter);
        timeout
    }

    /// Blocks the current task and put it into the wait queue, until the given
    /// `condition` becomes true, or the given duration has elapsed.
    ///
    /// Note that even other tasks notify this task, it will not wake up until
    /// the above conditions are met.
    #[cfg(feature = "irq")]
    pub fn wait_timeout_until<F>(&self, dur: core::time::Duration, condition: F) -> bool
    where
        F: Fn() -> bool,
    {
        declare_wait!(waiter);
        let deadline = axhal::time::current_time() + dur;
        let mut timeout = false;
        loop {
            if condition() {
                break;
            }
            //maybe wakeup by signal or others, should check before push
            self.queue.lock().prepare_to_wait(waiter.clone());
            timeout = schedule_timeout(deadline);
            if timeout {
                break;
            }
        }

        //maybe wakeup by timer or signal, try to delete again
        self.queue.lock().remove(&waiter);
        timeout
    }

    /// Wake up the given task in the wait queue.
    pub fn notify_task(&self, task: &AxTaskRef) -> bool {
        self.queue.lock().notify_task(task)
    }

    /// Wakes up one task in the wait queue, usually the first one.
    pub fn notify_one(&self) -> bool {
        self.queue.lock().notify_one()
    }

    /// Wakes all tasks in the wait queue.
    pub fn notify_all(&self) {
        self.queue.lock().notify_all()
    }
}
