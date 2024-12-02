//! 文件系统相关系统调用

pub mod ctype;
pub mod imp;

use crate::SyscallResult;
use axerrno::AxResult;
use axfs::api::{File, OpenFlags};
pub use ctype::FileDesc;
mod fs_syscall_id;
pub use fs_syscall_id::FsSyscallId::{self, *};
extern crate alloc;
use imp::*;

/// 若使用多次new file打开同名文件，那么不同new file之间读写指针不共享，但是修改的内容是共享的
pub fn new_file(path: &str, flags: &OpenFlags) -> AxResult<File> {
    let mut file = File::options();
    file.read(flags.readable());
    file.write(flags.writable());
    file.create(flags.creatable());
    file.create_new(flags.new_creatable());
    file.open(path)
}

/// 文件系统相关系统调用
pub fn fs_syscall(syscall_id: fs_syscall_id::FsSyscallId, args: [usize; 6]) -> SyscallResult {
    match syscall_id {
        OPENAT => syscall_openat(args),
        CLOSE => syscall_close(args),
        READ => syscall_read(args),
        WRITE => syscall_write(args),
        GETCWD => syscall_getcwd(args),
        PIPE2 => syscall_pipe2(args),
        DUP => syscall_dup(args),
        DUP3 => syscall_dup3(args),
        MKDIRAT => syscall_mkdirat(args),
        CHDIR => syscall_chdir(args),
        GETDENTS64 => syscall_getdents64(args),
        MOUNT => syscall_mount(args),
        UNMOUNT => syscall_umount(args),
        FSTAT => syscall_fstat(args),
        RENAMEAT | RENAMEAT2 => syscall_renameat2(args),
        READV => syscall_readv(args),
        WRITEV => syscall_writev(args),
        FCNTL64 => syscall_fcntl64(args),
        FSTATAT => syscall_fstatat(args),
        STATFS => syscall_statfs(args),
        FCHMODAT => syscall_fchmodat(args),
        FACCESSAT => syscall_faccessat(args),
        LSEEK => syscall_lseek(args),
        PREAD64 => syscall_pread64(args),
        PREADLINKAT => syscall_readlinkat(args),
        PWRITE64 => syscall_pwrite64(args),
        SENDFILE64 => syscall_sendfile64(args),
        FSYNC => Ok(0),
        FTRUNCATE64 => {
            syscall_ftruncate64(args)
            // 0
        }
        IOCTL => syscall_ioctl(args),
        // 不做处理即可
        SYNC => Ok(0),
        COPYFILERANGE => syscall_copyfilerange(args),
        LINKAT => sys_linkat(args),
        UNLINKAT => syscall_unlinkat(args),
        SYMLINKAT => Ok(0),
        UTIMENSAT => syscall_utimensat(args),
        EPOLL_CREATE => syscall_epoll_create1(args),
        EPOLL_CTL => syscall_epoll_ctl(args),
        EPOLL_PWAIT => syscall_epoll_pwait(args),
        PPOLL => syscall_ppoll(args),
        PSELECT6 => syscall_pselect6(args),
        STATX => syscall_statx(args),
        PIDFD_OPEN => syscall_pidfd_open(args),
        FCHOWN => Ok(0),
        #[cfg(not(target_arch = "x86_64"))]
        EVENTFD => syscall_eventfd(args),
        #[cfg(target_arch = "x86_64")]
        // eventfd syscall in x86_64 does not support flags, use 0 instead
        EVENTFD => syscall_eventfd([args[0], 0, 0, 0, 0, 0]),
        #[cfg(target_arch = "x86_64")]
        EVENTFD2 => syscall_eventfd(args),
        #[cfg(target_arch = "x86_64")]
        DUP2 => syscall_dup2(args),
        #[cfg(target_arch = "x86_64")]
        LSTAT => syscall_lstat(args),
        #[cfg(target_arch = "x86_64")]
        OPEN => syscall_open(args),
        #[cfg(target_arch = "x86_64")]
        PIPE => syscall_pipe(args),
        #[cfg(target_arch = "x86_64")]
        POLL => syscall_poll(args),
        #[cfg(target_arch = "x86_64")]
        STAT => syscall_stat(args),
        #[cfg(target_arch = "x86_64")]
        UNLINK => syscall_unlink(args),
        #[cfg(target_arch = "x86_64")]
        ACCESS => syscall_access(args),
        #[cfg(target_arch = "x86_64")]
        MKDIR => syscall_mkdir(args),
        #[cfg(target_arch = "x86_64")]
        RENAME => syscall_rename(args),
        #[cfg(target_arch = "x86_64")]
        RMDIR => syscall_rmdir(args),
        #[cfg(target_arch = "x86_64")]
        SELECT => syscall_select(args),
        #[cfg(target_arch = "x86_64")]
        READLINK => syscall_readlink(args),
        #[cfg(target_arch = "x86_64")]
        CREAT => syscall_creat(args),
        #[cfg(target_arch = "x86_64")]
        EPOLL_CREATE1 => syscall_epoll_create1(args),
        // EPOLL_CREATE1 => unimplemented!("epoll_create1"),
        #[cfg(target_arch = "x86_64")]
        EPOLL_WAIT => syscall_epoll_wait(args),
        // EPOLL_PWAIT => unimplemented!("epoll_ctl"),
        #[cfg(target_arch = "x86_64")]
        CHMOD => Ok(0),
        #[cfg(target_arch = "x86_64")]
        CHOWN => Ok(0),
        #[cfg(target_arch = "x86_64")]
        MKNOD => Ok(0),
    }
}
