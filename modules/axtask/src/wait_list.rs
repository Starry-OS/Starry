use crate::schedule::wakeup_task;
use crate::task::TaskState;
use crate::AxTaskRef;
use alloc::sync::Arc;
use core::ops::Deref;

use linked_list::{GetLinks, Links, List};

/// A task wrapper.
///
/// It add extra states to use in [`linked_list::List`].
pub struct WaitTaskNode {
    inner: AxTaskRef,
    links: Links<Self>,
}

impl GetLinks for WaitTaskNode {
    type EntryType = Self;

    #[inline]
    fn get_links(t: &Self) -> &Links<Self> {
        &t.links
    }
}

impl WaitTaskNode {
    /// Creates a new FifoTask from the inner task struct.
    pub const fn new(inner: AxTaskRef) -> Self {
        Self {
            inner,
            links: Links::new(),
        }
    }

    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &AxTaskRef {
        &self.inner
    }
}

impl Deref for WaitTaskNode {
    type Target = AxTaskRef;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A simple FIFO wait task list
///
/// When a task is added to the list, it's placed at the end of the waitlist.
/// When picking the next task to run, the head of the wait list is taken.
pub struct WaitTaskList {
    list: List<Arc<WaitTaskNode>>,
}

impl WaitTaskList {
    /// Creates a new empty WaitList.
    pub const fn new() -> Self {
        Self { list: List::new() }
    }

    /// add wait to list back
    pub fn prepare_to_wait(&mut self, task: Arc<WaitTaskNode>) {
        task.inner().set_state(TaskState::Blocking);
        self.list.push_back(task);
    }

    /// Removes the given Node
    ///
    /// # Safety
    ///
    /// Callers must ensure that `data` is either on this list or in no list. It being on another
    /// list leads to memory unsafety.
    pub fn remove(&mut self, node: &Arc<WaitTaskNode>) -> Option<Arc<WaitTaskNode>> {
        unsafe { self.list.remove(node) }
    }

    /// notify special task and remove it
    pub fn notify_task(&mut self, task: &AxTaskRef) -> bool {
        let mut cursor = self.list.cursor_front_mut();
        let wake = loop {
            match cursor.current() {
                Some(node) => {
                    if Arc::ptr_eq(node.inner(), task) {
                        wakeup_task(node.inner().clone());
                        break true;
                    }
                }
                None => break false,
            }
            cursor.move_next();
        };
        if wake {
            cursor.remove_current();
        }

        false
    }

    /// notify first task and remove it
    pub fn notify_one(&mut self) -> bool {
        if let Some(node) = self.list.pop_front() {
            wakeup_task(node.inner().clone());
            return true;
        }
        false
    }

    /// notify all task and remove it
    pub fn notify_all(&mut self) {
        loop {
            if !self.notify_one() {
                break;
            }
        }
    }
}
