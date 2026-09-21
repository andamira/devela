//
//! Initializes the onboard 72×40 OLED and draws a test pattern over I²C.
//

#![no_std]
#![no_main]

use devela::{BoardSuperMiniOled042 as Board, Ssd13xxI2c, whilst};
use devela_micros::devela;

devela::set_panic_handler! { loop }
devela::esp32_c3_direct_boot! { main }

fn main() -> ! {
    let serial = Board::USB_SERIAL;

    unsafe {
        while !serial.rx_ready() {} // Let the host reopen USB after flashing

        let mut i2c = Board::prepare_oled_i2c();
        let mut oled_io = Ssd13xxI2c::new(&mut i2c, Board::OLED_ADDR);

        serial.write_bytes_blocking(b"initializing oled...\r\n");
        Board::OLED.init(&mut oled_io).unwrap();

        serial.write_bytes_blocking(b"drawing 1px border...\r\n");
        Board::OLED.write_data(&mut oled_io, &OLED_BORDER).unwrap();

        serial.write_bytes_blocking(b"done\r\n");
    }
    loop {}
}

// native 72×40 framebuffer
const OLED_BORDER: [u8; Board::OLED.frame_bytes()] = {
    const WIDTH: usize = Board::OLED.width();
    const PAGES: usize = Board::OLED.page_count();
    let mut data = [0u8; Board::OLED.frame_bytes()];
    whilst! { page in 0..PAGES; {
        whilst! { x in 0..WIDTH; {
            data[page * WIDTH + x] =
                if x == 0 || x + 1 == WIDTH {
                    0xff
                } else if page == 0 {
                    0x01
                } else if page + 1 == PAGES {
                    0x80
                } else {
                    0x00
                };
        }}
    }}
    data
};
