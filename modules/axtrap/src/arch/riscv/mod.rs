#[macro_use]
mod macros;

use axhal::arch::TrapFrame;

use riscv::register::scause::{self, Exception as E, Trap};
use riscv::register::stvec;

use crate::trap::*;

#[cfg(feature = "monolithic")]
use linux_syscall_api::trap::MappingFlags;

include_trap_asm_marcos!();
core::arch::global_asm!(
    include_str!("trap.S"),
    trapframe_size = const core::mem::size_of::<TrapFrame>(),
);

/// Writes Supervisor Trap Vector Base Address Register (`stvec`).
#[inline]
pub fn set_trap_vector_base(stvec: usize) {
    unsafe { stvec::write(stvec, stvec::TrapMode::Direct) }
}

extern "C" {
    fn trap_vector_base();
}

/// To initialize the trap vector base address.
pub fn init_interrupt() {
    set_trap_vector_base(trap_vector_base as usize);
}

fn handle_breakpoint(sepc: &mut usize) {
    axlog::debug!("Exception(Breakpoint) @ {:#x} ", sepc);
    *sepc += 2
}

#[no_mangle]
pub fn riscv_trap_handler(tf: &mut TrapFrame, from_user: bool) {
    let scause = scause::read();
    // Read the stval before enable_irqs
    // Otherwise, the stval may be changed after the time interrupt is handled
    let stval = riscv::register::stval::read();

    #[cfg(feature = "monolithic")]
    linux_syscall_api::trap::record_trap(scause.code());

    match scause.cause() {
        Trap::Exception(E::Breakpoint) => handle_breakpoint(&mut tf.sepc),
        Trap::Interrupt(_) => handle_irq(scause.bits(), from_user),

        #[cfg(feature = "monolithic")]
        Trap::Exception(E::UserEnvCall) => {
            axhal::arch::enable_irqs();
            tf.sepc += 4;
            let result = handle_syscall(
                tf.regs.a7,
                [
                    tf.regs.a0, tf.regs.a1, tf.regs.a2, tf.regs.a3, tf.regs.a4, tf.regs.a5,
                ],
            );
            if -result == linux_syscall_api::SyscallError::ERESTART as isize {
                // Restart the syscall
                tf.rewind_pc();
            } else {
                tf.regs.a0 = result as usize;
            }
            axhal::arch::disable_irqs();
        }

        #[cfg(feature = "monolithic")]
        Trap::Exception(E::InstructionPageFault) => {
            if !from_user {
                unimplemented!(
                    "I page fault from kernel, addr: {:X}, sepc: {:X}",
                    stval,
                    tf.sepc
                );
            }
            handle_page_fault(stval.into(), MappingFlags::USER | MappingFlags::EXECUTE);
        }

        #[cfg(feature = "monolithic")]
        Trap::Exception(E::LoadPageFault) => {
            if !from_user {
                unimplemented!(
                    "L page fault from kernel, addr: {:X}, sepc: {:X}",
                    stval,
                    tf.sepc
                );
            }
            handle_page_fault(stval.into(), MappingFlags::USER | MappingFlags::READ);
        }

        #[cfg(feature = "monolithic")]
        Trap::Exception(E::StorePageFault) => {
            if !from_user {
                unimplemented!(
                    "S page fault from kernel, addr: {:X}, sepc: {:X}",
                    stval,
                    tf.sepc
                );
            }
            handle_page_fault(stval.into(), MappingFlags::USER | MappingFlags::WRITE);
        }

        _ => {
            panic!(
                "Unhandled trap {:?} @ {:#x}:\n{:#x?}",
                scause.cause(),
                tf.sepc,
                tf
            );
        }
    }

    #[cfg(feature = "monolithic")]
    {
        if from_user {
            handle_signals();
        }
    }
}
