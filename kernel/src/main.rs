#![no_std]
#![no_main]

mod arch;
mod boot;
mod print;

use core::panic::PanicInfo;

/// Kernel entry point.
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    /*
     * Bootloader initialization.
     *
     * The kernel does not know which bootloader is being used.
     * The boot layer handles that internally.
     */
    let boot_info = boot::init();

    /*
     * Initialize framebuffer-based console.
     */
    if let Some(framebuffer) = boot_info.framebuffer {
        unsafe {
            print::init(
                framebuffer.address,
                framebuffer.pitch,
                framebuffer.width,
                framebuffer.height,
            );
        }

        println!(" /\\_/\\  SWOS Kernel");
        println!("( o.o ) Version: Beta 0.0.2");
        println!(" > ^ <  Made by SnakeWolf in Japan");
        println!();

        println!("Print subsystem initialized.");
        println!("ASCII font: 128 characters.");
        println!("Framebuffer: {}x{}", framebuffer.width, framebuffer.height);
        println!();

        println!("Initializing architecture...");

        arch::init();

        println!("Architecture initialized.");
        println!();

        println!("Boot abstraction initialized.");
        println!("Limine framebuffer detected.");
        println!();

        println!("Kernel initialization complete.");
    }

    /*
     * No framebuffer was provided.
     *
     * There is currently no serial/debug console abstraction,
     * so the kernel simply halts.
     */
    arch::cpu::halt();
}

/// Panic handler.
#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &PanicInfo) -> ! {
    arch::cpu::halt();
}
