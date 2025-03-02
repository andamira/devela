// devela::examples::lang::js_web_api
//
//! A Web API canvas example.
//

#![no_std]
devela::define_panic_handler! { web_api }

use devela::{format_buf, Js};

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    Js::set_canvas("example_canvas_1");

    let mut buf = [0u8; 32];
    Js::console_log(format_buf![&mut buf, "time: {:.2}", Js::get_time()].unwrap());

    Js::fill_style(255, 0, 0);
    Js::fill_rect(10.0, 10.0, 100.0, 100.0);

    Js::fill_style(0, 255, 0); // Green
    Js::draw_circle(150.0, 50.0, 30.0);

    Js::stroke_style(0, 0, 255); // Blue
    Js::draw_line(200.0, 20.0, 300.0, 100.0);

    Js::console_log("example log");
    Js::console_info("example info");
    Js::console_debug("example debug");
    Js::console_error("example error");
    Js::console_trace();
    Js::console_log(format_buf![&mut buf, "time: {:.2}", Js::get_time()].unwrap());

    let mut buf = [0u8; 32];
    Js::console_warn(format_buf![&mut buf, "warn nº {}", 5].unwrap());

    Js::fill_style(0, 0, 0);
    Js::fill_text("Hello AABBCCDDEEFF!", 50.0, 50.0); // IMPROVE: SIZE

    Js::fill_style(200, 50, 200);
    let time = Js::get_time();
    Js::fill_text(format_buf![&mut buf, "time: {time:.2}"].unwrap(), 100.0, 100.0);
    Js::console_warn(format_buf![&mut buf, "time: {time:.2}"].unwrap());

    assert!(false);
}
