// devela::examples::lang::js_web
//
//! A Javascript Web API canvas example.
//

// global config
#![no_std]
#![allow(static_mut_refs, reason = "safe in single-threaded")]
devela::set_panic_handler![web];

use devela::{
    JsConsole as console, Wasm, Web, WebEventKind, WebEventMouse, WebEventPointer, format_buf,
};

/// Static string buffer for printing to the console.
static mut BUF: [u8; 256] = [0; 256];

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let buf: &mut [u8] = unsafe { &mut BUF };

    Web::set_canvas("#example_canvas_1");

    /* wasm architecture */

    let wasm_pages = Wasm::memory_pages();
    let wasm_bytes = Wasm::memory_bytes();
    console::log(&format_buf![?buf, "Wasm memory pages: {wasm_pages}, bytes: {wasm_bytes}"]);
    console::log(&format_buf![?buf, "Wasm::bulk-memory: {}", Wasm::has_bulk_memory()]);
    console::log(&format_buf![?buf, "Wasm::simd128: {}", Wasm::has_simd()]);
    console::log(&format_buf![?buf, "Wasm::mutable-globals: {}", Wasm::has_mutable_globals()]);

    /* console */

    console::log(format_buf![?buf, "example log at: {}ms", Web::performance_now()]);
    console::info("example info");
    console::debug("example debug");
    console::warn("example warn");
    console::error("example error");
    console::trace();

    /* window */

    // eval
    Web::window_eval("console.log('Hello from Rust!');");
    Web::window_eval_timeout("console.log('Delayed!');", 1000);
    Web::window_eval_interval("console.log('Repeating!');", 2000);
    let cleared = Web::window_eval_timeout("console.error('This should not run!');", 1500);
    Web::window_clear_timeout(cleared);

    /* draw shapes */

    Web::fill_style(255, 0, 0);
    Web::fill_rect(10.0, 10.0, 100.0, 100.0);
    Web::fill_style(0, 255, 0);
    Web::draw_circle(150.0, 50.0, 30.0);
    Web::stroke_style(0, 0, 255);
    Web::draw_line(200.0, 20.0, 300.0, 100.0);

    /* text */

    let metrics = Web::measure_text_full("Hello, world!");
    console::log(&format_buf![?buf, "{metrics:?}"]);

    /* events */

    // Add an event listener to the canvas for clicks
    Web::event_add_listener("#example_canvas_1", WebEventKind::Click, canvas_click);
    Web::fill_style(0, 0, 0);
    Web::fill_text("Click the canvas!", 60.0, 30.0);

    // mouse
    Web::event_add_listener_mouse("window", WebEventKind::MouseDown, my_mouse_callback);

    // pointer
    Web::event_add_listener_pointer("window", WebEventKind::PointerDown, my_pointer_callback);

    // panic!("let's trigger a panic");
}

/// Called when clicking on the canvas.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_click() {
    let buf: &mut [u8] = unsafe { &mut BUF };
    let time = Web::performance_now();

    let times = Web::performance_event_count(WebEventKind::Click) + 1;
    console::log(format_buf![?buf, "Canvas clicked {times} times"]);

    let origin = Web::performance_time_origin();
    console::log(format_buf![?buf, "origin {origin}ms"]);

    Web::fill_style(200, 0, 50);
    Web::fill_rect(50.0, 50.0, 100.0, 100.0);

    console::log(format_buf![?buf, "Canvas clicked at: {time}ms"]);
    Web::fill_style(50, 50, 200);
    Web::fill_text(format_buf![?buf, "time: {time}ms"], 60.0, 100.0);
}

#[unsafe(no_mangle)]
pub extern "C" fn my_mouse_callback(event: WebEventMouse) {
    let buf: &mut [u8] = unsafe { &mut BUF };
    console::log(&format_buf![?buf, "MOUSE: {event:?}"]);
}
#[unsafe(no_mangle)]
pub extern "C" fn my_pointer_callback(event: WebEventPointer) {
    let buf: &mut [u8] = unsafe { &mut BUF };
    console::log(&format_buf![?buf, "POINT: {event:?}"]);
}
