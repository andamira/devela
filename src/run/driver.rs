// devela::run::driver
//
//! Defines [`RunDriver`], [`RunDriverError`].
//

use crate::is;
use crate::{RunApp, RunBackend, RunControl, RunPhase, RunStep, Runtime, RuntimeTick};
#[cfg(doc)]
use crate::{RunFrame, RunPresent, RunRender};

#[doc = crate::_tags!(runtime error)]
/// Errors returned while driving a runtime step.
#[doc = crate::_doc_location!("run")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RunDriverError<BE, AE> {
    /// The runtime cannot advance in its current phase.
    InvalidPhase(RunPhase),
    /// The backend failed while collecting events or exposing frame context.
    Backend(BE),
    /// The app failed while processing a runtime step.
    App(AE),
}

#[doc = crate::_tags!(runtime)]
/// Orchestrates runtime progression over a backend and an app.
#[doc = crate::_doc_location!("run")]
///
/// `RunDriver` owns a [`Runtime`] and advances it step-by-step by:
/// - collecting events from a [`RunBackend`],
/// - building a [`RunStep`],
/// - calling [`RunApp::run_step`],
/// - and updating lifecycle and logical tick state.
///
/// This is the minimal runtime orchestrator.
/// It defines logical progression only.
///
/// Rendering and presentation are layered on top through [`RunFrame`],
/// [`RunRender`], and [`RunPresent`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RunDriver<R> {
    runtime: Runtime<R>,
}

#[rustfmt::skip]
impl<R> RunDriver<R> {
    /// Creates a new driver from an existing runtime context.
    pub const fn new(runtime: Runtime<R>) -> Self { Self { runtime } }

    /// Returns a shared reference to the runtime context.
    pub const fn runtime(&self) -> &Runtime<R> { &self.runtime }
    /// Returns a mutable reference to the runtime context.
    pub const fn runtime_mut(&mut self) -> &mut Runtime<R> { &mut self.runtime }

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


    /// Drives one runtime iteration.
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

    /// Runs until the app stops or an error occurs.
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
