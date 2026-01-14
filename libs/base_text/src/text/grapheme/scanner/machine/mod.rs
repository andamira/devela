// devela_base_text::text::grapheme::scanner::machine
//
//! Defines [`GraphemeMachine`].
//
// TOC
// - struct GraphemeMachine
// - enum GraphemeBoundary

use crate::{GraphemeProps, GraphemeScanner, Mem, charu, impl_trait};

mod state;
use state::GraphemeMachineState;

#[cfg(test)]
mod tests;

#[doc = crate::_tags!(text)]
#[doc = concat!["Streaming ", crate::_ABBR_EGC!(), " boundary detector."]]
/// Streaming grapheme cluster boundary detector.
#[doc = crate::_doc_location!("text/grapheme")]
///
/// Sequentially processes Unicode code points,
/// returning whether each starts a new cluster or continues the current one.
///
/// Internally tracks only previous code point properties
/// and a small state machine for efficient segmentation.
///
#[doc = crate::_doc!(vendor: "grapheme_machine")]
#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct GraphemeMachine {
    state: GraphemeMachineState,
    prev: Option<GraphemeProps>,
}

impl_trait! { PartialEq for GraphemeMachine |self, other| Self::eq(*self, *other) }
impl_trait! { Hash for GraphemeMachine |self, state| {
    self.state.hash(state); self.prev.hash(state);
} }

impl GraphemeMachine {
    /// Creates a new grapheme machine in the initial state.
    pub const fn new() -> Self {
        GraphemeMachine { state: GraphemeMachineState::Base, prev: None }
    }

    /// Advances the state machine with the given code point properties.
    ///
    /// Returns [`GraphemeBoundary::Split`] if the code point starts a new cluster,
    /// or [`GraphemeBoundary::Continue`] if it extends the current cluster.
    ///
    /// At start of input (no previous code point), always returns `Split`.
    pub const fn next_char_properties(&mut self, next: GraphemeProps) -> GraphemeBoundary {
        let (boundary, next_state) = self.state.transition(self.prev, next);
        self.state = next_state;
        self.prev = Some(next);
        if boundary { GraphemeBoundary::Split } else { GraphemeBoundary::Continue }
    }

    /// Advances the state machine with a [`charu`] scalar.
    ///
    /// See [`Self::next_char_properties`] for result interpretation.
    pub const fn next_charu(&mut self, c: charu) -> GraphemeBoundary {
        let props = GraphemeProps::for_charu(c);
        self.next_char_properties(props)
    }

    /// Advances the state machine with a [`char`] scalar.
    ///
    /// See [`Self::next_char_properties`] for result interpretation.
    ///
    /// Note: [`GraphemeProps`] lookup is optimized for [`charu`].
    /// Use [`Self::next_charu`] if you already have `charu` to avoid conversion.
    pub const fn next_char(&mut self, c: char) -> GraphemeBoundary {
        let props = GraphemeProps::for_char(c);
        self.next_char_properties(props)
    }

    /// Returns an iterator over [`charu`] scalars in `s` with their cluster actions.
    ///
    /// The iterator processes code points from `s` using [`Self::next_charu`].
    /// For consistent state tracking, consume the entire iterator.
    ///
    /// Does not call [`Self::end_of_input`] automatically, supporting streaming across buffers.
    pub const fn next_charu_from_str<'a>(&'a mut self, s: &'a str) -> GraphemeScanner<'a, charu> {
        GraphemeScanner::<charu>::new(self, s)
    }

    /// Like [`Self::next_charu_from_str`] but converts to [`char`] for convenience.
    pub const fn next_char_from_str<'a>(&'a mut self, s: &'a str) -> GraphemeScanner<'a, char> {
        GraphemeScanner::<char>::new(self, s)
    }

    /// Resets the state machine to initial "start of input" state.
    ///
    /// Use when the input stream ends or at non-text boundaries (e.g., markup tags).
    /// Always returns [`GraphemeBoundary::Split`] to mark the end of the final cluster.
    pub const fn end_of_input(&mut self) -> GraphemeBoundary {
        self.state = GraphemeMachineState::Base;
        self.prev = None;
        GraphemeBoundary::Split
    }

    /// Const-compatible `Eq`.
    pub const fn eq(self, other: Self) -> bool {
        self.state.eq(other.state)
            && match (self.prev, other.prev) {
                (Some(prev), Some(other_prev)) => prev.eq(other_prev),
                (None, None) => true,
                _ => false,
            }
    }
}

#[doc = crate::_tags!(text)]
/// Indicates how to handle a code point when detecting grapheme cluster boundaries.
#[doc = crate::_doc_location!("text/grapheme")]
///
/// Returned by [`GraphemeMachine`] for each code point processed, indicating
/// whether the code point continues the current grapheme cluster or starts a new one.
///
#[doc = crate::_doc!(vendor: "grapheme_machine")]
#[derive(Debug, Clone, Copy, Eq)]
pub enum GraphemeBoundary {
    /// Add this code point to the current grapheme cluster and continue.
    ///
    /// The code point extends the current cluster without creating a boundary.
    Continue,

    /// Finalize the current grapheme cluster and start a new one with this code point.
    ///
    /// The current cluster is complete before this code point.
    /// This code point becomes the first code point of the next cluster.
    Split,
}

impl_trait! { PartialEq for GraphemeBoundary |self, other| Self::eq(*self, *other) }
impl_trait! { Hash for GraphemeBoundary |self, state| { Mem::discriminant(self).hash(state); } }

impl GraphemeBoundary {
    /// Const-compatible `Eq`.
    pub const fn eq(self, other: Self) -> bool {
        matches!((self, other), (Self::Continue, Self::Continue) | (Self::Split, Self::Split))
    }
}
