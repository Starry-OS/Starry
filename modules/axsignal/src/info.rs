//! 触发信号时的信息，当SigAction指定需要信息时，将其返回给用户
//!
//! 错误信息：详细定义见 `https://man7.org/linux/man-pages/man2/rt_sigaction.2.html`

/// The information of the signal
///
/// When the `SigAction` specifies that it needs information, it will return it to the user
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SigInfo {
    /// The signal number
    pub si_signo: i32,
    /// An errno value
    pub si_errno: i32,
    /// The code of the signal
    pub si_code: i32,

    /// Padding
    #[allow(unused)]
    pub pad: u32,
    /// The process ID of the sender
    pub pid: i32,
    /// The real user ID of the sender
    pub uid: u32,
    /// The value sent with the signal
    pub si_val_int: i32,
    /// The value pointer of the signal
    pub si_val_ptr: usize,
}

impl Default for SigInfo {
    fn default() -> Self {
        Self {
            si_signo: 0,
            si_errno: 0,
            si_code: -6, // SI_TKILL
            pad: 0,
            pid: 0,
            uid: 0,
            si_val_int: 0,
            si_val_ptr: 0,
        }
    }
}
