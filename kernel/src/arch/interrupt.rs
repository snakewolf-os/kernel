//! SWOS interrupt abstraction.
//!
//! The common kernel layer uses this module without knowing
//! which CPU architecture is currently running.

/// Saved interrupt state.
///
/// The actual meaning of `value` is architecture-specific.
///
/// The common kernel must never inspect this value directly.
#[derive(Clone, Copy)]
pub struct InterruptState {
    value: u64,
}

impl InterruptState {
    /// Create an architecture-independent interrupt state.
    #[inline]
    pub(crate) const fn new(value: u64) -> Self {
        Self { value }
    }

    /// Get the architecture-specific raw value.
    #[inline]
    pub(crate) const fn value(self) -> u64 {
        self.value
    }
}

/// Initialize the interrupt subsystem.
#[inline]
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

/// Save the current interrupt state and disable interrupts.
///
/// This is used by kernel synchronization primitives that must
/// not be interrupted while holding a lock.
#[inline]
pub fn save_and_disable() -> InterruptState {
    #[cfg(target_arch = "x86_64")]
    {
        return crate::arch::x86_64::interrupt::save_and_disable();
    }

    #[cfg(target_arch = "aarch64")]
    {
        return crate::arch::aarch64::interrupt::save_and_disable();
    }

    #[cfg(target_arch = "riscv64")]
    {
        return crate::arch::riscv64::interrupt::save_and_disable();
    }

    #[cfg(target_arch = "loongarch64")]
    {
        return crate::arch::loongarch64::interrupt::save_and_disable();
    }
}

/// Restore a previously saved interrupt state.
#[inline]
pub fn restore(state: InterruptState) {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::interrupt::restore(state);

    #[cfg(target_arch = "aarch64")]
    crate::arch::aarch64::interrupt::restore(state);

    #[cfg(target_arch = "riscv64")]
    crate::arch::riscv64::interrupt::restore(state);

    #[cfg(target_arch = "loongarch64")]
    crate::arch::loongarch64::interrupt::restore(state);
}

/// Enable interrupts.
#[inline]
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
#[inline]
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
