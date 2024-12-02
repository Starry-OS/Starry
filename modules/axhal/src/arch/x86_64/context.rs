use taskctx::TaskContext;

use super::GdtStruct;

/// Saved registers when a trap (interrupt or exception) occurs.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapFrame {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,

    // Pushed by `trap.S`
    pub vector: u64,
    pub error_code: u64,

    // Pushed by CPU
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl TrapFrame {
    /// Whether the trap is from userspace.
    pub const fn is_user(&self) -> bool {
        self.cs & 0b11 == 3
    }

    /// To set the stack pointer
    pub fn set_user_sp(&mut self, user_sp: usize) {
        self.rsp = user_sp as _;
    }

    /// 用于第一次进入应用程序时的初始化
    pub fn app_init_context(app_entry: usize, user_sp: usize) -> Self {
        TrapFrame {
            rip: app_entry as _,
            cs: GdtStruct::UCODE64_SELECTOR.0 as _,
            #[cfg(feature = "irq")]
            rflags: x86_64::registers::rflags::RFlags::INTERRUPT_FLAG.bits() as _,
            rsp: user_sp as _,
            ss: GdtStruct::UDATA_SELECTOR.0 as _,
            ..Default::default()
        }
    }

    /// set the return code
    pub fn set_ret_code(&mut self, ret_value: usize) {
        self.rax = ret_value as _;
    }

    /// 设置TLS
    pub fn set_tls(&mut self, _tls_value: usize) {
        // panic!("set tls: {:#x}", tls_value);
        // unsafe {
        //     write_thread_pointer(tls_value);
        // }
        todo!("set tls");
    }

    /// 获取 sp
    pub fn get_sp(&self) -> usize {
        self.rsp as _
    }

    /// 设置 arg0
    pub fn set_arg0(&mut self, arg: usize) {
        self.rdi = arg as _;
    }

    /// 设置 arg1
    pub fn set_arg1(&mut self, arg: usize) {
        self.rsi = arg as _;
    }

    /// 设置 arg2
    pub fn set_arg2(&mut self, arg: usize) {
        self.rdx = arg as _;
    }

    /// 获取 pc
    pub fn get_pc(&self) -> usize {
        self.rip as _
    }

    /// 设置 pc
    pub fn set_pc(&mut self, pc: usize) {
        self.rip = pc as _;
    }

    /// pc 倒退到 syscall 指令的长度
    pub fn rewind_pc(&mut self) {
        self.rip -= 2;
    }

    /// 获取 ret
    pub fn get_ret_code(&self) -> usize {
        self.rax as _
    }

    /// 设置返回地址
    pub fn set_ra(&mut self, _ra: usize) {
        todo!()
    }

    /// 获取所有 syscall 参数
    pub fn get_syscall_args(&self) -> [usize; 6] {
        [self.rdi, self.rsi, self.rdx, self.r10, self.r8, self.r9].map(|n| n as _)
    }

    /// 获取 syscall id
    pub fn get_syscall_num(&self) -> usize {
        self.rax as _
    }
}

#[no_mangle]
#[cfg(feature = "monolithic")]
/// To handle the first time into the user space
///
/// 1. push the given trap frame into the kernel stack
/// 2. go into the user space
///
/// args:
///
/// 1. kernel_sp: the top of the kernel stack
///
/// 2. frame_base: the address of the trap frame which will be pushed into the kernel stack
pub fn first_into_user(kernel_sp: usize) {
    // Make sure that all csr registers are stored before enable the interrupt
    use memory_addr::VirtAddr;

    use crate::arch::flush_tlb;

    use super::disable_irqs;
    disable_irqs();
    flush_tlb(None);

    let trap_frame_size = core::mem::size_of::<TrapFrame>();
    let kernel_base = kernel_sp - trap_frame_size;
    crate::set_tss_stack_top(VirtAddr::from(kernel_sp));
    unsafe {
        core::arch::asm!(
            r"
                    mov     gs:[offset __PERCPU_KERNEL_RSP_OFFSET], {kernel_sp}

                    mov      rsp, {kernel_base}

                    pop rax
                    pop rcx
                    pop rdx
                    pop rbx
                    pop rbp
                    pop rsi
                    pop rdi
                    pop r8
                    pop r9
                    pop r10
                    pop r11
                    pop r12
                    pop r13
                    pop r14
                    pop r15
                    add rsp, 16

                    swapgs
                    iretq
                ",
            kernel_sp = in(reg) kernel_sp,
            kernel_base = in(reg) kernel_base,
        );
    };
}

/// To switch the context between two tasks
pub fn task_context_switch(prev_ctx: &mut TaskContext, next_ctx: &TaskContext) {
    #[cfg(feature = "fp_simd")]
    {
        prev_ctx.ext_state.save();
        next_ctx.ext_state.restore();
    }
    #[cfg(any(feature = "tls", feature = "monolithic"))]
    {
        prev_ctx.fs_base = super::read_thread_pointer();
        unsafe { super::write_thread_pointer(next_ctx.fs_base) };
    }
    #[cfg(feature = "monolithic")]
    unsafe {
        // change gs data
        core::arch::asm!("mov     gs:[offset __PERCPU_KERNEL_RSP_OFFSET], {kernel_sp}", 
                kernel_sp = in(reg) next_ctx.kstack_top.as_usize() + core::mem::size_of::<TrapFrame>());
    }
    crate::set_tss_stack_top(next_ctx.kstack_top + core::mem::size_of::<TrapFrame>());

    unsafe {
        // TODO: switch FP states
        taskctx::context_switch(&mut prev_ctx.rsp, &next_ctx.rsp)
    }
}
