pub mod console;
pub mod mem;
pub mod misc;
pub mod time;

#[cfg(feature = "irq")]
pub mod irq;

/// Initializes the platform devices for the primary CPU.
///
/// For example, the interrupt controller and the timer.
pub fn platform_init() {
    #[cfg(feature = "irq")]
    self::irq::init_percpu();
    self::time::init_percpu();
}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {
    #[cfg(feature = "irq")]
    self::irq::init_percpu();
    self::time::init_percpu();
}

/// Returns the name of the platform.
pub fn platform_name() -> &'static str {
    "riscv64_qemu_virt"
}
