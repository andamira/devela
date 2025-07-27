// devela::examples::lang::js_web
//
//! A Javascript Web API canvas example.
//

// global config
#![no_std]
#![allow(static_mut_refs, reason = "safe in single-threaded")]
devela::set_panic_handler! { web_api }

use devela::{Js, JsEventKind, JsEventMouse, JsEventPointer, Wasm, format_buf};

/// Static string buffer for printing to the console.
static mut BUF: [u8; 256] = [0; 256];

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let buf: &mut [u8] = unsafe { &mut BUF };

    Js::set_canvas("#example_canvas_1");

    /* wasm architecture */

    let wasm_pages = Wasm::memory_pages();
    let wasm_bytes = Wasm::memory_bytes();
    Js::console_log(&format_buf![?buf, "Wasm memory pages: {wasm_pages}, bytes: {wasm_bytes}"]);
    Js::console_log(&format_buf![?buf, "Wasm::bulk-memory: {}", Wasm::has_bulk_memory()]);
    Js::console_log(&format_buf![?buf, "Wasm::simd128: {}", Wasm::has_simd()]);
    Js::console_log(&format_buf![?buf, "Wasm::mutable-globals: {}", Wasm::has_mutable_globals()]);

    /* console */

    Js::console_log(format_buf![?buf, "example log at: {}ms", Js::performance_now()]);
    Js::console_info("example info");
    Js::console_debug("example debug");
    Js::console_warn("example warn");
    Js::console_error("example error");
    Js::console_trace();

    /* window */

    // eval
    Js::window_eval("console.log('Hello from Rust!');");
    Js::window_eval_timeout("console.log('Delayed!');", 1000);
    Js::window_eval_interval("console.log('Repeating!');", 2000);
    let cleared = Js::window_eval_timeout("console.error('This should not run!');", 1500);
    Js::window_clear_timeout(cleared);

    /* draw shapes */

    Js::fill_style(255, 0, 0);
    Js::fill_rect(10.0, 10.0, 100.0, 100.0);
    Js::fill_style(0, 255, 0);
    Js::draw_circle(150.0, 50.0, 30.0);
    Js::stroke_style(0, 0, 255);
    Js::draw_line(200.0, 20.0, 300.0, 100.0);

    /* text */

    let metrics = Js::measure_text_full("Hello, world!");
    Js::console_log(&format_buf![?buf, "{metrics:?}"]);

    /* events */

    // Add an event listener to the canvas for clicks
    Js::event_add_listener("#example_canvas_1", JsEventKind::Click, canvas_click);
    Js::fill_style(0, 0, 0);
    Js::fill_text("Click the canvas!", 60.0, 30.0);

    // mouse
    Js::event_add_listener_mouse("window", JsEventKind::MouseDown, my_mouse_callback);

    // pointer
    Js::event_add_listener_pointer("window", JsEventKind::PointerDown, my_pointer_callback);

    // panic!("let's trigger a panic");
}

/// Called when clicking on the canvas.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_click() {
    let buf: &mut [u8] = unsafe { &mut BUF };
    let time = Js::performance_now();

    let times = Js::performance_event_count(JsEventKind::Click) + 1;
    Js::console_log(format_buf![?buf, "Canvas clicked {times} times"]);

    let origin = Js::performance_time_origin();
    Js::console_log(format_buf![?buf, "origin {origin}ms"]);

    Js::fill_style(200, 0, 50);
    Js::fill_rect(50.0, 50.0, 100.0, 100.0);

    Js::console_log(format_buf![?buf, "Canvas clicked at: {time}ms"]);
    Js::fill_style(50, 50, 200);
    Js::fill_text(format_buf![?buf, "time: {time}ms"], 60.0, 100.0);
}

#[unsafe(no_mangle)]
pub extern "C" fn my_mouse_callback(event: JsEventMouse) {
    let buf: &mut [u8] = unsafe { &mut BUF };
    Js::console_log(&format_buf![?buf, "MOUSE: {event:?}"]);
}
#[unsafe(no_mangle)]
pub extern "C" fn my_pointer_callback(event: JsEventPointer) {
    let buf: &mut [u8] = unsafe { &mut BUF };
    Js::console_log(&format_buf![?buf, "POINT: {event:?}"]);
}
