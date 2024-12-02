use taskctx::TaskContext;

/// Saved registers when a trap (exception) occurs.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapFrame {
    /// General-purpose registers (R0..R30).
    pub r: [usize; 31],
    /// Stack Poiter
    pub usp: usize,
    /// Exception Link Register (ELR_EL1).
    pub elr: usize,
    /// Saved Process Status Register (SPSR_EL1).
    pub spsr: usize,
    /// Saved tpidr_el0.
    pub tpidr_el0: usize,
}

impl TrapFrame {
    /// To set the stack pointer
    pub fn set_user_sp(&mut self, user_sp: usize) {
        self.usp = user_sp;
    }

    pub fn get_sp(&self) -> usize {
        self.usp
    }

    pub fn get_pc(&self) -> usize {
        self.elr
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.elr = pc;
    }

    /// pc 倒退到 syscall 指令的长度
    pub fn rewind_pc(&mut self) {
        self.elr -= 4;
    }

    pub fn set_tls(&mut self, tls: usize) {
        self.tpidr_el0 = tls;
    }

    pub fn set_ret_code(&mut self, ret: usize) {
        self.r[0] = ret;
    }

    pub fn get_ret_code(&self) -> usize {
        self.r[0]
    }

    pub fn set_arg0(&mut self, param: usize) {
        self.r[0] = param;
    }

    pub fn set_arg1(&mut self, param: usize) {
        self.r[1] = param;
    }

    pub fn set_arg2(&mut self, param: usize) {
        self.r[2] = param;
    }

    /// set the return address
    pub fn set_ra(&mut self, param: usize) {
        self.r[30] = param;
    }

    /// 用于第一次进入应用程序时的初始化
    pub fn app_init_context(app_entry: usize, user_sp: usize) -> Self {
        let mut trap_frame = TrapFrame::default();
        trap_frame.set_user_sp(user_sp);
        trap_frame.elr = app_entry;
        trap_frame.spsr = 0x00000000;
        trap_frame
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
pub fn first_into_user(kernel_sp: usize) -> ! {
    use crate::arch::disable_irqs;

    let trap_frame_size = core::mem::size_of::<TrapFrame>();
    let kernel_base = kernel_sp - trap_frame_size;
    info!("kernel_base {:#x} kernel_sp{:#x}", kernel_base, kernel_sp);
    // 在保证将寄存器都存储好之后，再开启中断
    disable_irqs();
    crate::arch::flush_tlb(None);
    crate::arch::flush_icache_all();
    //crate::arch::flush_dcache_all();
    unsafe {
        core::arch::asm!(
            r"
            mov     sp, {kernel_base}
            ldp     x30, x9, [sp, 30 * 8]    // load user sp_el0
            ldp     x10, x11, [sp, 32 * 8]   // load ELR, SPSR
            msr     elr_el1, x10
            msr     spsr_el1, x11
        
            ldr     x12, [sp, 34 * 8]
        
            msr     tpidr_el0, x12  // restore user tls pointer
           
            mrs     x13,  sp_el0    // save current ktask ptr
            str     x13,  [sp, 31 * 8]
            msr     sp_el0, x9     // restore user sp
        
            ldp     x28, x29, [sp, 28 * 8]
            ldp     x26, x27, [sp, 26 * 8]
            ldp     x24, x25, [sp, 24 * 8]
            ldp     x22, x23, [sp, 22 * 8]
            ldp     x20, x21, [sp, 20 * 8]
            ldp     x18, x19, [sp, 18 * 8]
            ldp     x16, x17, [sp, 16 * 8]
            ldp     x14, x15, [sp, 14 * 8]
            ldp     x12, x13, [sp, 12 * 8]
            ldp     x10, x11, [sp, 10 * 8]
            ldp     x8, x9, [sp, 8 * 8]
            ldp     x6, x7, [sp, 6 * 8]
            ldp     x4, x5, [sp, 4 * 8]
            ldp     x2, x3, [sp, 2 * 8]
            ldp     x0, x1, [sp]
            add     sp, sp, 35 * 8
            eret
            ",
            kernel_base = in(reg) kernel_base,
        )
    }
    core::panic!("already in user mode!")
}

/// Switches to another task.
///
/// It first saves the current task's context from CPU to this place, and then
/// restores the next task's context from `next_ctx` to CPU.
pub fn task_context_switch(prev_ctx: &mut TaskContext, next_ctx: &TaskContext) {
    #[cfg(feature = "fp_simd")]
    prev_ctx.fp_state.switch_to(&next_ctx.fp_state);
    unsafe { taskctx::context_switch(prev_ctx, next_ctx) }
}
