//
//! Interactive ZX Spectrum paint application.
//
// 2735 bytes .tap

#![no_std]
#![no_main]

use devela::{Arch, Ptr, is, lets, use_as, whilst};
use devela_micros::{ComputerSpectrum48 as Spectrum, ProcessorZ80 as Z80, devela, spectrum_main};
use_as! {+Spectrum: devela_micros::{Attribute as Attr, Color, Key, Keys, UlaOut }}

const BRUSH_MAX_RADIUS: u8 = 8; // max diameter = radius * 2 + 1
const ROM_FONT_ADDR: u16 = 0x3D00;

spectrum_main! {
    let attr = Attr::new(Color::White, Color::Black).bright();

    unsafe {
        Spectrum::clear_bitmap();
        Spectrum::fill_attributes(attr);
        Spectrum::set_border(Color::Blue);
    }

    /* state */

    let movekey = MoveKeys::WSAD;

    let (mut x, mut y) = (128u8, 96u8); // cursor coordinates

    let mut move_phase = 0u8; // phase accumulator for speed control
    let mut move_rate = 50u8; // approx. 50px/s (maximum)

    let mut prev = Keys::NONE;
    let mut erase = false;
    let mut attr = Attr::new(Color::White, Color::Black).bright();
    let mut brush = 0u8; // brush radius

    let mut hud_dirty = true;

    let mut ula = UlaOut::new(attr.ink());

    unsafe { toggle_cursor(x, y) }; // show cursor at the start

    loop {
        Z80::halt();
        let keys = unsafe { Spectrum::read_keys() };

        move_phase += move_rate; // once per ~50 Hz frame
        let repeat = is! { move_phase >= 50, { move_phase -= 50; true }, false };

        unsafe { toggle_cursor(x, y) }; // hide cursor overlay

        /* drawing */

        // cursor movement
        macro_rules! move_if [ ($k:expr, $bound:expr => $step:expr) => {
            is![keys.is_pressed($k) && (keys.just_pressed(&prev, $k) || repeat) && $bound, $step];
        }];
        move_if!(movekey.up, y > 0 => y -= 1);
        move_if!(movekey.down, y < Spectrum::SCREEN_Y_MAX => y += 1);
        move_if!(movekey.left, x > 0 => x -= 1);
        move_if!(movekey.right, x < Spectrum::SCREEN_X_MAX => x += 1);

        // Color selection. Ink or paper depending on the Shift key status
        let selecting_paper = keys.is_pressed(Key::CapsShift);
        let current = is![selecting_paper, attr.paper(), attr.ink()];
        let color = select_color(&keys, current);
        attr = is![selecting_paper, attr.with_paper(color), attr.with_ink(color)];

        // show ink color in border
        ula = ula.with_border(attr.ink());
        unsafe { Spectrum::write_ula(ula) };

        // Z, X: change brush size
        if keys.just_pressed(&prev, Key::Z) && brush > 0 {
            brush -= 1;
            hud_dirty = true;
        }
        if keys.just_pressed(&prev, Key::X) && brush < BRUSH_MAX_RADIUS {
            brush += 1;
            hud_dirty = true;
        }
        // N, M: change speed
        if keys.just_pressed(&prev, Key::N) && move_rate > 1 {
            move_rate -= 1;
            hud_dirty = true;
        }
        if keys.just_pressed(&prev, Key::M) && move_rate < 50 {
            move_rate += 1;
            hud_dirty = true;
        }
        // E: toggle eraser
        if keys.just_pressed(&prev, Key::E) {
            erase = !erase;
            hud_dirty = true;
        }
        // Enter: fill paper
        if keys.just_pressed(&prev, Key::Enter) {
            unsafe {
                fill_paper(attr.paper());
            }
            hud_dirty = true;
        }
        // Space: paint ink brush
        if keys.is_pressed(Key::Space) {
            unsafe {
                paint_brush(x, y, brush, !erase, attr.ink());
            }
        }

        /* text */

        if hud_dirty {
            border_probe! { (ula)
                unsafe {
                    screen_draw_text(1, 1, concat![
                        "WASD  MOVE CURSOR \n",
                        "0-7   COLOR INK   \n",
                        "SHIFT+0-7 PAPER   \n",
                        "SPACE PAINT INK   \n",
                        "ENTER FILL PAPER  \n",
                        "Z/X   BRUSH SIZE  \n",
                        "N/M   SPEED CHANGE\n",
                        "B     BEEP!"
                    ], Attr::new(Color::White, Color::Black).bright());
                }
            }
            hud_dirty = false;
        }

        unsafe { toggle_cursor(x, y) }; // show cursor overlay again

        /* sound */

        is! { keys.just_pressed(&prev, Key::B), unsafe { beep(ula) } }

        prev = keys;
    }
}

/* subroutines: drawing */

unsafe fn toggle_cursor(x: u8, y: u8) {
    unsafe {
        Spectrum::toggle_pixel(x, y);
        is! { x > 0, Spectrum::toggle_pixel(x - 1, y) }
        is! { x < Spectrum::SCREEN_X_MAX, Spectrum::toggle_pixel(x + 1, y) }
        is! { y > 0, Spectrum::toggle_pixel(x, y - 1) }
        is! { y < Spectrum::SCREEN_Y_MAX, Spectrum::toggle_pixel(x, y + 1) }
    }
}

unsafe fn paint_brush(x: u8, y: u8, radius: u8, set: bool, ink: Color) {
    let x0 = x.saturating_sub(radius) as u16;
    let x1 = x.saturating_add(radius).min(Spectrum::SCREEN_X_MAX) as u16;
    let y0 = y.saturating_sub(radius) as u16;
    let y1 = y.saturating_add(radius).min(Spectrum::SCREEN_Y_MAX) as u16;
    whilst! { py in y0, ..=y1; {
        whilst! { px in x0, ..=x1; {
            unsafe { Spectrum::set_pixel(px as u8, py as u8, set) };
        }}
    }}
    if set {
        lets! { c0 = x0 >> 3, c1 = x1 >> 3, r0 = y0 >> 3, r1 = y1 >> 3 }
        whilst! { row in r0, ..=r1; {
            whilst! { column in c0, ..=c1; {
                let offset = row * Spectrum::SCREEN_COLUMNS as u16 + column;
                let old = Attr::from_u8(unsafe { Spectrum::read_attribute(offset) });
                unsafe { Spectrum::write_attribute(offset, old.with_ink(ink).to_u8()); }
            }}
        }}
    }
}

unsafe fn fill_paper(paper: Color) {
    whilst! { offset in 0u16..Spectrum::SCREEN_ATTR_LEN; {
        let old = Attr::from_u8(unsafe { Spectrum::read_attribute(offset) });
        let new = old.with_paper(paper);
        unsafe { Spectrum::write_attribute(offset, new.to_u8()) };
    }}
}

#[inline(never)]
#[rustfmt::skip]
fn select_color(keys: &Keys, current: Color) -> Color {
         if keys.is_pressed(Key::Num1) { Color::Blue }
    else if keys.is_pressed(Key::Num2) { Color::Red }
    else if keys.is_pressed(Key::Num3) { Color::Magenta }
    else if keys.is_pressed(Key::Num4) { Color::Green }
    else if keys.is_pressed(Key::Num5) { Color::Cyan }
    else if keys.is_pressed(Key::Num6) { Color::Yellow }
    else if keys.is_pressed(Key::Num7) { Color::White }
    else if keys.is_pressed(Key::Num0) { Color::Black }
    else { current }
}

/* subroutines: text */

unsafe fn screen_draw_char(column: u8, row: u8, ch: u8, attr: Attr) {
    is! { column >= Spectrum::SCREEN_COLUMNS || row >= Spectrum::SCREEN_ROWS, return }
    let ch = is![(32..=127).contains(&ch), ch, b'?'];
    let glyph = ROM_FONT_ADDR + (ch as u16 - 32) * 8;
    let y = row * 8;
    whilst! { line in 0u8..8; {
        let ptr = Ptr::without_provenance::<u8>((glyph + line as u16) as usize);
        let bits = unsafe { Ptr::read(ptr) };
        unsafe { Spectrum::write_bitmap_byte(column, y + line, bits) };
    }}
    unsafe { Spectrum::write_cell_attribute(column, row, attr) };
}
unsafe fn screen_draw_text(column: u8, row: u8, text: &str, attr: Attr) {
    let bytes = text.as_bytes();
    let (mut x, mut y) = (column, row);
    whilst! { i in 0usize..bytes.len(); {
        let ch = bytes[i];
        if ch == b'\n' {
            x = column;
            y += 1;
        } else {
            is! { x >= Spectrum::SCREEN_COLUMNS, { x = column; y += 1; }}
            is! { y >= Spectrum::SCREEN_ROWS, return }
            unsafe { screen_draw_char(x, y, ch, attr) };
            x += 1;
        }
    }}
}

/* subroutines: sound */

#[inline(never)]
unsafe fn beep(ula: UlaOut) {
    whilst! { cycle in 0u8..48; {
        unsafe { Spectrum::write_ula(ula.with_ear(true)) };
        beep_delay();
        unsafe { Spectrum::write_ula(ula.with_ear(false)) };
        beep_delay();
    }}
    unsafe { Spectrum::write_ula(ula.with_ear(false)) };
}
#[inline(never)]
fn beep_delay() {
    whilst! { i in 0u8..192; {
        Arch::nop();
    }}
}

/* misc. structs */

#[derive(Clone, Copy)]
pub struct MoveKeys {
    up: Key,
    down: Key,
    left: Key,
    right: Key,
}
impl MoveKeys {
    const fn new(up: Key, down: Key, left: Key, right: Key) -> Self {
        Self { up, down, left, right }
    }
    pub const WSAD: Self = Self::new(Key::W, Key::S, Key::A, Key::D);
    pub const QAOP: Self = Self::new(Key::Q, Key::A, Key::O, Key::P);
}

/* debug helpers */

macro_rules! border_probe {
    (($ula:expr) $($tt:tt)*) => {
        unsafe { Spectrum::write_ula($ula.with_border(Color::Red)) };
        $($tt)*
        unsafe { Spectrum::write_ula($ula) };
    }
}
use border_probe;
