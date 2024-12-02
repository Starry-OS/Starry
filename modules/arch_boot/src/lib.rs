//! The boot entry of the whole kernel, which will initialize the kernel and start the first user process.
//!
//! To ensure the one-way dependence on the calling relationship, the boot module is moved to the top level of the project.
#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]

use axconfig::TASK_STACK_SIZE;
use core::arch::asm;

mod platform;

#[cfg(feature = "smp")]
mod mp;

#[link_section = ".bss.stack"]
pub(crate) static mut BOOT_STACK: [u8; TASK_STACK_SIZE] = [0; TASK_STACK_SIZE];

/// The start address of the boot stack for current CPU.
#[no_mangle]
pub extern "C" fn current_boot_stack() -> *mut u8 {
    unsafe {
        let sp: usize;

        #[cfg(target_arch = "x86_64")]
        asm!("mov {}, rsp", out(reg) sp);

        #[cfg(target_arch = "aarch64")]
        asm!("mov {}, sp", out(reg) sp);

        #[cfg(target_arch = "riscv64")]
        asm!("mv {}, sp", out(reg) sp);

        let stack_low = BOOT_STACK.as_ptr() as usize;
        let stack_high = stack_low + TASK_STACK_SIZE;

        if sp >= stack_low && sp < stack_high {
            log::debug!("get sp {:#x} in boot_stack", sp);
            return BOOT_STACK.as_mut_ptr();
        }

        #[cfg(feature = "smp")]
        return mp::mp_boot_stack(sp);
        #[cfg(not(feature = "smp"))]
        return core::ptr::null_mut::<u8>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_boot_stack() {
        let sp = current_boot_stack();
        assert!(sp.is_null());
    }

    #[test]
    fn test_config() {
        assert!(axconfig::TASK_STACK_SIZE != 0);
    }
}
