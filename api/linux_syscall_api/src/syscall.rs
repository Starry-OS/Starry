use crate::{deal_result, SyscallResult};
use axlog::info;

#[no_mangle]
pub fn syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    #[allow(unused_mut, unused_assignments)]
    let mut ans: Option<SyscallResult> = None;

    if let Ok(net_syscall_id) = crate::syscall_net::NetSyscallId::try_from(syscall_id) {
        info!(
            "[syscall] id = {:#?}, args = {:?}, entry",
            net_syscall_id, args
        );

        (#[allow(unused_assignments)]
        ans) = Some(crate::syscall_net::net_syscall(net_syscall_id, args));
    }

    if let Ok(mem_syscall_id) = crate::syscall_mem::MemSyscallId::try_from(syscall_id) {
        info!(
            "[syscall] id = {:#?}, args = {:?}, entry",
            mem_syscall_id, args
        );
        (#[allow(unused_assignments)]
        ans) = Some(crate::syscall_mem::mem_syscall(mem_syscall_id, args));
    }

    if let Ok(fs_syscall_id) = crate::syscall_fs::FsSyscallId::try_from(syscall_id) {
        if syscall_id != 281 {
            info!(
                "[syscall] id = {:#?}, args = {:?}, entry",
                fs_syscall_id, args
            );
        }

        (#[allow(unused_assignments)]
        ans) = Some(crate::syscall_fs::fs_syscall(fs_syscall_id, args));
    }

    if let Ok(task_syscall_id) = crate::syscall_task::TaskSyscallId::try_from(syscall_id) {
        if syscall_id != 228 {
            info!(
                "[syscall] id = {:#?}, args = {:?}, entry",
                task_syscall_id, args
            );
        }

        (#[allow(unused_assignments)]
        ans) = Some(crate::syscall_task::task_syscall(task_syscall_id, args));
    }

    if ans.is_none() {
        panic!("unknown syscall id: {}", syscall_id);
    }
    let ans = deal_result(ans.unwrap());
    if syscall_id != 281 && syscall_id != 228 {
        info!("[syscall] id = {},return {}", syscall_id, ans);
    }
    ans
}
