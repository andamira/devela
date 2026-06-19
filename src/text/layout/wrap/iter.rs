// devela/src/text/layout/wrap.rs
//
//! Defines text wrapping types.
//

use crate::{_impl_init, ConstInit, is};
use crate::{TextCursor, TextIndex, TextLayoutSpan, TextSegment, TextSegmentKind, TextUnit};
use TextBreakKind as Break;

#[doc = crate::_tags!(text layout)]
/// Reason why a wrapped text line ended.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextBreakKind = 1|8),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextBreakKind {
    /// The line ended between segments.
    #[default]
    Between,

    /// The line ended by splitting a segment.
    Inside,

    /// The line ended because of an explicit hard break.
    Hard,
}
_impl_init![Self::Between => TextBreakKind];

#[doc = crate::_tags!(text layout)]
/// A wrapped visual text line.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextLine = 24),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextLine {
    /// Covered source range and consumed inline extent.
    pub span: TextLayoutSpan,

    /// Continuation point, if more content remains.
    pub carry: Option<TextCursor>,

    /// How this line ended.
    pub break_kind: TextBreakKind,
}
_impl_init![Self::INIT => TextLine];

impl TextLine {
    /// An empty line.
    pub const INIT: Self = Self {
        span: TextLayoutSpan::INIT,
        carry: None,
        break_kind: Break::Between,
    };
    /// Creates a wrapped text line.
    #[must_use]
    pub const fn new(
        span: TextLayoutSpan,
        carry: Option<TextCursor>,
        break_kind: TextBreakKind,
    ) -> Self {
        Self { span, carry, break_kind }
    }
}

#[doc = crate::_tags!(text layout iterator)]
/// Iterates wrapped visual lines over text segments.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TextWrapIter = 24|192),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TextWrapIter = 40|320),
}]
///
/// `TextWrapIter` applies simple prose wrapping over semantic segments:
/// boxes stay together when possible, glue collapses between boxes, and hard
/// breaks force a line boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextWrapIter<'a> {
    segments: &'a [TextSegment],
    cursor: usize,
    width: TextUnit,
    split: Option<TextIndex>,
}

impl<'a> TextWrapIter<'a> {
    /// Creates a text wrapping iterator.
    #[must_use]
    pub const fn new(segments: &'a [TextSegment], width: TextUnit) -> Self {
        Self { segments, cursor: 0, width, split: None }
    }

    /// Returns the current segment cursor.
    #[must_use]
    pub const fn cursor(&self) -> usize {
        self.cursor
    }
    /// Returns the wrapping width.
    #[must_use]
    pub const fn width(&self) -> TextUnit {
        self.width
    }
    /// Returns whether a segment is currently being split across lines.
    #[must_use]
    pub const fn is_splitting(&self) -> bool {
        self.split.is_some()
    }

    /// Computes the next wrapped visual line.
    ///
    /// Advances the internal segment cursor and returns the next [`TextLine`],
    /// or `None` when no further line can be produced.
    ///
    /// The returned line records the covered source range,
    /// consumed inline units, continuation cursor, and break kind.
    ///
    /// This performs semantic wrapping over [`TextSegment`]s.
    /// It does not shape, render, scroll, place, or interact with text.
    pub const fn next_line(&mut self) -> Option<TextLine> {
        is! { self.width == 0, return None }
        let (mut i, line_start, mut line_units, mut last_end);
        let mut break_kind = Break::Between;
        if let Some(split_start) = self.split {
            let seg = self.segments[self.cursor];
            let seg_end = seg.span.end();
            let remaining = seg_end.0 - split_start.0;
            let take = if remaining > self.width { self.width } else { remaining };
            let split_end = TextIndex(split_start.0 + take);
            line_start = split_start;
            line_units = take;
            last_end = split_end;
            if remaining > self.width {
                self.split = Some(split_end);
                let span = TextLayoutSpan::new(line_start, split_end, take);
                return Some(TextLine::new(span, Some(TextCursor::new(split_end)), Break::Inside));
            }
            self.split = None;
            self.cursor += 1;
            i = self.cursor;
            if line_units == self.width {
                let carry = is! { i < self.segments.len(), Some(TextCursor::new(last_end)), None };
                let span = TextLayoutSpan::new(line_start, last_end, line_units);
                return Some(TextLine::new(span, carry, Break::Between));
            }
        } else {
            while self.cursor < self.segments.len() && self.segments[self.cursor].kind.is_glue() {
                self.cursor += 1;
            }
            is! { self.cursor >= self.segments.len(), return None }
            i = self.cursor;
            line_start = self.segments[i].span.start();
            line_units = 0;
            last_end = line_start;
        }
        let mut pending_glue = false;
        while i < self.segments.len() {
            let seg = self.segments[i];
            match seg.kind {
                TextSegmentKind::HardBreak => {
                    break_kind = Break::Hard;
                    i += 1;
                    self.cursor = i;
                    let carry = if i < self.segments.len() {
                        Some(TextCursor::new(seg.span.end()))
                    } else {
                        None
                    };
                    let span = TextLayoutSpan::new(line_start, last_end, line_units);
                    return Some(TextLine::new(span, carry, break_kind));
                }
                TextSegmentKind::Glue => {
                    pending_glue = line_units > 0;
                    i += 1;
                }
                TextSegmentKind::Box => {
                    let glue_units = if pending_glue { 1 } else { 0 };
                    let needed = glue_units + seg.span.units;
                    if line_units > 0 && line_units + needed > self.width {
                        break_kind = Break::Between;
                        break;
                    }
                    if line_units == 0 && seg.span.units > self.width {
                        if can_split_by_unit(seg) {
                            let take = self.width;
                            let split_end = TextIndex(seg.span.start().0 + take);
                            self.split = Some(split_end);
                            self.cursor = i;
                            let span = TextLayoutSpan::new(seg.span.start(), split_end, take);
                            let cursor = TextCursor::new(split_end);
                            return Some(TextLine::new(span, Some(cursor), Break::Inside));
                        } else {
                            // If the segment cannot be split by simple unit/index arithmetic,
                            // keep it whole. This may produce an over-wide line,
                            // but preserves correct source ranges.
                            line_units = seg.span.units;
                            last_end = seg.span.end();
                            i += 1;
                            break;
                        }
                    }
                    line_units += needed;
                    last_end = seg.span.end();
                    pending_glue = false;
                    i += 1;
                }
            }
        }
        self.cursor = i;
        let carry = if self.cursor < self.segments.len() || self.split.is_some() {
            Some(TextCursor::new(last_end))
        } else {
            None
        };
        let span = TextLayoutSpan::new(line_start, last_end, line_units);
        Some(TextLine::new(span, carry, break_kind))
    }
}

impl Iterator for TextWrapIter<'_> {
    type Item = TextLine;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_line()
    }
}

const fn can_split_by_unit(seg: TextSegment) -> bool {
    seg.span.end().0 - seg.span.start().0 == seg.span.units
}
