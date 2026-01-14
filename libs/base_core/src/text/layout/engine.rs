// devela_base_core::text::layout::engine
//
//! Defines [`TextFit`], [`TextLayout`], [`TextLayoutStep`].
//!
//! > Everything that does layout or reports layout.
//

use crate::{
    _impl_init, TextCohesion, TextCursor, TextIndex, TextSpan, TextSymbol, TextUnit, is, unwrap,
};

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

#[doc = crate::_tags!(text layout namespace)]
/// Text layout engine configuration.
#[doc = crate::_doc_location!("text/layout")]
///
/// `TextLayout` performs one-dimensional layout negotiation between a
/// symbol stream and an available inline extent. It holds layout policies
/// but does not store mutable state.
///
/// Layout proceeds incrementally via repeated layout steps.
#[derive(Clone, Copy, Debug, Default)]
pub struct TextLayout;
_impl_init![ConstInitCore: Self => TextLayout];

impl TextLayout {
    /// Performs a single inline layout step.
    ///
    /// This method negotiates between a symbol stream and an available inline extent,
    /// starting at the given cursor. It consumes a prefix of the symbol stream,
    /// subject to space and cohesion rules, and emits at most one contiguous span.
    ///
    /// Repeated calls with the returned cursor implement wrapping or pagination.
    ///
    /// Cohesion rules:
    /// - `Atomic`: must fit entirely or stop layout.
    /// - `Breakable`: may be partially consumed if space runs out.
    /// - `Elidable`: may be skipped at zero cost.
    pub const fn step(
        &self,
        symbols: &[TextSymbol],
        cursor: TextCursor,
        extent: Option<TextUnit>, // None = infinite
        out_spans: &mut [TextSpan],
    ) -> TextLayoutStep {
        // Current position in the symbol stream (machine index for slicing).
        let mut index = cursor.index.as_usize();
        // Remaining inline space available to this step.
        let mut remaining = unwrap![some_or extent, TextUnit::MAX];
        // Total space consumed by this step.
        let mut consumed: TextUnit = 0;
        // Start index of the current contiguous span, if any symbol was consumed.
        let mut span_start: Option<usize> = None;
        // Accumulated units for the current span.
        let mut span_units: TextUnit = 0;
        // Whether we stopped due to partial consumption of a breakable symbol.
        let mut partial_break: bool = false;
        // Index of the symbol partially consumed (valid only if `partial_break`).
        let mut partial_index: usize = 0;

        macro_rules! consume_whole {
            ($sym:ident) => {{
                is![span_start.is_none(); span_start = Some(index)];
                span_units += $sym.units;
                consumed += $sym.units;
                remaining -= $sym.units;
                index += 1;
            }};
        }

        while index < symbols.len() && remaining > 0 {
            let sym = symbols[index];
            match sym.cohesion {
                // Elidable symbols may always be skipped.
                TextCohesion::Elidable => index += 1,
                // Atomic symbols must fit entirely or stop layout.
                TextCohesion::Atomic => is![sym.units <= remaining; consume_whole!(sym); break],
                // Breakable symbols may be partially consumed if space runs out.
                TextCohesion::Breakable => {
                    if sym.units <= remaining {
                        consume_whole!(sym);
                    } else {
                        // Partial consumption: consume remaining space,
                        // do not advance the symbol index.
                        is![span_start.is_none(); span_start = Some(index)];
                        span_units += remaining;
                        consumed += remaining;
                        partial_break = true;
                        partial_index = index;
                        break;
                    }
                }
            }
        }
        let mut span_count = 0;
        // Finalize the span if any symbols were consumed.
        if let Some(start) = span_start {
            // A partial break still covers the logical symbol range [i, i+1).
            let end = is![partial_break; (partial_index + 1) as u32; index as u32];
            out_spans[0] = TextSpan::from_prim(start as u32, end, span_units);
            span_count = 1;
        }
        // Fit classification is based on symbol exhaustion, not space exhaustion.
        let fit = if consumed == 0 {
            TextFit::None
        } else if index >= symbols.len() {
            TextFit::Full
        } else {
            TextFit::Partial
        };
        // Carry cursor indicates where layout should resume.
        let carry = if partial_break {
            Some(TextCursor { index: TextIndex(partial_index as u32) })
        } else if index < symbols.len() {
            Some(TextCursor { index: TextIndex(index as u32) })
        } else {
            None
        };
        TextLayoutStep { span_count, consumed, carry, fit }
    }
}

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
