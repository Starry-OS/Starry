pub mod apic;

pub mod dtables;
mod uart16550;

pub mod mem;
pub mod misc;
pub mod time;

#[cfg(feature = "irq")]
pub mod irq {
    pub use super::apic::*;
}

pub mod console {
    pub use super::uart16550::*;
}

pub use dtables::set_tss_stack_top;

/// Initializes the platform devices for the primary CPU.
pub fn platform_init() {
    self::apic::init_primary();
    self::time::init_primary();
}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {
    self::apic::init_secondary();
    self::time::init_secondary();
}

/// Returns the name of the platform.
pub fn platform_name() -> &'static str {
    "x86_pc"
}
