//! x86_64 interrupt implementation.

use super::super::interrupt::InterruptState;

/// Initialize the x86_64 interrupt subsystem.
pub fn init() {
    // IDT initialization will be implemented next.
}

/// Save the current interrupt state and disable interrupts.
#[inline]
pub fn save_and_disable() -> InterruptState {
    let flags: u64;

    unsafe {
        core::arch::asm!(
            "pushfq",
            "pop {}",
            out(reg) flags,
            options(nomem, preserves_flags)
        );

        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
    }

    InterruptState::new(flags)
}

/// Restore a previously saved interrupt state.
#[inline]
pub fn restore(state: InterruptState) {
    let flags = state.value();

    unsafe {
        core::arch::asm!(
            "push {}",
            "popfq",
            in(reg) flags,
            options(nomem, preserves_flags)
        );
    }
}

/// Enable maskable interrupts.
#[inline]
pub fn enable() {
    unsafe {
        core::arch::asm!("sti", options(nomem, nostack, preserves_flags));
    }
}

/// Disable maskable interrupts.
#[inline]
pub fn disable() {
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
    }
}
