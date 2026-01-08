// devela::ui::back::miniquad::service
//
//! Defines [`MiniquadEventHandlerExt`] and [`MiniquadService`].
//
// TOC
// - MiniquadEventHandlerExt
// - MiniquadService

use crate::{Box, ToString};
use crate::{UiCap, UiCapImage, UiCapInput, UiCapWindow, UiService, is};
#[cfg(doc)]
use ::miniquad::FilterMode;
use ::miniquad::{EventHandler, conf::Conf};

#[doc = crate::_tags!(event ui)]
/// An extension trait for miniquad's `MiniquadEventHandler`.
#[doc = crate::_doc_location!("ui/back/miniquad")]
///
/// It allows lazy initialization and other methods.
//
// https://docs.rs/miniquad/latest/miniquad/trait.EventHandler.html
pub trait MiniquadEventHandlerExt: EventHandler {
    /// Returns the event handler initialized.
    fn init(self) -> Self;

    /// Whether it interpolates the scaled pixels.
    fn interpolation(&self) -> bool;
    /// Set whether to `interpolate` the scaled pixels.
    fn set_interpolation(&mut self, interpolate: bool);

    /// Whether the aspect ratio is maintained on window resize.
    fn maintain_aspect_ratio(&self) -> bool;
    /// Set whether to `maintain` the aspect ratio on window resize.
    fn set_maintain_aspect_ratio(&mut self, maintain: bool);
}

impl<T: MiniquadEventHandlerExt + 'static> UiService for MiniquadService<T> {
    fn capabilities(&self) -> UiCap {
        let image = Some(UiCapImage { rgb: true, ..Default::default() });
        let input = Some(UiCapInput { keyboard: true, mouse: true, ..Default::default() });
        let window = Some(UiCapWindow { multi: false });
        UiCap { image, input, window, ..Default::default() }
    }
    fn version(&self) -> (u32, u32, u32) {
        (0, 4, 7)
    }
}

#[doc = crate::_tags!(event runtime)]
/// `miniquad`'s UI Service.
#[doc = crate::_doc_location!("ui/back/miniquad")]
#[derive(Debug)]
pub struct MiniquadService<T: MiniquadEventHandlerExt + 'static> {
    handler: Option<T>,
    conf: Conf,

    /* pixel buffer settings */
    width: u32,
    height: u32,
}

impl<T: MiniquadEventHandlerExt + 'static> Default for MiniquadService<T> {
    /// Returns a default `MiniquadService` without a handler.
    fn default() -> Self {
        Self {
            handler: None,
            conf: Conf::default(),
            //
            width: 0,
            height: 0,
        }
    }
}

/// # builder methods
impl<T: MiniquadEventHandlerExt + 'static> MiniquadService<T> {
    /// Returns a default `MiniquadService` with the given `handler`,
    pub fn with(handler: T) -> Self {
        Self { handler: Some(handler), ..Default::default() }
    }

    /// Initialize the miniquad rendering context, opening the window.
    ///
    /// This must be the last method to call.
    ///
    /// # Panics
    /// Panics if the handler is not set (e.g. by constructing with `default()`)
    pub fn init(self) {
        let handler = self.handler.expect("Event handler is required");
        ::miniquad::start(self.conf, move || Box::new(handler.init()));
    }

    /* builder methods */

    /// Sets the given handler.
    pub fn handler(mut self, handler: T) -> Self {
        self.handler = Some(handler);
        self
    }

    /// Sets the given miniquad configuration.
    pub fn conf(mut self, conf: Conf) -> Self {
        self.conf = conf;
        self
    }

    /// Whether the event loop should block until [`schedule_update`] is called.
    ///
    /// [`schedule_update`]: crate::MiniquadWindow::schedule_update
    pub fn blocking_event_loop(mut self, blocking: bool) -> Self {
        self.conf.platform.blocking_event_loop = blocking;
        self
    }

    /* window */

    /// Sets the window title.
    pub fn title(mut self, title: impl ToString) -> Self {
        self.conf.window_title = title.to_string();
        self
    }

    /// Sets the window size.
    ///
    /// NOTE: in X11 it needs to have `window_resizable` set to `true` for it to have any effect.
    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        assert![width <= i32::MAX as u32];
        assert![height <= i32::MAX as u32];
        self.conf.window_width = width as i32;
        self.conf.window_height = height as i32;
        self
    }

    /// Whether the window should be `resizable` by the user.
    pub fn window_resizable(mut self, resizable: bool) -> Self {
        self.conf.window_resizable = resizable;
        self
    }

    /// Whether the window should be `fullscreen`.
    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.conf.fullscreen = fullscreen;
        self
    }

    /* buffer */

    /// Sets the size of the pixel buffer.
    pub fn buffer_size(mut self, width: u32, height: u32) -> Self {
        assert![width <= i32::MAX as u32];
        assert![height <= i32::MAX as u32];
        self.width = width;
        self.height = height;
        self
    }
    /// If `linear == true` uses [`FilterMode::Linear`], otherwise [`FilterMode::Nearest`].
    ///
    /// If the `handler` has not been set, this does nothing.
    pub fn interpolation(mut self, linear: bool) -> Self {
        is![let Some(h) = self.handler.as_mut(); h.set_interpolation(linear)];
        self
    }

    /// Set whether to `maintain` the aspect ratio on window resize.
    ///
    /// If the `handler` has not been set, this does nothing.
    pub fn maintain_aspect_ratio(mut self, maintain: bool) -> Self {
        is![let Some(h) = self.handler.as_mut(); h.set_maintain_aspect_ratio(maintain)];
        self
    }
}

/// # getters and setters
impl<T: MiniquadEventHandlerExt + 'static> MiniquadService<T> {
    /// Whether the aspect ratio is maintained on window resize.
    pub fn get_interpolation(self) -> bool {
        self.handler.as_ref().map(|h| h.interpolation()).unwrap_or_default()
    }
    /// Set whether to `maintain` the aspect ratio on window resize.
    pub fn set_interpolation(&mut self, interpolate: bool) {
        is![let Some(h) = self.handler.as_mut(); h.set_interpolation(interpolate)];
    }

    /// Whether the aspect ratio is maintained on window resize.
    pub fn get_maintain_aspect_ratio(self) -> bool {
        self.handler.as_ref().map(|h| h.maintain_aspect_ratio()).unwrap_or_default()
    }
    /// Set whether to `maintain` the aspect ratio on window resize.
    pub fn set_maintain_aspect_ratio(&mut self, maintain: bool) {
        is![let Some(h) = self.handler.as_mut(); h.set_maintain_aspect_ratio(maintain)];
    }
}
