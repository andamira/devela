// devela_base_core::text::grapheme::scanner::machine::state

use crate::{GraphemePropCb, GraphemePropInCb, GraphemeProps, Mem, impl_trait};

/// State for tracking grapheme cluster boundary detection progress.
///
/// Encodes the relevant history of category transitions to detect
/// arbitrary-length clusters with finite storage.
///
/// Note: While this machine detects cluster boundaries, rendering very long
/// clusters may not be supported by the underlying system.
#[derive(Clone, Copy, Debug, Default, Eq)]
pub(crate) enum GraphemeMachineState {
    /// Initial state at text start or after a boundary.
    #[default]
    Base,

    /// Saw one RegionalIndicator, awaiting second for emoji flag (GB12/GB13).
    AwaitRegionalPair,

    /// Extended_Pictographic* Extend* sequence, watching for ZWJ (GB11).
    BeforeZwj,

    /// Just saw ZWJ after pictographic sequence, GB11 active.
    AfterZwj,

    /// InCB=Consonant (Extend*) sequence for Indic conjunct rules (GB9c).
    IndicConsonant,

    /// InCB=Consonant Linker+ (Extend*) sequence for Indic conjunct rules (GB9c).
    IndicLinker,
}

impl_trait! { PartialEq for GraphemeMachineState |self, other| Self::eq(*self, *other) }
impl_trait! { Hash for GraphemeMachineState |self, state| { Mem::discriminant(self).hash(state); } }

impl GraphemeMachineState {
    /// Const-compatible `Eq`.
    pub const fn eq(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Base, Self::Base)
                | (Self::AwaitRegionalPair, Self::AwaitRegionalPair)
                | (Self::BeforeZwj, Self::BeforeZwj)
                | (Self::AfterZwj, Self::AfterZwj)
                | (Self::IndicConsonant, Self::IndicConsonant)
                | (Self::IndicLinker, Self::IndicLinker)
        )
    }

    /// Given the previous category and the next category, returns whether
    /// there is a grapheme cluster boundary between two characters of those
    /// categories in the current state, and the state that should be used for
    /// the next transition.
    ///
    /// Use [`GcProperties::Other`] to represent the absense of a character at
    /// the start or end of input.
    ///
    /// Correct use requires that the `prev` of one call equals the `next`
    /// of the previous call that generated the new state. If that is not
    /// upheld then the results are unspecified.

    /// Determines if there's a grapheme boundary between `prev` and `next` characters.
    ///
    /// Returns `(has_boundary, new_state)` where:
    /// - `has_boundary`: `true` if characters should be split into separate clusters.
    /// - `new_state`: Updated state for processing the next character.
    ///
    /// # Correct Use
    /// - Pass `None` for `prev` at text start.
    /// - Use the returned state for the next call.
    /// - Ensure `prev` from current call matches `next` from previous call.
    pub const fn transition(
        self,
        prev: Option<GraphemeProps>,
        next: GraphemeProps,
    ) -> (bool, GraphemeMachineState) {
        use GraphemePropCb::{
            CR, Extend, ExtendedPictographic, L, LF, LV, LVT, Prepend, RegionalIndicator,
            SpacingMark, T, V, Zwj,
        };

        let next_state = self.next_state(next);
        // Start of input always has a boundary
        let Some(prev) = prev else {
            return (true, next_state);
        };

        // GB1/GB2: Start/end boundaries handled by the `prev.is_none()` case above

        #[rustfmt::skip]
        macro_rules! pair_matches {
            ($prev:pat, $next:pat) => {
                matches!(prev.gcb_property(), $prev) && matches!(next.gcb_property(), $next)
            };
        }
        #[rustfmt::skip]
        macro_rules! one_matches {
            ($which:expr, $pat:pat) => { matches!($which.gcb_property(), $pat) };
        }

        // GB3: Keep CR+LF together
        if pair_matches!(CR, LF) {
            return (false, next_state);
        }
        // GB4/5: Break around controls
        if prev.is_any_control() || next.is_any_control() {
            return (true, next_state);
        }
        // GB6-8: Keep Hangul syllables together
        if pair_matches!(L, L | V | LV | LVT)
            || pair_matches!(LV | V, V | T)
            || pair_matches!(LVT | T, T)
        {
            return (false, next_state);
        }

        // GB9: Keep extending characters and ZWJ with previous
        if one_matches!(next, Extend | Zwj) {
            return (false, next_state);
        }
        // GB9a: Keep SpacingMarks with previous
        if one_matches!(next, SpacingMark) {
            return (false, next_state);
        }
        // GB9b: Keep Prepend characters with next
        if one_matches!(prev, Prepend) {
            return (false, next_state);
        }
        // GB9c: Keep Indic conjunct sequences together
        if self.gb9c_active() {
            if matches!(prev.incb_property(), GraphemePropInCb::Linker | GraphemePropInCb::Extend)
                && matches!(next.incb_property(), GraphemePropInCb::Consonant)
            {
                return (false, next_state);
            }
        }
        // GB10: (Obsolete - removed from Unicode specification)

        // GB11: Keep emoji ZWJ sequences together
        if self.gb11_active() && pair_matches!(Zwj, ExtendedPictographic) {
            return (false, next_state);
        }
        // GB12/13: Keep emoji flag pairs together
        if self.gb13_active() && pair_matches!(RegionalIndicator, RegionalIndicator) {
            return (false, next_state);
        }

        // GB999: Default to breaking
        (true, next_state)
    }

    /// Returns the next state after processing the given character.
    const fn next_state(self, next: GraphemeProps) -> Self {
        use {GraphemeMachineState as S, GraphemePropCb as P};

        // Start new sequences regardless of current state
        if matches!(next.gcb_property(), P::ExtendedPictographic) {
            return S::BeforeZwj;
        }
        if matches!(next.incb_property(), GraphemePropInCb::Consonant) {
            return S::IndicConsonant;
        }
        let gc_prop = next.gcb_property();
        let incb_prop = next.incb_property();

        match self {
            S::Base => match gc_prop {
                P::RegionalIndicator => S::AwaitRegionalPair,
                _ => S::Base,
            },
            S::AwaitRegionalPair => S::Base,
            S::BeforeZwj => match gc_prop {
                P::Zwj => S::AfterZwj,
                P::Extend => S::BeforeZwj,
                _ => S::Base,
            },
            S::AfterZwj => S::Base,
            S::IndicConsonant => match incb_prop {
                GraphemePropInCb::Linker => S::IndicLinker,
                GraphemePropInCb::Extend => S::IndicConsonant,
                _ => S::Base,
            },
            S::IndicLinker => match incb_prop {
                GraphemePropInCb::Linker | GraphemePropInCb::Extend => S::IndicLinker,
                _ => S::Base,
            },
        }
    }

    /// GB9c is only active in IndicConsonantExtendLinkerLinker state.
    #[inline(always)] #[rustfmt::skip]
    const fn gb9c_active(self) -> bool { matches!(self, Self::IndicLinker) }

    /// GB11 is only active in AfterZwj state.
    #[inline(always)] #[rustfmt::skip]
    const fn gb11_active(self) -> bool { matches!(self, Self::AfterZwj) }

    /// GB12/GB13 are only active in AwaitRegionalPair state.
    #[inline(always)] #[rustfmt::skip]
    const fn gb13_active(self) -> bool { matches!(self, Self::AwaitRegionalPair) }
}
