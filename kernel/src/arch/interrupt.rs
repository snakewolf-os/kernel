//! SWOS interrupt abstraction.
//!
//! Architecture-specific interrupt handling is delegated
//! to the current architecture backend.

/// Initialize the interrupt subsystem.
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::interrupt::init();

    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::interrupt::init();

    #[cfg(target_arch = "riscv64")]
    crate::arch::riscv64::interrupt::init();

    #[cfg(target_arch = "loongarch64")]
    crate::arch::loongarch64::interrupt::init();
}

/// Enable interrupts.
pub fn enable() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::interrupt::enable();

    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::interrupt::enable();

    #[cfg(target_arch = "riscv64")]
    crate::arch::riscv64::interrupt::enable();

    #[cfg(target_arch = "loongarch64")]
    crate::arch::loongarch64::interrupt::enable();
}

/// Disable interrupts.
pub fn disable() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::interrupt::disable();

    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::interrupt::disable();

    #[cfg(target_arch = "riscv64")]
    crate::arch::riscv64::interrupt::disable();

    #[cfg(target_arch = "loongarch64")]
    crate::arch::loongarch64::interrupt::disable();
}
