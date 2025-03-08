// devela::ui::back::miniquad::window

use crate::{Box, /*MiniquadRenderingBackend, */ String, Vec};
#[cfg(feature = "std")]
use ::miniquad::window::dropped_file_path;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use ::miniquad::window::get_window_position;
use ::miniquad::window::{
    blocking_event_loop, cancel_quit, clipboard_get, clipboard_set, dpi_scale, dropped_file_bytes,
    dropped_file_count, high_dpi, new_rendering_backend, order_quit, request_quit, schedule_update,
    screen_size, set_cursor_grab, set_fullscreen, set_mouse_cursor, set_window_position,
    set_window_size, show_keyboard, show_mouse,
};
use ::miniquad::{CursorIcon, RenderingBackend};

/// A wrapper namespace over [`miniquad::window`] functions.
pub struct MiniquadWindow;

#[rustfmt::skip]
impl MiniquadWindow {
    /// Returns a new rendering backend.
    ///
    /// It's normally `GlContext`, or maybe `MetalContext` in macos.
    pub fn new_rendering_backend() -> Box<dyn RenderingBackend> { new_rendering_backend() }

    /* event loop */

    /// Returns `true` if the event loop blocks until [`schedule_update`] is called.
    ///
    /// See also: `MiniquadService::`[`blocking_event_loop()`], and
    /// `::miniquad::`[`blocking_event_loop`][blocking_event_loop].
    ///
    /// [`schedule_update`]: Self::schedule_update
    /// [`set_blocking_event_loop()`]: crate::MiniquadService::set_blocking_event_loop
    pub fn blocking_event_loop() -> bool { blocking_event_loop() }

    /// Requests an immediate update, ensuring `update()` and `draw()` are called without waiting.
    ///
    /// This must be called from an implementor of [`MiniquadEventHandler`]
    /// and requires [`conf.platform`]`.`[`blocking_event_loop`] to be enabled.
    ///
    /// This can significantly reduce CPU usage while waiting for events.
    ///
    /// It is recommended to call this at the end of `resize_event`
    /// or after relevant mouse/keyboard input.
    ///
    /// [`MiniquadEventHandler`]: crate::MiniquadEventHandler
    /// [`conf.platform`]: crate::MiniquadConf#structfield.platform
    /// [`blocking_event_loop`]: crate::MiniquadPlatform#structfield.blocking_event_loop
    pub fn schedule_update() { schedule_update(); }

    /* clipboard */

    /// Get current OS clipboard value.
    pub fn clipboard_get() -> Option<String> { clipboard_get() }

    /// Save value to OS clipboard
    pub fn clipboard_set(data: &str) { clipboard_set(data); }

    /* dropping files */

    /// Returns the contents of a dropped file as a byte vector, if available.
    ///
    /// The `index` parameter specifies which dropped file to retrieve, starting from 0.
    pub fn dropped_file_bytes(index: usize) -> Option<Vec<u8>> { dropped_file_bytes(index) }

    /// Returns the number of files that have been dropped.
    pub fn dropped_file_count() -> usize { dropped_file_count() }

    /// Returns the file path of a dropped file, if available.
    ///
    /// The `index` parameter specifies which dropped file to retrieve, starting from 0.
    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    pub fn dropped_file_path(index: usize) -> Option<crate::PathBuf> { dropped_file_path(index) }

    /* dpi */

    /// The dpi scaling factor (window pixels to framebuffer pixels).
    ///
    /// See: [High DPI Rendering][::miniquad::conf#high-dpi-rendering].
    pub fn dpi_scale() -> f32 { dpi_scale() }

    /// Returns `true` if `high_dpi` was requested and it's running in a high-dpi scenario.
    pub fn high_dpi() -> bool { high_dpi() }

    /* quit */

    /// This function simply quits the application without giving the user a chance to intervene.
    ///
    /// Usually this might be called when the user clicks the ‘Ok’ button in a
    /// ‘Really Quit?’ dialog box Window might not be actually closed right away
    /// (exit(0) might not happen in the order_quit implmentation) and execution
    /// might continue for some time after But the window is going to be
    /// inevitably closed at some point.
    pub fn quit() { order_quit(); }

    /// Triggers the “quit_requested_event” event.
    ///
    /// If the event handler callback does nothing, the application will be quit as usual.
    /// To prevent this, call [`cancel_quit`][Self::cancel_quit] from inside the event handler.
    pub fn request_quit() { request_quit(); }

    /// Cancels a pending quit request, no matter who it was requested.
    ///
    /// The only place where calling this function makes sense is from inside the
    /// event handler callback when the “quit_requested_event” event has been received.
    pub fn cancel_quit() { cancel_quit(); }

    /* window shape */

    /// The current framebuffer size in pixels.
    ///
    /// See [High DPI Rendering][::miniquad::conf#high-dpi-rendering] and
    /// `::miniquad::`[`screen_size`][screen_size].
    pub fn get_size() -> (f32, f32) { screen_size() }

    /// Set the application’s window size.
    ///
    /// See `::miniquad::`[`set_window_size`][set_window_size].
    pub fn set_size(width: u32, height: u32) { set_window_size(width, height); }

    /// Get the position of the window.
    ///
    /// See `::miniquad::`[`get_window_position`][set_window_position].
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    #[cfg_attr(nightly_doc, doc(cfg(any(target_os = "windows", target_os = "linux"))))]
    pub fn get_position() -> (u32, u32) { get_window_position() }

    /// Set the window position.
    ///
    /// See `::miniquad::`[`set_window_position`][set_window_position].
    pub fn set_position(x: u32, y: u32) { set_window_position(x, y); }

    /// Sets the full screen mode.
    pub fn set_fullscreen(fullscreen: bool) { set_fullscreen(fullscreen); }

    /* keyboard & mouse */

    /// Show/hide onscreen keyboard. Only works on Android right now.
    pub fn show_keyboard(shown: bool) { show_keyboard(shown); }

    /// Show or hide the mouse cursor
    pub fn show_mouse(shown: bool) { show_mouse(shown); }

    /// Set the mouse cursor icon.
    pub fn set_mouse_cursor(icon: CursorIcon) { set_mouse_cursor(icon); }

    /// Capture mouse cursor to the current window On WASM this will automatically
    /// hide cursor.
    ///
    /// On desktop this will bound cursor to windows border.
    ///
    /// NOTICE: on desktop cursor will not be automatically released after window
    /// lost focus so `set_cursor_grab(false)` on window's focus lost is recommended.
    //
    // WAIT: implement window focus events
    pub fn set_cursor_grab(grab: bool) { set_cursor_grab(grab); }
}
