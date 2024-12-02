use crate::futex::{FutexKey, FutexQ};
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use axlog::info;

use crate::jhash::jhash2;
use axsync::Mutex;
use lazy_static::lazy_static;

/// the number of hash buckets, must be a power of 2
const FUTEX_HASH_SIZE: usize = 256;

lazy_static! {
    /// A global futex queues, which stores all futex queues
    /// It only holds the mutex through `futex_hash_bucket`
    pub static ref FUTEXQUEUES: FutexQueues = {
        info!("Initializing futex queues");
        FutexQueues::new(FUTEX_HASH_SIZE)
    };
}

#[allow(unused)]
/// print all futex_q that are in the FUTEXQUEUES
pub fn display_futexqueues() {
    axlog::warn!("[display_futexqueues]");
    for i in 0..FUTEX_HASH_SIZE {
        let hash_bucket = FUTEXQUEUES.buckets[i].lock();
        if !hash_bucket.is_empty() {
            for futex_q in hash_bucket.iter() {
                axlog::warn!(
                    "task {} is still wait for {:?}",
                    futex_q.task.id().as_u64(),
                    futex_q.key
                );
            }
        }
        drop(hash_bucket);
    }
}

/// the outer vector is the bucket, the inner vector is the futex queue
pub struct FutexQueues {
    /// buckets of futex queues
    pub buckets: Box<[Mutex<VecDeque<FutexQ>>]>,
}

impl FutexQueues {
    fn new(size: usize) -> Self {
        let mut buckets = Vec::with_capacity(size);
        for _ in 0..size {
            buckets.push(Mutex::new(VecDeque::new()));
        }
        Self {
            buckets: buckets.into_boxed_slice(),
        }
    }
}

/// Hash a [`FutexKey`] to a bucket index
pub fn futex_hash(futex_key: &FutexKey) -> usize {
    let key = &[futex_key.pid, futex_key.aligned, futex_key.offset];
    let hash = jhash2(key, key[2]);
    hash as usize & (FUTEX_HASH_SIZE - 1)
}
