use axconfig::{SMP, TASK_STACK_SIZE};
use axhal::mem::{virt_to_phys, VirtAddr};

#[link_section = ".bss.stack"]
static mut SECONDARY_BOOT_STACK: [[u8; TASK_STACK_SIZE]; SMP - 1] = [[0; TASK_STACK_SIZE]; SMP - 1];

/// To start secondary CPUs after the primary CPU has been started.
pub(crate) fn start_secondary_cpus(primary_cpu_id: usize) {
    let mut logic_cpu_id = 0;
    for i in 0..SMP {
        if i != primary_cpu_id {
            let stack_top = virt_to_phys(VirtAddr::from(unsafe {
                SECONDARY_BOOT_STACK[logic_cpu_id].as_ptr_range().end as usize
            }));

            log::debug!("starting CPU {}...", i);
            crate::platform::mp::start_given_secondary_cpu(i, stack_top);
            logic_cpu_id += 1;

            loop {
                if axruntime::entered_cpus_num() > logic_cpu_id {
                    break;
                }
            }
        }
    }
}

pub(crate) unsafe fn mp_boot_stack(sp: usize) -> *mut u8 {
    for i in 0..SMP {
        let stack_low = SECONDARY_BOOT_STACK[i].as_ptr() as usize;
        let stack_high = stack_low + TASK_STACK_SIZE;

        if sp >= stack_low && sp < stack_high {
            log::info!("get sp {:#x} in second boot_stack", sp);
            return SECONDARY_BOOT_STACK[i].as_mut_ptr();
        }
    }
    0 as *mut u8
}
