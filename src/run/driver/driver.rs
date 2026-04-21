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

/// Use these when the renderable scene is stored separately from the app,
/// or is intentionally passed as an independent value.
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
    /// 7. update runtime phase and tick state.
    ///
    /// The rendered frame observes the post-step application state.
    ///
    /// See also [`step_frame_from_app`][Self::step_frame_from_app]
    /// for app-owned scenes borrowed after logical stepping.
    pub fn step_frame<B, A, R, P, S: ?Sized, RE, PE>(
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
        crate::_run_driver_step_run_frame_body!(@check
            self.runtime, backend, app, renderer, presenter, scene(scene), events, control,
            self.runtime.tick_once(), self.runtime.transition(RunPhase::Stopped));
        Ok(control)
    }

    /// Repeatedly drives frame iterations until the app stops or an error occurs.
    ///
    /// This attempts to enter the [`Running`][RunPhase::Running] phase
    /// before the first frame step.
    ///
    /// See also [`run_frame_from_app`][Self::run_frame_from_app]
    /// for app-owned scenes borrowed after each logical step.
    pub fn run_frame<B, A, R, P, S: ?Sized, RE, PE>(
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
            crate::_run_driver_step_run_frame_body!(
                self.runtime,
                backend,
                app,
                renderer,
                presenter,
                scene(scene),
                events,
                control,
                self.runtime.tick_once(),
                self.runtime.transition(RunPhase::Stopped),
                break
            );
        }
        Ok(())
    }
}

/// Use these when the renderable scene is borrowed from app state
/// after logical stepping.
impl<T> RunDriver<T> {
    /// Drives one runtime iteration including rendering and presentation,
    /// deriving the renderable scene from `app` after logical stepping.
    ///
    /// `scene_of` is called after [`RunApp::run_step`], so rendering observes
    /// the post-step state without holding an immutable borrow of `app`
    /// across the logic step.
    ///
    /// See also [`step_frame`][Self::step_frame]
    /// for scenes passed independently from the application state.
    pub fn step_frame_from_app<B, A, R, P, S: ?Sized, Q, RE, PE>(
        &mut self,
        backend: &mut B,
        app: &mut A,
        renderer: &mut R,
        presenter: &mut P,
        scene_of: Q,
        events: &mut [A::Event],
    ) -> Result<RunControl, RunDriverFrameError<B::Error, A::Error, RE, PE>>
    where
        B: RunBackend<Event = A::Event>,
        A: RunApp,
        Q: for<'a> Fn(&'a A) -> &'a S,
        for<'a> R: RunRender<S, A::Event, B::Context<'a>, Error = RE>,
        for<'a> P: RunPresent<
                A::Event,
                B::Context<'a>,
                Input<'a> = <R as RunRender<S, A::Event, B::Context<'a>>>::Output<'a>,
                Error = PE,
            >,
    {
        crate::_run_driver_step_run_frame_body!(@check
        self.runtime, backend, app, renderer, presenter, scene_of(scene_of), events, control,
        self.runtime.tick_once(), self.runtime.transition(RunPhase::Stopped));
        Ok(control)
    }

    /// Repeatedly drives frame iterations until the app stops or an error occurs,
    /// deriving the renderable scene from `app` after each logical step.
    ///
    /// This attempts to enter the [`Running`][RunPhase::Running] phase
    /// before the first frame step.
    ///
    /// `scene_of` is called after each [`RunApp::run_step`].
    ///
    /// See also [`run_frame`][Self::run_frame]
    /// for scenes passed independently from the application state.
    pub fn run_frame_from_app<B, A, R, P, S: ?Sized, Q, RE, PE>(
        &mut self,
        backend: &mut B,
        app: &mut A,
        renderer: &mut R,
        presenter: &mut P,
        scene_of: Q,
        events: &mut [A::Event],
    ) -> Result<(), RunDriverFrameError<B::Error, A::Error, RE, PE>>
    where
        B: RunBackend<Event = A::Event>,
        A: RunApp,
        Q: for<'a> Fn(&'a A) -> &'a S,
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
            crate::_run_driver_step_run_frame_body!(
                self.runtime,
                backend,
                app,
                renderer,
                presenter,
                scene_of(scene_of),
                events,
                control,
                self.runtime.tick_once(),
                self.runtime.transition(RunPhase::Stopped),
                break
            );
        }
        Ok(())
    }
}
