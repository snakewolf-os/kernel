/// Common CPU abstraction.
///
/// This module intentionally contains no architecture-specific
/// instructions. The actual implementation is delegated to the
/// current architecture backend.

/// Initialize the CPU subsystem.
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::cpu::init();

    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::cpu::init();

    #[cfg(target_arch = "riscv64")]
    crate::arch::riscv64::cpu::init();

    #[cfg(target_arch = "loongarch64")]
    crate::arch::loongarch64::cpu::init();
}

/// Halt the CPU forever.
///
/// The implementation is provided by the architecture backend.
pub fn halt() -> ! {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::cpu::halt();

    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::cpu::halt();

    #[cfg(target_arch = "riscv64")]
    crate::arch::riscv64::cpu::halt();

    #[cfg(target_arch = "loongarch64")]
    crate::arch::loongarch64::cpu::halt();

    // Unsupported architecture fallback.
    loop {
        core::hint::spin_loop();
    }
}
