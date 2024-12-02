use axhal::mem::{virt_to_phys, PhysAddr, VirtAddr};

/// Starts the given secondary CPU with its boot stack.
pub fn start_given_secondary_cpu(cpu_id: usize, stack_top: PhysAddr) {
    extern "C" {
        fn _start_secondary();
    }
    let real_cpu_id = of::cpus()
        .expect("Failed to read cpu info")
        .nth(cpu_id)
        .expect("not correct cpu_id")
        .ids()
        .first();
    let entry = virt_to_phys(VirtAddr::from(_start_secondary as usize));
    axhal::platform::psci::cpu_on(real_cpu_id, entry.as_usize(), stack_top.as_usize());
}
