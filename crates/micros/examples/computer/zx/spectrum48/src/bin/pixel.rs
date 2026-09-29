//
//! Interactive ZX Spectrum pixel painter.
//
// 2186 bytes tap

#![no_std]
#![no_main]

use devela::{is, use_as};
use devela_micros::{ComputerSpectrum48 as Spectrum, ProcessorZ80 as Z80, devela};
use_as! { +Spectrum: devela_micros::{Attribute as Attr, Color, Key, Keys} }

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

    let mut prev = Keys::NONE;
    let (mut x, mut y) = (128u8, 96u8);

    unsafe { toggle_cursor(x, y) };

    let mut frame = 0u8;

    loop {
        Z80::halt();
        frame = frame.wrapping_add(1);

        let keys = unsafe { Spectrum::read_keys() };
        let repeat = frame & 1 == 0; // movement at ~25 Hz while held

        unsafe { toggle_cursor(x, y) };

        is! {
            keys.is_pressed(Key::Q)
                && (keys.just_pressed(prev, Key::Q) || repeat)
                && y > 0,
            y -= 1
        }
        is! {
            keys.is_pressed(Key::A)
                && (keys.just_pressed(prev, Key::A) || repeat)
                && y < Spectrum::SCREEN_Y_MAX,
            y += 1
        }
        is! {
            keys.is_pressed(Key::O)
                && (keys.just_pressed(prev, Key::O) || repeat)
                && x > 0,
            x -= 1
        }
        is! {
            keys.is_pressed(Key::P)
                && (keys.just_pressed(prev, Key::P) || repeat)
                && x < Spectrum::SCREEN_X_MAX,
            x += 1
        }
        is! { keys.is_pressed(Key::Space), unsafe { Spectrum::set_pixel(x, y, true) } }

        // simpler version, without auto-repeat, saves 328 bytes
        // is! { keys.just_pressed(prev, Key::Q) && y > 0, y -= 1 }
        // is! { keys.just_pressed(prev, Key::A) && y + 1 < Spectrum::SCREEN_X_MAX, y += 1 }
        // is! { keys.just_pressed(prev, Key::O) && x > 0, x -= 1 }
        // is! { keys.just_pressed(prev, Key::P) && x + 1 < Spectrum::SCREEN_Y_MAX, x += 1 }
        // is! { keys.just_pressed(prev, Key::Space), unsafe { Spectrum::set_pixel(x, y, true) } }

        unsafe { toggle_cursor(x, y) };
        prev = keys;
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
