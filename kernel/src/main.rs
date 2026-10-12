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

/// x86 I/Oポートへ1バイト送信
#[inline]
unsafe fn outb(port: u16, val: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// シリアルポートへ1バイト送信
unsafe fn serial_write_byte(byte: u8) {
    unsafe {
        outb(0x3F8, byte);
    }
}

/// シリアルポートへ文字列を出力
unsafe fn serial_print(s: &str) {
    for b in s.bytes() {
        unsafe {
            serial_write_byte(b);
        }
    }
}

/// Kernel entry point
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    unsafe {
        serial_print("[KERNEL] kmain started!\n");
    }

    // LimineからFramebufferを取得
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().first() {
            let fb = *framebuffer;

            /*
             * Limine 0.6.5 の address() は *mut ()
             * を返すため、FramebufferWriterが扱う
             * *mut u8 に変換する。
             */
            let address = fb.address() as *mut u8;

            // Print subsystem初期化
            unsafe {
                print::init(
                    address,
                    fb.pitch as usize,
                    fb.width as usize,
                    fb.height as usize,
                );
            }

            println!(" /\\_/\\  SWOS Kernel");
            println!("( o.o ) Version: Beta 0.0.2");
            println!(" > ^ <  Made by SnakeWolf in Japan");
            println!();

            println!("Print subsystem initialized.");
            println!("ASCII font: 128 characters.");
            println!("Framebuffer: {}x{}", fb.width, fb.height);
            println!();

            println!("Scrolling test:");
            println!("----------------");

            // for i in 0..100 {
            //     println!("Line {:03}", i);
            // }
        }
    }

    hcf();
}

/// CPUを停止
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
