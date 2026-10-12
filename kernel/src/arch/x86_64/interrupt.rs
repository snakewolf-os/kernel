//! x86_64 interrupt implementation.

/// Initialize the x86_64 interrupt subsystem.
pub fn init() {
    // Interrupt Descriptor Table will be initialized here.
}

/// Enable maskable interrupts.
pub fn enable() {
    unsafe {
        core::arch::asm!("sti", options(nomem, nostack, preserves_flags));
    }
}

/// Disable maskable interrupts.
pub fn disable() {
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
    }
}
