//! Signal ucontext types and operations.

/// The thread is currently executing on the alternate signal stack.
pub const SS_ONSTACK: u32 = 1;
/// The alternate signal stack is disabled.
pub const SS_DISABLE: u32 = 2;
/// The alternate signal stack has been marked to be autodisarmed as described above.
pub const SS_AUTODISARM: u32 = 1_u32 << 31;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        mod riscv;
        pub use self::riscv::*;
    } else if #[cfg(target_arch = "aarch64")]{
        mod aarch64;
        pub use self::aarch64::*;
    }
}
