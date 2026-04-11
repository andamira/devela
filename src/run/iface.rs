// devela::run::iface
//
//! Defines [`RunApp`], [`RunRender`], [`RunPresent`].
//!
//! Runtime-facing interfaces for stepwise participants.
//

use crate::{RunControl, RunFrame, RunPhase, RunStep, RuntimeTick};

#[doc = crate::_tags!(runtime)]
/// App logic driven step-by-step by a runtime.
#[doc = crate::_doc_location!("run")]
///
/// A runtime or driver gathers events, builds a [`RunStep`], and calls
/// [`run_step`][Self::run_step] to let the app update its state.
///
/// This trait only defines logical progression.
/// It does not define rendering, pacing, or backend services.
pub trait RunApp {
    /// The event type consumed by each runtime step.
    type Event;
    /// The error type returned by each runtime step.
    type Error;

    /// Advances the app by one runtime step.
    ///
    /// Returns a [`RunControl`] indicating whether execution should continue
    /// or stop after the current step.
    ///
    /// Returns an error if the step cannot be completed.
    fn run_step(&mut self, step: RunStep<'_, Self::Event>) -> Result<RunControl, Self::Error>;
}

#[doc = crate::_tags!(runtime)]
/// Minimal backend contract for runtime-driven frontends.
#[doc = crate::_doc_location!("run")]
///
/// A `RunBackend` does two things:
/// - gathers normalized events for the next runtime iteration,
/// - exposes a short-lived backend context for that iteration.
///
/// It does **not** own:
/// - the runtime lifecycle,
/// - logical ticking,
/// - pacing,
/// - or application state progression.
///
/// Those belong to the runtime and the app layer.
///
/// The associated `Context<'a>` is intentionally ephemeral.
/// Typical examples include:
/// - a borrowed terminal output surface,
/// - a borrowed canvas handle,
/// - a borrowed window/document pair,
/// - or a lightweight host snapshot for the current frame.
pub trait RunBackend {
    /// The normalized event type collected by this backend.
    type Event;

    /// The error type returned by backend operations.
    type Error;

    /// The per-frame context exposed by this backend.
    ///
    /// This may borrow backend state and is expected to be short-lived.
    type Context<'a>
    where
        Self: 'a;

    /// Collects backend events into `out`.
    ///
    /// Returns the number of written events.
    ///
    /// Implementations may write fewer than `out.len()` events.
    /// Excess pending events may remain buffered internally for later calls.
    fn collect_events(&mut self, out: &mut [Self::Event]) -> Result<usize, Self::Error>;

    /// Returns the backend context for the current frame.
    ///
    /// This should be cheap and non-owning whenever possible.
    fn context(&mut self) -> Self::Context<'_>;

    /// Builds a [`RunFrame`] from the supplied logical state and gathered events.
    ///
    /// This is a convenience helper built on top of [`context`][Self::context].
    fn frame<'a>(
        &'a mut self,
        tick: RuntimeTick,
        phase: RunPhase,
        events: &'a [Self::Event],
    ) -> RunFrame<'a, Self::Event, Self::Context<'a>>
    where
        Self: Sized,
    {
        RunFrame::from_parts(tick, phase, events, self.context())
    }
}

#[doc = crate::_tags!(runtime)]
/// Rendering logic driven by a runtime step.
#[doc = crate::_doc_location!("run")]
///
/// A runtime or driver builds a [`RunStep`] and calls [`run_render`][Self::run_render]
/// to let a renderer project a scene or app state into its output representation.
///
/// This trait defines rendering only.
/// It does not define logical progression, pacing, or final presentation.
pub trait RunRender<S, E = (), C = ()> {
    /// The successful result of a render step.
    ///
    /// Use `()` when rendering only updates internal state or buffers.
    type Output;
    /// The error type returned by rendering.
    type Error;

    /// Renders `scene` for the current runtime step.
    ///
    /// Returns an implementation-defined output on success.
    fn run_render(
        &mut self,
        step: RunFrame<'_, E, C>,
        scene: &S,
    ) -> Result<Self::Output, Self::Error>;
}

#[doc = crate::_tags!(runtime)]
/// Presentation finalization driven by a runtime.
#[doc = crate::_doc_location!("run")]
///
/// A runtime or driver calls [`run_present`][Self::run_present]
/// after rendering to finalize or expose the prepared output.
///
/// Typical examples include:
/// - flushing a terminal buffer,
/// - swapping a backbuffer,
/// - submitting queued draw commands.
///
/// This trait defines presentation only.
/// It does not define logical progression or rendering.
pub trait RunPresent {
    /// The successful result of a presentation step.
    ///
    /// Use `()` when presentation only finalizes side effects.
    type Output;
    /// The error type returned by presentation.
    type Error;

    /// Finalizes the prepared output for presentation.
    ///
    /// Returns an implementation-defined output on success.
    fn run_present(&mut self) -> Result<Self::Output, Self::Error>;
}
