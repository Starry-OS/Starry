use axerrno::{AxError, AxResult};
use axfs::api::port::{
    ConsoleWinSize, FileExt, FileIO, FileIOType, OpenFlags, FIOCLEX, TCGETS, TIOCGPGRP, TIOCGWINSZ,
    TIOCSPGRP,
};
use axhal::console::{getchar, putchar, write_bytes};
use axio::{Read, Seek, SeekFrom, Write};
use axlog::warn;
use axsync::Mutex;
use axtask::yield_now;

extern crate alloc;
use alloc::string::String;
/// stdin file for getting chars from console
pub struct Stdin {
    pub flags: Mutex<OpenFlags>,
}

/// stdout file for putting chars to console
pub struct Stdout {
    pub flags: Mutex<OpenFlags>,
}

/// stderr file for putting chars to console
pub struct Stderr {
    #[allow(unused)]
    pub flags: Mutex<OpenFlags>,
}

pub const LF: u8 = 0x0au8;
pub const CR: u8 = 0x0du8;
pub const DL: u8 = 0x7fu8;
pub const BS: u8 = 0x08u8;

pub const SPACE: u8 = 0x20u8;

pub const BACKSPACE: [u8; 3] = [BS, SPACE, BS];

fn stdin_read(buf: &mut [u8]) -> AxResult<usize> {
    let ch: u8;
    // busybox
    if buf.len() == 1 {
        loop {
            match getchar() {
                Some(c) => {
                    ch = c;
                    break;
                }
                None => {
                    yield_now();
                    continue;
                }
            }
        }
        unsafe {
            buf.as_mut_ptr().write_volatile(ch);
        }
        Ok(1)
    } 
    else {
        // user appilcation
        let mut line = String::new();
        loop {
            let c = getchar();
            if let Some(c) = c {
                match c {
                    LF | CR => {
                        // convert '\r' to '\n'
                        line.push('\n');
                        putchar(b'\n');
                        break;
                    }
                    BS | DL => {
                        if !line.is_empty() {
                            write_bytes(&BACKSPACE);
                            line.pop();
                        }
                    }
                    _ => {
                        // echo 
                        putchar(c);
                        line.push(c as char);
                    }
                }
            } else {
                yield_now();
                continue;
            }
        }
        let len = line.len();
        buf[..len].copy_from_slice(line.as_bytes());
        Ok(len)
    }
}

fn stdout_write(buf: &[u8]) -> AxResult<usize> {
    write_bytes(buf);
    Ok(buf.len())
}

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> AxResult<usize> {
        stdin_read(buf)
    }
}

impl Write for Stdin {
    fn write(&mut self, _: &[u8]) -> AxResult<usize> {
        panic!("Cannot write to stdin!");
    }
    fn flush(&mut self) -> axio::Result {
        panic!("Flushing stdin")
    }
}

impl Seek for Stdin {
    fn seek(&mut self, _pos: SeekFrom) -> AxResult<u64> {
        Err(AxError::Unsupported) // 如果没有实现seek, 则返回Unsupported
    }
}

impl FileExt for Stdin {
    fn executable(&self) -> bool {
        false
    }
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
}

impl FileIO for Stdin {
    fn read(&self, buf: &mut [u8]) -> AxResult<usize> {
        stdin_read(buf)
    }

    fn get_type(&self) -> FileIOType {
        FileIOType::Stdin
    }

    fn ready_to_read(&self) -> bool {
        true
    }

    fn ready_to_write(&self) -> bool {
        false
    }

    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        false
    }

    fn executable(&self) -> bool {
        false
    }

    fn ioctl(&self, request: usize, data: usize) -> AxResult<isize> {
        match request {
            TIOCGWINSZ => {
                let winsize = data as *mut ConsoleWinSize;
                unsafe {
                    *winsize = ConsoleWinSize::default();
                }
                Ok(0)
            }
            TCGETS | TIOCSPGRP => {
                warn!("stdin TCGETS | TIOCSPGRP, pretend to be tty.");
                // pretend to be tty
                Ok(0)
            }

            TIOCGPGRP => {
                warn!("stdin TIOCGPGRP, pretend to be have a tty process group.");
                unsafe {
                    *(data as *mut u32) = 0;
                }
                Ok(0)
            }
            FIOCLEX => Ok(0),
            _ => Err(AxError::Unsupported),
        }
    }

    fn set_status(&self, flags: OpenFlags) -> bool {
        if flags.contains(OpenFlags::CLOEXEC) {
            *self.flags.lock() = OpenFlags::CLOEXEC;
            true
        } else {
            false
        }
    }

    fn get_status(&self) -> OpenFlags {
        *self.flags.lock()
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

impl Read for Stdout {
    fn read(&mut self, _: &mut [u8]) -> AxResult<usize> {
        panic!("Cannot read from stdin!");
    }
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> AxResult<usize> {
        write_bytes(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> AxResult {
        // stdout is always flushed
        Ok(())
    }
}

impl Seek for Stdout {
    fn seek(&mut self, _pos: SeekFrom) -> AxResult<u64> {
        Err(AxError::Unsupported) // 如果没有实现seek, 则返回Unsupported
    }
}

impl FileExt for Stdout {
    fn readable(&self) -> bool {
        false
    }
    fn writable(&self) -> bool {
        true
    }
    fn executable(&self) -> bool {
        false
    }
}
impl FileIO for Stdout {
    fn write(&self, buf: &[u8]) -> AxResult<usize> {
        stdout_write(buf)
    }

    fn flush(&self) -> AxResult {
        // stdout is always flushed
        Ok(())
    }

    fn readable(&self) -> bool {
        false
    }

    fn writable(&self) -> bool {
        true
    }

    fn executable(&self) -> bool {
        false
    }

    fn get_type(&self) -> FileIOType {
        FileIOType::Stdout
    }

    fn ready_to_read(&self) -> bool {
        false
    }

    fn ready_to_write(&self) -> bool {
        true
    }

    fn set_status(&self, flags: OpenFlags) -> bool {
        if flags.contains(OpenFlags::CLOEXEC) {
            *self.flags.lock() = flags;
            true
        } else {
            false
        }
    }

    fn get_status(&self) -> OpenFlags {
        *self.flags.lock()
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

    fn ioctl(&self, request: usize, data: usize) -> AxResult<isize> {
        match request {
            TIOCGWINSZ => {
                let winsize = data as *mut ConsoleWinSize;
                unsafe {
                    *winsize = ConsoleWinSize::default();
                }
                Ok(0)
            }
            TCGETS | TIOCSPGRP => {
                warn!("stdout TCGETS | TIOCSPGRP, pretend to be tty.");
                // pretend to be tty
                Ok(0)
            }

            TIOCGPGRP => {
                warn!("stdout TIOCGPGRP, pretend to be have a tty process group.");
                unsafe {
                    *(data as *mut u32) = 0;
                }
                Ok(0)
            }
            FIOCLEX => Ok(0),
            _ => Err(AxError::Unsupported),
        }
    }
}

impl Read for Stderr {
    fn read(&mut self, _: &mut [u8]) -> AxResult<usize> {
        panic!("Cannot read from stdout!");
    }
}

impl Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> AxResult<usize> {
        write_bytes(buf);
        Ok(buf.len())
    }

    /// Stderr is always flushed
    fn flush(&mut self) -> axio::Result {
        Ok(())
    }
}

impl Seek for Stderr {
    fn seek(&mut self, _pos: SeekFrom) -> AxResult<u64> {
        Err(AxError::Unsupported) // 如果没有实现seek, 则返回Unsupported
    }
}

impl FileExt for Stderr {
    fn readable(&self) -> bool {
        false
    }
    fn writable(&self) -> bool {
        true
    }
    fn executable(&self) -> bool {
        false
    }
}

impl FileIO for Stderr {
    fn write(&self, buf: &[u8]) -> AxResult<usize> {
        write_bytes(buf);
        Ok(buf.len())
    }

    /// Stderr is always flushed
    fn flush(&self) -> axio::Result {
        Ok(())
    }

    fn readable(&self) -> bool {
        false
    }

    fn writable(&self) -> bool {
        true
    }

    fn executable(&self) -> bool {
        false
    }

    fn get_type(&self) -> FileIOType {
        FileIOType::Stderr
    }

    fn ready_to_read(&self) -> bool {
        false
    }

    fn ready_to_write(&self) -> bool {
        true
    }

    fn ioctl(&self, request: usize, data: usize) -> AxResult<isize> {
        match request {
            TIOCGWINSZ => {
                let winsize = data as *mut ConsoleWinSize;
                unsafe {
                    *winsize = ConsoleWinSize::default();
                }
                Ok(0)
            }
            TCGETS | TIOCSPGRP => {
                warn!("stderr TCGETS | TIOCSPGRP, pretend to be tty.");
                // pretend to be tty
                Ok(0)
            }

            TIOCGPGRP => {
                warn!("stderr TIOCGPGRP, pretend to be have a tty process group.");
                unsafe {
                    *(data as *mut u32) = 0;
                }
                Ok(0)
            }
            FIOCLEX => Ok(0),
            _ => Err(AxError::Unsupported),
        }
    }
}
