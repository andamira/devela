//
//! Initializes the onboard 72×40 OLED and draws a test pattern over I²C.
//
// 10704 bytes

#![no_std]
#![no_main]

use devela::{
    BitmapPage8, Canvas, CanvasRasterExt, FmtNum, Fonts, I2cTarget, Pcg32, RandTry, lets, pos,
    region,
};
use devela_micros::{
    BoardSuperMiniOled042 as Board, Esp32C3SystemTimer, Esp32C3Uart, McuEsp32C3 as Mcu, Ssd13xx,
    devela,
};

devela::set_panic_handler! { loop }
devela::esp32_c3_direct_boot! { main }

const WIDTH: u32 = Board::OLED.width();
const HEIGHT: u32 = Board::OLED.height();
const PIXELS: u32 = WIDTH * HEIGHT;

type OledFrame =
    BitmapPage8<{ WIDTH as usize }, { HEIGHT as usize }, { Board::OLED.frame_bytes() }>;

fn main() -> ! {
    unsafe {
        Mcu::prepare_uart0_default();
        let uart = Board::UART0;
        uart.configure_8n1_xtal(Mcu::XTAL_HZ, 115_200);
        let timer = Mcu::prepare_systimer_unit0();

        let mut i2c = Board::prepare_oled_i2c();
        let target = I2cTarget::new(&mut i2c, Board::OLED_ADDR);
        let mut oled_io = target.cmd_data(Ssd13xx::I2C_CMD, Ssd13xx::I2C_DATA);
        Board::OLED.init(&mut oled_io).unwrap();

        let mut frame = OledFrame::new();

        let mut hw_rng = Board::MCU.rng();
        let seed = hw_rng.rand_try_next_u64().unwrap();
        let stream = hw_rng.rand_try_next_u64().unwrap();
        let mut rng = Pcg32::new(seed, stream);

        lets! { mut frames = 0_u32, mut draw_ticks = 0_u64, mut transfer_ticks = 0_u64 }

        let mut report_start = timer.unit0_count();

        loop {
            let frame_start = timer.unit0_count();
            draw_frame(&mut frame, &mut rng);
            let draw_end = timer.unit0_count();

            Board::OLED.write_data(&mut oled_io, frame.bytes()).unwrap();

            let frame_end = timer.unit0_count();

            draw_ticks += Esp32C3SystemTimer::elapsed_ticks(frame_start, draw_end);
            transfer_ticks += Esp32C3SystemTimer::elapsed_ticks(draw_end, frame_end);
            frames += 1;

            let elapsed = Esp32C3SystemTimer::elapsed_ticks(report_start, frame_end);

            if elapsed >= Esp32C3SystemTimer::COUNTER_HZ {
                report_stats(uart, frames, elapsed, draw_ticks, transfer_ticks);
                frames = 0;
                draw_ticks = 0;
                transfer_ticks = 0;
                // Exclude UART reporting time from the following measurement window.
                report_start = timer.unit0_count();
            }
        }
    }
}

/* drawing */

fn draw_frame(frame: &mut OledFrame, rng: &mut Pcg32) {
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
        frame,
        pos![text_x as i64, text_top as i64 + font.baseline() as i64],
        TEXT,
        true,
    )
    .unwrap();
}

/* reporting */

unsafe fn report_stats(
    uart: Esp32C3Uart,
    frames: u32,
    elapsed: u64,
    draw_ticks: u64,
    transfer_ticks: u64,
) {
    let hz = Esp32C3SystemTimer::COUNTER_HZ;

    let fps_100 = frames as u64 * hz * 100 / elapsed;

    let draw_ms_100 = avg_ms_100(draw_ticks, frames);
    let transfer_ms_100 = avg_ms_100(transfer_ticks, frames);
    let frame_ms_100 = avg_ms_100(draw_ticks + transfer_ticks, frames);

    unsafe {
        uart.write_bytes_blocking(b"fps=");
        write_fixed2(uart, fps_100);

        uart.write_bytes_blocking(b" draw=");
        write_fixed2(uart, draw_ms_100);
        uart.write_bytes_blocking(b"ms");

        uart.write_bytes_blocking(b" tx=");
        write_fixed2(uart, transfer_ms_100);
        uart.write_bytes_blocking(b"ms");

        uart.write_bytes_blocking(b" frame=");
        write_fixed2(uart, frame_ms_100);
        uart.write_bytes_blocking(b"ms\r\n");
    }
}
fn avg_ms_100(ticks: u64, frames: u32) -> u64 {
    ticks * 100_000 / (Esp32C3SystemTimer::COUNTER_HZ * frames as u64)
}
unsafe fn write_num(uart: Esp32C3Uart, value: u64) {
    let mut buf = [0_u8; 20];
    let len = FmtNum(value).write(&mut buf, 0);
    unsafe { uart.write_bytes_blocking(&buf[..len]) };
}
unsafe fn write_fixed2(uart: Esp32C3Uart, value_100: u64) {
    unsafe {
        write_num(uart, value_100 / 100);
        uart.write_byte_blocking(b'.');
        uart.write_byte_blocking(b'0' + ((value_100 / 10) % 10) as u8);
        uart.write_byte_blocking(b'0' + (value_100 % 10) as u8);
    }
}
