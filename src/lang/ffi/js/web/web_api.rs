// devela::lang::ffi::js::web::web_api
// (in sync with ./web_api.js)
//
//! Defines [`Web`] and implements Web API methods.
//
// TOC
// - definition
// - helpers
// - methods

use devela::js_reexport;
#[cfg(feature = "unsafe_ffi")]
#[allow(unused_imports, reason = "not(windows)")]
use devela::{
    Js, JsInstant, JsTextMetrics, JsTextMetricsFull, JsTimeout, TaskPoll, WebEventKind,
    WebEventMouse, WebEventPointer, WebPermission, WebPermissionState, WebWorker, WebWorkerError,
    WebWorkerJob, js_bool, js_doc, js_int32, js_number, js_uint32, transmute,
};
#[cfg(all(feature = "alloc", feature = "unsafe_ffi"))]
use devela::{String, Vec, vec_ as vec};

/* definition */

#[doc = crate::TAG_NAMESPACE!()]
/// A Web API namespace.
///
/// # Features
/// All methods depend on the `unsafe_ffi` feature and the `wasm32` architecture.
///
/// # Methods
/// - core APis
///   - [console](#web-api-console)
//    - document
///   - [events](#web-api-events)
///   - [history & location](#web-api-history--location)
///   - [permissions](#web-api-permissions)
//   - [url](#web-api-url--urlsearchparams)
///   - [window](#web-api-window)
/// - extended APis
///   - media & graphics
///     - [canvas](#web-api-canvas)
///  - performance & optimization
///    - [performance](#web-api-performance)
///  - advanced & experimental
///    - [workers](#web-api-workers)
pub struct Web;

/* methods: core APIs */

/// # Web API Events
///
/// Provides event handling for interactivity.
/// Uses Rust function pointers to manage callbacks.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Event>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventTarget>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventListener>
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    #[doc = js_doc!("EventTarget", "addEventListener")]
    /// Attaches a Rust function `event` listener from an `element`.
    pub fn event_add_listener(element: &str, event: WebEventKind, rust_fn: extern "C" fn()) {
        unsafe {
            event_add_listener(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), rust_fn as usize);
        }
    }
    #[doc = js_doc!("EventTarget", "removeEventListener")]
    /// Removes a a Rust function `event` listener from an `element`.
    pub fn event_remove_listener(element: &str, event: WebEventKind, rust_fn: extern "C" fn()) {
        unsafe {
            event_remove_listener(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), rust_fn as usize);
        }
    }
    #[doc = js_doc!("EventTarget", "addEventListener")]
    /// Attaches a JavaScript function `event` listener on an `element`.
    pub fn event_add_listener_js(element: &str, event: WebEventKind, js_fn_name: &str) {
        unsafe {
            event_add_listener_js(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), js_fn_name.as_ptr(), js_fn_name.len());
        }
    }
    #[doc = js_doc!("EventTarget", "removeEventListener")]
    /// Removes a JavaScript function `event` listener from an `element`.
    pub fn event_remove_listener_js(element: &str, event: WebEventKind, js_fn_name: &str) {
        unsafe {
            event_remove_listener_js(element.as_ptr(), element.len(), event.as_str().as_ptr(),
            event.as_str().len(), js_fn_name.as_ptr(), js_fn_name.len());
        }
    }
    //
    #[doc = js_doc!("EventTarget", "addEventListener")]
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
    #[doc = js_doc!("EventTarget", "addEventListener")]
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
    /// # Example
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
        buttons: js_int32, x: js_number, y: js_number, etype: js_int32, time_stamp: js_number) {
        let callback = callback_ptr as *const ();
        let callback: extern "C" fn(WebEventMouse) = unsafe { transmute(callback) };
        let etype = WebEventKind::from_repr(etype as u8);
        let time_stamp = JsInstant::from_millis_f64(time_stamp);
        callback(WebEventMouse::new(x, y, button as u8, buttons as u8, etype, time_stamp));
    }
    /// WebAssembly mouse event callback dispatcher.
    ///
    /// - Called from JavaScript when a pointer event is fired.
    /// - Passes the `WebEventPointer` struct to the Rust callback.
    ///
    /// # Safety
    /// - `callback_ptr` must be a valid function pointer.
    // WAIT: In firefox + linux + X11 needs the env variable `MOZ_USE_XINPUT2=1`
    // https://bugzilla.mozilla.org/show_bug.cgi?id=1606832
    // https://bugzilla.mozilla.org/show_bug.cgi?id=1207700
    // https://bugzilla.mozilla.org/show_bug.cgi?id=1822714
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn wasm_callback_pointer(callback_ptr: usize, id: js_int32,
        x: js_number, y: js_number, pressure: js_number, tilt_x: js_int32, tilt_y: js_int32,
        twist: js_int32, etype: js_int32, time_stamp: js_number) {
        let callback = callback_ptr as *const ();
        let callback: extern "C" fn(WebEventPointer) = unsafe { transmute(callback) };
        let etype = WebEventKind::from_repr(etype as u8);
        let time_stamp = JsInstant::from_millis_f64(time_stamp);
        callback(WebEventPointer::new(x, y, pressure, id, tilt_x as i8, tilt_y as i8, twist as u16,
            etype, time_stamp));
    }
}
js_reexport! {
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
}

/// # Web API history & location
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/History>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Location>
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    #[doc = js_doc!("History", "back")]
    /// Moves the browser back one step in the session history.
    pub fn history_back() { unsafe { history_back(); } }

    #[doc = js_doc!("History", "forward")]
    /// Moves the browser forward one step in the session history.
    pub fn history_forward() { unsafe { history_forward(); } }

    #[doc = js_doc!("History", "go")]
    /// Moves the browser to a specific point in the session history.
    /// Use negative values to go back, positive to go forward.
    pub fn history_go(delta: js_int32) { unsafe { history_go(delta); } }

    #[doc = js_doc!("History", "pushState")]
    /// Adds an entry to the session history stack.
    pub fn history_push_state(state: &str, title: &str, url: &str) {
        unsafe { history_push_state(state.as_ptr(), state.len(), title.as_ptr(), title.len(),
            url.as_ptr(), url.len()); }
    }

    #[doc = js_doc!("History", "replaceState")]
    /// Modifies the current history entry without creating a new one.
    pub fn history_replace_state(state: &str, title: &str, url: &str) {
        unsafe { history_replace_state(state.as_ptr(), state.len(), title.as_ptr(), title.len(),
            url.as_ptr(), url.len()); }
    }

    #[doc = js_doc!("Location", "reload")]
    /// Reloads the current document.
    pub fn location_reload() { unsafe { location_reload(); } }

    #[doc = js_doc!("Location", "assign")]
    /// Loads the specified URL.
    pub fn location_assign(url: &str) { unsafe { location_assign(url.as_ptr(), url.len()); } }

    #[doc = js_doc!("Location", "replace")]
    /// Replaces the current document with the given URL,
    /// without creating a new entry in the history.
    pub fn location_replace(url: &str) { unsafe { location_replace(url.as_ptr(), url.len()); } }
}
js_reexport! {
    [ module: "api_history_location" ]
    unsafe fn history_back();
    unsafe fn history_forward();
    unsafe fn history_go(delta: js_int32);
    unsafe fn "history_pushState" history_push_state(state_ptr: *const u8, state_len: usize,
        title_ptr: *const u8, title_len: usize, url_ptr: *const u8, url_len: usize);
    unsafe fn "history_replaceState" history_replace_state(state_ptr: *const u8, state_len: usize,
        title_ptr: *const u8, title_len: usize, url_ptr: *const u8, url_len: usize);
    //
    unsafe fn location_reload();
    unsafe fn location_assign(url_ptr: *const u8, url_len: usize);
    unsafe fn location_replace(url_ptr: *const u8, url_len: usize);
}

/// # Web API permissions
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    #[doc = js_doc!("Permissions", "query")]
    /// Queries the status of a given permission.
    ///
    /// Returns `Granted`, `Denied`, `Prompt`, or `Unknown` if unsupported.
    pub fn permissions_query(permission: WebPermission) -> WebPermissionState {
        unsafe { permissions_query(permission.as_str().as_ptr(), permission.as_str().len()) }
        .into()
    }
}
js_reexport! {
    [ module: "api_permissions" ]
    unsafe fn permissions_query(name_ptr: *const u8, name_len: usize) -> js_int32;
}

/// # Web API window
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Window>
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    // TODO
    // #[doc = js_doc!("Window", "document")]
    // /// Returns a reference to the document contained in the window.
    // pub fn window_document() -> WebDocument { WebDocument::get() }

    #[doc = js_doc!("Window", "setTimeout")]
    /// Calls a function after a delay in milliseconds.
    pub fn window_set_timeout(callback: extern "C" fn(), delay_ms: js_uint32) -> JsTimeout {
        JsTimeout { id: unsafe { window_set_timeout(callback as usize, delay_ms) } }
    }
    #[doc = js_doc!("Window", "setInterval")]
    /// Calls a function repeatedly at a fixed interval in milliseconds.
    pub fn window_set_interval(callback: extern "C" fn(), interval_ms: js_uint32) -> JsTimeout {
        JsTimeout { id: unsafe { window_set_interval(callback as usize, interval_ms) } }
    }
    #[doc = js_doc!("Window", "clearTimeout")]
    #[doc = js_doc!("Window", "clearInterval")]
    /// Cancels a timeout or interval.
    pub fn window_clear_timeout(id: JsTimeout) { window_clear_timeout(id.id); }

    /// Executes JavaScript code immediately.
    /// ## Security Warning
    /// - Avoid passing untrusted input, as this executes arbitrary JS.
    /// - Ensure all evaluated code is **safe and controlled**.
    pub fn window_eval(js_code: &str) { unsafe { window_eval(js_code.as_ptr(), js_code.len()); } }
    #[doc = js_doc!("Window", "setTimeout")]
    /// Executes JavaScript code after a delay in milliseconds.
    pub fn window_eval_timeout(js_code: &str, delay_ms: js_uint32) -> JsTimeout { JsTimeout {
        id: unsafe { window_eval_timeout(js_code.as_ptr(), js_code.len(), delay_ms) } } }
    #[doc = js_doc!("Window", "setInterval")]
    /// Executes JavaScript code repeatedly at a fixed interval in milliseconds.
    pub fn window_eval_interval(js_code: &str, interval_ms: js_uint32) -> JsTimeout { JsTimeout {
        id: unsafe { window_eval_interval(js_code.as_ptr(), js_code.len(), interval_ms) } } }

    #[doc = js_doc!("Window", "requestAnimationFrame")]
    /// Requests an animation frame, executing the given `callback`.
    pub fn window_request_animation_frame(callback: extern "C" fn()) -> js_uint32 {
        unsafe { window_request_animation_frame(callback as usize) } }
    /// Cancels a request for an animation frame.
    pub fn window_cancel_animation_frame(id: js_uint32) { window_cancel_animation_frame(id); }
}
js_reexport! { [module: "api_window"]
    unsafe fn window_set_timeout(callback_ptr: usize, delay_ms: js_uint32) -> js_uint32;
    unsafe fn window_set_interval(callback_ptr: usize, interval_ms: js_uint32) -> js_uint32;
    safe fn window_clear_timeout(timeout_id: js_uint32);
    //
    unsafe fn window_eval(js_code_ptr: *const u8, js_code_len: usize);
    unsafe fn window_eval_timeout(js_code_ptr: *const u8, js_code_len: usize, delay_ms: js_uint32)
        -> js_uint32;
    unsafe fn window_eval_interval(js_code_ptr: *const u8, js_code_len: usize,
        interval_ms: js_uint32) -> js_uint32;
    //
    unsafe fn window_request_animation_frame(callback_ptr: usize) -> js_uint32;
    safe fn window_cancel_animation_frame(requestId: js_uint32);
}

/* methods: extended APIs */

/// # Web API canvas
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D>
/// - <https://html.spec.whatwg.org/multipage/canvas.html>
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    /* misc. */
    /// Sets the active canvas using a CSS `selector`.
    pub fn set_canvas(selector: &str) { unsafe { set_canvas(selector.as_ptr(), selector.len()); } }

    /* color settings */
    #[doc = js_doc!(canvas "fillStyle")]
    /// Sets the color or style for filling shapes.
    pub fn fill_style(r: u8, g: u8, b: u8) { fill_style(r, g, b); }
    #[doc = js_doc!(canvas "strokeStyle")]
    /// Sets the color or style for lines.
    pub fn stroke_style(r: u8, g: u8, b: u8) { stroke_style(r, g, b); }

    /* drawing rectangles */
    #[doc = js_doc!(canvas "fillRect")]
    /// Draws a filled rectangle.
    pub fn fill_rect(x: js_number, y: js_number, w: js_number, h: js_number)
        { fill_rect(x, y, w, h); }
    #[doc = js_doc!(canvas "strokeRect")]
    /// Draws a rectangular outline.
    pub fn stroke_rect(x: js_number, y: js_number, w: js_number, h: js_number)
        { stroke_rect(x, y, w, h); }
    #[doc = js_doc!(canvas "clearRect")]
    /// Clears the specified rectangular area, making it fully transparent.
    pub fn clear_rect(x: js_number, y: js_number, w: js_number, h: js_number)
        { clear_rect(x, y, w, h); }

    /* drawing shapes */
    /// Draws a line.
    pub fn draw_line(x1: js_number, y1: js_number, x2: js_number, y2: js_number)
        { draw_line(x1, y1, x2, y2); }
    /// Draws a circle.
    pub fn draw_circle(x: js_number, y: js_number, radius: js_number)
        { draw_circle(x, y, radius); }

    /* drawing text */
    #[doc = js_doc!(canvas "fillText")]
    /// Draws filled text at the specified position.
    pub fn fill_text(text: &str, x: js_number, y: js_number) {
        unsafe { fill_text(text.as_ptr(), text.len(), x, y); }
    }
    #[doc = js_doc!(canvas "strokeText")]
    /// Draws text outline at the specified position.
    pub fn stroke_text(text: &str, x: js_number, y: js_number) {
        unsafe { stroke_text(text.as_ptr(), text.len(), x, y); }
    }
    #[doc = js_doc!(canvas "measureText")]
    /// Measures the essential properties of text.
    pub fn measure_text(text: &str) -> JsTextMetrics {
        let (mut metrics, ptr, len) = (JsTextMetrics::default(), text.as_ptr(), text.len());
        unsafe { measure_text(ptr, len, &mut metrics as *mut JsTextMetrics); }
        metrics
    }
    #[doc = js_doc!(canvas "measureTextFull")]
    /// Measures all available text metrics.
    pub fn measure_text_full(text: &str) -> JsTextMetricsFull {
        let (mut metrics, ptr, len) = (JsTextMetricsFull::default(), text.as_ptr(), text.len());
        unsafe { measure_text_full(ptr, len, &mut metrics as *mut JsTextMetricsFull); }
        metrics
    }
}
js_reexport! {
    [ module: "api_canvas" ]
    /* misc. */
    unsafe fn set_canvas(str_ptr: *const u8, str_len: usize);
    /* color settings */
    safe fn "fillStyle" fill_style(r: u8, g: u8, b: u8);
    safe fn "strokeStyle" stroke_style(r: u8, g: u8, b: u8);
    /* drawing rectangles */
    safe fn "fillRect" fill_rect(x: js_number, y: js_number, w: js_number, h: js_number);
    safe fn "strokeRect" stroke_rect(x: js_number, y: js_number, w: js_number, h: js_number);
    safe fn "clearRect" clear_rect(x: js_number, y: js_number, w: js_number, h: js_number);
    /* drawing paths */
    //
    /* drawing shapes */
    safe fn draw_line(x1: js_number, y1: js_number, x2: js_number, y2: js_number);
    safe fn draw_circle(x: js_number, y: js_number, radius: js_number);

    /* drawing text */
    unsafe fn "fillText" fill_text(str_ptr: *const u8, str_len: usize, x: js_number, y: js_number);
    unsafe fn "strokeText" stroke_text(str_ptr: *const u8, str_len: usize,
        x: js_number, y: js_number);
    unsafe fn "measureText" measure_text(text_ptr: *const u8, text_len: usize,
        out_metrics: *mut JsTextMetrics);
    unsafe fn "measureTextFull" measure_text_full(text_ptr: *const u8, text_len: usize,
        out_metrics: *mut JsTextMetricsFull);
}

/// # Web API performance
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Performance>
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    #[doc = js_doc!("Performance", "now")]
    /// Retrieves a high-resolution timestamp in milliseconds.
    pub fn performance_now() -> JsInstant { JsInstant::from_millis_f64(performance_now()) }
    #[doc = js_doc!("Performance", "timeOrigin")]
    /// Retrieves the time origin in milliseconds.
    pub fn performance_time_origin() -> JsInstant {
        JsInstant::from_millis_f64(performance_time_origin()) }
    #[doc = js_doc!("Performance", "eventCounts")]
    /// Retrieves the count of recorded events.
    pub fn performance_event_count(event: WebEventKind) -> js_uint32 {
        let name = event.as_str();
        unsafe { performance_event_count(name.as_ptr(), name.len()) }
    }
}
js_reexport! {
    [ module: "api_performance" ]
    safe fn "now" performance_now() -> js_number;
    safe fn "timeOrigin" performance_time_origin() -> js_number;
    unsafe fn "eventCounts" performance_event_count(event_ptr: *const u8, event_len: usize)
        -> js_uint32;
}

/// Web API workers
#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
impl Web {
    /// Spawns a Web Worker and returns its ID.
    pub fn worker_spawn(script: &str) -> Result<WebWorker, WebWorkerError> {
        let id = unsafe { worker_spawn(script.as_ptr(), script.len()) };
        if id == 0 { Err(WebWorkerError::InvalidScript) } else { Ok(WebWorker { id }) }
    }
    /// Checks if this worker is still active by querying JavaScript.
    pub fn worker_is_active(worker: WebWorker) -> js_bool { worker_is_active(worker.id()) }
    /// Stops a specific Web Worker by ID.
    pub fn worker_stop(worker: WebWorker) { worker_stop(worker.id()); }
    /// Stops all Web Workers.
    pub fn worker_stop_all() { worker_stop_all(); }
    /// Returns the number of active workers.
    pub fn worker_list_len() -> usize { worker_list_len() as usize }
    /// Returns the list of active worker IDs.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn worker_list() -> Vec<WebWorker> {
        let len = worker_list_len() as usize;
        let mut workers = vec![WebWorker::default(); len];
        let count = Web::worker_list_buf(&mut workers);
        workers.truncate(count);
        workers
    }
    /// Writes active worker handles into a buffer and returns the number written.
    pub fn worker_list_buf(buffer: &mut [WebWorker]) -> usize {
        let len = worker_list_len() as usize;
        let count = len.min(buffer.len());
        unsafe { worker_list(buffer.as_mut_ptr() as *mut js_uint32, count as js_uint32); }
        count
    }
    /// Sends a message to a specific Web Worker.
    pub fn worker_send_message(worker: WebWorker, msg: &str) {
        unsafe { worker_send_message(worker.id(), msg.as_ptr(), msg.len()); }
    }
    /// Requests execution of JavaScript inside a worker.
    pub fn worker_eval(worker: WebWorker, js_code: &str) -> WebWorkerJob {
        let id = unsafe { worker_eval(worker.id(), js_code.as_ptr(), js_code.len()) };
        WebWorkerJob { worker, id }
    }
    /// Polls for the result of a JavaScript execution in a worker.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn worker_poll(job: WebWorkerJob) -> TaskPoll<Result<String, WebWorkerError>> {
        if !job.worker().is_active() { return TaskPoll::Ready(Err(WebWorkerError::WorkerNotFound)); }
        let mut first_check = true;
        let result = Js::read_string_capped(128, false, |ptr, cap| {
            let res = unsafe { worker_poll_buf(job.id(), ptr, cap as usize) as js_int32 };
            if first_check {
                first_check = false; // Intercept status codes before bothering with decoding
                if res == 0 { return 0; } else if res == -1 { return -1; } // pending or not found
            }
            res
        });
        match result.as_str() {
            "" => TaskPoll::Pending, // Covers 0 and -1 (mapped above)
            _ => TaskPoll::Ready(Ok(result)),
        }
    }
    /// Polls for the result of a JavaScript execution in a worker, and writes it into `buffer`.
    ///
    /// If ready, returns the number of bytes written to the buffer.
    pub fn worker_poll_buf(job: WebWorkerJob, buffer: &mut [u8])
        -> TaskPoll<Result<usize, WebWorkerError>> {
        if !job.worker().is_active() { return TaskPoll::Ready(Err(WebWorkerError::WorkerNotFound)); }
        let written = unsafe { worker_poll_buf(job.id(), buffer.as_mut_ptr(), buffer.len()) };
        match written {
            0 => TaskPoll::Pending,
            js_uint32::MAX => TaskPoll::Ready(Err(WebWorkerError::JobNotFound)),
            _ => TaskPoll::Ready(Ok(written as usize)),
        }
    }
    /// Cancels an ongoing JavaScript evaluation.
    pub fn worker_eval_cancel(job: WebWorkerJob) { worker_cancel_eval(job.id()); }
}
js_reexport! { [module: "api_workers"]
    unsafe fn worker_spawn(script_ptr: *const u8, script_len: usize) -> js_uint32;
    safe fn worker_is_active(worker_id: js_uint32) -> js_bool;
    safe fn worker_stop(worker_id: js_uint32);
    safe fn worker_stop_all();
    safe fn worker_list_len() -> js_uint32;
    unsafe fn worker_list(worker_list_ptr: *mut js_uint32, len: js_uint32) -> js_uint32;
    unsafe fn worker_send_message(worker_id: js_uint32, msg_ptr: *const u8, msg_len: usize);
    unsafe fn worker_eval(worker_id: js_uint32, js_code_ptr: *const u8, js_code_len: usize)
        -> js_uint32;
    unsafe fn worker_poll_buf(job_id: js_uint32, buffer_ptr: *mut u8, buffer_len: usize)
        -> js_uint32;
    safe fn worker_cancel_eval(job_id: js_uint32);
}
