//! Futex operations and flags for posix syscall.
//!
//! Details are listed in <https://manpages.debian.org/unstable/manpages-dev/futex.2.en.html#FUTEX_WAIT>

numeric_enum_macro::numeric_enum! {
    #[repr(i32)]
    #[allow(missing_docs)]
    #[allow(non_camel_case_types)]
    #[derive(Eq, PartialEq, Debug, Copy, Clone)]
    /// Futex operation for posix syscall listed in
    /// <https://manpages.debian.org/unstable/manpages-dev/futex.2.en.html#FUTEX_WAIT>
    pub enum FutexOp {
        WAIT = 0,
        WAKE = 1,
        FD = 2,
        REQUEUE = 3,
        CMP_REQUEUE = 4,
        WAKE_OP = 5,
        LOCK_PI = 6,
        UNLOCK_PI = 7,
        TRYLOCK_PI = 8,
        WAIT_BITSET = 9,
        WAKE_BITSET = 10,
        WAIT_REQUEUE_PI = 11,
        CMP_REQUEUE_PI = 12,
        LOCK_PI2 = 13,
    }
}

bitflags::bitflags! {
    #[allow(missing_docs)]
    #[derive(PartialEq, Eq, Debug)]
    /// Futex flags for posix syscall listed in <https://manpages.debian.org/unstable/manpages-dev/futex.2.en.html#FUTEX_WAIT>
    pub struct FutexFlags: i32 {
        /// Futex is shared with other processes
        const SHARED = 0x10;
        /// Futex is process-private and not shared with another process
        const PRIVATE = 128;
        /// Futex is associated with CLOCK_REALTIME
        const CLOCK_REALTIME = 256;
        /// If the futex contains this flag,it will matche any bitset value
        const BITSET_MATCH_ANY = -1;
    }
}
