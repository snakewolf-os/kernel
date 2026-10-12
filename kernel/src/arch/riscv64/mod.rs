pub mod cpu;

/// Initialize RISC-V architecture support.
pub fn init() {
    cpu::init();
}
