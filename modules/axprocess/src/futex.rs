//! 实现与futex相关的系统调用
use crate::{current_process, current_task, signal::current_have_signals, yield_now_task};
use axerrno::LinuxError;
use axfutex::{flags::FutexFlags, futex_hash, FutexKey, FutexQ, FUTEXQUEUES};
use axhal::mem::VirtAddr;
use axlog::info;
use core::time::Duration;
//use axtask::WaitQueue;

extern crate alloc;

type AxSyscallResult = Result<isize, axerrno::LinuxError>;

/// waiting queue which stores tasks waiting for futex variable
//pub static WAIT_FOR_FUTEX: WaitQueue = WaitQueue::new();

#[derive(Default)]
/// 用于存储 robust list 的结构
pub struct FutexRobustList {
    /// The location of the head of the robust list in user space
    pub head: usize,
    /// The length of the robust list
    pub len: usize,
}

impl FutexRobustList {
    /// Create a new robust list
    pub fn new(head: usize, len: usize) -> Self {
        Self { head, len }
    }
}

fn futex_get_value_locked(vaddr: VirtAddr) -> AxSyscallResult {
    let process = current_process();
    if process.manual_alloc_for_lazy(vaddr).is_ok() {
        let real_futex_val = unsafe { (vaddr.as_usize() as *const u32).read_volatile() };
        Ok(real_futex_val as isize)
    } else {
        Err(LinuxError::EFAULT)
    }
}

fn get_futex_key(uaddr: VirtAddr, flags: &FutexFlags) -> FutexKey {
    if flags.contains(FutexFlags::SHARED) {
        /* Todo: after implement inode layer
        let inode = uaddr.get_inode();
        let page_index = uaddr.get_page_index();
        let offset = uaddr.get_offset();
        FutexKey::new(inode, page_index, offset)
        */
        // TODO: Distinguishing between anonymous and file mappings
        let pid = 0;
        let aligned = uaddr.align_down_4k().as_usize();
        let offset = uaddr.align_offset_4k() as u32;
        FutexKey::new(pid, aligned, offset)
    } else {
        let pid = current_process().pid();
        let aligned = uaddr.align_down_4k().as_usize();
        let offset = uaddr.align_offset_4k() as u32;
        FutexKey::new(pid, aligned, offset)
    }
}

/// Wait on a futex variable
///
/// Details: <https://manpages.debian.org/unstable/manpages-dev/futex.2.en.html#FUTEX_WAIT>
pub fn futex_wait(
    vaddr: VirtAddr,
    flags: FutexFlags,
    expected_val: u32,
    deadline: Option<Duration>,
    bitset: u32,
) -> AxSyscallResult {
    info!(
        "[futex_wait] current task: {:?}, vaddr: {:?}, flags: {:?}, val: {:?}, deadline: {:?}",
        current_task().id(),
        vaddr,
        flags,
        expected_val,
        deadline
    );
    let mut is_timeout = false;

    // we may be victim of spurious wakeups, so we need to loop
    loop {
        let key = get_futex_key(vaddr, &flags);
        let real_futex_val = futex_get_value_locked(vaddr)?;
        if expected_val != real_futex_val as u32 {
            return Err(LinuxError::EAGAIN);
        }
        // 比较后相等，放入等待队列
        let mut hash_bucket = FUTEXQUEUES.buckets[futex_hash(&key)].lock();
        let cur_futexq = FutexQ::new(key, current_task().as_task_ref().clone(), bitset);
        hash_bucket.push_back(cur_futexq);

        // drop lock to avoid deadlock
        drop(hash_bucket);

        /* There is something wrong with WaitQueues, tasks are woken up unexpectedly
        if let Some(deadline) = deadline {
            // There is something wrong with WaitQueues
            is_tiemout = WAIT_FOR_FUTEX.wait_timeout(deadline);
        }
        else {
            // If timeout is NULL, the operation can block indefinitely.
            yield_now_task();
        }
        */

        if let Some(deadline) = deadline {
            let now = axhal::time::current_time();
            is_timeout = deadline < now;
        }
        if deadline.is_none() || !is_timeout {
            yield_now_task();
        }
        // If we were woken (and unqueued), we succeeded, whatever.
        // We doesn't care about the reason of wakeup if we were unqueued.
        let mut hash_bucket = FUTEXQUEUES.buckets[futex_hash(&key)].lock();
        let cur_id = current_task().id().as_u64();
        //if let Some(idx) = hash_bucket.iter().position(|futex_q| futex_q.task.id().as_u64() == cur_id) {
        if let Some(idx) = hash_bucket
            .iter()
            .position(|futex_q| futex_q.task.id().as_u64() == cur_id)
        {
            hash_bucket.remove(idx);
            if is_timeout {
                return Err(LinuxError::ETIMEDOUT);
            }
            if current_have_signals() {
                return Err(LinuxError::EINTR);
            }
        } else {
            // the task is woken up anyway
            return Ok(0);
        }
    }
}

/// Wake up tasks waiting on a futex variable
///
/// Details: <https://manpages.debian.org/unstable/manpages-dev/futex.2.en.html#FUTEX_WAKE>
///
/// Tips: There is no need to check the bitset, faster than futex_wake_bitset
pub fn futex_wake(vaddr: VirtAddr, flags: FutexFlags, nr_waken: u32) -> AxSyscallResult {
    info!(
        "[futex_wake] vaddr: {:?}, flags: {:?}, nr_waken: {:?}",
        vaddr, flags, nr_waken
    );
    let mut ret = 0;
    let key = get_futex_key(vaddr, &flags);
    let mut hash_bucket = FUTEXQUEUES.buckets[futex_hash(&key)].lock();

    if hash_bucket.is_empty() {
        info!("hash_bucket is empty");
        return Ok(0);
    } else {
        hash_bucket.retain(|futex_q| {
            if ret < nr_waken && futex_q.key == key {
                //let wake_up = WAIT_FOR_FUTEX.notify_task(&futex_q.task);
                info!("wake up task {:?}", futex_q.task.id());
                ret += 1;
                return false;
            }
            true
        })
    }
    // drop hash_bucket to avoid deadlock
    drop(hash_bucket);
    Ok(ret as isize)
}

/// Wake up tasks specified by a bitset waiting on a futex variable
pub fn futex_wake_bitset(
    vaddr: VirtAddr,
    flags: FutexFlags,
    nr_waken: u32,
    bitset: u32,
) -> AxSyscallResult {
    info!(
        "[futex_wake_bitset] vaddr: {:?}, flags: {:?}, nr_waken: {:?}, bitset: {:x}",
        vaddr, flags, nr_waken, bitset
    );
    if bitset == 0 {
        return Err(LinuxError::EINVAL);
    }
    let mut ret = 0;
    let key = get_futex_key(vaddr, &flags);
    let mut hash_bucket = FUTEXQUEUES.buckets[futex_hash(&key)].lock();
    if hash_bucket.is_empty() {
        return Ok(0);
    } else {
        hash_bucket.retain(|futex_q| {
            if ret == nr_waken {
                return true;
            }
            if (futex_q.bitset & bitset) != 0 && futex_q.key == key {
                //WAIT_FOR_FUTEX.notify_task(&futex_q.task);
                ret += 1;
                return false;
            }
            true
        })
    }
    // drop hash_bucket to avoid deadlock
    drop(hash_bucket);
    Ok(ret as isize)
}

/// Requeue tasks waiting on a futex variable
pub fn futex_requeue(
    uaddr: VirtAddr,
    flags: FutexFlags,
    nr_waken: u32,
    uaddr2: VirtAddr,
    nr_requeue: u32,
) -> AxSyscallResult {
    let mut ret = 0;
    let mut requeued = 0;
    let key = get_futex_key(uaddr, &flags);
    let req_key = get_futex_key(uaddr2, &flags);

    if key == req_key {
        return futex_wake(uaddr, flags, nr_waken);
    }

    let mut hash_bucket = FUTEXQUEUES.buckets[futex_hash(&key)].lock();
    if hash_bucket.is_empty() {
        return Ok(0);
    } else {
        while let Some(futex_q) = hash_bucket.pop_front() {
            if futex_q.key == key {
                //WAIT_FOR_FUTEX.notify_task(&futex_q.task);
                ret += 1;
                if ret == nr_waken {
                    break;
                }
            }
        }
        if hash_bucket.is_empty() {
            return Ok(ret as isize);
        }
        // requeue the rest of the waiters
        let mut req_bucket = FUTEXQUEUES.buckets[futex_hash(&req_key)].lock();
        while let Some(futex_q) = hash_bucket.pop_front() {
            req_bucket.push_back(futex_q);
            requeued += 1;
            if requeued == nr_requeue {
                break;
            }
        }
    }
    drop(hash_bucket);
    Ok(ret as isize)
}
