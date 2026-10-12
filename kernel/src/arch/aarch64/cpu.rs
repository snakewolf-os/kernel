/// AArch64 CPU implementation.

/// Initialize the AArch64 CPU subsystem.
pub fn init() {
    // CPU initialization will be added here later.
}

/// Halt the AArch64 CPU.
///
/// `wfi` waits for an interrupt or exception.
pub fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
        }
    }
}
