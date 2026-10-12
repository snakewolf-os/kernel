pub mod cpu;
pub mod interrupt;

pub fn init() {
    cpu::init();
    interrupt::init();
}
