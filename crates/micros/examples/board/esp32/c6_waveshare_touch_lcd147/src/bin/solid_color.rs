//
//! Initializes the JD9853 LCD and fills the visible panel with one RGB565 color.
//
// 2848 bytes

#![no_std]
#![no_main]

use devela::{Arch, is, unwrap};
use devela_micros::{BoardWaveshareC6TouchLcd147 as Board, devela};

devela::set_panic_handler! { loop }
devela::esp32_c6_direct_boot! { main }

fn delay_ms(ms: u32) {
    // Deliberately conservative. The JD9853 delays are minimum waits.
    for _ in 0..ms * 400_000 {
        Arch::relax();
    }
}

fn main() -> ! {
    unsafe {
        let (reset, backlight) = (Board::LCD_RST, Board::LCD_BL);

        // Keep the panel dark while its controller and RAM are initialized.
        backlight.set_output_low();
        reset.set_output_high();
        delay_ms(10);
        reset.set_low();
        delay_ms(10);
        reset.set_high();
        delay_ms(20);

        let mut io = unwrap![ok_or Board::prepare_lcd_spi(), loop { Arch::relax(); }];

        is! { Board::LCD.init(&mut io, delay_ms).is_err(), loop { Arch::relax(); } }
        is! { Board::LCD.begin_memory_write(&mut io).is_err(), loop { Arch::relax(); } }
        // RGB565 red. Pixel bytes are sent MSB first.
        is! { io.write_data_repeated_u16_be(0xf800, Board::LCD.frame_pixels()).is_err(), // red
        // is! { io.write_data_repeated_u16_be(0x07e0, Board::LCD.frame_pixels()).is_err(), // green
        // is! { io.write_data_repeated_u16_be(0x001f, Board::LCD.frame_pixels()).is_err(), // blue
        loop { Arch::relax(); }}

        backlight.set_high();
    }

    loop {
        Arch::relax();
    }
}
