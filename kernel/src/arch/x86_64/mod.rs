pub mod cpu;

/// Initialize x86_64 architecture support.
pub fn init() {
    cpu::init();
}
