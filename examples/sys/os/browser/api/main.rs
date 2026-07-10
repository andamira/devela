// devela/examples/sys/os/browser/api/main.rs
//
//! A Javascript Web API canvas example.
//

// global config
#![no_std]
#![allow(clippy::deref_addrof, reason = "safe references to static mut")]
// https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html#safe-references

use devela::{JsConsole as console, Wasm, WasmAlloc, format_buf as fmt, set_panic_handler};
use devela::{
    Web, WebDocument as document, WebEventKey, WebEventKind, WebEventMouse, WebEventPointer,
    WebEventWheel, WebWindow as window,
};

set_panic_handler![web];

#[global_allocator]
static ALLOCATOR: WasmAlloc = WasmAlloc::INIT;

/// Global string buffer for printing to the console without allocation.
static mut BUF: [u8; 1024] = [0; 1024];

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let buf = unsafe { &mut *&raw mut BUF };

    // secondary buffer
    let mut buffer2 = [0; 1024];
    let buf2 = &mut buffer2;

    #[cfg(feature = "time")]
    let start_time = Web::performance_now();

    /* log */

    console::info("# log");

    #[cfg(feature = "time")]
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
    // allocation
    console::log(fmt![?buf, "Wasm::heap_base():: {}", Wasm::heap_base()]);
    let rem1 = Wasm::remaining_memory();
    console::log(fmt![?buf, "Wasm remaining memory: {rem1}. Allocating 20 pages…"]);
    let _v = devela::Vec::<u8>::with_capacity(20 * Wasm::PAGE_BYTES);
    let rem2 = Wasm::remaining_memory();
    let diff = rem1 - rem2;
    console::log(fmt![?buf, "Wasm remaining memory: {rem2} ({diff} difference)"]);
    assert_eq![20 * Wasm::PAGE_BYTES, diff];

    /* window */

    console::info("# window");

    window::set_name("sopanum1");

    // Alternative methods when getting a string.
    console::log(fmt![?buf, "  name:       {:?}", window::name(buf2)]);
    console::log(fmt![?buf, "  name_alloc: {:?}", window::name_alloc()]);

    let window_state = window::state();
    console::debug(fmt![?buf, "  {window_state:?}"]);

    console::log(fmt![?buf, "  is_closed: {:?}", window::is_closed()]);
    console::log(fmt![?buf, "  is_coi: {:?}", window::is_coi()]);
    console::log(fmt![?buf, "  is_secure: {:?}", window::is_secure()]);
    console::log(fmt![?buf, "  is_popup: {:?}", window::is_popup()]);

    // eval
    window::eval("console.log('Hello from Rust!');");
    #[cfg(feature = "time")]
    {
        window::eval_timeout("console.log('Delayed!');", 1000);
        window::eval_interval("console.log('Repeating!');", 2000);
        let cleared = window::eval_timeout("console.error('This should not run!');", 1500);
        window::clear_timeout(cleared);
    }

    /* document */

    console::info("# document");

    console::log(fmt![?buf, "  is_compat_mode: {:?}", document::is_compat_mode()]);
    console::log(fmt![?buf, "  is_hidden: {:?}", document::is_hidden()]);
    console::log(fmt![?buf, "  content_type: {:?}", document::content_type(buf2)]);

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

    // keys
    Web::event_add_listener_key("window", WebEventKind::KeyDown, my_key_callback);
    Web::event_add_listener_key("window", WebEventKind::KeyUp, my_key_callback);

    // mouse
    Web::event_add_listener_mouse("window", WebEventKind::MouseDown, my_mouse_callback);
    // Web::event_add_listener_mouse("window", WebEventKind::MouseMove, my_mouse_callback);

    // pointer
    Web::event_add_listener_pointer("window", WebEventKind::PointerDown, my_pointer_callback);
    // Web::event_add_listener_pointer("window", WebEventKind::PointerUp, my_pointer_callback);
    // Web::event_add_listener_pointer("window", WebEventKind::PointerMove, my_pointer_callback);

    // wheel
    Web::event_add_listener_wheel("window", WebEventKind::Wheel, my_wheel_callback);

    // panic!("let's trigger a panic");
}

/// Called when clicking on the canvas.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_click() {
    let buf = unsafe { &mut *&raw mut BUF };

    Web::fill_style(200, 0, 50);
    Web::fill_rect(50.0, 50.0, 100.0, 100.0);

    #[cfg(feature = "time")]
    {
        let time = Web::performance_now();
        let times = Web::performance_event_count(WebEventKind::Click) + 1;

        console::log(fmt![?buf, "Canvas clicked {times} times"]);

        let origin = Web::performance_time_origin();
        console::log(fmt![?buf, "origin: {origin}ms"]);

        console::log(fmt![?buf, "Canvas clicked at: {time}ms"]);

        Web::fill_style(50, 50, 200);
        Web::fill_text(fmt![?buf, "time: {time}ms"], 60.0, 100.0);
    }
    #[cfg(not(feature = "time"))]
    {
        Web::fill_style(50, 50, 200);
        Web::fill_text(fmt![?buf, "`time` disabled"], 60.0, 100.0);
    }
}

#[unsafe(no_mangle)] #[rustfmt::skip]
pub extern "C" fn my_key_callback(event: WebEventKey) {
    let buf = unsafe { &mut *&raw mut BUF };
    console::log(fmt![?buf, "KEY: {:?}", event]);
}
#[unsafe(no_mangle)]
pub extern "C" fn my_mouse_callback(event: WebEventMouse) {
    let buf = unsafe { &mut *&raw mut BUF };
    console::log(fmt![?buf, "MOUSE: {:?}", event]);
}
#[unsafe(no_mangle)]
pub extern "C" fn my_pointer_callback(event: WebEventPointer) {
    let buf = unsafe { &mut *&raw mut BUF };
    console::log(fmt![?buf, "POINT: {:?}", event]);
}
#[unsafe(no_mangle)]
pub extern "C" fn my_wheel_callback(event: WebEventWheel) {
    let buf = unsafe { &mut *&raw mut BUF };
    console::log(fmt![?buf, "WHEEL: {:?}", event]);
}
