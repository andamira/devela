// devela/src/text/layout/line.rs
//
//! Defines [`TextLineIter`].
//

use crate::{ConstInit, is};
use crate::{TextCursor, TextLayout, TextLayoutSpan, TextLayoutStep, TextSymbol, TextUnit};

#[doc = crate::_tags!(text layout iterator)]
/// Iterates fixed-width inline layout steps over a symbol stream.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TextLineIter = 24|192),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TextLineIter = 40|320),
}]
///
/// `TextLineIter` repeatedly applies [`TextLayout::step`] with the same inline
/// width, advancing through the returned carry cursor.
///
/// This is mechanical line stepping. It does not perform semantic prose wrapping,
/// whitespace trimming, paragraph handling, indentation, or hard-break handling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextLineIter<'a> {
    layout: &'a TextLayout,
    symbols: &'a [TextSymbol],
    cursor: TextCursor,
    width: TextUnit,
    done: bool,
}
impl<'a> TextLineIter<'a> {
    /// Creates a line iterator starting at the beginning of `symbols`.
    #[must_use]
    pub const fn new(layout: &'a TextLayout, symbols: &'a [TextSymbol], width: TextUnit) -> Self {
        Self::from_cursor(layout, symbols, TextCursor::INIT, width)
    }
    /// Creates a line iterator starting at `cursor`.
    #[must_use]
    pub const fn from_cursor(
        layout: &'a TextLayout,
        symbols: &'a [TextSymbol],
        cursor: TextCursor,
        width: TextUnit,
    ) -> Self {
        Self { layout, symbols, cursor, width, done: false }
    }

    /// Returns the current continuation cursor.
    pub const fn cursor(&self) -> TextCursor {
        self.cursor
    }
    /// Returns the fixed inline width used by this iterator.
    #[must_use]
    pub const fn width(&self) -> TextUnit {
        self.width
    }
    /// Returns whether this iterator has reached the end of the symbol stream.
    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.done
    }

    /// Computes the next fixed-width layout step.
    ///
    /// Writes the produced spans into `out_spans` and returns the step outcome.
    ///
    /// The valid output span prefix is `&out_spans[..step.span_count]`.
    pub const fn next(&mut self, out_spans: &mut [TextLayoutSpan]) -> Option<TextLayoutStep> {
        is! { self.done, return None }
        let step = self.layout.step(self.symbols, self.cursor, Some(self.width), out_spans);
        if step.span_count == 0 {
            self.done = true;
            return None;
        }
        match step.carry {
            Some(cursor) => self.cursor = cursor,
            None => self.done = true,
        }
        Some(step)
    }
}
