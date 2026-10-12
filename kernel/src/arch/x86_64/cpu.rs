/// x86_64 CPU implementation.

/// Initialize the x86_64 CPU subsystem.
pub fn init() {
    // CPU initialization will be added here later.
}

/// Halt the x86_64 CPU forever.
pub fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}
