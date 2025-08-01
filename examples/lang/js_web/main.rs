// devela::examples::lang::js_web
//
//! A Javascript Web API canvas example.
//

// global config
#![no_std]
#![allow(static_mut_refs, reason = "safe in single-threaded")]
devela::set_panic_handler![web];

use devela::{
    JsConsole as console, Wasm, Web, WebEventKind, WebEventMouse, WebEventPointer,
    WebWindow as window, format_buf as fmt,
};

/// Static string buffer for printing to the console without allocation.
static mut BUF: [u8; 1024] = [0; 1024];
/// Secondary buffer for when `BUF` is already busy.
static mut BUF2: [u8; 1024] = [0; 1024];

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let (buf, buf2): (&mut [u8], &mut [u8]) = unsafe { (&mut BUF, &mut BUF2) };
    let start_time = Web::performance_now();

    /* log */

    console::info("# log");

    console::log(fmt![?buf, "example log at: {start_time}ms"]);
    console::debug("example debug");
    console::warn("example warn");
    console::error("example error");
    // console::trace();

    /* wasm architecture */

    console::info("# wasm");

    let wasm_pages = Wasm::memory_pages();
    let wasm_bytes = Wasm::memory_bytes();
    console::log(fmt![?buf, "Wasm memory pages: {wasm_pages}, bytes: {wasm_bytes}"]);
    console::log(fmt![?buf, "Wasm::bulk-memory: {}", Wasm::has_bulk_memory()]);
    console::log(fmt![?buf, "Wasm::simd128: {}", Wasm::has_simd()]);
    console::log(fmt![?buf, "Wasm::mutable-globals: {}", Wasm::has_mutable_globals()]);

    /* window */

    console::info("# window");

    window::set_name("sopanum1");
    console::log(fmt![?buf, "  name: {:?}", window::name_buf(buf2)]);

    let window_state = window::state();
    console::debug(fmt![?buf, "  {window_state:?}"]);

    console::log(fmt![?buf, "  is_closed: {:?}", window::is_closed()]);
    console::log(fmt![?buf, "  is_coi: {:?}", window::is_coi()]);
    console::log(fmt![?buf, "  is_secure: {:?}", window::is_secure()]);
    console::log(fmt![?buf, "  is_popup: {:?}", window::is_popup()]);

    // eval
    window::eval("console.log('Hello from Rust!');");
    window::eval_timeout("console.log('Delayed!');", 1000);
    window::eval_interval("console.log('Repeating!');", 2000);
    let cleared = window::eval_timeout("console.error('This should not run!');", 1500);
    window::clear_timeout(cleared);

    /* canvas */

    console::info("# canvas");

    Web::set_canvas("#example_canvas_1");

    Web::fill_style(255, 0, 0);
    Web::fill_rect(10.0, 10.0, 100.0, 100.0);
    Web::fill_style(0, 255, 0);
    Web::draw_circle(150.0, 50.0, 30.0);
    Web::stroke_style(0, 0, 255);
    Web::draw_line(200.0, 20.0, 300.0, 100.0);

    let text_click = "Click the canvas!";
    Web::fill_style(0, 0, 0);
    Web::fill_text(text_click, 60.0, 30.0);

    let text_metrics = Web::measure_text_full(text_click);
    console::log(fmt![?buf, "{text_metrics:?}"]);

    /* events */

    console::info("# events");

    // Add an event listener to the canvas for clicks
    Web::event_add_listener("#example_canvas_1", WebEventKind::Click, canvas_click);

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
    console::log(fmt![?buf, "Canvas clicked {times} times"]);

    let origin = Web::performance_time_origin();
    console::log(fmt![?buf, "origin {origin}ms"]);

    Web::fill_style(200, 0, 50);
    Web::fill_rect(50.0, 50.0, 100.0, 100.0);

    console::log(fmt![?buf, "Canvas clicked at: {time}ms"]);
    Web::fill_style(50, 50, 200);
    Web::fill_text(fmt![?buf, "time: {time}ms"], 60.0, 100.0);
}

#[unsafe(no_mangle)]
pub extern "C" fn my_mouse_callback(event: WebEventMouse) {
    let buf: &mut [u8] = unsafe { &mut BUF };
    console::log(fmt![?buf, "MOUSE: {event:?}"]);
}
#[unsafe(no_mangle)]
pub extern "C" fn my_pointer_callback(event: WebEventPointer) {
    let buf: &mut [u8] = unsafe { &mut BUF };
    console::log(fmt![?buf, "POINT: {event:?}"]);
}
