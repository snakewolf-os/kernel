//! Boot abstraction.
//!
//! Kernel code should use this module instead of accessing
//! the bootloader implementation directly.

mod limine;

/// Framebuffer information provided by the bootloader.
#[derive(Clone, Copy)]
pub struct FramebufferInfo {
    pub address: *mut u8,
    pub pitch: usize,
    pub width: usize,
    pub height: usize,
}

/// Boot information available to the kernel.
#[derive(Clone, Copy)]
pub struct BootInfo {
    pub framebuffer: Option<FramebufferInfo>,
}

/// Initialize the boot layer.
///
/// The current implementation uses Limine internally.
pub fn init() -> BootInfo {
    limine::init()
}
