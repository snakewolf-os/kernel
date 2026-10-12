#![no_std]
#![no_main]

mod print;

use core::arch::asm;
use limine::request::FramebufferRequest;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

unsafe fn serial_write_byte(byte: u8) {
    outb(0x3F8, byte);
}

unsafe fn serial_print(s: &str) {
    for b in s.bytes() {
        serial_write_byte(b);
    }
}

/// Kernel entry point
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    serial_print("[KERNEL] kmain started!\n");

    // フレームバッファの取得
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().first() {
            let fb = *framebuffer;

            // printモジュールの初期化
            print::FramebufferWriter::init(
                fb.address(),
                fb.pitch,
                fb.width as usize,
                fb.height as usize,
            );

            // 念願の println! マクロを使用！
            println!("=== SnakeWolf OS Booted Successfully ===");
            println!("OS: SnakeWolf OS (x86_64)");
            println!("Status: READY");
        }
    }

    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}
