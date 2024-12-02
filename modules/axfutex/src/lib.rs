//! A module to implement futexes with jhash algorithm.
//!
//! Futex is a fast userspace mutex, which is used to implement synchronization primitives.
#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(stmt_expr_attributes)]

extern crate alloc;

pub mod flags;
mod futex;
mod jhash;
mod queues;
pub use queues::{futex_hash, FUTEXQUEUES};

pub use futex::{FutexKey, FutexQ};
