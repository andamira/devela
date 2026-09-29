//
//! Interactive ZX Spectrum pixel painter.
//
// 1507 bytes .tap

#![no_std]
#![no_main]

use devela::{is, use_as};
use devela_micros::{ComputerSpectrum48 as Spectrum, ProcessorZ80 as Z80, devela};
use_as! {+Spectrum: devela_micros::{Attribute as Attr, Color, Key, Keys, UlaOut }}

devela::set_panic_handler! { loop }

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
    let (mut x, mut y) = (128u8, 96u8);
    let mut frame = 0u8;
    let mut prev = Keys::NONE;
    let mut erase = false;
    let mut ink = Color::White;
    let paper = Color::Black;
    let mut ula = UlaOut::new(ink);

    unsafe { toggle_cursor(x, y) };

    loop {
        Z80::halt();
        frame = frame.wrapping_add(1);

        let keys = unsafe { Spectrum::read_keys() };
        let repeat = frame & 1 == 0; // movement at ~25 Hz while held

        unsafe { toggle_cursor(x, y) };

        /* movement */

        is! {
            keys.is_pressed(Key::Q)
                && (keys.just_pressed(&prev, Key::Q) || repeat)
                && y > 0,
            y -= 1
        }
        is! {
            keys.is_pressed(Key::A)
                && (keys.just_pressed(&prev, Key::A) || repeat)
                && y < Spectrum::SCREEN_Y_MAX,
            y += 1
        }
        is! {
            keys.is_pressed(Key::O)
                && (keys.just_pressed(&prev, Key::O) || repeat)
                && x > 0,
            x -= 1
        }
        is! {
            keys.is_pressed(Key::P)
                && (keys.just_pressed(&prev, Key::P) || repeat)
                && x < Spectrum::SCREEN_X_MAX,
            x += 1
        }

        /* draw */

        ink = select_color(&keys, ink);
        is! { keys.just_pressed(&prev, Key::E), erase = !erase }

        // show ink color in border
        ula = ula.with_border(ink);
        unsafe { Spectrum::write_ula(ula) };

        if keys.is_pressed(Key::Space) {
            unsafe {
                if erase {
                    Spectrum::set_pixel(x, y, false);
                } else {
                    Spectrum::set_pixel(x, y, true);
                    let attr = Attr::new(ink, paper).bright();
                    Spectrum::write_cell_attribute(x >> 3, y >> 3, attr);
                }
            }
        }

        unsafe { toggle_cursor(x, y) };
        prev = keys;
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

unsafe fn toggle_cursor(x: u8, y: u8) {
    unsafe {
        Spectrum::toggle_pixel(x, y);
        is! { x > 0, Spectrum::toggle_pixel(x - 1, y) }
        is! { x < Spectrum::SCREEN_X_MAX, Spectrum::toggle_pixel(x + 1, y) }
        is! { y > 0, Spectrum::toggle_pixel(x, y - 1) }
        is! { y < Spectrum::SCREEN_Y_MAX, Spectrum::toggle_pixel(x, y + 1) }
    }
}
