// devela::run::driver::driver
//
//! Defines [`RunDriver`].
//

#[cfg(doc)]
use crate::RunFrame;
use crate::is;
use crate::{
    RunApp, RunBackend, RunControl, RunDriverError, RunDriverFrameError, RunPhase, RunPresent,
    RunRender, RunStep, Runtime, RuntimeTick,
};

#[doc = crate::_tags!(runtime)]
/// Orchestrates stepwise runtime progression over a backend and an app.
#[doc = crate::_doc_location!("run")]
///
/// `RunDriver` owns a [`Runtime<T>`] and advances it one step at a time.
/// For each step it:
/// - gathers events from a [`RunBackend`],
/// - builds a [`RunStep`],
/// - calls [`RunApp::run_step`],
/// - and updates logical tick and lifecycle state.
///
/// This is the minimal runtime orchestrator.
/// It defines logical progression only.
///
/// Rendering and presentation are layered on top through
/// [`RunBackend::frame`], [`RunRender`], and [`RunPresent`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RunDriver<T> {
    runtime: Runtime<T>,
}

#[rustfmt::skip]
impl<T> RunDriver<T> {
    /// Creates a new driver from an existing runtime.
    pub const fn new(runtime: Runtime<T>) -> Self { Self { runtime } }

    /// Returns a shared reference to the runtime.
    pub const fn runtime(&self) -> &Runtime<T> { &self.runtime }
    /// Returns a mutable reference to the runtime.
    pub const fn runtime_mut(&mut self) -> &mut Runtime<T> { &mut self.runtime }

    /// Returns the current logical tick.
    pub const fn tick(&self) -> RuntimeTick { self.runtime.tick() }
    /// Returns the current run phase.
    pub const fn phase(&self) -> RunPhase { self.runtime.phase() }

    /// Returns `true` if logical progression is currently allowed.
    pub const fn can_advance(&self) -> bool { self.runtime.can_advance() }

    /// Attempts to transition the runtime into the running phase.
    pub const fn start(&mut self) -> bool { self.runtime.transition(RunPhase::Running) }
    /// Attempts to transition the runtime into the stopping phase.
    pub const fn stop(&mut self) -> bool { self.runtime.transition(RunPhase::Stopped) }
}

impl<T> RunDriver<T> {
    /// Drives one runtime iteration.
    ///
    /// Order:
    /// 1. collect backend events,
    /// 2. build a [`RunStep`],
    /// 3. advance app logic,
    /// 4. update lifecycle and logical tick state.
    ///
    /// This is the non-rendering counterpart of [`step_frame`][Self::step_frame].
    pub fn step<B, A>(
        &mut self,
        backend: &mut B,
        app: &mut A,
        events: &mut [A::Event],
    ) -> Result<RunControl, RunDriverError<B::Error, A::Error>>
    where
        B: RunBackend<Event = A::Event>,
        A: RunApp,
    {
        if !self.runtime.can_advance() {
            return Err(RunDriverError::InvalidPhase(self.runtime.phase()));
        }
        let written = backend.collect_events(events).map_err(RunDriverError::Backend)?;
        let step = RunStep::new(self.runtime.tick(), self.runtime.phase(), &events[..written]);
        let control = app.run_step(step).map_err(RunDriverError::App)?;
        match control {
            RunControl::Continue => self.runtime.tick_once(),
            RunControl::Stop => {
                self.runtime.transition(RunPhase::Stopped);
            }
        }
        Ok(control)
    }

    /// Runs logical progression until the app stops or an error occurs.
    ///
    /// This method attempts to enter the [`Running`][RunPhase::Running] phase
    /// before the first step.
    pub fn run<B, A>(
        &mut self,
        backend: &mut B,
        app: &mut A,
        events: &mut [A::Event],
    ) -> Result<(), RunDriverError<B::Error, A::Error>>
    where
        B: RunBackend<Event = A::Event>,
        A: RunApp,
    {
        self.start();
        while self.runtime.can_advance() {
            is! { matches!(self.step(backend, app, events)?, RunControl::Stop), break }
        }
        Ok(())
    }
}

/// Expands the shared core of [`RunDriver::step_frame`] and [`RunDriver::run_frame`].
///
/// This exists to avoid repeating the same higher-ranked `RunRender`/`RunPresent`
/// orchestration body in two methods, which currently helps sidestep compiler
/// limitations around reusing the generic frame-step path through a normal helper.
///
/// The macro performs:
/// - phase validation,
/// - event collection,
/// - logical stepping,
/// - frame construction,
/// - rendering,
/// - presentation,
/// - and runtime progression update.
///
/// The optional trailing expression is executed when the app returns
/// [`RunControl::Stop`], allowing callers such as `run_frame` to `break`.
macro_rules! _step_frame_body {
    // with phase check
    (@check $self:ident, $backend:ident, $app:ident, $renderer:ident, $presenter:ident,
     $scene:ident, $events:ident, $control:ident $(,$break:expr)?) => {
        if !$self.runtime.can_advance() {
            return Err(RunDriverFrameError::InvalidPhase($self.runtime.phase()));
        }
        _step_frame_body!(
            $self, $backend, $app, $renderer, $presenter, $scene, $events, $control $(,$break)?);
    };
    // without phase check
    ($self:ident, $backend:ident, $app:ident, $renderer:ident, $presenter:ident,
     $scene:ident, $events:ident, $control:ident $(,$break:expr)?) => {
        // 1. Gather normalized backend events into caller-provided storage.
        let written = $backend.collect_events($events).map_err(RunDriverFrameError::Backend)?;
        let $events = &$events[..written];
        // 2. Build the logical step snapshot.
        let step = RunStep::new($self.runtime.tick(), $self.runtime.phase(), $events);
        // 3. Advance app logic first, so rendering observes post-step state.
        let $control = $app.run_step(step).map_err(RunDriverFrameError::App)?;
        // 4. Build one backend-facing frame snapshot shared by render and present.
        let mut frame = $backend.frame($self.runtime.tick(), $self.runtime.phase(), $events);
        // 5. Render the current scene or app-facing projection.
        let artifact = $renderer.run_render(&mut frame, $scene).map_err(RunDriverFrameError::Render)?;
        // 6. Finalize or expose the rendered artifact.
        $presenter.run_present(&mut frame, artifact).map_err(RunDriverFrameError::Present)?;
        // 7. Update logical progression.
        match $control {
            RunControl::Continue => $self.runtime.tick_once(),
            RunControl::Stop => {
                $self.runtime.transition(RunPhase::Stopped);
                $($break)?
            }
        }
    };
}

impl<T> RunDriver<T> {
    /// Drives one runtime iteration including rendering and presentation.
    ///
    /// Order:
    /// 1. collect backend events,
    /// 2. build a [`RunStep`],
    /// 3. advance app logic,
    /// 4. build a [`RunFrame`],
    /// 5. render `scene`,
    /// 6. present the renderer output,
    /// 7. update lifecycle and logical tick state.
    ///
    /// The rendered frame corresponds to the post-step application state.
    pub fn step_frame<B, A, R, P, S, RE, PE>(
        &mut self,
        backend: &mut B,
        app: &mut A,
        renderer: &mut R,
        presenter: &mut P,
        scene: &S,
        events: &mut [A::Event],
    ) -> Result<RunControl, RunDriverFrameError<B::Error, A::Error, RE, PE>>
    where
        B: RunBackend<Event = A::Event>,
        A: RunApp,
        for<'a> R: RunRender<S, A::Event, B::Context<'a>, Error = RE>,
        for<'a> P: RunPresent<
                A::Event,
                B::Context<'a>,
                Input<'a> = <R as RunRender<S, A::Event, B::Context<'a>>>::Output<'a>,
                Error = PE,
            >,
    {
        _step_frame_body!(@check self, backend, app, renderer, presenter, scene, events, control);
        Ok(control)
    }

    /// Runs until the app stops or an error occurs,
    /// including rendering and presentation.
    ///
    /// This method attempts to enter the [`Running`][RunPhase::Running] phase
    /// before the first frame step.
    pub fn run_frame<B, A, R, P, S, RE, PE>(
        &mut self,
        backend: &mut B,
        app: &mut A,
        renderer: &mut R,
        presenter: &mut P,
        scene: &S,
        events: &mut [A::Event],
    ) -> Result<(), RunDriverFrameError<B::Error, A::Error, RE, PE>>
    where
        B: RunBackend<Event = A::Event>,
        A: RunApp,
        for<'a> R: RunRender<S, A::Event, B::Context<'a>, Error = RE>,
        for<'a> P: RunPresent<
                A::Event,
                B::Context<'a>,
                Input<'a> = <R as RunRender<S, A::Event, B::Context<'a>>>::Output<'a>,
                Error = PE,
            >,
    {
        self.start();
        while self.runtime.can_advance() {
            _step_frame_body!(
                self, backend, app, renderer, presenter, scene, events, control, break
            );
        }
        Ok(())
    }
}
