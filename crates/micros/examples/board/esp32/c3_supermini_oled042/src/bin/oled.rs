//
//! Initializes the onboard 72×40 OLED and draws a test pattern over I²C.
//

#![no_std]
#![no_main]

use devela::{BoardSuperMiniOled042 as Board, I2cWrite, whilst};
use devela_micros::devela;

devela::set_panic_handler! { loop }
devela::esp32_c3_direct_boot! { main }

fn main() -> ! {
    let serial = Board::USB_SERIAL;

    unsafe {
        while !serial.rx_ready() {} // Let the host reopen USB after flashing

        let mut i2c = Board::prepare_oled_i2c();

        serial.write_bytes_blocking(b"initializing oled...\r\n");
        oled_init(&mut i2c).unwrap();

        serial.write_bytes_blocking(b"drawing 1px border...\r\n");
        oled_data(&mut i2c, &OLED_BORDER).unwrap();

        serial.write_bytes_blocking(b"done\r\n");
    }
    loop {}
}

const OLED_COMMAND_CONTROL: &[u8] = &[0x00];
const OLED_DATA_CONTROL: &[u8] = &[0x40];

fn oled_init<I: I2cWrite>(i2c: &mut I) -> Result<(), I::Error> {
    oled_commands(i2c, OLED_INIT)?;
    oled_commands(i2c, OLED_WINDOW)
}
// SSD1306 I²C command framing
fn oled_commands<I: I2cWrite>(i2c: &mut I, commands: &[u8]) -> Result<(), I::Error> {
    i2c.write_slices(Board::OLED_ADDR, &[OLED_COMMAND_CONTROL, commands])
}
// SSD1306 I²C data framing
fn oled_data<I: I2cWrite>(i2c: &mut I, data: &[u8]) -> Result<(), I::Error> {
    i2c.write_slices(Board::OLED_ADDR, &[OLED_DATA_CONTROL, data])
}

// SSD1306 commands
const OLED_INIT: &[u8] = &[
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

// SSD1306 addressing
const OLED_WINDOW: &[u8] = &[
    0x21, // set column range
    Board::OLED_RAM_COLUMN_OFFSET,
    Board::OLED_RAM_COLUMN_LAST,
    0x22, // set page range
    0,
    Board::OLED_RAM_PAGE_LAST,
];

// native 72×40 framebuffer
const OLED_BORDER: [u8; Board::OLED_FRAME_BYTES] = {
    const WIDTH: usize = Board::OLED_WIDTH;
    const PAGES: usize = Board::OLED_PAGE_COUNT;
    let mut data = [0u8; Board::OLED_FRAME_BYTES];
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
