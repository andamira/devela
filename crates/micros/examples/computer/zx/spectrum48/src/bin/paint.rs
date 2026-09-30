//
//! Interactive ZX Spectrum paint application.
//
// 3185 bytes .tap

#![no_std]
#![no_main]

use devela::{Arch, Ptr, is, lets, use_as, whilst};
use devela_micros::{ComputerSpectrum48 as Spectrum, ProcessorZ80 as Z80, devela, spectrum_main};
use_as! {+Spectrum: devela_micros::{Attribute as Attr, Color, Key, Keys, UlaOut }}

/* misc. settings */

const BRUSH_MAX_RADIUS: u8 = 8; // max diameter = radius * 2 + 1
const ROM_FONT_ADDR: u16 = 0x3D00;

/* canvas settings */

// for now: 6144-byte bitmap and 768-byte attribute area (256×192 screen and 32×24 cells)

const CANVAS_WIDTH: u16 = Spectrum::SCREEN_WIDTH;
const CANVAS_HEIGHT: u16 = Spectrum::SCREEN_HEIGHT;

const CANVAS_COLUMNS: u16 = CANVAS_WIDTH.div_ceil(8);
const CANVAS_ROWS: u16 = CANVAS_HEIGHT.div_ceil(8);

const CANVAS_BITMAP_LEN: usize = (CANVAS_COLUMNS * CANVAS_HEIGHT) as usize;
const CANVAS_ATTR_LEN: usize = (CANVAS_COLUMNS * CANVAS_ROWS) as usize;

static mut CANVAS_BITMAP: [u8; CANVAS_BITMAP_LEN] = [0; CANVAS_BITMAP_LEN];
static mut CANVAS_ATTRS: [u8; CANVAS_ATTR_LEN] = [0; CANVAS_ATTR_LEN];

/* HUD settings */

const HUD_C0: u8 = 1;
const HUD_C1: u8 = 18;
const HUD_R0: u8 = 1;
const HUD_R1: u8 = 8;

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

    let mut ula = UlaOut::new(attr.ink());

    unsafe {
        canvas_fill_attributes(attr);
        present_all();
        screen_draw_text(1, 1,
            concat![
                "WASD  MOVE CURSOR \n",
                "0-7   COLOR INK   \n",
                "SHIFT+0-7 PAPER   \n",
                "SPACE PAINT INK   \n",
                "ENTER FILL PAPER  \n",
                "Z/X   BRUSH SIZE  \n",
                "N/M   SPEED CHANGE\n",
                "B     BEEP!"
            ],
            Attr::new(Color::White, Color::Black).bright(),
        );
        Spectrum::set_border(Color::Blue);
    }


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
        }
        if keys.just_pressed(&prev, Key::X) && brush < BRUSH_MAX_RADIUS {
            brush += 1;
        }
        // N, M: change speed
        if keys.just_pressed(&prev, Key::N) && move_rate > 1 {
            move_rate -= 1;
        }
        if keys.just_pressed(&prev, Key::M) && move_rate < 50 {
            move_rate += 1;
        }
        // E: toggle eraser
        if keys.just_pressed(&prev, Key::E) {
            erase = !erase;
        }
        // Enter: fill paper
        if keys.just_pressed(&prev, Key::Enter) {
            unsafe {
                fill_paper(attr.paper());
            }
        }
        // Space: paint ink brush
        if keys.is_pressed(Key::Space) {
            unsafe {
                paint_brush(x, y, brush, !erase, attr.ink());
            }
        }

        /* text */

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
            unsafe { canvas_set_pixel(px, py, set) };
        }}
    }}
    lets! { c0 = x0 >> 3, c1 = x1 >> 3, r0 = y0 >> 3, r1 = y1 >> 3 }
    if set {
        whilst! { row in r0, ..=r1; {
            whilst! { column in c0, ..=c1; {
                let old = unsafe { canvas_attribute(column, row) };
                unsafe { canvas_set_attribute(column, row, old.with_ink(ink)); }
            }}
        }}
    }
    unsafe { present_damage(c0, r0, c1, r1) };
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

/* subroutines: canvas */

#[inline(always)]
const fn canvas_bitmap_offset(column: u16, y: u16) -> usize {
    (y * CANVAS_COLUMNS + column) as usize
}
#[inline(always)]
const fn canvas_attr_offset(column: u16, row: u16) -> usize {
    (row * CANVAS_COLUMNS + column) as usize
}

#[inline(always)]
unsafe fn canvas_bitmap_byte(column: u16, y: u16) -> u8 {
    let ptr = (&raw mut CANVAS_BITMAP).cast::<u8>();
    unsafe { ptr.add(canvas_bitmap_offset(column, y)).read() }
}

#[inline(always)]
unsafe fn canvas_set_pixel(x: u16, y: u16, set: bool) {
    is! { x >= CANVAS_WIDTH || y >= CANVAS_HEIGHT, return }
    let offset = canvas_bitmap_offset(x >> 3, y);
    let ptr = unsafe { (&raw mut CANVAS_BITMAP).cast::<u8>().add(offset) };
    let mask = 0x80u8 >> (x & 7);
    let old = unsafe { ptr.read() };
    let new = is![set, old | mask, old & !mask];

    unsafe { ptr.write(new) };
}

#[inline(always)]
unsafe fn canvas_attribute(column: u16, row: u16) -> Attr {
    let ptr = (&raw mut CANVAS_ATTRS).cast::<u8>();
    let value = unsafe { ptr.add(canvas_attr_offset(column, row)).read() };
    Attr::from_u8(value)
}
#[inline(always)]
unsafe fn canvas_set_attribute(column: u16, row: u16, attr: Attr) {
    let ptr = (&raw mut CANVAS_ATTRS).cast::<u8>();
    unsafe {
        ptr.add(canvas_attr_offset(column, row)).write(attr.to_u8());
    }
}
unsafe fn canvas_fill_attributes(attr: Attr) {
    let ptr = (&raw mut CANVAS_ATTRS).cast::<u8>();
    whilst! { offset in 0usize..CANVAS_ATTR_LEN; {
        unsafe { ptr.add(offset).write(attr.to_u8()) };
    }}
}

unsafe fn present_cell(column: u8, row: u8) {
    let y0 = row as u16 * 8;
    whilst! { line in 0u8..8; {
        let bits = unsafe { canvas_bitmap_byte(column as u16, y0 + line as u16) };
        unsafe { Spectrum::write_bitmap_byte(column, row * 8 + line, bits); }
    }}
    let attr = unsafe { canvas_attribute(column as u16, row as u16) };
    unsafe {
        Spectrum::write_cell_attribute(column, row, attr);
    }
}
unsafe fn present_all() {
    whilst! { row in 0u8..Spectrum::SCREEN_ROWS; {
        whilst! { column in 0u8..Spectrum::SCREEN_COLUMNS; {
            unsafe { present_cell(column, row) };
        }}
    }}
}

/* subroutines: hud */

#[inline(always)]
const fn hud_covers(column: u8, row: u8) -> bool {
    column >= HUD_C0 && column <= HUD_C1 && row >= HUD_R0 && row <= HUD_R1
}
unsafe fn present_damage(c0: u16, r0: u16, c1: u16, r1: u16) {
    whilst! { row in r0, ..=r1; {
        whilst! { column in c0, ..=c1; {
            let (column, row) = (column as u8, row as u8);
            is! { !hud_covers(column, row), unsafe { present_cell(column, row) } }
        }}
    }}
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

#[allow(unused)]
macro_rules! border_probe {
    (($ula:expr) $($tt:tt)*) => {
        unsafe { Spectrum::write_ula($ula.with_border(Color::Red)) };
        $($tt)*
        unsafe { Spectrum::write_ula($ula) };
    }
}
#[allow(unused)]
use border_probe;
