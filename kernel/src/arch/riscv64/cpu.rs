/// RISC-V 64 CPU implementation.

/// Initialize the RISC-V CPU subsystem.
pub fn init() {
    // CPU initialization will be added here later.
}

/// Halt the RISC-V CPU.
///
/// `wfi` waits for an interrupt.
pub fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
        }
    }
}
