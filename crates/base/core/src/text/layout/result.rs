// devela_base_core::text::layout::result
//
//! Defines [`TextFit`], [`TextLayoutStep`].
//!
//! > Everything that reports layout.
//

use crate::{_impl_init, TextCursor, TextUnit};

#[doc = crate::_tags!(text layout result)]
/// Result of testing whether text fits within an inline extent.
#[doc = crate::_doc_location!("text/layout")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TextFit {
    /// All required symbols fit within the extent.
    Full,

    /// Some symbols fit, but space was exhausted.
    Partial,

    /// No symbols fit within the extent.
    ///
    /// This is the default.
    #[default]
    None,
}
_impl_init![ConstInitCore: Self::None => TextFit];

#[doc = crate::_tags!(text layout result)]
/// Result of a single text layout step.
#[doc = crate::_doc_location!("text/layout")]
///
/// A layout step consumes a prefix of the symbol stream within a given inline extent,
/// and reports:
/// - how much space was used
/// - which spans were produced
/// - whether layout should continue
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextLayoutStep {
    /// Number of valid spans written to the output buffer.
    pub span_count: usize,

    /// Total inline space consumed by this step.
    pub consumed: TextUnit,

    /// Continuation cursor, if not all symbols were consumed.
    ///
    /// `None` indicates that layout reached the end of the symbol stream.
    pub carry: Option<TextCursor>,

    /// Result of testing whether the text fit the available extent.
    pub fit: TextFit,
}
impl TextLayoutStep {
    /// Creates a new text layout step outcome.
    pub const fn new(
        span_count: usize,
        consumed: TextUnit,
        carry: Option<TextCursor>,
        fit: TextFit,
    ) -> Self {
        TextLayoutStep { span_count, consumed, carry, fit }
    }
}
_impl_init![ConstInitCore: Self::new(0, TextUnit::INIT, None, TextFit::INIT) => TextLayoutStep];
