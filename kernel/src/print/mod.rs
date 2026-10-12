pub mod cursor;
pub mod font;
pub mod lock;
pub mod writer;

use core::fmt;

use lock::SpinLock;
use writer::FramebufferWriter;

/// グローバルFramebuffer Writer
///
/// static mutは使用しない。
///
/// SpinLock + IRQ無効化によって
/// Writerへのアクセスを排他制御する。
static WRITER: SpinLock<Option<FramebufferWriter>> = SpinLock::new(None);

/// フレームバッファ出力を初期化する
///
/// # Safety
///
/// `address` が有効なFramebufferを指している必要がある。
pub unsafe fn init(address: *mut u8, pitch: usize, width: usize, height: usize) {
    let writer = unsafe { FramebufferWriter::new(address, pitch, width, height) };

    let mut guard = WRITER.lock_irqsave();

    *guard = Some(writer);

    if let Some(writer) = guard.as_mut() {
        writer.clear();
    }
}

/// print! / println! の内部処理
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    let mut guard = WRITER.lock_irqsave();

    if let Some(writer) = guard.as_mut() {
        let _ = writer.write_fmt(args);
    }
}

/// 画面をクリアする
pub fn clear() {
    let mut guard = WRITER.lock_irqsave();

    if let Some(writer) = guard.as_mut() {
        writer.clear();
    }
}

/// 文字色を変更する
pub fn set_color(color: u32) {
    let mut guard = WRITER.lock_irqsave();

    if let Some(writer) = guard.as_mut() {
        writer.set_color(color);
    }
}

/// 背景色を変更する
pub fn set_background(color: u32) {
    let mut guard = WRITER.lock_irqsave();

    if let Some(writer) = guard.as_mut() {
        writer.set_background(color);
    }
}

/// カーソル位置を設定する
pub fn set_cursor(x: usize, y: usize) {
    let mut guard = WRITER.lock_irqsave();

    if let Some(writer) = guard.as_mut() {
        writer.set_cursor(x, y);
    }
}

/// カーソル位置を取得する
pub fn cursor_position() -> (usize, usize) {
    let guard = WRITER.lock_irqsave();

    if let Some(writer) = guard.as_ref() {
        return writer.cursor_position();
    }

    (0, 0)
}

/// print! マクロ
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(
            core::format_args!($($arg)*)
        )
    };
}

/// println! マクロ
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
