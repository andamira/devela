// devela/examples/sys/os/browser/api/main.rs
//
//! A Javascript Web API canvas example.
//

#![no_std]
#![allow(clippy::deref_addrof, reason = "safe references to static mut")]
// https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html#safe-references

use devela::{Event, JsConsole as console, JsInstant, Wasm, WasmAlloc};
use devela::{Web, WebDocument as document, WebWindow as window};
use devela::{
    WebEventIngress, WebEventKey, WebEventKind, WebEventMouse, WebEventPointer, WebEventWheel,
};
use devela::{format_buf as fmt, items, set_panic_handler};

set_panic_handler![web];

#[global_allocator]
static ALLOCATOR: WasmAlloc = WasmAlloc::INIT;

/// Global formatting buffer shared by browser callbacks and frame processing.
static mut BUF: [u8; 1024] = [0; 1024];

/// Runs `f` with exclusive access to the global formatting buffer.
fn with_buf<R>(f: impl FnOnce(&mut [u8]) -> R) -> R {
    // SAFETY: This example runs on one browser main thread. Callers keep the
    // access within `f` and do not trigger nested access to `BUF`.
    f(unsafe { &mut *&raw mut BUF })
}

/// Maximum number of normalized web events buffered between animation frames.
///
/// Manual keyboard and wheel stress could overflow a capacity of 7;
/// a capacity of 16 left comfortable headroom.
const EVENT_CAP: usize = 16;

/// Global event ingress shared by browser callbacks and frame processing.
static mut INGRESS: WebEventIngress<EVENT_CAP> = WebEventIngress::new();

/// Runs `f` with exclusive access to the global web event ingress.
fn with_ingress<R>(f: impl FnOnce(&mut WebEventIngress<EVENT_CAP>) -> R) -> R {
    // SAFETY: This example runs on one browser main thread. Callers keep the
    // access within `f` and do not trigger nested access to `INGRESS`.
    f(unsafe { &mut *&raw mut INGRESS })
}

/// Initializes the Web API demonstration and starts its animation-frame loop.
#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let mut log_buffer = [0; 1024];
    let buf = &mut log_buffer;

    // secondary buffer
    let mut buffer2 = [0; 1024];
    let buf2 = &mut buffer2;

    let start_time = Web::performance_now();

    /* log */

    console::info("# log");
    console::log(fmt![?buf, "example log at: {start_time}ms"]);
    console::debug("example debug");
    console::warn("example warn");
    console::error("example error");
    // console::trace();
    // panic!("let's trigger a panic");

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
    window::set_name("web_api example");

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
    window::eval_timeout("console.log('Delayed!');", 1000);
    window::eval_interval("console.log('Repeating!');", 2000);
    let cleared = window::eval_timeout("console.error('This should not run!');", 1500);
    window::clear_timeout(cleared);

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
    register_ingress_listeners();
    Web::event_add_listener("#example_canvas_1", WebEventKind::Click, canvas_click);

    /* frame loop */

    window::request_animation_frame(web_frame);
}

/// Runs one browser-driven application frame and schedules the next one.
#[unsafe(no_mangle)]
pub extern "C" fn web_frame(_timestamp: JsInstant) {
    drain_events();
    // update();
    // draw();

    // with_buf(|b| console::log(fmt![?b, "web frame at {_timestamp}ms"]));
    window::request_animation_frame(web_frame);
}

/// Drains the normalized events queued since the previous frame.
fn drain_events() {
    while let Some(event) = with_ingress(|ingress| ingress.poll_event()) {
        handle_event(event);
    }
    report_dropped_events();
}

/// Reports and resets the ingress overflow count.
fn report_dropped_events() {
    let dropped = with_ingress(|ingress| ingress.take_dropped());
    if dropped != 0 {
        with_buf(|b| console::warn(fmt![?b, "EVENTS DROPPED: {dropped}"]));
    }
}

/// Handles one normalized application event.
fn handle_event(event: Event) {
    with_buf(|buf| console::log(fmt![?buf, "POLL: {event:?}"]));
}

// /// Advances the application state for the current frame.
// fn update() { }

// /// Presents the application state for the current frame.
// fn draw() { }

/// Registers the browser event listeners that feed the global ingress.
fn register_ingress_listeners() {
    Web::event_add_listener_key("window", WebEventKind::KeyDown, key_cb);
    Web::event_add_listener_key("window", WebEventKind::KeyUp, key_cb);

    Web::event_add_listener_mouse("window", WebEventKind::MouseDown, mouse_cb);
    // Web::event_add_listener_mouse("window", WebEventKind::MouseMove, mouse_cb);

    Web::event_add_listener_pointer("window", WebEventKind::PointerDown, pointer_cb);
    // Web::event_add_listener_pointer("window", WebEventKind::PointerUp, pointer_cb);
    // Web::event_add_listener_pointer("window", WebEventKind::PointerMove, pointer_cb);

    Web::event_add_listener_wheel("window", WebEventKind::Wheel, wheel_cb);
}
items! {
    extern "C" fn key_cb(event: WebEventKey) { with_ingress(|i| i.push_key(event)); }
    extern "C" fn mouse_cb(event: WebEventMouse) { with_ingress(|i| i.push_mouse(event)); }
    extern "C" fn pointer_cb(event: WebEventPointer) { with_ingress(|i| i.push_pointer(event)); }
    extern "C" fn wheel_cb(event: WebEventWheel) { with_ingress(|i| i.push_wheel(event)); }
}

/// Handles the canvas click directly in the browser callback.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_click() {
    Web::fill_style(200, 0, 50);
    Web::fill_rect(50.0, 50.0, 100.0, 100.0);

    let time = Web::performance_now();
    let times = Web::performance_event_count(WebEventKind::Click) + 1;
    with_buf(|b| console::log(fmt![?b, "Canvas clicked {times} times"]));

    let origin = Web::performance_time_origin();
    with_buf(|b| console::log(fmt![?b, "origin: {origin}ms"]));
    with_buf(|b| console::log(fmt![?b, "Canvas clicked at: {time}ms"]));

    Web::fill_style(50, 50, 200);
    with_buf(|b| Web::fill_text(fmt![?b, "time: {time}ms"], 60.0, 100.0));
}
