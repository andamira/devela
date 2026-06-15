// devela/src/ui/frame/id.rs
//
//!
//

use crate::{UiId, UiKey, UiScope};

#[doc = crate::_tags!(ui)]
/// Immediate UI frame context.
#[doc = crate::_doc_meta!{
    location("ui/frame"),
    #[cfg(pointer_target_width = "32")]
    test_size_of(UiFrame = 12|96; niche Option),
    #[cfg(pointer_target_width = "64")]
    test_size_of(UiFrame = 16|128; niche Option),
}]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiFrame {
    phase: UiPhase,
    scope: UiScope,
}
impl UiFrame {
    /// Constructs a frame at the root scope and initial phase.
    pub const fn new() -> Self {
        Self { phase: UiPhase::Begin, scope: UiScope::ROOT }
    }
    /// Constructs a frame from its phase and scope.
    pub const fn from_parts(phase: UiPhase, scope: UiScope) -> Self {
        Self { phase, scope }
    }
    /// Returns the current UI frame phase.
    pub const fn phase(&self) -> UiPhase {
        self.phase
    }
    /// Returns the current identity scope.
    pub const fn scope(&self) -> UiScope {
        self.scope
    }
    /// Returns this frame with a different phase.
    pub const fn with_phase(self, phase: UiPhase) -> Self {
        Self { phase, ..self }
    }
    /// Returns this frame with a different identity scope.
    pub const fn with_scope(self, scope: UiScope) -> Self {
        Self { scope, ..self }
    }
    /// Resolves a key inside this frame's current scope.
    pub const fn id(&self, key: UiKey) -> UiId {
        self.scope.resolve(key)
    }
    /// Returns this frame inside a child scope resolved from `key`.
    pub const fn enter(self, key: UiKey) -> Self {
        Self { scope: self.scope.enter(key), ..self }
    }
}

#[doc = crate::_tags!(ui time)]
/// Phase of UI frame processing.
#[must_use]
#[doc = crate::_doc_meta!{location("ui/frame")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UiPhase {
    /// Initial frame setup.
    #[default]
    Begin,

    /// Event/input consumption.
    Event,

    /// Layout computation.
    Layout,

    /// Interaction routing.
    Route,

    /// Semantic output collection.
    Semantic,

    /// View projection.
    View,

    /// Frame finalization.
    End,
}
