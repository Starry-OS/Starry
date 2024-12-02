//! Define the trap handler for the whole kernel
pub use axhal::{mem::VirtAddr, paging::MappingFlags};

use crate::syscall::syscall;

fn time_stat_from_kernel_to_user() {
    axprocess::time_stat_from_kernel_to_user();
}

fn time_stat_from_user_to_kernel() {
    axprocess::time_stat_from_user_to_kernel();
}

/// Handle the interrupt
///
/// # Arguments
///
/// * `irq_num` - The number of the interrupt
///
/// * `from_user` - Whether the interrupt is from user space
pub fn handle_irq(irq_num: usize, from_user: bool) {
    // trap进来，统计时间信息
    // 只有当trap是来自用户态才进行统计
    if from_user {
        time_stat_from_user_to_kernel();
    }
    axhal::irq::dispatch_irq(irq_num);
    if from_user {
        time_stat_from_kernel_to_user();
    }
}

/// Handle the syscall
///
/// # Arguments
///
/// * `syscall_id` - The id of the syscall
///
/// * `args` - The arguments of the syscall
pub fn handle_syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    time_stat_from_user_to_kernel();
    let ans = syscall(syscall_id, args);
    time_stat_from_kernel_to_user();
    ans
}

/// Handle the page fault exception
///
/// # Arguments
///
/// * `addr` - The address where the page fault occurs
///
/// * `flags` - The permission which the page fault needs
pub fn handle_page_fault(addr: VirtAddr, flags: MappingFlags) {
    time_stat_from_user_to_kernel();
    axprocess::handle_page_fault(addr, flags);
    time_stat_from_kernel_to_user();
}

/// To handle the pending signals for current process
pub fn handle_signals() {
    time_stat_from_user_to_kernel();
    axprocess::signal::handle_signals();
    time_stat_from_kernel_to_user();
}

/// Record the occurrence of a syscall
pub fn record_trap(syscall_code: usize) {
    axfs::axfs_ramfs::INTERRUPT.lock().record(syscall_code);
}
