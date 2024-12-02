//! The entry of trap handler, which will handle the context switch and distribute the trap to the corresponding function.
//!
//! The trapframe is defined in `axhal` module, and the trap handler is defined in `axtrap` module.
//!
//! The reason for the separation of definition and implementation is to ensure a one-way dependence on the calling relationship.
//!
//! The trap handler need to dispatch the trap to the corresponding functions, which use `trapframe` struct to finish their functions.
//!
//! If you want to use this module, please ensure that the trap context layout you pass to the handler is
//! in consistent or compatible with the trap frame layout defined in [axhal](https://github.com/Azure-stars/Starry/tree/main/modules/axhal).
//!
//! Otherwise, you may need to modify the unsafe assembly code of this module to complete the privilege switching function
#![cfg_attr(not(test), no_std)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(const_option)]
#![feature(doc_auto_cfg)]

mod arch;

mod trap;

pub use arch::init_interrupt;
