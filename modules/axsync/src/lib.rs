//! [ArceOS](https://github.com/rcore-os/arceos) synchronization primitives.
//!
//! Currently supported primitives:
//!
//! - [`Mutex`]: A mutual exclusion primitive.
//! - mod [`spin`](spinlock): spin-locks.
//!
//! # Cargo Features
//!
//! - `multitask`: For use in the multi-threaded environments. If the feature is
//!   not enabled, [`Mutex`] will be an alias of [`spin::SpinNoIrq`]. This
//!   feature is enabled by default.

#![cfg_attr(not(test), no_std)]
#![feature(doc_cfg)]
cfg_if::cfg_if! {
    if #[cfg(feature = "multitask")] {
        extern crate axtask;
        extern crate alloc;

        mod mutex;
        mod completion;

        pub use self::completion::Completion;

        #[doc(cfg(feature = "multitask"))]
        pub use self::mutex::{Mutex, MutexGuard};
    } else {
        #[doc(cfg(not(feature = "multitask")))]
        pub use spinlock::{SpinNoIrq as Mutex, SpinNoIrqGuard as MutexGuard};
    }
}

pub use spinlock as spin;
