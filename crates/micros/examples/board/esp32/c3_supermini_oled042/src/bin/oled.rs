//
//! Initializes the onboard 72×40 OLED and draws a test pattern over I²C.
//
// 7824 bytes
//

#![no_std]
#![no_main]

use devela::{BitmapPage8, Canvas, CanvasRasterExt, Fonts, Pcg32, pos, region};
use devela_micros::{BoardSuperMiniOled042 as Board, Ssd13xxI2c, devela};

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

        const WIDTH: u32 = Board::OLED.width();
        const HEIGHT: u32 = Board::OLED.height();
        const PIXELS: u32 = WIDTH * HEIGHT;
        type OledFrame =
            BitmapPage8<{ WIDTH as usize }, { HEIGHT as usize }, { Board::OLED.frame_bytes() }>;

        let mut rng = Pcg32::new(WIDTH as u64 * 2, HEIGHT as u64 * 3);
        let mut frame = OledFrame::new();

        /* frame */

        frame.canvas_clear(false).unwrap();
        frame.canvas_draw_region(region!((0_u32, 0), (WIDTH, HEIGHT)), true).unwrap();

        /* random pixels */

        for _ in 0..180 {
            let pixel = rng.next_bounded(PIXELS);
            let (x, y) = (pixel % WIDTH, pixel / WIDTH);
            frame.canvas_set_color(pos![x, y], true).unwrap();
        }

        /* random rectangles */

        const LEFT: u32 = 1;
        const TOP: u32 = 1;
        const RIGHT: u32 = WIDTH - 1;
        const BOTTOM: u32 = HEIGHT - 1;

        for _ in 0..6 {
            let x = LEFT + rng.next_bounded(RIGHT - LEFT);
            let y = TOP + rng.next_bounded(BOTTOM - TOP);
            let (available_width, available_height) = (RIGHT - x, BOTTOM - y);
            let width = 1 + rng.next_bounded(available_width.min(18));
            let height = 1 + rng.next_bounded(available_height.min(12));
            let rect = region!((x, y), (width, height));
            if rng.next_u32() & 1 == 0 {
                frame.canvas_draw_region(rect, true).unwrap();
            } else {
                frame.canvas_fill_region(rect, true).unwrap();
            }
        }

        /* random clipped lines */

        for _ in 0..4 {
            let start = pos![
                rng.next_bounded(WIDTH + 24) as i64 - 12,
                rng.next_bounded(HEIGHT + 24) as i64 - 12,
            ];
            let end = pos![
                rng.next_bounded(WIDTH + 24) as i64 - 12,
                rng.next_bounded(HEIGHT + 24) as i64 - 12,
            ];
            frame.canvas_draw_line(start, end, true).unwrap();
        }

        /* text */

        const TEXT: &str = "DEVELA";
        let font = Fonts::BIT_5_6;
        let text_width = font.text_width(TEXT) as u32;
        let text_height = font.height() as u32;
        let text_x = (WIDTH - text_width) / 2;
        let text_top = 16_u32;
        let plaque = region!(
            (text_x.saturating_sub(4), text_top.saturating_sub(4)),
            (text_width + 8, text_height + 8),
        );
        frame.canvas_fill_region(plaque, false).unwrap();
        frame.canvas_draw_region(plaque, true).unwrap();
        font.draw_canvas(
            &mut frame,
            pos![text_x as i64, text_top as i64 + font.baseline() as i64,],
            TEXT,
            true,
        )
        .unwrap();

        Board::OLED.write_data(&mut oled_io, frame.bytes()).unwrap();
        serial.write_bytes_blocking(b"done\r\n");
    }
    loop {}
}
