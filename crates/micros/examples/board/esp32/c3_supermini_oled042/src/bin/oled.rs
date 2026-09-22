//
//! Initializes the onboard 72×40 OLED and draws a test pattern over I²C.
//
// 2640 bytes

#![no_std]
#![no_main]

use devela::{BitmapPage8, BoardSuperMiniOled042 as Board, Pcg32, Ssd13xxI2c};
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

        serial.write_bytes_blocking(b"drawing...\r\n");

        const WIDTH: u32 = Board::OLED.width() as u32;
        const HEIGHT: u32 = Board::OLED.height() as u32;
        const PIXELS: u32 = (WIDTH * HEIGHT) as u32;
        type OledFrame = BitmapPage8<
            { Board::OLED.width() },
            { Board::OLED.height() },
            { Board::OLED.frame_bytes() },
        >;

        let mut rng = Pcg32::new(WIDTH as u64, HEIGHT as u64);
        let mut frame = OledFrame::new();

        /* outer frame */

        frame.draw_rect(0, 0, WIDTH, HEIGHT, true);
        frame.draw_rect(5, 5, WIDTH - 10, HEIGHT - 10, true);

        /* random pixels */

        for _ in 0..240 {
            let pixel = rng.next_bounded(PIXELS);
            frame.set_pixel(pixel % WIDTH, pixel / WIDTH, true);
        }

        /* random rectangles */

        const LEFT: u32 = 1;
        const TOP: u32 = 1;
        const RIGHT: u32 = WIDTH - 1;
        const BOTTOM: u32 = HEIGHT - 1;
        for _ in 0..8 {
            let x = LEFT + rng.next_bounded(RIGHT - LEFT);
            let y = TOP + rng.next_bounded(BOTTOM - TOP);

            let (available_width, available_height) = (RIGHT - x, BOTTOM - y);

            let width = 1 + rng.next_bounded(available_width.min(18));
            let height = 1 + rng.next_bounded(available_height.min(12));

            frame.fill_rect(x, y, width, height, true);
        }

        Board::OLED.write_data(&mut oled_io, frame.bytes()).unwrap();
        serial.write_bytes_blocking(b"done\r\n");
    }
    loop {}
}
