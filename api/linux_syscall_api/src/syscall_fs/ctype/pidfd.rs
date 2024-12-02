extern crate alloc;
use alloc::sync::Arc;
use axfs::api::{FileIO, OpenFlags};
use axprocess::{current_process, Process, PID2PC};
use axsync::Mutex;

use crate::{SyscallError, SyscallResult};

pub struct PidFd {
    flags: Mutex<OpenFlags>,
    process: Arc<Process>,
}

impl PidFd {
    /// Create a new PidFd
    pub fn new(process: Arc<Process>, flags: OpenFlags) -> Self {
        Self {
            flags: Mutex::new(flags),
            process,
        }
    }

    pub fn pid(&self) -> u64 {
        self.process.pid()
    }
}

impl FileIO for PidFd {
    fn read(&self, _buf: &mut [u8]) -> axerrno::AxResult<usize> {
        Err(axerrno::AxError::Unsupported)
    }
    fn write(&self, _buf: &[u8]) -> axerrno::AxResult<usize> {
        Err(axerrno::AxError::Unsupported)
    }
    fn seek(&self, _pos: axfs::api::SeekFrom) -> axerrno::AxResult<u64> {
        Err(axerrno::AxError::Unsupported)
    }
    /// To check whether the target process is still alive
    fn readable(&self) -> bool {
        self.process.get_zombie()
    }

    fn writable(&self) -> bool {
        false
    }

    fn executable(&self) -> bool {
        false
    }

    fn get_type(&self) -> axfs::api::FileIOType {
        axfs::api::FileIOType::Other
    }

    fn get_status(&self) -> OpenFlags {
        *self.flags.lock()
    }

    fn set_status(&self, flags: OpenFlags) -> bool {
        *self.flags.lock() = flags;
        true
    }

    fn set_close_on_exec(&self, is_set: bool) -> bool {
        if is_set {
            // 设置close_on_exec位置
            *self.flags.lock() |= OpenFlags::CLOEXEC;
        } else {
            *self.flags.lock() &= !OpenFlags::CLOEXEC;
        }
        true
    }
}

pub fn new_pidfd(pid: u64, mut flags: OpenFlags) -> SyscallResult {
    // It is set to close the file descriptor on exec
    flags |= OpenFlags::CLOEXEC;
    let pid2fd = PID2PC.lock();

    let pidfd = pid2fd
        .get(&pid)
        .map(|target_process| PidFd::new(Arc::clone(target_process), flags))
        .ok_or(SyscallError::EINVAL)?;
    drop(pid2fd);
    let process = current_process();
    let mut fd_table = process.fd_manager.fd_table.lock();
    let fd = process
        .alloc_fd(&mut fd_table)
        .map_err(|_| SyscallError::EMFILE)?;
    fd_table[fd] = Some(Arc::new(pidfd));
    Ok(fd as isize)
}
