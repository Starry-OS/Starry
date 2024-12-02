//! 信号处理时保存的用户上下文。

/// 处理信号时使用的栈
///
/// 详细信息见`https://man7.org/linux/man-pages/man2/sigaltstack.2.html`
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SignalStack {
    /// Base address of the stack
    pub sp: usize,
    /// Flags for the stack
    pub flags: u32,
    /// Size of the stack
    pub size: usize,
}

impl Default for SignalStack {
    fn default() -> Self {
        Self {
            sp: 0,
            // 代表SS_DISABLE，即不使用栈
            flags: super::SS_DISABLE,
            size: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// The `mcontext` struct for the signal action
pub struct MContext {
    fault_address: usize,
    regs: [usize; 31],
    sp: usize,
    pc: usize,
    pstate: usize,
    reserved: [usize; 256 * 2],
}

impl Default for MContext {
    fn default() -> Self {
        Self {
            fault_address: 0,
            regs: [0; 31],
            sp: 0,
            pc: 0,
            pstate: 0,
            reserved: [0; 512],
        }
    }
}

impl MContext {
    fn init_by_pc(pc: usize) -> Self {
        Self {
            fault_address: 0,
            regs: [0; 31],
            sp: 0,
            pc,
            pstate: 0,
            reserved: [0; 512],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
/// The user context saved for the signal action, which can be accessed by the signal handler
pub struct SignalUserContext {
    flags: usize,
    link: usize,
    stack: SignalStack,
    sigmask: [u64; 17],
    mcontext: MContext,
}

impl Default for SignalUserContext {
    fn default() -> Self {
        Self {
            flags: 0,
            link: 0,
            stack: SignalStack::default(),
            mcontext: MContext::default(),
            sigmask: [0; 17],
        }
    }
}

impl SignalUserContext {
    /// init the user context by the pc and the mask
    pub fn init(pc: usize, _mask: usize) -> Self {
        Self {
            flags: 0,
            link: 0,
            stack: SignalStack::default(),
            mcontext: MContext::init_by_pc(pc),
            sigmask: [0; 17],
        }
    }

    /// get the pc from the user context
    pub fn get_pc(&self) -> usize {
        self.mcontext.pc
    }
}
