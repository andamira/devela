// devela::examples::lang::js_web_api
//
//! A Web API canvas example.
//

#![no_std]
devela::define_panic_handler! { web_api }

use devela::{format_buf, Js, JsEvent};

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    Js::set_canvas("example_canvas_1");

    let mut buf = [0u8; 32];
    Js::console_log(format_buf![&mut buf, "time: {}ms", Js::get_time()].unwrap());

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
    Js::console_log(format_buf![&mut buf, "time: {}", Js::get_time()].unwrap());

    let mut buf = [0u8; 32];
    Js::console_warn(format_buf![&mut buf, "warn nº {}", 5].unwrap());

    // Add an event listener to the canvas for clicks
    Js::event_add_listener("#example_canvas_1", JsEvent::Click, canvas_click);
    Js::fill_style(0, 0, 0);
    Js::fill_text("Click the canvas!", 60.0, 30.0);

    panic!("let's trigger a panic");
}

/// Called when clicking on the canvas.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_click() {
    let time = Js::get_time();
    let mut buf = [0u8; 64];

    Js::fill_style(200, 0, 50);
    Js::fill_rect(50.0, 50.0, 100.0, 100.0);

    Js::console_log(format_buf![&mut buf, "Canvas clicked at: {time}ms"].unwrap());
    Js::fill_style(50, 50, 200);
    Js::fill_text(format_buf![&mut buf, "time: {time}ms"].unwrap(), 60.0, 100.0);
}
