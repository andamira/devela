// devela::examples::lang::js_web_api
//
//! A Web API canvas example.
//

// global config
#![no_std]
#![allow(static_mut_refs, reason = "safe in single-threaded")]
devela::define_panic_handler! { web_api }

use devela::{format_buf, Js, JsEvent, Wasm};

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    Js::set_canvas("#example_canvas_1");

    let mut buf = [0u8; 256];

    /* wasm architecture */

    Js::console_log(&format_buf![?&mut buf, "Wasm memory: {}, bytes: {}",
        Wasm::memory_size(), Wasm::memory_bytes()]);
    Js::console_log(&format_buf![?&mut buf, "Wasm::bulk-memory: {}", Wasm::has_bulk_memory()]);
    Js::console_log(&format_buf![?&mut buf, "Wasm::simd128: {}", Wasm::has_simd()]);
    Js::console_log(&format_buf![?&mut buf, "Wasm::mutable-globals: {}",
        Wasm::has_mutable_globals()]);

    /* console */

    Js::console_log(format_buf![?&mut buf, "example log at: {}ms", Js::performance_now()]);
    Js::console_info("example info");
    Js::console_debug("example debug");
    Js::console_warn("example warn");
    Js::console_error("example error");
    Js::console_trace();

    /* eval */

    Js::eval("console.log('Hello from Rust!');");
    Js::eval_timeout("console.log('Delayed!');", 1000);
    Js::eval_interval("console.log('Repeating!');", 2000);
    let cleared = Js::eval_timeout("console.error('This should not run!');", 1500);
    Js::eval_timeout_clear(cleared);

    /* draw shapes */

    Js::fill_style(255, 0, 0);
    Js::fill_rect(10.0, 10.0, 100.0, 100.0);
    Js::fill_style(0, 255, 0);
    Js::draw_circle(150.0, 50.0, 30.0);
    Js::stroke_style(0, 0, 255);
    Js::draw_line(200.0, 20.0, 300.0, 100.0);

    /* text */

    let metrics = Js::measure_text_full("Hello, world!");
    Js::console_log(&format_buf![?&mut buf, "{metrics:?}"]);

    // Add an event listener to the canvas for clicks
    Js::event_add_listener("#example_canvas_1", JsEvent::Click, canvas_click);
    Js::fill_style(0, 0, 0);
    Js::fill_text("Click the canvas!", 60.0, 30.0);

    panic!("let's trigger a panic");
}

/// Called when clicking on the canvas.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_click() {
    let time = Js::performance_now();

    static mut BUF: [u8; 64] = [0; 64];
    let buf: &mut [u8] = unsafe { &mut BUF };

    let times = Js::performance_event_count(JsEvent::Click) + 1;
    Js::console_log(format_buf![?buf, "Canvas clicked {times} times"]);

    let origin = Js::performance_time_origin();
    Js::console_log(format_buf![?buf, "origin {origin}ms"]);

    Js::fill_style(200, 0, 50);
    Js::fill_rect(50.0, 50.0, 100.0, 100.0);

    Js::console_log(format_buf![?buf, "Canvas clicked at: {time}ms"]);
    Js::fill_style(50, 50, 200);
    Js::fill_text(format_buf![?buf, "time: {time}ms"], 60.0, 100.0);
}
