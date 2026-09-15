// devela/examples/sys/hw/mcu/esp32/c3_supermini_oled042/src/bin/i2c_probe.rs
//
//! Probes the board's built-in OLED over I²C.
//

#![no_std]
#![no_main]

use devela::{BoardSuperMiniOled042 as Board, McuEsp32C3 as Mcu};

devela::set_panic_handler! { loop }
devela::esp32_c3_direct_boot! { main }

fn main() -> ! {
    let serial = Board::USB_SERIAL;
    unsafe {
        while !serial.rx_ready() {} // Let the host reopen USB after flashing
        serial.write_bytes_blocking(b"preparing i2c0...\r\n");
        Mcu::prepare_i2c0(Board::OLED_SDA, Board::OLED_SCL, Board::OLED_I2C_HZ);

        serial.write_bytes_blocking(b"probing oled at 0x3c...\r\n");
        match Board::OLED_I2C.probe_blocking(Board::OLED_ADDR) {
            Ok(()) => serial.write_bytes_blocking(b"ACK!\r\n"),
            Err(_) => serial.write_bytes_blocking(b"no ACK\r\n"),
        }

        serial.write_bytes_blocking(b"initializing oled...\r\n");
        Board::OLED_I2C.write_blocking(Board::OLED_ADDR, OLED_INIT).unwrap();

        serial.write_bytes_blocking(b"drawing 1px border...\r\n");
        Board::OLED_I2C.write_blocking(Board::OLED_ADDR, OLED_WINDOW).unwrap();
        Board::OLED_I2C.write_blocking(Board::OLED_ADDR, &OLED_BORDER).unwrap();

        serial.write_bytes_blocking(b"done\r\n");
    }
    loop {}
}

const OLED_ALL_ON: &[u8] = &[0x00, 0xA5];
const OLED_INIT: &[u8] = &[
    0x00, // I2C control byte: following bytes are commands
    0xAE, // display off
    0xD5, 0x80, // display clock
    0xA8, 0x27, // multiplex ratio: 40 rows
    0xD3, 0x00, // display offset
    0xAD, 0x30, // internal IREF
    0x8D, 0x14, // charge pump on
    0x40, // start line 0
    0xA6, // normal display
    0xA4, // display follows RAM
    0x20, 0x00, // horizontal addressing mode
    0xA1, // segment remap
    0xC8, // reversed COM scan
    0xDA, 0x12, // COM configuration
    0x81, 0xAF, // contrast
    0xD9, 0x22, // precharge
    0xDB, 0x20, // VCOMH
    0x2E, // deactivate scrolling
    0xAF, // display on
];
const OLED_FRAME_BYTES: usize = Board::OLED_WIDTH * Board::OLED_HEIGHT / 8;

const OLED_BORDER: [u8; OLED_FRAME_BYTES + 1] = {
    const WIDTH: usize = Board::OLED_WIDTH;
    const HEIGHT: usize = Board::OLED_HEIGHT;
    const PAGES: usize = HEIGHT / 8;
    let mut data = [0u8; OLED_FRAME_BYTES + 1];
    // SSD1306 I²C control byte: following bytes are display data.
    data[0] = 0x40;
    let mut page = 0;
    while page < PAGES {
        let mut x = 0;
        while x < WIDTH {
            data[1 + page * WIDTH + x] = if x == 0 || x + 1 == WIDTH {
                0xff
            } else if page == 0 {
                0x01
            } else if page + 1 == PAGES {
                0x80
            } else {
                0x00
            };
            x += 1;
        }
        page += 1;
    }
    data
};

const OLED_WINDOW: &[u8] = &[
    0x00,    // command stream
    0xA4,    // display follows RAM
    0x21,    // set column range
    28,      // first visible SSD1306 column
    28 + 71, // last visible column
    0x22,    // set page range
    0,
    4,
];
