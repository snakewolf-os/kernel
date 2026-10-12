/// LoongArch64 CPU implementation.

/// Initialize the LoongArch64 CPU subsystem.
pub fn init() {
    // CPU initialization will be added here later.
}

/// Halt the LoongArch64 CPU.
///
/// A dedicated idle instruction can be introduced once
/// the low-level LoongArch CPU layer is implemented.
pub fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
