//! completion
//! If you have one or more threads that must wait for some kernel activity
//! to have reached a point or a specific state, completions can provide a
//! race-free solution to this problem. Semantically they are somewhat like a
//! pthread_barrier() and have similar use-cases.
//!
//! Completions are a code synchronization mechanism which is preferable to any
//! misuse of locks/semaphores and busy-loops. Any time you think of using
//! yield() or some quirky msleep(1) loop to allow something else to proceed,
//! you probably want to look into using one of the wait_for_completion*()
//! calls and complete() instead.
//!
//! Completions are built on top of the waitqueue and wakeup infrastructure of
//! scheduler(axtask). The event the threads on the waitqueue are waiting for
//! is reduced to a simple flag in 'struct completion', appropriately called "done".
//!
use alloc::sync::Arc;
use axtask::schedule;
#[cfg(feature = "irq")]
use axtask::schedule_timeout;
use axtask::{WaitTaskList, WaitTaskNode};
#[cfg(feature = "irq")]
use core::time::Duration;
use spinlock::SpinNoIrq;

use axtask::declare_wait;

struct Inner {
    queue: WaitTaskList,
    done: i32,
}

impl Inner {
    /// Creates a new completion
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            queue: WaitTaskList::new(),
            done: 0,
        }
    }
}

/// Cpmpletion struct, it protect by done
pub struct Completion {
    inner: SpinNoIrq<Inner>,
}

// SAFETY: have it's own SpinNoIrq protect
unsafe impl Sync for Completion {}
unsafe impl Send for Completion {}

impl Completion {
    /// Creates a new completion
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            inner: SpinNoIrq::new(Inner::new()),
        }
    }

    /// reinit completion calller make sure no thread in waitQ
    #[inline(always)]
    pub fn reinit(&self) {
        self.inner.lock().done = 0;
    }

    /// waits for completion of a task
    pub fn wait_for_completion(&self) {
        declare_wait!(waiter);
        loop {
            let mut inner = self.inner.lock();
            assert!(inner.done >= 0);
            if inner.done == 0 {
                inner.queue.prepare_to_wait(waiter.clone());
                drop(inner);
                schedule();
            } else {
                inner.done -= 1;
                break;
            }
        }
    }

    #[cfg(feature = "irq")]
    /// waits for completion of a task (w/timeout secs)
    pub fn wait_for_completion_timeout(&self, secs: u64) -> bool {
        declare_wait!(waiter);
        let deadline = axhal::time::current_time() + Duration::from_secs(secs);
        let timeout = loop {
            let mut inner = self.inner.lock();
            assert!(inner.done >= 0);
            if inner.done == 0 {
                inner.queue.prepare_to_wait(waiter.clone());
                drop(inner);
                if schedule_timeout(deadline) {
                    break true;
                }
            } else {
                inner.done -= 1;
                break false;
            }
        };

        timeout
    }

    /// signals a single thread waiting on this completion
    pub fn complete(&self) {
        let mut inner = self.inner.lock();
        inner.done += 1;
        inner.queue.notify_one();
    }

    /// signals a single thread waiting on this completion
    pub fn complete_all(&self) {
        let mut inner = self.inner.lock();
        inner.done = i32::MAX;
        inner.queue.notify_all();
    }
}
