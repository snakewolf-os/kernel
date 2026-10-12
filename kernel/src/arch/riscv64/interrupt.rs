//! RISC-V 64 interrupt implementation.

use super::super::interrupt::InterruptState;

/// Initialize the RISC-V interrupt subsystem.
pub fn init() {
    // Interrupt controller initialization will be implemented later.
}

/// Save the current interrupt state and disable interrupts.
#[inline]
pub fn save_and_disable() -> InterruptState {
    let status: u64;

    unsafe {
        core::arch::asm!(
            "csrrc {}, sstatus, 0x2",
            out(reg) status,
            options(nomem, preserves_flags)
        );
    }

    InterruptState::new(status)
}

/// Restore a previously saved interrupt state.
#[inline]
pub fn restore(state: InterruptState) {
    let status = state.value();

    unsafe {
        core::arch::asm!(
            "csrw sstatus, {}",
            in(reg) status,
            options(nomem, preserves_flags)
        );
    }
}

/// Enable supervisor interrupts.
#[inline]
pub fn enable() {
    unsafe {
        core::arch::asm!(
            "csrsi sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Disable supervisor interrupts.
#[inline]
pub fn disable() {
    unsafe {
        core::arch::asm!(
            "csrci sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        );
    }
}
