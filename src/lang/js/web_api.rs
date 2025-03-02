// devela::lang:js::web_api
//
//! Implements Web API methods for [`Js`].
//
// In sync with `./web_api.js`.
//
// TOC
// - core APis
//   - console
//   - events
//   - history
//   - permissions
//
// - extended APis
//   - media & graphics
//     - canvas
//   - system & hardware
//   - performance & optimization
//     - time
//   - advanced & experimental

use devela::{js_reexport, transmute, Js, JsEvent, JsPermission, JsPermissionState};

// helper for Web API doc links
#[rustfmt::skip]
macro_rules! web_api {
    ($path_with_end_bar:literal, $method:literal) => { concat!["([", $method,
        "](https://developer.mozilla.org/en-US/docs/Web/API/",
        $path_with_end_bar, $method, "))" ]
    };
    (canvas $method:literal) => { concat!["([", $method,
        "](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/",
        $method, "))" ]
    };
    (console $method:literal) => { concat!["([", $method,
        "](https://developer.mozilla.org/en-US/docs/Web/API/console/",
        $method, "_static))" ] };
}

/* core APIs */

/// # Web API console
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/console>
#[rustfmt::skip]
impl Js {
    #[doc = web_api!(console "debug")]
    /// Outputs a message to the console with the debug log level.
    pub fn console_debug(text: &str) { unsafe { console_debug(text.as_ptr(), text.len()); } }
    #[doc = web_api!(console "error")]
    /// Outputs a message to the console with the error log level.
    pub fn console_error(text: &str) { unsafe { console_error(text.as_ptr(), text.len()); } }
    #[doc = web_api!(console "info")]
    /// Outputs a message to the console with the info log level.
    pub fn console_info(text: &str) { unsafe { console_info(text.as_ptr(), text.len()); } }
    #[doc = web_api!(console "log")]
    /// Outputs a message to the console.
    pub fn console_log(text: &str) { unsafe { console_log(text.as_ptr(), text.len()); } }
    #[doc = web_api!(console "trace")]
    /// Outputs a stack trace.
    pub fn console_trace() { unsafe { console_trace(); } }
    #[doc = web_api!(console "warn")]
    /// Outputs a message to the console with the warning log level.
    pub fn console_warn(text: &str) { unsafe { console_warn(text.as_ptr(), text.len()); } }
    //
    #[doc = web_api!(console "group")]
    /// Creates a new inline group, indenting all following output by another level.
    pub fn console_group(text: &str) { unsafe { console_group(text.as_ptr(), text.len()); } }
    #[doc = web_api!(console "groupEnd")]
    /// Exits the current inline group.
    pub fn console_group_end() { unsafe { console_group_end(); } }
}
js_reexport! {
    [ module: "api_console" ]
    unsafe fn "console_debug" console_debug(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_error" console_error(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_info" console_info(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_log" console_log(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_trace" console_trace();
    unsafe fn "console_warn" console_warn(str_ptr: *const u8, str_len: usize);
    //
    unsafe fn "console_group" console_group(str_ptr: *const u8, str_len: usize);
    unsafe fn "console_groupEnd" console_group_end();
}

/// # Web API Events
///
/// Provides event handling for interactivity.
/// Uses Rust function pointers to manage callbacks.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Event>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventTarget>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventListener>
#[rustfmt::skip]
impl Js {
    #[doc = web_api!("Event", "addEventListener")]
    /// Attaches a Rust function `event` listener from an `element`.
    pub fn event_add_listener(element: &str, event: JsEvent, rust_fn: extern "C" fn()) {
        unsafe {
            event_add_listener(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), rust_fn as usize);
        }
    }
    #[doc = web_api!("Event", "removeEventListener")]
    /// Removes a a Rust function `event` listener from an `element`.
    pub fn event_remove_listener(element: &str, event: JsEvent, rust_fn: extern "C" fn()) {
        unsafe {
            event_remove_listener(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), rust_fn as usize);
        }
    }
    #[doc = web_api!("Event", "addEventListenerJs")]
    /// Attaches a JavaScript function `event` linstener to an `element`.
    pub fn event_add_listener_js(element: &str, event: JsEvent, js_fn_name: &str) {
        unsafe {
            event_add_listener_js(element.as_ptr(), element.len(),
            event.as_str().as_ptr(), event.as_str().len(), js_fn_name.as_ptr(), js_fn_name.len());
        }
    }
    #[doc = web_api!("Event", "removeEventListenerJs")]
    /// Removes a JavaScript function `event` listener from an `element`.
    pub fn event_remove_listener_js(element: &str, event: JsEvent, js_fn_name: &str) {
        unsafe {
            event_remove_listener_js(
                element.as_ptr(), element.len(),
                event.as_str().as_ptr(), event.as_str().len(),
                js_fn_name.as_ptr(), js_fn_name.len()
            );
        }
    }

    /// Callback dispatcher for WebAssembly events.
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
    /// pub extern "C" fn my_callback() {
    ///     Js::console_log("Button clicked!");
    /// }
    /// Js::event_add_listener("#my_button", JsEvent::Click, my_callback);
    /// ```
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn wasm_callback(callback_ptr: usize) {
        let callback = callback_ptr as *const ();
        let callback: extern "C" fn() = unsafe { transmute(callback) };
        callback();
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
}

/// # Web API history & navigation
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/History>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Location>
#[rustfmt::skip]
impl Js {
    #[doc = web_api!("History", "back")]
    /// Moves the browser back one step in the session history.
    pub fn history_back() { unsafe { history_back(); } }

    #[doc = web_api!("History", "forward")]
    /// Moves the browser forward one step in the session history.
    pub fn history_forward() { unsafe { history_forward(); } }

    #[doc = web_api!("History", "go")]
    /// Moves the browser to a specific point in the session history.
    /// Use negative values to go back, positive to go forward.
    pub fn history_go(delta: i32) { unsafe { history_go(delta); } }

    #[doc = web_api!("History", "pushState")]
    /// Adds an entry to the session history stack.
    pub fn history_push_state(state: &str, title: &str, url: &str) {
        unsafe { history_push_state(state.as_ptr(), state.len(), title.as_ptr(), title.len(),
            url.as_ptr(), url.len()); }
    }

    #[doc = web_api!("History", "replaceState")]
    /// Modifies the current history entry without creating a new one.
    pub fn history_replace_state(state: &str, title: &str, url: &str) {
        unsafe { history_replace_state(state.as_ptr(), state.len(), title.as_ptr(), title.len(),
            url.as_ptr(), url.len()); }
    }

    #[doc = web_api!("Location", "reload")]
    /// Reloads the current document.
    pub fn location_reload() { unsafe { location_reload(); } }

    #[doc = web_api!("Location", "assign")]
    /// Loads the specified URL.
    pub fn location_assign(url: &str) { unsafe { location_assign(url.as_ptr(), url.len()); } }

    #[doc = web_api!("Location", "replace")]
    /// Replaces the current document with the specified URL without creating a new entry in the history.
    pub fn location_replace(url: &str) { unsafe { location_replace(url.as_ptr(), url.len()); } }
}
js_reexport! {
    [ module: "api_history_navigation" ]
    unsafe fn "history_back" history_back();
    unsafe fn "history_forward" history_forward();
    unsafe fn "history_go" history_go(delta: i32);
    unsafe fn "history_pushState" history_push_state(state_ptr: *const u8, state_len: usize,
        title_ptr: *const u8, title_len: usize, url_ptr: *const u8, url_len: usize);
    unsafe fn "history_replaceState" history_replace_state(state_ptr: *const u8, state_len: usize,
        title_ptr: *const u8, title_len: usize, url_ptr: *const u8, url_len: usize);
    //
    unsafe fn "location_reload" location_reload();
    unsafe fn "location_assign" location_assign(url_ptr: *const u8, url_len: usize);
    unsafe fn "location_replace" location_replace(url_ptr: *const u8, url_len: usize);
}

/// # Web API permissions
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API>
#[rustfmt::skip]
impl Js {
    #[doc = web_api!("Permissions", "query")]
    /// Queries the status of a given permission.
    ///
    /// Returns `Granted`, `Denied`, `Prompt`, or `Unknown` if unsupported.
    pub fn permissions_query(permission: JsPermission) -> JsPermissionState {
        unsafe { permissions_query(permission.as_str().as_ptr(), permission.as_str().len()) }
        .into()
    }

}
js_reexport! {
    [ module: "api_permissions" ]
    unsafe fn "permissions_query" permissions_query(name_ptr: *const u8, name_len: usize) -> i32;
}

/* extended APIs */

/// # Web API canvas
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D>
/// - <https://html.spec.whatwg.org/multipage/canvas.html>
#[rustfmt::skip]
impl Js {
    /* custom */
    /// Sets the active canvas by ID.
    pub fn set_canvas(id: &str) { unsafe { set_canvas(id.as_ptr(), id.len()); } }

    /* colors and styles */

    #[doc = web_api!(canvas "fillStyle")]
    /// Sets the color or style for filling shapes.
    pub fn fill_style(r: u8, g: u8, b: u8) { fill_style(r, g, b); }

    #[doc = web_api!(canvas "strokeStyle")]
    /// Sets the color or style for lines.
    pub fn stroke_style(r: u8, g: u8, b: u8) { stroke_style(r, g, b); }

    /* draw */

    #[doc = web_api!(canvas "fillRect")]
    /// Draws a filled rectangle.
    pub fn fill_rect(x: f64, y: f64, w: f64, h: f64) { fill_rect(x, y, w, h); }

    ///
    pub fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64) { draw_line(x1, y1, x2, y2); }

    ///
    pub fn draw_circle(x: f64, y: f64, radius: f64) { draw_circle(x, y, radius); }

    /* text */

    #[doc = web_api!(canvas "fillText")]
    /// Draws a filled text string at the specified coordinates
    pub fn fill_text(text: &str, x: f64, y: f64) {
        unsafe { fill_text(text.as_ptr(), text.len(), x, y); }
    }
}
js_reexport! {
    [ module: "api_canvas" ]
    /* custom */
    unsafe fn set_canvas(str_ptr: *const u8, str_len: usize);

    /* draw */
    safe fn "fillRect" fill_rect(x: f64, y: f64, w: f64, h: f64);
    safe fn "fillStyle" fill_style(r: u8, g: u8, b: u8);
    safe fn "strokeStyle" stroke_style(r: u8, g: u8, b: u8);
    safe fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64);
    safe fn draw_circle(x: f64, y: f64, radius: f64);

    /* text */
    unsafe fn "fillText" fill_text(str_ptr: *const u8, str_len: usize, x: f64, y: f64);
}

/// # Web API time
#[rustfmt::skip]
impl Js {
    ///
    pub fn get_time() -> f64 { get_time() }
}
js_reexport! {
    [ module: "api_timing" ]
    safe fn get_time() -> f64;
}
