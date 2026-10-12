pub mod cursor;
pub mod font;
pub mod writer;

use core::fmt;

use writer::FramebufferWriter;

static mut WRITER: Option<FramebufferWriter> = None;

/// フレームバッファ出力を初期化する
pub unsafe fn init(address: *mut u8, pitch: usize, width: usize, height: usize) {
    let writer = unsafe { FramebufferWriter::new(address, pitch, width, height) };

    unsafe {
        let writer_ptr = &raw mut WRITER;

        *writer_ptr = Some(writer);

        if let Some(writer) = (*writer_ptr).as_mut() {
            writer.clear();
        }
    }
}

/// print! / println! の内部処理
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    unsafe {
        let writer_ptr = &raw mut WRITER;

        if let Some(writer) = (*writer_ptr).as_mut() {
            let _ = writer.write_fmt(args);
        }
    }
}

/// 画面をクリアする
pub fn clear() {
    unsafe {
        let writer_ptr = &raw mut WRITER;

        if let Some(writer) = (*writer_ptr).as_mut() {
            writer.clear();
        }
    }
}

/// 文字色を変更する
pub fn set_color(color: u32) {
    unsafe {
        let writer_ptr = &raw mut WRITER;

        if let Some(writer) = (*writer_ptr).as_mut() {
            writer.set_color(color);
        }
    }
}

/// 背景色を変更する
pub fn set_background(color: u32) {
    unsafe {
        let writer_ptr = &raw mut WRITER;

        if let Some(writer) = (*writer_ptr).as_mut() {
            writer.set_background(color);
        }
    }
}

/// カーソル位置を設定する
pub fn set_cursor(x: usize, y: usize) {
    unsafe {
        let writer_ptr = &raw mut WRITER;

        if let Some(writer) = (*writer_ptr).as_mut() {
            writer.set_cursor(x, y);
        }
    }
}

/// カーソル位置を取得する
pub fn cursor_position() -> (usize, usize) {
    unsafe {
        let writer_ptr = &raw const WRITER;

        if let Some(writer) = (*writer_ptr).as_ref() {
            return writer.cursor_position();
        }
    }

    (0, 0)
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(
            core::format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print::_print(
            core::format_args!("\n")
        )
    };

    ($($arg:tt)*) => {
        $crate::print::_print(
            core::format_args!(
                "{}\n",
                core::format_args!($($arg)*)
            )
        )
    };
}
