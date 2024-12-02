use axhal::arch::TrapFrame;
use x86::{controlregs::cr2, irq::*};

mod idt;

#[cfg(feature = "monolithic")]
mod syscall;

core::arch::global_asm!(include_str!("trap.S"));

const IRQ_VECTOR_START: u8 = 0x20;
const IRQ_VECTOR_END: u8 = 0xff;

/// To init the interrupt entry
pub fn init_interrupt() {
    // To init the IDT
    idt::init_idt();
    #[cfg(feature = "monolithic")]
    syscall::init_syscall();
}

#[no_mangle]
fn x86_trap_handler(tf: &mut TrapFrame) {
    // Read cr2 before enable_irqs
    // Otherwise, the cr2 may be changed after the time interrupt is handled
    let cr2 = unsafe { cr2() };

    match tf.vector as u8 {
        PAGE_FAULT_VECTOR => {
            if tf.is_user() {
                axlog::debug!(
                    "User #PF @ {:#x}, fault_vaddr={:#x}, error_code={:#x}",
                    tf.rip,
                    cr2,
                    tf.error_code,
                );
                #[cfg(feature = "monolithic")]
                {
                    use axhal::paging::MappingFlags;
                    //  31              15                             4               0
                    // +---+--  --+---+-----+---+--  --+---+----+----+---+---+---+---+---+
                    // |   Reserved   | SGX |   Reserved   | SS | PK | I | R | U | W | P |
                    // +---+--  --+---+-----+---+--  --+---+----+----+---+---+---+---+---+
                    let mut map_flags = MappingFlags::USER; // TODO: add this flags through user tf.
                    if tf.error_code & (1 << 1) != 0 {
                        map_flags |= MappingFlags::WRITE;
                    }
                    if tf.error_code & (1 << 2) != 0 {
                        map_flags |= MappingFlags::USER;
                    }
                    if tf.error_code & (1 << 3) != 0 {
                        map_flags |= MappingFlags::READ;
                    }
                    if tf.error_code & (1 << 4) != 0 {
                        map_flags |= MappingFlags::EXECUTE;
                    }
                    axlog::debug!("error_code: {:?}", tf.error_code);
                    crate::trap::handle_page_fault(cr2.into(), map_flags);
                }
            } else {
                panic!(
                    "Kernel #PF @ {:#x}, fault_vaddr={:#x}, error_code={:#x}:\n{:#x?}",
                    tf.rip, cr2, tf.error_code, tf,
                );
            }
        }
        BREAKPOINT_VECTOR => axlog::debug!("#BP @ {:#x} ", tf.rip),
        GENERAL_PROTECTION_FAULT_VECTOR => {
            panic!(
                "#GP @ {:#x}, error_code={:#x}:\n{:#x?}",
                tf.rip, tf.error_code, tf
            );
        }
        IRQ_VECTOR_START..=IRQ_VECTOR_END => crate::trap::handle_irq(tf.vector as _, false),
        _ => {
            panic!(
                "Unhandled exception {} (error_code = {:#x}) @ {:#x}:\n{:#x?}",
                tf.vector, tf.error_code, tf.rip, tf
            );
        }
    }
    #[cfg(feature = "monolithic")]
    if tf.is_user() {
        crate::trap::handle_signals();
    }
}
