//! LoongArch64 interrupt implementation.

use super::super::interrupt::InterruptState;

/// Initialize the LoongArch64 interrupt subsystem.
pub fn init() {
    // Interrupt controller initialization will be implemented later.
}

/// Save the current interrupt state and disable interrupts.
#[inline]
pub fn save_and_disable() -> InterruptState {
    InterruptState::new(0)
}

/// Restore a previously saved interrupt state.
#[inline]
pub fn restore(_state: InterruptState) {
    // TODO: Implement LoongArch64 interrupt state restoration.
}

/// Enable interrupts.
#[inline]
pub fn enable() {
    // TODO: Implement LoongArch64 interrupt enable.
}

/// Disable interrupts.
#[inline]
pub fn disable() {
    // TODO: Implement LoongArch64 interrupt disable.
}
