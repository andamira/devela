// devela::sys::device::display::x11::runtime::frontend
//
//! Defines [`XBackend`], (`XFrameCtx`), [`XFrontend`].
//

use crate::{_run_driver_step_run_frame_body, RunPhase, RunPresent, RunStep};
use crate::{Event, RunApp, RunBackend, RunControl, RunDriver, RunDriverFrameError, RunRender};
use crate::{XDisplay, XError, XImageMode, XPresent, XPresenter, XSurfaceFrame, XWindow};
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
#[rustfmt::skip]
impl XFrontend {
    const fn _new(backend: XBackend, presenter: XPresenter) -> Self { Self { backend, presenter } }

    /// Opens an X11 frontend.
    pub fn open(x: i16, y: i16, width: u16, height: u16) -> Result<Self, XError> {
        Self::open_with(XImageMode::Auto, x, y, width, height)
    }
    /// Opens an X11 frontend with the given image `mode`.
    pub fn open_with(mode: XImageMode, x: i16, y: i16, width: u16, height: u16)
        -> Result<Self, XError> {
        Ok(Self::_new(XBackend::open(x, y, width, height)?, XPresenter::new(mode)))
    }

    /// Returns a shared reference to the X11 Display server.
    pub const fn display(&self) -> &XDisplay { &self.backend.display }
    /// Returns an exclusive reference to the X11 Display server.
    pub const fn display_mut(&mut self) -> &mut XDisplay { &mut self.backend.display }

    /// Returns whether MIT-SHM is available on this display.
    #[cfg(ffi_xcb_shm··)]
    pub const fn has_shm(&self) -> bool { self.backend.display.has_shm() }

    /// Returns the configured X11 image presentation mode.
    ///
    /// This is the mode requested when opening the frontend.
    /// It may still be [`XImageMode::Auto`] before any presentation occurs.
    pub const fn mode(&self) -> XImageMode { self.presenter.mode() }

    /// Returns the currently resolved X11 image presentation mode, if any.
    ///
    /// This resolves the active backing chosen after presentation begins.
    ///
    /// It returns `None` while no presentation surface has been created yet.
    pub fn active_mode(&self) -> Option<XImageMode> { self.presenter.active_mode() }

    /// Polls the next event without blocking.
    ///
    /// See [`XDisplay::poll_event`] for more information.
    pub fn poll_event(&mut self) -> Event { self.backend.display.poll_event() }

    /// Waits for the next event, blocking until one is available.
    ///
    /// See [`XDisplay::wait_event`] for more information.
    pub fn wait_event(&mut self) -> Event { self.backend.display.wait_event() }
}
impl XFrontend {
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
}
impl XFrontend {
    /// Renders directly into the retained X11 surface and presents it.
    ///
    /// This bypasses [`XRasterRenderer`][crate::XRasterRenderer]
    /// and avoids the scene-to-surface copy.
    pub fn with_surface_frame<F, T>(
        &mut self,
        width: u16,
        height: u16,
        depth: u8,
        clear_redraw: bool,
        render: F,
    ) -> Result<T, XError>
    where
        F: FnOnce(&mut XSurfaceFrame<'_>) -> Result<T, XError>,
    {
        let result = {
            let mut frame =
                self.presenter.surface_frame(&self.backend.display, width, height, depth)?;
            render(&mut frame)?
        };
        self.presenter.present_surface(
            &mut self.backend.display,
            &mut self.backend.window,
            clear_redraw,
        )?;
        Ok(result)
    }

    /// Advances one runtime frame and renders directly into the retained X11 surface.
    ///
    /// This is the X11-specific zero-copy rendering path. It bypasses
    /// [`XRasterRenderer`][crate::XRasterRenderer] and exposes the
    /// retained surface bytes for the duration of one frame.
    ///
    /// This avoids the intermediate artifact-to-surface copy,
    /// but it also gives up backend independence.
    //
    // BENCH NOTE: On local X11 SHM tests at 2560x1440-like scale, direct surface
    // rendering was ~5-20% faster than the artifact path depending on workload.
    // Presentation remained the dominant cost. Keeping both paths.
    #[allow(clippy::too_many_arguments)]
    #[allow(private_bounds, reason = "private XFrameCtx")]
    pub fn step_frame_surface<R, A, F, RE>(
        &mut self,
        driver: &mut RunDriver<R>,
        app: &mut A,
        width: u16,
        height: u16,
        depth: u8,
        clear_redraw: bool,
        events: &mut [Event],
        render: F,
    ) -> Result<RunControl, RunDriverFrameError<XError, A::Error, RE, XError>>
    where
        A: RunApp<Event = Event>,
        F: FnOnce(&mut XSurfaceFrame<'_>) -> Result<(), RE>,
    {
        // 1. Collect backend events.
        let written = self.backend.collect_events(events).map_err(RunDriverFrameError::Backend)?;
        let events = &events[..written];
        // 2. Build the runtime step.
        let step = RunStep::new(driver.runtime_mut().tick(), driver.phase(), events);
        // 3. Advance app logic.
        let control = app.run_step(step).map_err(RunDriverFrameError::App)?;
        // 4B. Stop before rendering/presenting.
        if matches!(control, RunControl::Stop) {
            driver.runtime_mut().transition(RunPhase::Stopped);
            return Ok(control);
        }
        // 5B. Ensure the retained surface and render directly into it.
        let mut surface = self
            .presenter
            .surface_frame(&self.backend.display, width, height, depth)
            .map_err(RunDriverFrameError::Present)?;
        render(&mut surface).map_err(RunDriverFrameError::Render)?;
        // 6B. Present after the surface borrow ends.
        self.presenter
            .present_surface(&mut self.backend.display, &mut self.backend.window, clear_redraw)
            .map_err(RunDriverFrameError::Present)?;
        // 7B. Advance runtime.
        driver.runtime_mut().tick_once();
        Ok(control)
    }
}
