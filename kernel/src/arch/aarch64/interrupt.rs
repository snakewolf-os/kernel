//! AArch64 interrupt implementation.

use super::super::interrupt::InterruptState;

/// Initialize the AArch64 interrupt subsystem.
pub fn init() {
    // Interrupt controller initialization will be implemented later.
}

/// Save the current interrupt state and disable interrupts.
#[inline]
pub fn save_and_disable() -> InterruptState {
    let daif: u64;

    unsafe {
        core::arch::asm!(
            "mrs {}, daif",
            out(reg) daif,
            options(nomem, preserves_flags)
        );

        core::arch::asm!("msr daifset, #2", options(nomem, nostack, preserves_flags));
    }

    InterruptState::new(daif)
}

/// Restore a previously saved interrupt state.
#[inline]
pub fn restore(state: InterruptState) {
    let daif = state.value();

    unsafe {
        core::arch::asm!(
            "msr daif, {}",
            in(reg) daif,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Enable IRQ interrupts.
#[inline]
pub fn enable() {
    unsafe {
        core::arch::asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags));
    }
}

/// Disable IRQ interrupts.
#[inline]
pub fn disable() {
    unsafe {
        core::arch::asm!("msr daifset, #2", options(nomem, nostack, preserves_flags));
    }
}
