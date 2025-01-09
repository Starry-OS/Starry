//! 提供和 task 模块相关的 syscall

mod task_syscall_id;

use crate::SyscallResult;
pub use task_syscall_id::TaskSyscallId::{self, *};

mod imp;

pub use imp::*;

/// 进行 syscall 的分发
pub fn task_syscall(syscall_id: task_syscall_id::TaskSyscallId, args: [usize; 6]) -> SyscallResult {
    match syscall_id {
        EXIT => syscall_exit(args),
        EXECVE => syscall_exec(args),
        CLONE => syscall_clone(args),
        CLONE3 => syscall_clone3(args),
        NANO_SLEEP => syscall_sleep(args),
        SCHED_YIELD => syscall_yield(),
        TIMES => syscall_time(args),
        UNAME => syscall_uname(args),
        GETTIMEOFDAY => syscall_get_time_of_day(args),
        GETPGID => syscall_getpgid(),
        SETPGID => syscall_setpgid(args),
        GETPID => syscall_getpid(),

        GETPPID => syscall_getppid(),
        WAIT4 => syscall_wait4(args),
        GETRANDOM => syscall_getrandom(args),

        SIGSUSPEND => syscall_sigsuspend(args),

        SIGACTION => syscall_sigaction(args),

        KILL => syscall_kill(args),

        TKILL => syscall_tkill(args),

        TGKILL => syscall_tgkill(args),

        SIGPROCMASK => syscall_sigprocmask(args),
        SIGALTSTACK => syscall_sigaltstack(args),
        SIGRETURN => syscall_sigreturn(),
        EXIT_GROUP => syscall_exit(args),
        SET_TID_ADDRESS => syscall_set_tid_address(args),
        PRLIMIT64 => syscall_prlimit64(args),
        CLOCK_GET_TIME => syscall_clock_get_time(args),
        GETUID => syscall_getuid(),
        GETEUID => syscall_geteuid(),
        GETGID => syscall_getgid(),
        SETGID => Ok(0),
        GETEGID => syscall_getegid(),
        GETTID => syscall_gettid(),
        FUTEX => syscall_futex(args),
        SET_ROBUST_LIST => syscall_set_robust_list(args),
        GET_ROBUST_LIST => syscall_get_robust_list(args),
        SYSINFO => syscall_sysinfo(args),
        SETITIMER => syscall_settimer(args),
        GETTIMER => syscall_gettimer(args),
        SETSID => syscall_setsid(),
        GETRUSAGE => syscall_getrusage(args),
        UMASK => syscall_umask(args),
        // 不做处理即可
        SIGTIMEDWAIT => Ok(0),
        SYSLOG => Ok(0),
        MADVICE => Ok(0),
        SCHED_SETAFFINITY => Ok(0),
        SCHED_GETAFFINITY => syscall_sched_getaffinity(args),
        SCHED_SETSCHEDULER => syscall_sched_setscheduler(args),
        SCHED_GETSCHEDULER => syscall_sched_getscheduler(args),
        #[cfg(target_arch = "x86_64")]
        SCHED_GET_PRORITY_MAX => syscall_sched_getscheduler_max(args),
        #[cfg(target_arch = "x86_64")]
        SCHED_GET_PRORITY_MIN => syscall_sched_getscheduler_min(args),
        GET_MEMPOLICY => Ok(0),
        CLOCK_GETRES => syscall_clock_getres(args),
        CLOCK_NANOSLEEP => syscall_clock_nanosleep(args),
        PRCTL => syscall_prctl(args),
        PIDFD_SEND_SIGNAL => syscall_pidfd_send_signal(args),
        // syscall below just for x86_64
        #[cfg(target_arch = "x86_64")]
        VFORK => syscall_vfork(),
        #[cfg(target_arch = "x86_64")]
        ARCH_PRCTL => syscall_arch_prctl(args),
        #[cfg(target_arch = "x86_64")]
        FORK => syscall_fork(),
        #[cfg(target_arch = "x86_64")]
        ALARM => Ok(0),
        #[cfg(target_arch = "x86_64")]
        RSEQ => Ok(0),
        #[cfg(target_arch = "x86_64")]
        TIME => Ok(0),
        #[allow(unused)]
        _ => {
            panic!("Invalid Syscall Id: {:?}!", syscall_id);
            // return -1;
            // exit(-1)
        }
    }
}
