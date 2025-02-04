// devela::ui::service::miniquad::service
//
//! Defines [`MiniquadService`] and [`MiniquadEventHandlerExt`].
//

#[cfg(feature = "alloc")]
use crate::{Box, ToString};
use crate::{
    MiniquadConf, MiniquadEventHandler, UiCap, UiCapImage, UiCapInput, UiCapWindow, UiService,
};
#[cfg(doc)]
use ::miniquad::FilterMode;

/// A wrapper over `MiniquadEventHandler` to allow lazy initialization.
pub trait MiniquadEventHandlerExt: MiniquadEventHandler {
    /// Returns the event handler initialized.
    fn init(self) -> Self;
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

/// `miniquad`'s UI Service.
///
/// Works like a builder.
#[derive(Default)]
pub struct MiniquadService<T: MiniquadEventHandlerExt + 'static> {
    handler: Option<T>,
    conf: MiniquadConf,

    /* pixel buffer settings */
    width: u32,
    height: u32,
    linear_interpolation: bool,
    maintain_aspect_ratio: bool,
}

/// # builder methods.
impl<T: MiniquadEventHandlerExt + 'static> MiniquadService<T> {
    /// Returns a new `MiniquadService` with the given `handler`, using default [`MiniquadConf`].
    pub fn with(handler: T) -> Self {
        Self {
            handler: Some(handler),
            conf: MiniquadConf::default(),
            //
            width: 0,
            height: 0,
            linear_interpolation: false,
            maintain_aspect_ratio: false,
        }
    }
    /// Initialize the miniquad rendering context, opening the window.
    ///
    /// This must be the last method to call.
    pub fn init(self) {
        let handler = self.handler.expect("Event handler is required");
        ::miniquad::start(self.conf, move || Box::new(handler.init()));
    }

    /* builder methods */

    /// Sets the given miniquad configuration.
    pub fn conf(mut self, conf: MiniquadConf) -> Self {
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
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
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
    /// If `linear == true` it uses [`FilterMode:Linear`], otherwise uses [`FilterMode::Nearest`].
    pub fn linear_interpolation(mut self, linear: bool) -> Self {
        self.linear_interpolation = linear;
        self
    }
    /// Maintains the aspect ratio by drawing horizontal bands.
    pub fn maintain_aspect_ratio(mut self, maintain: bool) -> Self {
        self.maintain_aspect_ratio = maintain;
        self
    }
}
