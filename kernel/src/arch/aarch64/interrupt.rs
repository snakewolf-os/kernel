//! AArch64 interrupt implementation.

/// Initialize the AArch64 interrupt subsystem.
pub fn init() {
    // Interrupt controller initialization will be added here later.
}

/// Enable interrupts.
pub fn enable() {
    unsafe {
        core::arch::asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags));
    }
}

/// Disable interrupts.
pub fn disable() {
    unsafe {
        core::arch::asm!("msr daifset, #2", options(nomem, nostack, preserves_flags));
    }
}
