// devela::lang::ffi::js::web::window
//
//! Defines [`WebWindow`], [`WebWindowState`].
//!
//

#[cfg(feature = "_float_f32")]
use devela::Float;
#[cfg(feature = "alloc")]
use devela::String;
#[allow(unused_imports, reason = "not(windows)")]
use devela::{
    _js_extern, _js_method_str_alloc, Js, JsTimeout, WebDocument, js_bool, js_int32, js_number,
    js_uint32,
};
use devela::{Distance, Extent, js_doc, offset_of};

/// Handle to the browser's global [Window] and [Screen] associated APIs.
///
/// [Window]: https://developer.mozilla.org/en-US/docs/Web/API/Window
/// [Screen]: https://developer.mozilla.org/en-US/docs/Web/API/Window/screen
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WebWindow;

#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl WebWindow {
    #[doc = js_doc!("Window", "document")]
    /// Returns the `document` object.
    pub fn document(&self) -> WebDocument { WebDocument }

    /// Returns a new up-to-date `WebWindowState`.
    pub fn state() -> WebWindowState { WebWindowState::new() }

    #[doc = js_doc!("Window", "closed")]
    /// Whether the current window is closed or not.
    pub fn is_closed() -> js_bool { window_is_closed() }

    #[doc = js_doc!("Window", "crossOriginIsolated")]
    /// Whether the website is in a cross-origin isolation state.
    pub fn is_coi() -> js_bool { window_is_coi() }

    #[doc = js_doc!("Window", "isSecureContext")]
    /// Whether the current [context is secure][0].
    ///
    /// [0]: https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts
    pub fn is_secure() -> js_bool { window_is_secure() }

    #[doc = js_doc!("Window", "locationbar")]
    /// Whether the window is a popup or not.
    pub fn is_popup() -> js_bool { window_is_popup() }

    /* texts */

    _js_method_str_alloc! {
        #[doc = js_doc!("Window", "name")]
        /// Gets the window name.
        name, window_name
    }

    #[doc = js_doc!("Window", "name")]
    /// Sets the current window `name`.
    pub fn set_name(name: &str) { unsafe { window_set_name(name.as_ptr(), name.len() as u32); } }

    /* timeout */

    #[doc = js_doc!("Window", "setTimeout")]
    /// Calls a function after a delay in milliseconds.
    pub fn set_timeout(callback: extern "C" fn(), delay_ms: js_uint32) -> JsTimeout {
        JsTimeout { id: unsafe { window_set_timeout(callback as usize, delay_ms) } } }

    #[doc = js_doc!("Window", "setInterval")]
    /// Calls a function repeatedly at a fixed interval in milliseconds.
    pub fn set_interval(callback: extern "C" fn(), interval_ms: js_uint32) -> JsTimeout {
        JsTimeout { id: unsafe { window_set_interval(callback as usize, interval_ms) } } }

    #[doc = js_doc!("Window", "clearTimeout")]
    #[doc = js_doc!("Window", "clearInterval")]
    /// Cancels a timeout or interval.
    pub fn clear_timeout(id: JsTimeout) { window_clear_timeout(id.id); }

    /* eval */

    /// Executes JavaScript code immediately.
    /// ## Security Warning
    /// - Avoid passing untrusted input, as this executes arbitrary JS.
    /// - Ensure all evaluated code is **safe and controlled**.
    pub fn eval(js_code: &str) { unsafe { window_eval(js_code.as_ptr(), js_code.len()); } }

    #[doc = js_doc!("Window", "setTimeout")]
    /// Executes JavaScript code after a delay in milliseconds.
    pub fn eval_timeout(js_code: &str, delay_ms: js_uint32) -> JsTimeout { JsTimeout {
        id: unsafe { window_eval_timeout(js_code.as_ptr(), js_code.len(), delay_ms) } } }

    #[doc = js_doc!("Window", "setInterval")]
    /// Executes JavaScript code repeatedly at a fixed interval in milliseconds.
    pub fn eval_interval(js_code: &str, interval_ms: js_uint32) -> JsTimeout { JsTimeout {
        id: unsafe { window_eval_interval(js_code.as_ptr(), js_code.len(), interval_ms) } } }

    /* animation */

    #[doc = js_doc!("Window", "requestAnimationFrame")]
    /// Requests an animation frame, executing the given `callback`.
    pub fn request_animation_frame(callback: extern "C" fn()) -> js_uint32 {
        unsafe { window_request_animation_frame(callback as usize) } }
    /// Cancels a request for an animation frame.
    pub fn cancel_animation_frame(id: js_uint32) { window_cancel_animation_frame(id); }
}
_js_extern! {
    [module: "api_window"]
    unsafe fn window_state(data: *mut u8);
    safe fn window_is_closed() -> js_bool;
    safe fn window_is_coi() -> js_bool;
    safe fn window_is_secure() -> js_bool;
    safe fn window_is_popup() -> js_bool;
    // texts
    unsafe fn window_name(buf_ptr: *mut u8, max_len: js_uint32) -> js_int32;
    unsafe fn window_set_name(str_ptr: *const u8, str_len: js_uint32);
    // timeout
    unsafe fn window_set_timeout(callback_ptr: usize, delay_ms: js_uint32) -> js_uint32;
    unsafe fn window_set_interval(callback_ptr: usize, interval_ms: js_uint32) -> js_uint32;
    safe fn window_clear_timeout(timeout_id: js_uint32);
    // eval
    unsafe fn window_eval(js_code_ptr: *const u8, js_code_len: usize);
    unsafe fn window_eval_timeout(js_code_ptr: *const u8, js_code_len: usize, delay_ms: js_uint32)
        -> js_uint32;
    unsafe fn window_eval_interval(js_code_ptr: *const u8, js_code_len: usize,
        interval_ms: js_uint32) -> js_uint32;
    // animation
    unsafe fn window_request_animation_frame(callback_ptr: usize) -> js_uint32;
    safe fn window_cancel_animation_frame(requestId: js_uint32);
}

/// Aggregates the live state of a [`WebWindow`], including its geometry and screen context.
///
/// It has a size of 52 Bytes.
///
/// ### Performance
/// All fields are fetched in a single JS→Rust call.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)] // manual: Debug
pub struct WebWindowState {
    /* window */
    #[doc = js_doc!("Window", "innerWidth")]
    #[doc = js_doc!("Window", "innerHeight")]
    /// The extent in pixels of the content of the browser window including any rendered scrollbars.
    pub inner_size: Extent<u32, 2>,

    #[doc = js_doc!("Window", "outerWidth")]
    #[doc = js_doc!("Window", "outerHeight")]
    /// The extent in pixels of the outside of the browser window.
    pub outer_size: Extent<u32, 2>,

    /* screen */
    #[doc = js_doc!("Window", "screenLeft")]
    #[doc = js_doc!("Window", "screenTop")]
    /// The window's offset in pixels from the screen's top-left origin.
    pub screen_offset: Distance<i32, 2>,

    #[doc = js_doc!("Screen", "width")]
    #[doc = js_doc!("Screen", "height")]
    /// The extent of the screen in pixels.
    pub screen_size: Extent<u32, 2>,

    #[doc = js_doc!("Screen", "availWidth")]
    #[doc = js_doc!("Screen", "availHeight")]
    /// The extent of the screen in pixels, minus user interface features displayed.
    pub screen_usable_size: Extent<u32, 2>,

    /* misc. */
    #[doc = js_doc!("Window", "devicePixelRatio")]
    /// The device pixel ratio of the resolution in physical pixels to the resolution in CSS pixels.
    ///
    /// The value changes with the zoom on desktops yet remains static on mobile devices.
    pub dpr: f32,

    #[doc = js_doc!("Screen", "colorDepth")]
    /// The screen color depth, in bits per single pixel. It could be 8, 16, 24, 32 or 64.
    pub bpp: u8,

    // TODO: add bitpacked flags (is_popup, is_secure, etc.)
    //
    /// Explicit padding to align.
    _pad: [u8; 3],
}
impl WebWindowState {
    const __ASSERT_FIELD_OFFSETS: () = const {
        assert!(offset_of!(Self, inner_size) == 0);
        assert!(offset_of!(Self, outer_size) == 8);
        assert!(offset_of!(Self, screen_offset) == 16);
        assert!(offset_of!(Self, screen_size) == 24);
        assert!(offset_of!(Self, screen_usable_size) == 32);
        assert!(offset_of!(Self, dpr) == 40);
        assert!(offset_of!(Self, bpp) == 44);
    };

    /// Returns a new up-to-date `WebWindowState`.
    ///
    /// # Safety
    /// - JavaScript must write all non-padding fields at correct offsets.
    #[cfg(feature = "unsafe_ffi")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
    pub fn new() -> WebWindowState {
        let mut state = WebWindowState::default();
        unsafe {
            window_state(&mut state as *mut WebWindowState as *mut u8);
        }
        state
    }

    /// Overwrites this `WebWindowState` with the latest live metrics.
    #[cfg(feature = "unsafe_ffi")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
    pub fn update(&mut self) {
        unsafe { window_state(self as *mut Self as *mut u8) };
    }

    /// Validates the internal consistency of window metrics.
    ///
    /// Returns `true` if all these conditions hold:
    /// - No dimensions are zero (invalid window state)
    /// - Inner size ≤ outer size (logical constraint)
    /// - Outer size ≤ screen size (unless multi-monitor)
    /// - Device pixel ratio is sane (0.2 <= dpr <= 10.0)
    /// - Screen color depth is plausible (8 <= depth <= 64)
    // - Popup flags don't contradict window dimensions
    pub const fn is_valid(&self) -> bool {
        // 1. Non-zero dimensions
        let non_zero = self.inner_size.x() > 0
            && self.inner_size.y() > 0
            && self.outer_size.x() > 0
            && self.outer_size.y() > 0;

        // 2. Inner <= Outer
        let inner_le_outer = self.inner_size.dim[0] <= self.outer_size.dim[0]
            && self.inner_size.dim[1] <= self.outer_size.dim[1];

        // 3. Outer <= Screen (with 10px tolerance for window chrome)
        let outer_le_screen = (self.outer_size.dim[0] <= self.screen_size.dim[0] + 10)
            && (self.outer_size.dim[1] <= self.screen_size.dim[1] + 10);

        // 4. Sane DPR range
        let sane_dpr = self.dpr >= 0.2 && self.dpr <= 10.0;

        // 5. Plausible color depth
        let sane_bpp = self.bpp >= 8 && self.bpp <= 64;

        // // 6. Popup consistency
        // let valid_popup = !self.is_popup() || (
        //     // Popups shouldn't fill the screen
        //     self.outer_size.dim[0] < self.screen_usable_size.dim[0] - 10 &&
        //     self.outer_size.dim[1] < self.screen_usable_size.dim[1] - 10
        // );

        non_zero && inner_le_outer && outer_le_screen && sane_dpr && sane_bpp // && valid_popup
    }

    /* derived metrics */

    /// Returns the thickness of the window chrome (frame, scrollbars, etc.) in logical pixels.
    ///
    /// This is the difference between the outer and inner window sizes.
    pub const fn chrome_size(&self) -> Extent<u32, 2> {
        Extent::new([
            self.outer_size.x() - self.inner_size.x(),
            self.outer_size.y() - self.inner_size.y(),
        ])
    }
    /// Checks if the window is approximately maximized (fills the available screen space).
    ///
    /// Tolerance: The window must be within 1 pixel of the screen's usable size.
    pub const fn is_maximized(&self) -> bool {
        self.outer_size.x() >= self.screen_usable_size.x()
            && self.outer_size.y() >= self.screen_usable_size.y()
    }
    /// Checks if the window is in a portrait orientation (height > width).
    pub const fn is_portrait(&self) -> bool {
        self.inner_size.y() > self.inner_size.x()
    }

    /// Returns the physical size of the window in hardware pixels, truncating fractional values.
    ///
    /// Computed as `(inner_size * device_pixel_ratio)`.
    ///
    /// For rounded values, use [`physical_size_rounded()`][Self::physical_size_rounded].
    #[allow(rustdoc::broken_intra_doc_links, reason = "_float_f32")]
    pub const fn physical_size(&self) -> Extent<u32, 2> {
        Extent::new([
            (self.inner_size.x() as f32 * self.dpr) as u32,
            (self.inner_size.y() as f32 * self.dpr) as u32,
        ])
    }
    /// Returns the physical size of the window in hardware pixels, rounded to the nearest integer.
    ///
    /// Computed as `(inner_size * device_pixel_ratio).round()`.
    ///
    /// It's more accurate and expensive than [`physical_size()`][Self::physical_size].
    #[cfg(feature = "_float_f32")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
    pub const fn physical_size_rounded(&self) -> Extent<u32, 2> {
        Extent::new([
            Float(self.inner_size.x() as f32 * self.dpr).const_round().0 as u32,
            Float(self.inner_size.y() as f32 * self.dpr).const_round().0 as u32,
        ])
    }

    /// Returns the window's distance to each screen edge in logical pixels.
    ///
    /// Order: `[left, top, right, bottom]`. Negative values mean the window is outside the screen.
    pub const fn screen_margins(&self) -> [i32; 4] {
        [
            self.screen_offset.dim[0],
            self.screen_offset.dim[1],
            (self.screen_size.x() as i32)
                - (self.screen_offset.dim[0] + self.outer_size.x() as i32),
            (self.screen_size.y() as i32)
                - (self.screen_offset.dim[1] + self.outer_size.y() as i32),
        ]
    }
}

impl crate::Debug for WebWindowState {
    fn fmt(&self, f: &mut crate::Formatter<'_>) -> crate::FmtResult<()> {
        let mut state = f.debug_struct("WebWindowState");
        /* fields */
        state
            .field("inner_size", &self.inner_size)
            .field("outer_size", &self.outer_size)
            .field("screen_offset", &self.screen_offset)
            .field("screen_size", &self.screen_size)
            .field("screen_usable_size", &self.screen_usable_size)
            .field("dpr", &self.dpr)
            .field("bpp", &self.bpp)
            /* derived */
            .field("chrome_size()", &self.chrome_size())
            .field("is_maximized()", &self.is_maximized())
            .field("is_portrait()", &self.is_portrait())
            .field("is_valid()", &self.is_valid())
            .field("physical_size()", &self.physical_size());
        #[cfg(feature = "_float_f32")]
        state.field("physical_size_rounded()", &self.physical_size_rounded());
        state.field("screen_margins()", &self.screen_margins()).finish_non_exhaustive() // .finish()
    }
}
