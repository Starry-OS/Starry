#[cfg(not(any(
    all(platform_family = "aarch64-raspi"),
    all(platform_family = "aarch64-phytiumpi"),
)))]
pub mod psci;

cfg_if::cfg_if! {
    if #[cfg(feature = "irq")] {
        mod gic;
        pub mod irq {
            pub use super::gic::*;
        }
    }
}

mod generic_timer;
pub mod time {
    pub use super::generic_timer::*;
}

cfg_if::cfg_if! {
    if #[cfg(any(platform_family = "aarch64-bsta1000b", platform_family= "aarch64-rk3588j"))] {
        mod dw_apb_uart;
        pub mod console {
            pub use super::dw_apb_uart::*;
        }
    } else if #[cfg(any(platform_family = "aarch64-raspi", platform_family = "aarch64-qemu-virt",platform_family = "aarch64-phytiumpi"))] {
        mod pl011;
        pub mod console {
            pub use super::pl011::*;
        }
    }
}

pub mod mem;

/// Initializes the platform devices for the primary CPU.
///
/// For example, the interrupt controller and the timer.
pub fn platform_init() {
    #[cfg(feature = "irq")]
    crate::platform::irq::init_primary();
    crate::platform::time::init_percpu();
    #[cfg(feature = "irq")]
    crate::platform::console::init_irq();
}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {
    #[cfg(feature = "irq")]
    crate::platform::irq::init_secondary();
    crate::platform::time::init_percpu();
}

/// Returns the name of the platform.
pub fn platform_name() -> &'static str {
    of::machin_name().unwrap_or(axconfig::PLATFORM)
}
