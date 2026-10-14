//
//! Defines [`GraphemeMachine`], [`GraphemeBoundary`].
//

use super::GraphemeMachineState;
#[cfg(doc)]
use crate::{CharIter, GraphemeIter};
use crate::{GraphemeProps, GraphemeScanner, Mem, charu, impl_trait};

#[doc = crate::_tags!(text)]
/// Streaming extended grapheme cluster boundary detector.
#[doc = crate::_doc_meta!{
    location("text/unicode/grapheme", struct GraphemeMachine),
    vendor("grapheme_machine"),
    test_size_of(GraphemeMachine = 3|24; niche Option),
}]
/// Processes Unicode scalars incrementally, retaining the state
/// needed to decide whether each scalar starts a new grapheme cluster.
///
/// For traversing a string while preserving this machine's state,
/// use [`next_char_from_str`][Self::next_char_from_str] or
/// [`next_charu_from_str`][Self::next_charu_from_str].
/// Both return a borrowing [`GraphemeScanner`].
///
/// For one input with independently owned segmentation state, see [`GraphemeIter`].
/// For scalar traversal without segmentation, see [`CharIter`].
///
/// # Examples
/// ```
/// use devela::{GraphemeBoundary as B, GraphemeMachine};
///
/// let mut machine = GraphemeMachine::new();
///
/// let mut scan = machine.next_char_from_str("a\u{0301}b");
///
/// assert_eq!(scan.next(), Some((B::Split, 'a')));
/// assert_eq!(scan.next(), Some((B::Continue, '\u{0301}')));
/// assert_eq!(scan.next(), Some((B::Split, 'b')));
/// ```
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

    /// Creates a borrowing grapheme scanner over `s`, yielding
    /// [`charu`] scalars and their [`GraphemeBoundary`] decisions.
    ///
    /// The returned [`GraphemeScanner`] updates this machine as it
    /// advances, so the same machine can continue with another chunk.
    /// Consume the scanner fully before continuing with the next chunk.
    pub const fn next_charu_from_str<'a>(&'a mut self, s: &'a str) -> GraphemeScanner<'a, charu> {
        GraphemeScanner::<charu>::new(self, s)
    }

    /// Like [`Self::next_charu_from_str`], but yields [`char`] instead of [`charu`].
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
#[doc = crate::_doc_meta!{
    location("text/unicode/grapheme", enum GraphemeBoundary),
    vendor("grapheme_machine"),
    test_size_of(GraphemeBoundary = 1|8; niche Option),
}]
/// Returned by [`GraphemeMachine`] for each code point processed, indicating
/// whether the code point continues the current grapheme cluster or starts a new one.
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
