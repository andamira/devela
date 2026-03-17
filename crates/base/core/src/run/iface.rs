// devela_base_core::run::iface

use crate::{RunControl, RunStep};

/// App logic driven step-by-step by a runtime.
///
/// A runtime or driver gathers events, builds a [`RunStep`], and calls
/// [`step`][Self::step] to let the app update its state.
///
/// This trait only defines logical progression.
/// It does not define rendering, pacing, or backend services.
pub trait RunApp {
    /// The event type consumed by each runtime step.
    type Event;

    /// Advances the app by one runtime step.
    ///
    /// Returns a [`RunControl`] indicating whether
    /// execution should continue or stop after the current step.
    fn run_step(&mut self, step: RunStep<'_, Self::Event>) -> RunControl;
}
