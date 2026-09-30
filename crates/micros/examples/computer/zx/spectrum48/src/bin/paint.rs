//
//! Interactive ZX Spectrum paint application.
//
// 2250 bytes .tap

#![allow(linker_messages)] // TEMP
#![no_std]
#![no_main]

use devela::{Arch, is, lets, use_as, whilst};
use devela_micros::{ComputerSpectrum48 as Spectrum, ProcessorZ80 as Z80, devela};
use_as! {+Spectrum: devela_micros::{Attribute as Attr, Color, Key, Keys, UlaOut }}

devela::set_panic_handler! { loop }

#[derive(Clone, Copy)]
struct MoveKeys {
    up: Key,
    down: Key,
    left: Key,
    right: Key,
}
#[allow(unused)]
impl MoveKeys {
    const fn new(up: Key, down: Key, left: Key, right: Key) -> Self {
        Self { up, down, left, right }
    }
    const WSAD: Self = Self::new(Key::W, Key::S, Key::A, Key::D);
    const QAOP: Self = Self::new(Key::Q, Key::A, Key::O, Key::P);
}

const BRUSH_MAX_RADIUS: u8 = 8; // max diameter = radius * 2 + 1

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn start() {
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

    let mut frame = 0u8;
    let mut prev = Keys::NONE;
    let mut erase = false;
    let mut attr = Attr::new(Color::White, Color::Black).bright();
    let mut brush = 0u8; // radius: 0, 1, …, 5 → 1×1, 3×3, …, 11×11

    let mut ula = UlaOut::new(attr.ink());

    unsafe { toggle_cursor(x, y) }; // show cursor at the start

    loop {
        Z80::halt();
        let keys = unsafe { Spectrum::read_keys() };

        frame = frame.wrapping_add(1);

        move_phase += move_rate; // once per ~50 Hz frame
        let repeat = is! { move_phase >= 50, { move_phase -= 50; true }, false };

        unsafe { toggle_cursor(x, y) }; // hide cursor overlay

        /* drawing */

        // cursor movement
        move_if!(prev, keys, repeat, movekey.up, y > 0, y -= 1);
        move_if!(prev, keys, repeat, movekey.down, y < Spectrum::SCREEN_Y_MAX, y += 1);
        move_if!(prev, keys, repeat, movekey.left, x > 0, x -= 1);
        move_if!(prev, keys, repeat, movekey.right, x < Spectrum::SCREEN_X_MAX, x += 1);

        // Color selection. Ink or paper depending on the Shift key status
        let selecting_paper = keys.is_pressed(Key::CapsShift);
        let current = is![selecting_paper, attr.paper(), attr.ink()];
        let color = select_color(&keys, current);
        attr = is![selecting_paper, attr.with_paper(color), attr.with_ink(color)];

        // show ink color in border
        ula = ula.with_border(attr.ink());
        unsafe { Spectrum::write_ula(ula) };

        // Z, X: brush size
        is! { keys.just_pressed(&prev, Key::Z) && brush > 0, brush -= 1 }
        is! { keys.just_pressed(&prev, Key::X) && brush < BRUSH_MAX_RADIUS, brush += 1 }
        // N, M: speed change
        is! { keys.just_pressed(&prev, Key::N) && move_rate > 1, move_rate -= 1 }
        is! { keys.just_pressed(&prev, Key::M) && move_rate < 50, move_rate += 1 }
        // E: eraser selector
        is! { keys.just_pressed(&prev, Key::E), erase = !erase }
        // Enter: fill paper
        is! { keys.just_pressed(&prev, Key::Enter), unsafe { fill_paper(attr.paper()) } }
        // Space: paint brush
        is! { keys.is_pressed(Key::Space), unsafe { paint_brush(x, y, brush, !erase, attr.ink()) } }

        unsafe { toggle_cursor(x, y) }; // show again cursor overlay

        /* sound */

        is! { keys.just_pressed(&prev, Key::B), unsafe { beep(ula) } }

        prev = keys;
    }
}

#[expect(unused)]
unsafe fn paint_pixel(x: u8, y: u8, set: bool, ink: Color) {
    unsafe { Spectrum::set_pixel(x, y, set) };
    if set {
        let (column, row) = (x >> 3, y >> 3);
        let offset = row as u16 * Spectrum::SCREEN_COLUMNS as u16 + column as u16;
        let old = Attr::from_u8(unsafe { Spectrum::read_attribute(offset) });
        let new = old.with_ink(ink);
        unsafe { Spectrum::write_attribute(offset, new.to_u8()) };
    }
}

unsafe fn paint_brush(x: u8, y: u8, radius: u8, set: bool, ink: Color) {
    let x0 = x.saturating_sub(radius);
    let x1 = x.saturating_add(radius).min(Spectrum::SCREEN_X_MAX);
    let y0 = y.saturating_sub(radius);
    let y1 = y.saturating_add(radius).min(Spectrum::SCREEN_Y_MAX);
    whilst! { py in y0, ..=y1; {
        whilst! { px in x0, ..=x1; {
            unsafe { Spectrum::set_pixel(px, py, set) };
        }}
    }}
    if set {
        lets! { c0 = x0 >> 3, c1 = x1 >> 3, r0 = y0 >> 3, r1 = y1 >> 3 }
        whilst! { row in r0, ..=r1; {
            whilst! { column in c0, ..=c1; {
                let offset = row as u16 * Spectrum::SCREEN_COLUMNS as u16 + column as u16;
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

unsafe fn toggle_cursor(x: u8, y: u8) {
    unsafe {
        Spectrum::toggle_pixel(x, y);
        is! { x > 0, Spectrum::toggle_pixel(x - 1, y) }
        is! { x < Spectrum::SCREEN_X_MAX, Spectrum::toggle_pixel(x + 1, y) }
        is! { y > 0, Spectrum::toggle_pixel(x, y - 1) }
        is! { y < Spectrum::SCREEN_Y_MAX, Spectrum::toggle_pixel(x, y + 1) }
    }
}

#[inline(never)]
fn select_color(keys: &Keys, current: Color) -> Color {
    if keys.is_pressed(Key::Num1) {
        Color::Blue
    } else if keys.is_pressed(Key::Num2) {
        Color::Red
    } else if keys.is_pressed(Key::Num3) {
        Color::Magenta
    } else if keys.is_pressed(Key::Num4) {
        Color::Green
    } else if keys.is_pressed(Key::Num5) {
        Color::Cyan
    } else if keys.is_pressed(Key::Num6) {
        Color::Yellow
    } else if keys.is_pressed(Key::Num7) {
        Color::White
    } else if keys.is_pressed(Key::Num0) {
        Color::Black
    } else {
        current
    }
}

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

macro_rules! move_if {
    ($prev:expr, $keys:expr, $repeat:expr, $key:expr, $bound:expr, $step:expr) => {
        if $keys.is_pressed($key) && ($keys.just_pressed(&$prev, $key) || $repeat) && $bound {
            $step
        }
    };
}
use move_if;
