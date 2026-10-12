use core::fmt::{self, Write};

use super::cursor::Cursor;
use super::font;

const CHAR_SPACING: usize = 1;
const LINE_SPACING: usize = 2;
const TAB_SIZE: usize = 4;
const BYTES_PER_PIXEL: usize = 4;

pub struct FramebufferWriter {
    address: *mut u8,
    pitch: usize,
    width: usize,
    height: usize,

    cursor: Cursor,

    foreground: u32,
    background: u32,
}

impl FramebufferWriter {
    /// FramebufferWriterを生成
    ///
    /// # Safety
    ///
    /// `address` は有効なFramebufferを指している必要があります。
    pub unsafe fn new(address: *mut u8, pitch: usize, width: usize, height: usize) -> Self {
        Self {
            address,
            pitch,
            width,
            height,

            cursor: Cursor::new(),

            foreground: 0xFFFF_FFFF,
            background: 0x0000_0000,
        }
    }

    /// 画面を背景色でクリア
    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, self.background);
            }
        }

        self.cursor.reset();
    }

    /// 前景色設定
    pub fn set_color(&mut self, color: u32) {
        self.foreground = color;
    }

    /// 背景色設定
    pub fn set_background(&mut self, color: u32) {
        self.background = color;
    }

    /// ピクセルを書き込む
    fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = y * self.pitch + x * BYTES_PER_PIXEL;

        unsafe {
            self.address.add(offset).cast::<u32>().write_volatile(color);
        }
    }

    /// 文字を描画
    fn draw_char(&mut self, c: char) {
        let ascii = if c.is_ascii() { c as u8 } else { b'?' };

        let bitmap = font::get(ascii);

        let x = self.cursor.x;
        let y = self.cursor.y;

        for row in 0..font::FONT_HEIGHT {
            let bits = bitmap[row];

            for col in 0..font::FONT_WIDTH {
                let bit = 1 << (7 - col);

                let color = if bits & bit != 0 {
                    self.foreground
                } else {
                    self.background
                };

                self.put_pixel(x + col, y + row, color);
            }
        }

        self.cursor.x += font::FONT_WIDTH + CHAR_SPACING;

        // 右端に到達したら自動改行
        if self.cursor.x + font::FONT_WIDTH > self.width {
            self.new_line();
        }
    }

    /// 改行
    fn new_line(&mut self) {
        self.cursor.x = 0;

        self.cursor.y += font::FONT_HEIGHT + LINE_SPACING;

        // 画面下端に到達したらスクロール
        if self.cursor.y + font::FONT_HEIGHT > self.height {
            self.scroll();
        }
    }

    /// 画面を1行上へスクロール
    fn scroll(&mut self) {
        let line_height = font::FONT_HEIGHT + LINE_SPACING;

        if line_height >= self.height {
            self.clear();
            return;
        }

        // 既存の画面を1行分上へ移動
        for y in line_height..self.height {
            for x in 0..self.width {
                let source_offset = y * self.pitch + x * BYTES_PER_PIXEL;

                let destination_offset = (y - line_height) * self.pitch + x * BYTES_PER_PIXEL;

                unsafe {
                    let pixel = self
                        .address
                        .add(source_offset)
                        .cast::<u32>()
                        .read_volatile();

                    self.address
                        .add(destination_offset)
                        .cast::<u32>()
                        .write_volatile(pixel);
                }
            }
        }

        // 最下段をクリア
        for y in (self.height - line_height)..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, self.background);
            }
        }

        // 移動した分だけカーソルを戻す
        self.cursor.y -= line_height;
    }

    /// カーソル位置取得
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor.x, self.cursor.y)
    }

    /// カーソル位置設定
    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.cursor.set(x, y);
    }
}

impl Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            match c {
                '\n' => {
                    self.new_line();
                }

                '\r' => {
                    self.cursor.x = 0;
                }

                '\t' => {
                    for _ in 0..TAB_SIZE {
                        self.draw_char(' ');
                    }
                }

                '\x08' => {
                    if self.cursor.x >= font::FONT_WIDTH + CHAR_SPACING {
                        self.cursor.x -= font::FONT_WIDTH + CHAR_SPACING;
                    }
                }

                _ => {
                    self.draw_char(c);
                }
            }
        }

        Ok(())
    }
}
