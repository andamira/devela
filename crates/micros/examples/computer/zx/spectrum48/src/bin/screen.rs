//
//! Static ZX Spectrum display test pattern.
//
// 126 bytes .tap

#![no_std]
#![no_main]

use devela_micros::{ComputerSpectrum48 as Spectrum, SpectrumColor, devela};

devela::set_panic_handler! { loop }

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn start() {
    unsafe { Spectrum::set_border(SpectrumColor::Blue) };

    // Fine vertical stripes.
    for i in 0..Spectrum::SCREEN_BITMAP_LEN {
        unsafe { Spectrum::write_bitmap(i, 0xAA) };
    }

    // Eight broad paper-color bands, with bright white stripe pixels.
    for i in 0..Spectrum::SCREEN_ATTR_LEN {
        let column = i % 32;
        let paper = (column / 4) as u8;
        let attribute = 0x40 | (paper << 3) | SpectrumColor::White as u8;

        unsafe { Spectrum::write_attribute(i, attribute) };
    }
}
