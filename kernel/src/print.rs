use core::fmt::{self, Write};

/// 画面描画とカーソルを管理するライター構造体
pub struct FramebufferWriter {
    address: *mut u32,
    pitch: u64,
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize,
}

// 複数の場所から安全に書き込めるようにするためのグローバルインスタンス保持用
static mut WRITER: Option<FramebufferWriter> = None;

impl FramebufferWriter {
    /// ライターの初期化
    pub unsafe fn init(address: *mut u32, pitch: u64, width: usize, height: usize) {
        let writer = FramebufferWriter {
            address,
            pitch,
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
        };
        WRITER = Some(writer);
    }

    /// 1文字を描画する内部関数
    fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.new_line();
            return;
        }

        // 画面端に来たら改行
        if self.cursor_x + 8 >= self.width {
            self.new_line();
        }

        // 画面下端を超えた場合の簡易処理（スクロールなし、または一番上に巻き戻し）
        if self.cursor_y + 16 >= self.height {
            self.cursor_x = 0;
            self.cursor_y = 0;
        }

        self.draw_font_char(self.cursor_x, self.cursor_y, c, 0xFFFF_FFFF);
        self.cursor_x += 9; // 文字幅 8px + 隙間 1px
    }

    fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 18; // フォント高さ 16px + 行間 2px
    }

    /// 8x16ドットの簡易フォント描画
    fn draw_font_char(&mut self, x: usize, y: usize, c: char, color: u32) {
        let font_bitmap = get_font_bitmap(c);

        for (row, &byte) in font_bitmap.iter().enumerate() {
            for col in 0..8 {
                if (byte & (1 << (7 - col))) != 0 {
                    let px = x + col;
                    let py = y + row;
                    if px < self.width && py < self.height {
                        let pixel_offset = (py as u64 * self.pitch) + (px as u64 * 4);
                        unsafe {
                            self.address
                                .add(pixel_offset as usize)
                                .cast::<u32>()
                                .write(color);
                        }
                    }
                }
            }
        }
    }
}

// core::fmt::Write トレイトを実装することで format! や print! が使えるようになる
impl Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

/// 外部から安全に書き込むためのグローバル関数
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        if let Some(writer) = &mut WRITER {
            writer.write_fmt(args).unwrap();
        }
    }
}

/// print! マクロの定義
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*))
    };
}

/// println! マクロの定義
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

/// 最小限のASCIIフォントデータ (8x16)
fn get_font_bitmap(c: char) -> &'static [u8; 16] {
    match c {
        'A' => &[
            0x18, 0x24, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        'B' => &[
            0x7C, 0x42, 0x42, 0x7C, 0x42, 0x42, 0x42, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        'C' => &[
            0x3C, 0x42, 0x40, 0x40, 0x40, 0x40, 0x42, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        'O' => &[
            0x3C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        'S' => &[
            0x3C, 0x42, 0x40, 0x3C, 0x02, 0x02, 0x42, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        // デフォルト（未定義文字は四角形）
        _ => &[
            0x7E, 0x81, 0xA5, 0x81, 0xBD, 0x99, 0x81, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    }
}
