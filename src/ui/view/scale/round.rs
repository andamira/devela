// devela/ui/view/scale/round.rs
//
//! Defines [`UiRound`].
//

#[doc = crate::_tags!(ui quant)]
/// Rounding policy used when projecting UI layout space.
#[doc = crate::_doc_meta! { location("ui/view") }]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum UiRound {
    /// Round toward negative infinity.
    Floor,

    /// Round toward positive infinity.
    Ceil,

    /// Round to the nearest value.
    ///
    /// Halfway cases currently round away from zero.
    #[default]
    Nearest,

    /// Round a rectangle outward so it fully covers the source.
    Outward,

    /// Round a rectangle inward so it remains fully inside the source.
    Inward,
}
