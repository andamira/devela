// devela::sys::device::display::x11::runtime::frontend
//
//! Defines [`XBackend`], (`XFrameCtx`), [`XFrontend`].
//

use crate::{_run_driver_step_run_frame_body, RunPhase, RunPresent, RunStep};
use crate::{Event, RunApp, RunBackend, RunControl, RunDriver, RunDriverFrameError, RunRender};
use crate::{XDisplay, XError, XImageMode, XPresent, XPresenter, XWindow};
use crate::{is, whilst};

#[doc = crate::_tags!(unix runtime)]
/// Borrowed X11 backend context for one runtime frame.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// Provides temporary access to the display connection and target window
/// during rendering and presentation.
pub(crate) struct XFrameCtx<'a> {
    pub(crate) display: &'a mut XDisplay,
    pub(crate) window: &'a mut XWindow,
}

#[doc = crate::_tags!(unix runtime)]
/// A single-window X11 runtime backend.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// `XBackend` owns the X11 display connection and window used for event
/// collection and per-frame backend access.
#[derive(Debug)]
pub(crate) struct XBackend {
    display: XDisplay,
    window: XWindow,
}
impl XBackend {
    /// Opens a single-window X11 backend.
    pub fn open(x: i16, y: i16, width: u16, height: u16) -> Result<Self, XError> {
        let mut display = XDisplay::open()?;
        let window = XWindow::new(&mut display, x, y, width, height, 1)?;
        Ok(Self { display, window })
    }
}
impl RunBackend for XBackend {
    type Event = Event;
    type Error = XError;
    type Context<'a>
        = XFrameCtx<'a>
    where
        Self: 'a;
    fn collect_events(&mut self, out: &mut [Self::Event]) -> Result<usize, Self::Error> {
        whilst! { written in 0..out.len(); {
            let ev = self.display.poll_event();
            is! { ev.is_none(), break }
            out[written] = ev;
        }}
        Ok(written)
    }
    fn context(&mut self) -> Self::Context<'_> {
        XFrameCtx {
            display: &mut self.display,
            window: &mut self.window,
        }
    }
}

#[doc = crate::_tags!(unix runtime)]
/// An X11 frontend for frame-driven runtimes.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// Owns the current X11 runtime backend and its presentation finalizer,
/// while leaving runtime control, app logic, and rendering policy external.
///
/// The current implementation manages one window.
/// That is an implementation detail, not the intended long-term boundary.
#[derive(Debug)]
pub struct XFrontend {
    backend: XBackend,
    presenter: XPresenter,
}
impl XFrontend {
    /// Opens an X11 frontend.
    pub fn open(x: i16, y: i16, width: u16, height: u16) -> Result<Self, XError> {
        Self::open_with(XImageMode::Auto, x, y, width, height)
    }

    /// Opens an X11 frontend with the given image `mode`.
    pub fn open_with(
        mode: XImageMode,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<Self, XError> {
        Ok(Self {
            backend: XBackend::open(x, y, width, height)?,
            presenter: XPresenter::new(mode),
        })
    }

    /// Drives one X11 runtime iteration including rendering and presentation.
    #[allow(private_bounds, reason = "private XFrameCtx")]
    pub fn step_frame<T, A, R, S, RE>(
        &mut self,
        driver: &mut RunDriver<T>,
        app: &mut A,
        renderer: &mut R,
        scene: &S,
        events: &mut [A::Event],
    ) -> Result<RunControl, RunDriverFrameError<XError, A::Error, RE, XError>>
    where
        A: RunApp<Event = Event>,
        for<'a> R: RunRender<S, Event, XFrameCtx<'a>, Error = RE, Output<'a> = XPresent<'a>>,
    {
        // NOTE: because of compiler limitations we can't do just this:
        // driver.step_frame(&mut self.backend, app, renderer, &mut self.presenter, scene, events)
        _run_driver_step_run_frame_body!(@check
            driver, self.backend, app, renderer, self.presenter, scene(scene), events, control,
            driver.runtime_mut().tick_once(), driver.runtime_mut().transition(RunPhase::Stopped));
        Ok(control)
    }

    /// Runs until the app stops or an error occurs,
    /// including rendering and presentation.
    #[allow(private_bounds, reason = "private XFrameCtx")]
    pub fn run_frame<T, A, R, S, RE>(
        &mut self,
        driver: &mut RunDriver<T>,
        app: &mut A,
        renderer: &mut R,
        scene: &S,
        events: &mut [A::Event],
    ) -> Result<(), RunDriverFrameError<XError, A::Error, RE, XError>>
    where
        A: RunApp<Event = Event>,
        for<'a> R: RunRender<S, Event, XFrameCtx<'a>, Error = RE, Output<'a> = XPresent<'a>>,
    {
        // NOTE: because of compiler limitations we can't do just this:
        // driver.run_frame(&mut self.backend, app, renderer, &mut self.presenter, scene, events)
        driver.start();
        while driver.can_advance() {
            _run_driver_step_run_frame_body!(
                driver,
                self.backend,
                app,
                renderer,
                self.presenter,
                scene(scene),
                events,
                control,
                driver.runtime_mut().tick_once(),
                driver.runtime_mut().transition(RunPhase::Stopped),
                break
            );
        }
        Ok(())
    }

    /// Returns a shared reference to the X11 Display server.
    pub const fn display(&self) -> &XDisplay {
        &self.backend.display
    }
    /// Returns an exclusive reference to the X11 Display server.
    pub const fn display_mut(&mut self) -> &mut XDisplay {
        &mut self.backend.display
    }

    /// Returns whether MIT-SHM is available on this display.
    pub const fn has_shm(&self) -> bool {
        self.backend.display.has_shm()
    }

    /// Returns the configured X11 image presentation mode.
    ///
    /// This is the mode requested when opening the frontend.
    /// It may still be [`XImageMode::Auto`] before any presentation occurs.
    pub const fn mode(&self) -> XImageMode {
        self.presenter.mode()
    }

    /// Returns the currently resolved X11 image presentation mode, if any.
    ///
    /// This resolves the active backing chosen after presentation begins.
    ///
    /// It returns `None` while no presentation surface has been created yet.
    pub fn active_mode(&self) -> Option<XImageMode> {
        self.presenter.active_mode()
    }
}
