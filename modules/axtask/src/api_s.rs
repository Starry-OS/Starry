//! Task APIs for single-task configuration.

/// For single-task situation, we just relax the CPU and wait for incoming
/// interrupts.
pub fn yield_now() {
    if cfg!(feature = "irq") {
        axhal::arch::wait_for_irqs();
    } else {
        core::hint::spin_loop();
    }
}

/// For single-task situation, we just busy wait for the given duration.
pub fn sleep(dur: core::time::Duration) {
    axhal::time::busy_wait(dur);
}

/// For single-task situation, we just busy wait until reaching the given
/// deadline.
pub fn sleep_until(deadline: axhal::time::TimeValue) {
    axhal::time::busy_wait_until(deadline);
}

// arch_boot
extern "C" {
    fn current_boot_stack() -> *mut u8;
}

pub fn global_unique_ts() -> (usize, usize) {
    let boot_stack = unsafe { current_boot_stack() as usize };
    (boot_stack, boot_stack + axconfig::TASK_STACK_SIZE)
}

pub fn dump_curr_backtrace() {
    //Init Unwind instance from current context
    use axbacktrace::{dump_backtrace, StackInfo, Unwind, UnwindIf};
    let stack = global_unique_ts();
    let stack_info = StackInfo::new(stack.0, stack.1);
    axlog::info!(
        "dump task stack range: {:#016x}: {:#016x}",
        stack.0,
        stack.1
    );
    let mut unwind = Unwind::new_from_cur_ctx(stack_info);
    // dump current task trace
    dump_backtrace(&mut unwind);
}
