use axtask::AxTaskRef;

/// Kernel futex
pub struct FutexQ {
    /// The `val` of the futex
    /// the task in the queue waiting for the same futex may have different `val`
    pub key: FutexKey,
    /// the task which is waiting for the futex
    pub task: AxTaskRef,
    /// the bitset of the futex
    pub bitset: u32,
}

impl FutexQ {
    /// Create a new futex queue
    pub fn new(key: FutexKey, task: AxTaskRef, bitset: u32) -> Self {
        Self { key, task, bitset }
    }
    /// check if the futex queues matches the key
    pub fn match_key(&self, key: &FutexKey) -> bool {
        self.key == *key
    }
}

/// Futexes are matched on equal values of this key.
///
/// The key type depends on whether it's a shared or private mapping.
/// use pid to replace the mm_struct pointer
/// **only support private futex now**
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct FutexKey {
    /// use pid to replace the mm_struct pointer to distinguish different processes
    /// only support private futex now
    pub pid: u32,
    // aligned to page size addr
    pub(crate) aligned: u32,
    // offset in page
    pub(crate) offset: u32,
}

impl FutexKey {
    #[allow(missing_docs)]
    pub fn new(pid: u64, aligned: usize, offset: u32) -> Self {
        Self {
            pid: pid as u32,
            aligned: aligned as u32,
            offset,
        }
    }
}
