use std::sync::OnceLock;
use core::fmt;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

const COLS: usize = 80;
const ROWS: usize = 25;
const CHAR_H: u32 = 16;

struct CanvasBuffer {
    chars: [[u8; COLS]; ROWS],
    col: usize,
    row: usize,
}

impl CanvasBuffer {
    const fn new() -> Self {
        Self {
            chars: [[b' '; COLS]; ROWS],
            col: 0,
            row: 0,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= COLS {
                    self.new_line();
                }
                self.chars[self.row][self.col] = byte;
                self.col += 1;
            }
        }
    }

    fn new_line(&mut self) {
        if self.row < ROWS - 1 {
            self.row += 1;
        } else {
            for r in 0..ROWS - 1 {
                self.chars[r] = self.chars[r + 1];
            }
            self.chars[ROWS - 1] = [b' '; COLS];
        }
        self.col = 0;
    }

    fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.chars[self.row][self.col] = b' ';
        }
    }
}

static CANVAS_BUF: spin::Mutex<CanvasBuffer> = spin::Mutex::new(CanvasBuffer::new());

static CTX: OnceLock<CanvasRenderingContext2d> = OnceLock::new();

pub fn init_canvas() {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    let canvas = document
        .get_element_by_id("vga")
        .expect("no #vga element")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("#vga is not a canvas");

    canvas.set_width(800);
    canvas.set_height(ROWS as u32 * CHAR_H);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.set_fill_style(&JsValue::from_str("black"));
    ctx.fill_rect(0.0, 0.0, 800.0, (ROWS as u32 * CHAR_H) as f64);
    ctx.set_font("16px monospace");
    ctx.set_text_baseline("top");

    CTX.set(ctx).ok();
}

fn render_all(ctx: &CanvasRenderingContext2d) {
    let buf = CANVAS_BUF.lock();

    ctx.set_fill_style(&JsValue::from_str("black"));
    ctx.fill_rect(0.0, 0.0, 800.0, (ROWS as u32 * CHAR_H) as f64);
    ctx.set_fill_style(&JsValue::from_str("#FFFF00"));
    ctx.set_font("16px monospace");
    ctx.set_text_baseline("top");

    for row in 0..ROWS {
        let y = (row as u32 * CHAR_H) as f64;
        if let Ok(row_str) = core::str::from_utf8(&buf.chars[row]) {
            let _ = ctx.fill_text(row_str, 0.0, y);
        }
    }
}

pub fn _print(args: fmt::Arguments) {
    let mut s = alloc::string::String::new();
    let _ = core::fmt::write(&mut s, args);

    {
        let mut buf = CANVAS_BUF.lock();
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => buf.write_byte(byte),
                _ => buf.write_byte(b'?'),
            }
        }
    }

    if let Some(ctx) = CTX.get() {
        render_all(ctx);
    }
}

pub fn backspace() {
    CANVAS_BUF.lock().backspace();

    if let Some(ctx) = CTX.get() {
        render_all(ctx);
    }
}
