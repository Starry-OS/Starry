pub fn handle_irq(_irq_num: usize, _from_user: bool) {
    #[cfg(feature = "irq")]
    {
        let guard = kernel_guard::NoPreempt::new();
        // trap进来，统计时间信息
        // 只有当trap是来自用户态才进行统计
        #[cfg(feature = "monolithic")]
        linux_syscall_api::trap::handle_irq(_irq_num, _from_user);

        #[cfg(not(feature = "monolithic"))]
        axhal::irq::dispatch_irq(_irq_num);
        drop(guard); // rescheduling may occur when preemption is re-enabled.

        #[cfg(feature = "preempt")]
        axtask::current_check_preempt_pending();
    }
}

#[cfg(feature = "monolithic")]
pub use linux_syscall_api::trap::{handle_page_fault, handle_signals, handle_syscall};
