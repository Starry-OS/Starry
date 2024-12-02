//! 相关系统调用的具体实现
extern crate alloc;
use super::socket::*;
use core::slice::{from_raw_parts, from_raw_parts_mut};

use alloc::sync::Arc;
use axfs::api::{FileIO, OpenFlags};

use crate::{syscall_fs::ctype::pipe::make_pipe, MessageHeader, SyscallError, SyscallResult};
use axerrno::AxError;
use axlog::{debug, error, info, warn};

use axprocess::current_process;
use num_enum::TryFromPrimitive;

pub const SOCKET_TYPE_MASK: usize = 0xFF;

/// # Arguments
/// * `domain` - usize
/// * `s_type` - usize
/// * `protocol` - usize
pub fn syscall_socket(args: [usize; 6]) -> SyscallResult {
    let domain = args[0];
    let s_type = args[1];
    let _protocol = args[2];
    let Ok(domain) = Domain::try_from(domain) else {
        error!("[socket()] Address Family not supported: {domain}");
        // return ErrorNo::EAFNOSUPPORT as isize;
        return Err(SyscallError::EAFNOSUPPORT);
    };
    let Ok(socket_type) = SocketType::try_from(s_type & SOCKET_TYPE_MASK) else {
        // return ErrorNo::EINVAL as isize;
        return Err(SyscallError::EINVAL);
    };
    let socket = Socket::new(domain, socket_type);
    socket.set_nonblocking((s_type & SOCK_NONBLOCK) != 0);
    socket.set_close_on_exec((s_type & SOCK_CLOEXEC) != 0);
    let curr = current_process();
    let mut fd_table = curr.fd_manager.fd_table.lock();
    let Ok(fd) = curr.alloc_fd(&mut fd_table) else {
        return Err(SyscallError::EMFILE);
    };

    fd_table[fd] = Some(Arc::new(socket));

    debug!("[socket()] create socket {fd}");

    Ok(fd as isize)
}

/// # Arguments
/// * `fd` - usize
/// * `addr` - *const u8
/// * `addr_len` - usize
pub fn syscall_bind(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let addr = args[1] as *const u8;
    let _addr_len = args[2];
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };
    // different action for AF_INET and AF_UNIX
    let addr = unsafe { socket_address_from(addr, socket) };

    info!("[bind()] binding socket {} to {:?}", fd, addr);

    Ok(socket.bind(addr).map_or(-1, |_| 0))
}

// TODO: support change `backlog` for tcp socket
/// # Arguments
/// * `fd` - usize
/// * `backlog` - usize
pub fn syscall_listen(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let _backlog = args[1];
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };

    Ok(socket.listen().map_or(-1, |_| 0))
}

/// # Arguments
/// * `fd` - usize
/// * `addr_buf` - *mut u8
/// * `addr_len` - *mut u32
/// * `flags` - usize
pub fn syscall_accept4(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let addr_buf = args[1] as *mut u8;
    let addr_len = args[2] as *mut u32;
    let flags = args[3];
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };

    debug!("[accept()] socket {fd} accept");

    // socket.accept() might block, we need to release all lock now.
    if curr.manual_alloc_type_for_lazy(addr_len).is_err() {
        return Err(SyscallError::EFAULT);
    }
    let buf_len = unsafe { *addr_len } as usize;
    if (buf_len as i32) < 0 {
        return Err(SyscallError::EINVAL);
    }
    if curr
        .manual_alloc_range_for_lazy(args[1].into(), (args[1] + buf_len).into())
        .is_err()
    {
        return Err(SyscallError::EFAULT);
    }
    match socket.accept() {
        Ok((s, addr)) => {
            let _ = unsafe { socket_address_to(addr, addr_buf, buf_len, addr_len) };

            let mut fd_table = curr.fd_manager.fd_table.lock();
            let Ok(new_fd) = curr.alloc_fd(&mut fd_table) else {
                return Err(SyscallError::ENFILE); // Maybe ENFILE
            };

            debug!("[accept()] socket {fd} accept new socket {new_fd} {addr:?}");

            // handle flags

            s.set_nonblocking((flags & SOCK_NONBLOCK) != 0);

            s.set_close_on_exec((flags & SOCK_CLOEXEC) != 0);

            fd_table[new_fd] = Some(Arc::new(s));
            Ok(new_fd as isize)
        }
        Err(AxError::Unsupported) => Err(SyscallError::EOPNOTSUPP),
        Err(AxError::Interrupted) => Err(SyscallError::EINTR),
        Err(AxError::ConnectionReset) => Err(SyscallError::ECONNABORTED),
        Err(AxError::WouldBlock) => Err(SyscallError::EAGAIN),
        Err(e) => {
            error!("error: {:?}", e);
            Err(SyscallError::EPERM)
        }
    }
}

/// # Arguments
/// * `fd` - usize
/// * `addr_buf` - *const u8
/// * `addr_len` - usize
pub fn syscall_connect(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let addr_buf = args[1] as *const u8;
    let _addr_len = args[2];
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };

    let addr = unsafe { socket_address_from(addr_buf, socket) };

    info!("[connect()] socket {fd} connecting to {addr:?}");

    match socket.connect(addr) {
        Ok(_) => Ok(0),
        Err(AxError::WouldBlock) => Err(SyscallError::EINPROGRESS),
        Err(AxError::Interrupted) => Err(SyscallError::EINTR),
        Err(AxError::AlreadyExists) => Err(SyscallError::EISCONN),
        // TODO：add more error code
        Err(_) => Err(SyscallError::ECONNREFUSED),
    }
}

/// NOTE: linux man 中没有说明若socket未bound应返回什么错误
/// # Arguments
/// * `fd` - usize
/// * `addr` - *mut u8
/// * `addr_len` - *mut u32
pub fn syscall_get_sock_name(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let addr = args[1] as *mut u8;
    let addr_len = args[2] as *mut u32;
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };
    if curr.manual_alloc_type_for_lazy(addr_len).is_err() {
        return Err(SyscallError::EFAULT);
    }
    let buf_len = unsafe { *addr_len } as usize;
    if (buf_len as i32) < 0 {
        return Err(SyscallError::EINVAL);
    }
    if curr
        .manual_alloc_range_for_lazy(args[1].into(), (args[1] + buf_len).into())
        .is_err()
    {
        return Err(SyscallError::EFAULT);
    }

    debug!("[getsockname()] socket {fd}");

    let Ok(name) = socket.name() else {
        return Err(SyscallError::EPERM);
    };

    info!("[getsockname()] socket {fd} name: {:?}", name);
    Ok(unsafe { socket_address_to(name, addr, buf_len, addr_len) }.map_or(-1, |_| 0))
}

#[allow(unused)]
/// # Arguments
/// * `fd` - usize
/// * `addr_buf` - *mut u8
/// * `addr_len` - *mut u32
pub fn syscall_getpeername(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let addr_buf = args[1] as *mut u8;
    let addr_len = args[2] as *mut u32;
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let buf_len = match curr.manual_alloc_type_for_lazy(addr_len) {
        Ok(_) => unsafe { *addr_len as usize },
        Err(_) => return Err(SyscallError::EFAULT),
    };
    // It seems it could be negative according to Linux man page.
    if (buf_len as i32) < 0 {
        return Err(SyscallError::EINVAL);
    }

    if curr
        .manual_alloc_range_for_lazy(
            (addr_buf as usize).into(),
            unsafe { addr_buf.add(buf_len as usize) as usize }.into(),
        )
        .is_err()
    {
        return Err(SyscallError::EFAULT);
    }

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };
    if curr.manual_alloc_for_lazy(args[2].into()).is_err() {
        return Err(SyscallError::EFAULT);
    }

    if curr
        .manual_alloc_range_for_lazy(args[1].into(), (args[1] + buf_len).into())
        .is_err()
    {
        return Err(SyscallError::EFAULT);
    }
    match socket.peer_name() {
        Ok(name) => {
            Ok(unsafe { socket_address_to(name, addr_buf, buf_len, addr_len) }.map_or(-1, |_| 0))
        }
        Err(AxError::NotConnected) => Err(SyscallError::ENOTCONN),
        Err(_) => unreachable!(),
    }
}

// TODO: flags
/// Calling sendto() will bind the socket if it's not bound.
/// # Arguments
/// * `fd` - usize
/// * `buf` - *const u8
/// * `len` - usize
/// * `flags` - usize
/// * `addr` - *const u8
/// * `addr_len` - usize
pub fn syscall_sendto(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let buf = args[1] as *const u8;
    let len = args[2];
    let _flags = args[3];
    let addr = args[4] as *const u8;
    let addr_len = args[5];
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };

    if buf.is_null() {
        return Err(SyscallError::EFAULT);
    }
    let Ok(buf) = curr
        .manual_alloc_range_for_lazy(
            (buf as usize).into(),
            unsafe { buf.add(len) as usize }.into(),
        )
        .map(|_| unsafe { from_raw_parts(buf, len) })
    else {
        error!("[sendto()] buf address {buf:?} invalid");
        return Err(SyscallError::EFAULT);
    };

    let addr = if !addr.is_null() && addr_len != 0 {
        match curr.manual_alloc_range_for_lazy(
            (addr as usize).into(),
            unsafe { addr.add(addr_len) as usize }.into(),
        ) {
            Ok(_) => Some(unsafe { socket_address_from(addr, socket) }),
            Err(_) => {
                error!("[sendto()] addr address {addr:?} invalid");
                return Err(SyscallError::EFAULT);
            }
        }
    } else {
        None
    };
    match socket.sendto(buf, addr) {
        Ok(len) => {
            info!("[sendto()] socket {fd} sent {len} bytes to addr {:?}", addr);
            Ok(len as isize)
        }
        Err(AxError::Interrupted) => Err(SyscallError::EINTR),
        Err(AxError::Again) | Err(AxError::WouldBlock) => Err(SyscallError::EAGAIN),
        Err(AxError::NotConnected) => Err(SyscallError::ENOTCONN),
        Err(AxError::ConnectionReset) => Err(SyscallError::EPIPE),
        Err(e) => {
            error!("[sendto()] socket {fd} send error: {e:?}");
            Err(SyscallError::EPERM)
        }
    }
}

/// # Arguments
/// * `fd` - usize
/// * `buf` - *mut u8
/// * `len` - usize
/// * `flags` - usize
/// * `addr_buf` - *mut u8
/// * `addr_len` - *mut u32
pub fn syscall_recvfrom(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let buf = args[1] as *mut u8;
    let len = args[2];
    let _flags = args[3];
    let addr_buf = args[4] as *mut u8;
    let addr_len = args[5] as *mut u32;
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };
    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };
    if !addr_len.is_null() && curr.manual_alloc_type_for_lazy(addr_len).is_err() {
        error!("[recvfrom()] addr_len address {addr_len:?} invalid");
        return Err(SyscallError::EFAULT);
    }

    if !addr_buf.is_null() && curr.manual_alloc_type_for_lazy(addr_buf).is_err() {
        return Err(SyscallError::EFAULT);
    }

    if curr
        .manual_alloc_range_for_lazy(args[1].into(), (args[1] + len).into())
        .is_err()
    {
        error!(
            "[recvfrom()] addr_buf address {addr_buf:?}, len: {} invalid",
            unsafe { *addr_len }
        );
        return Err(SyscallError::EFAULT);
    }
    let buf = unsafe { from_raw_parts_mut(buf, len) };
    info!("recv addr: {:?}", socket.name().unwrap());
    match socket.recv_from(buf) {
        Ok((len, addr)) => {
            info!("socket {fd} recv {len} bytes from {addr:?}");
            if !addr_buf.is_null() && !addr_len.is_null() {
                let buf_len = unsafe { *addr_len } as usize;
                if (buf_len as i32) < 0 {
                    return Err(SyscallError::EINVAL);
                }
                Ok(
                    unsafe { socket_address_to(addr, addr_buf, buf_len, addr_len) }
                        .map_or(-1, |_| len as isize),
                )
            } else {
                Ok(len as isize)
            }
        }
        Err(AxError::NotConnected) => Err(SyscallError::ENOTCONN),
        Err(AxError::ConnectionRefused) => Err(SyscallError::ECONNREFUSED),
        Err(AxError::Interrupted) => Err(SyscallError::EINTR),
        Err(AxError::Timeout) | Err(AxError::WouldBlock) => Err(SyscallError::EAGAIN),
        Err(_) => Err(SyscallError::EPERM),
    }
}

pub fn syscall_sendmsg(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let msg = args[1] as *mut MessageHeader;
    let _flags = args[2];
    let curr = current_process();
    let msg = unsafe { &*msg };

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };

    let msg_header = msg;
    let iove = unsafe { &*msg_header.iovec };
    let buf = iove.base;
    let len = iove.len;
    let Ok(buf) = curr
        .manual_alloc_range_for_lazy(
            (buf as usize).into(),
            unsafe { buf.add(len) as usize }.into(),
        )
        .map(|_| unsafe { from_raw_parts(buf, len) })
    else {
        error!("[sendto()] buf address {buf:?} invalid");
        return Err(SyscallError::EFAULT);
    };

    let Ok(addr) = socket.peer_name() else {
        return Err(SyscallError::EPERM);
    };

    match socket.sendto(buf, Some(addr)) {
        Ok(len) => Ok(len as isize),
        Err(AxError::Interrupted) => Err(SyscallError::EINTR),
        Err(AxError::Again) | Err(AxError::WouldBlock) => Err(SyscallError::EAGAIN),
        Err(AxError::NotConnected) => Err(SyscallError::ENOTCONN),
        Err(AxError::ConnectionReset) => Err(SyscallError::EPIPE),
        Err(e) => {
            error!("[sendmsg()] socket {fd} send error: {e:?}");
            Err(SyscallError::EPERM)
        }
    }
}

/// NOTE: only support socket level options (SOL_SOCKET)
/// # Arguments
/// * `fd` - usize
/// * `level` - usize
/// * `opt_name` - usize
/// * `opt_value` - *const u8
/// * `opt_len` - u32
pub fn syscall_set_sock_opt(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let level = args[1];
    let opt_name = args[2];
    let opt_value = args[3] as *const u8;
    let opt_len = args[4] as u32;
    let Ok(level) = SocketOptionLevel::try_from(level) else {
        error!("[setsockopt()] level {level} not supported");
        unimplemented!();
    };

    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };

    let opt = unsafe { from_raw_parts(opt_value, opt_len as usize) };

    match level {
        SocketOptionLevel::IP => {
            let Ok(option) = IpOption::try_from(opt_name) else {
                warn!("[setsockopt()] option {opt_name} not supported in socket level");
                return Ok(0);
            };

            option.set(socket, opt)
        }
        SocketOptionLevel::Socket => {
            let Ok(option) = SocketOption::try_from(opt_name) else {
                warn!("[setsockopt()] option {opt_name} not supported in socket level");
                return Ok(0);
            };

            option.set(socket, opt)
        }
        SocketOptionLevel::Tcp => {
            let Ok(option) = TcpSocketOption::try_from(opt_name) else {
                warn!("[setsockopt()] option {opt_name} not supported in tcp level");
                return Ok(0);
            };

            option.set(socket, opt)
        }
        SocketOptionLevel::IPv6 => {
            let Ok(option) = Ipv6Option::try_from(opt_name) else {
                warn!("[setsockopt()] option {opt_name} not supported in ipv6 level");
                return Ok(0);
            };

            option.set(socket, opt)
        }
    }
}

/// # Arguments
/// * `fd` - usize
/// * `level` - usize
/// * `opt_name` - usize
/// * `opt_value` - *mut u8
/// * `opt_len` - *mut u32
pub fn syscall_get_sock_opt(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let level = args[1];
    let opt_name = args[2];
    let opt_value = args[3] as *mut u8;
    let opt_len = args[4] as *mut u32;
    let Ok(level) = SocketOptionLevel::try_from(level) else {
        error!("[setsockopt()] level {level} not supported");
        unimplemented!();
    };

    if opt_value.is_null() || opt_len.is_null() {
        return Err(SyscallError::EFAULT);
    }

    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };

    if curr
        .manual_alloc_type_for_lazy(opt_len as *const u32)
        .is_err()
    {
        error!("[getsockopt()] opt_len address {opt_len:?} invalid");
        return Err(SyscallError::EFAULT);
    }
    if curr
        .manual_alloc_range_for_lazy(
            (opt_value as usize).into(),
            (unsafe { opt_value.add(*opt_len as usize) } as usize).into(),
        )
        .is_err()
    {
        error!(
            "[getsockopt()] opt_value {opt_value:?}, len {} invalid",
            unsafe { *opt_len }
        );
        return Err(SyscallError::EFAULT);
    }

    match level {
        SocketOptionLevel::IP => {}
        SocketOptionLevel::Socket => {
            let Ok(option) = SocketOption::try_from(opt_name) else {
                panic!("[setsockopt()] option {opt_name} not supported in socket level");
            };

            option.get(socket, opt_value, opt_len);
        }
        SocketOptionLevel::Tcp => {
            let Ok(option) = TcpSocketOption::try_from(opt_name) else {
                panic!("[setsockopt()] option {opt_name} not supported in tcp level");
            };

            if option == TcpSocketOption::TCP_INFO {
                return Err(SyscallError::ENOPROTOOPT);
            }

            option.get(socket, opt_value, opt_len);
        }
        // TODO: achieve the real implementation of ipv6
        SocketOptionLevel::IPv6 => {}
    }

    Ok(0)
}

#[derive(TryFromPrimitive)]
#[repr(usize)]
enum SocketShutdown {
    Read = 0,
    Write = 1,
    ReadWrite = 2,
}

/// # Arguments
/// * `fd` - usize
/// * `how` - usize
pub fn syscall_shutdown(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let how = args[1];
    let curr = current_process();

    let file = match curr.fd_manager.fd_table.lock().get(fd) {
        Some(Some(file)) => file.clone(),
        _ => return Err(SyscallError::EBADF),
    };

    let Some(socket) = file.as_any().downcast_ref::<Socket>() else {
        return Err(SyscallError::ENOTSOCK);
    };

    let Ok(how) = SocketShutdown::try_from(how) else {
        return Err(SyscallError::EINVAL);
    };

    match how {
        SocketShutdown::Read => {
            error!("[shutdown()] SHUT_RD is noop")
        }
        SocketShutdown::Write => socket.shutdown(),
        SocketShutdown::ReadWrite => {
            socket.abort();
        }
    }

    Ok(0)
}

pub fn syscall_socketpair(args: [usize; 6]) -> SyscallResult {
    let fd: *mut u32 = args[3] as *mut u32;
    let s_type = args[1];
    let domain = args[0];
    let process = current_process();
    if process.manual_alloc_for_lazy((fd as usize).into()).is_err() {
        return Err(SyscallError::EINVAL);
    }
    if domain != Domain::AF_UNIX as usize {
        panic!();
    }
    if SocketType::try_from(s_type & SOCKET_TYPE_MASK).is_err() {
        // return ErrorNo::EINVAL as isize;
        return Err(SyscallError::EINVAL);
    };

    let (fd1, fd2) = make_socketpair(s_type);
    let mut fd_table = process.fd_manager.fd_table.lock();
    let fd_num = if let Ok(fd) = process.alloc_fd(&mut fd_table) {
        fd
    } else {
        return Err(SyscallError::EPERM);
    };
    fd_table[fd_num] = Some(fd1);

    let fd_num2 = if let Ok(fd) = process.alloc_fd(&mut fd_table) {
        fd
    } else {
        return Err(SyscallError::EPERM);
    };
    axlog::info!("alloc fd1 {} fd2 {} as socketpair", fd_num, fd_num2);
    fd_table[fd_num2] = Some(fd2);

    unsafe {
        core::ptr::write(fd, fd_num as u32);
        core::ptr::write(fd.offset(1), fd_num2 as u32);
    }

    Ok(0)
}

/// return sockerpair read write
pub fn make_socketpair(socket_type: usize) -> (Arc<Socket>, Arc<Socket>) {
    let s_type = SocketType::try_from(socket_type & SOCKET_TYPE_MASK).unwrap();
    let mut fd1 = Socket::new(Domain::AF_UNIX, s_type);
    let mut fd2 = Socket::new(Domain::AF_UNIX, s_type);
    let mut pipe_flag = OpenFlags::empty();
    if socket_type & SOCK_NONBLOCK != 0 {
        pipe_flag |= OpenFlags::NON_BLOCK;
        fd1.set_nonblocking(true);
        fd2.set_nonblocking(true);
    }
    if socket_type & SOCK_CLOEXEC != 0 {
        pipe_flag |= OpenFlags::CLOEXEC;
        fd1.set_close_on_exec(true);
        fd2.set_close_on_exec(true);
    }
    let (pipe1, pipe2) = make_pipe(pipe_flag);
    fd1.buffer = Some(pipe1);
    fd2.buffer = Some(pipe2);
    (Arc::new(fd1), Arc::new(fd2))
}
