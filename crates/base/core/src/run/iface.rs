// devela_base_core::run::iface
//
//! Defines `RunApp`, `RunRender`, `RunPresent`.
//!
//! Runtime-facing interfaces for stepwise participants.
//

use crate::{RunControl, RunStep};

#[doc = crate::_tags!(runtime)]
/// App logic driven step-by-step by a runtime.
#[doc = crate::_doc_location!("run")]
///
/// A runtime or driver gathers events, builds a [`RunStep`], and calls
/// [`step`][Self::step] to let the app update its state.
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
/// Rendering logic driven by a runtime step.
#[doc = crate::_doc_location!("run")]
///
/// A runtime or driver builds a [`RunStep`] and calls [`run_render`][Self::run_render]
/// to let a renderer project a scene or app state into its output representation.
///
/// This trait defines rendering only.
/// It does not define logical progression, pacing, or final presentation.
pub trait RunRender<S, E = ()> {
    /// The successful result of a render step.
    ///
    /// Use `()` when rendering only updates internal state or buffers.
    type Output;
    /// The error type returned by rendering.
    type Error;

    /// Renders `scene` for the current runtime step.
    ///
    /// Returns an implementation-defined output on success.
    fn run_render(&mut self, step: RunStep<'_, E>, scene: &S) -> Result<Self::Output, Self::Error>;
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
