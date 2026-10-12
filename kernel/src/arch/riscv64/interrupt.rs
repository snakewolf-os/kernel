//! RISC-V 64 interrupt implementation.

/// Initialize the RISC-V interrupt subsystem.
pub fn init() {
    // Interrupt controller initialization will be added here later.
}

/// Enable supervisor interrupts.
pub fn enable() {
    unsafe {
        core::arch::asm!(
            "csrsi sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Disable supervisor interrupts.
pub fn disable() {
    unsafe {
        core::arch::asm!(
            "csrci sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        );
    }
}
