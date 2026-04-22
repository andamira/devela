// devela::sys::os::browser::web::api::events
// (in sync with ./web_api.js)
//
//! Implements the web events API.
//

use crate::EventWheelUnit;
use crate::transmute;
use crate::{_js_doc, _js_extern, JsInstant, js_int32, js_number};
use crate::{Web, WebEventKind, WebEventMouse, WebEventPointer, WebEventWheel};

/// # Web API Events
///
/// Provides event handling for interactivity.
/// Uses Rust function pointers to manage callbacks.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Event>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventTarget>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventListener>
#[rustfmt::skip]
impl Web {
    #[doc = _js_doc!("EventTarget", "addEventListener")]
    /// Attaches a Rust function `event` listener from an `element`.
    pub fn event_add_listener(element: &str, event: WebEventKind, rust_fn: extern "C" fn()) {
        unsafe {
            event_add_listener(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), rust_fn as usize);
        }
    }
    #[doc = _js_doc!("EventTarget", "removeEventListener")]
    /// Removes a a Rust function `event` listener from an `element`.
    pub fn event_remove_listener(element: &str, event: WebEventKind, rust_fn: extern "C" fn()) {
        unsafe {
            event_remove_listener(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), rust_fn as usize);
        }
    }
    #[doc = _js_doc!("EventTarget", "addEventListener")]
    /// Attaches a JavaScript function `event` listener on an `element`.
    pub fn event_add_listener_js(element: &str, event: WebEventKind, js_fn_name: &str) {
        unsafe {
            event_add_listener_js(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), js_fn_name.as_ptr(), js_fn_name.len());
        }
    }
    #[doc = _js_doc!("EventTarget", "removeEventListener")]
    /// Removes a JavaScript function `event` listener from an `element`.
    pub fn event_remove_listener_js(element: &str, event: WebEventKind, js_fn_name: &str) {
        unsafe {
            event_remove_listener_js(element.as_ptr(), element.len(), event.as_str().as_ptr(),
            event.as_str().len(), js_fn_name.as_ptr(), js_fn_name.len());
        }
    }
    //
    #[doc = _js_doc!("EventTarget", "addEventListener")]
    /// Attaches a Rust function as a `mouse event` listener on an `element`.
    ///
    /// The callback receives `WebEventMouse` with button, buttons mask, and coordinates.
    ///
    /// This will trigger on pointer events as well.
    pub fn event_add_listener_mouse(element: &str, event: WebEventKind,
        callback: extern "C" fn(WebEventMouse)) {
        unsafe {
            event_add_listener_mouse(element.as_ptr(), element.len(),
                event.as_str().as_ptr(), event.as_str().len(), callback as usize);
        }
    }
    #[doc = _js_doc!("EventTarget", "addEventListener")]
    /// Attaches a Rust function as a `pointer event` listener on an `element`.
    ///
    /// The callback receives `WebEventPointer` with id, coordinates, and pressure.
    pub fn event_add_listener_pointer(element: &str, event: WebEventKind,
        callback: extern "C" fn(WebEventPointer)) {
        unsafe {
            event_add_listener_pointer(element.as_ptr(), element.len(),
                event.as_str().as_ptr(), event.as_str().len(), callback as usize);
        }
    }
    #[doc = _js_doc!("EventTarget", "addEventListener")]
    /// Attaches a Rust function as a `wheel event` listener on an `element`.
    ///
    /// The callback receives [`WebEventWheel`] with raw browser deltas, unit,
    /// held-button mask, and viewport-relative coordinates.
    pub fn event_add_listener_wheel(element: &str, event: WebEventKind,
        callback: extern "C" fn(WebEventWheel)) {
        unsafe {
            event_add_listener_wheel(element.as_ptr(), element.len(),
                event.as_str().as_ptr(), event.as_str().len(), callback as usize);
        }
    }

    /* callbacks */

    /// Callback dispatcher for WebAssembly events without args.
    ///
    /// - This function is used by JavaScript to invoke a Rust function pointer.
    /// - It allows Rust event listeners to execute when triggered by JS.
    /// - The `callback_ptr` is a function pointer cast from JS,
    ///   which is then transmuted into a callable Rust function.
    ///
    /// # Safety
    /// `callback_ptr` must be a valid function pointer.
    ///
    /// # Examples
    /// ```ignore
    /// #[unsafe(no_mangle)]
    /// pub extern "C" fn my_callback() { Web::console_log("Button clicked!"); }
    /// Web::event_add_listener("#my_button", WebEventKind::Click, my_callback);
    /// ```
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn wasm_callback(callback_ptr: usize) {
        let callback = callback_ptr as *const ();
        let callback: extern "C" fn() = unsafe { transmute(callback) };
        callback();
    }
    /// WebAssembly mouse event callback dispatcher.
    ///
    /// - Called from JavaScript when a mouse event is fired.
    /// - Passes the `WebEventMouse` struct to the Rust callback.
    ///
    /// # Safety
    /// - `callback_ptr` must be a valid function pointer.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn wasm_callback_mouse(callback_ptr: usize, button: js_int32,
        buttons: js_int32, x: js_number, y: js_number, etype: js_int32, timestamp: js_number) {
        let callback = callback_ptr as *const ();
        let callback: extern "C" fn(WebEventMouse) = unsafe { transmute(callback) };
        let etype = WebEventKind::from_repr(etype as u8);
        let timestamp = JsInstant::from_millis_f64(timestamp);
        callback(WebEventMouse::new(x, y, button as u8, buttons as u8, etype, timestamp));
    }
    /// WebAssembly mouse event callback dispatcher.
    ///
    /// - Called from JavaScript when a pointer event is fired.
    /// - Passes the `WebEventPointer` struct to the Rust callback.
    ///
    /// # Safety
    /// - `callback_ptr` must be a valid function pointer.
    // WAIT: In firefox + linux + X11 needs the env variable `MOZ_USE_XINPUT2=1`
    // https://bugzilla.mozilla.org/show_bug.cgi?id=1606832 (closed)
    // https://bugzilla.mozilla.org/show_bug.cgi?id=1207700
    // https://bugzilla.mozilla.org/show_bug.cgi?id=1822714
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn wasm_callback_pointer(callback_ptr: usize, id: js_int32,
        x: js_number, y: js_number, pressure: js_number, tilt_x: js_int32, tilt_y: js_int32,
        twist: js_int32, etype: js_int32, timestamp: js_number) {
        let callback = callback_ptr as *const ();
        let callback: extern "C" fn(WebEventPointer) = unsafe { transmute(callback) };
        let etype = WebEventKind::from_repr(etype as u8);
        let timestamp = JsInstant::from_millis_f64(timestamp);
        callback(WebEventPointer::new(x, y, pressure, id, tilt_x as i8, tilt_y as i8, twist as u16,
            etype, timestamp));
    }
    /// WebAssembly wheel event callback dispatcher.
    ///
    /// - Called from JavaScript when a wheel event is fired.
    /// - Passes the `WebEventWheel` struct to the Rust callback.
    ///
    /// # Safety
    /// - `callback_ptr` must be a valid function pointer.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn wasm_callback_wheel(callback_ptr: usize, x: js_number, y: js_number,
        delta_x: js_number, delta_y: js_number, buttons: js_int32, unit: js_int32,
        timestamp: js_number) {
        let callback = callback_ptr as *const ();
        let callback: extern "C" fn(WebEventWheel) = unsafe { transmute(callback) };
        let unit = EventWheelUnit::from_web(unit as u8);
        let timestamp = JsInstant::from_millis_f64(timestamp);
        callback(WebEventWheel::new(x, y, delta_x, delta_y, buttons as u8, unit, timestamp));
    }
}
_js_extern! {
    [ module: "api_events" ]
    unsafe fn "event_addListener" event_add_listener(element_ptr: *const u8,
        element_len: usize, event_ptr: *const u8, event_len: usize, callback_ptr: usize);
    unsafe fn "event_removeListener" event_remove_listener(element_ptr: *const u8,
        element_len: usize, event_ptr: *const u8, event_len: usize, callback_ptr: usize);
    unsafe fn "event_addListenerJs" event_add_listener_js(
        element_ptr: *const u8, element_len: usize,
        event_ptr: *const u8, event_len: usize, js_fn_ptr: *const u8, js_fn_len: usize
    );
    unsafe fn "event_removeListenerJs" event_remove_listener_js(
        element_ptr: *const u8, element_len: usize,
        event_ptr: *const u8, event_len: usize, js_fn_ptr: *const u8, js_fn_len: usize
    );
    //
    unsafe fn "event_addListenerMouse" event_add_listener_mouse(element_ptr: *const u8,
        element_len: usize, event_ptr: *const u8, event_len: usize, callback_ptr: usize);
    unsafe fn "event_addListenerPointer" event_add_listener_pointer(element_ptr: *const u8,
        element_len: usize, event_ptr: *const u8, event_len: usize, callback_ptr: usize);
    unsafe fn "event_addListenerWheel" event_add_listener_wheel(element_ptr: *const u8,
        element_len: usize, event_ptr: *const u8, event_len: usize, callback_ptr: usize);
}
