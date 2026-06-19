// devela/src/text/layout/wrap.rs
//
//! Defines [`TextSegmentKind`], [`TextSegment`].
//

use crate::{_impl_init, CharIter, ConstInit, TextLayoutSpan, TextUnit, is};
use TextSegmentKind as Kind;

#[doc = crate::_tags!(text layout)]
/// Segment behavior during text wrapping.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextSegmentKind = 1|8),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextSegmentKind {
    /// Visible content that normally stays together.
    #[default]
    Box,

    /// Collapsible boundary space between boxes.
    Glue,

    /// Explicit forced line break.
    HardBreak,
}
_impl_init![Self::Box => TextSegmentKind];
#[rustfmt::skip]
impl TextSegmentKind {
    /// Returns whether this segment kind is a box.
    #[must_use]
    pub const fn is_box(self) -> bool { matches!(self, Self::Box) }

    /// Returns whether this segment kind is glue.
    #[must_use]
    pub const fn is_glue(self) -> bool { matches!(self, Self::Glue) }

    /// Returns whether this segment kind is a hard break.
    #[must_use]
    pub const fn is_hard_break(self) -> bool { matches!(self, Self::HardBreak) }

    /// Returns whether both segment kinds are the same.
    #[must_use]
    pub const fn eq(self, other: Self) -> bool {
        matches![(self, other),
            (Self::Box, Self::Box) | (Self::Glue, Self::Glue) | (Self::HardBreak, Self::HardBreak)
        ]
    }
}

#[doc = crate::_tags!(text layout)]
/// A semantic text segment used for wrapping.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextSegment = 16|128),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextSegment {
    /// Covered symbol range and whole-segment inline extent.
    pub span: TextLayoutSpan,

    /// Segment behavior during wrapping.
    pub kind: TextSegmentKind,
}
_impl_init![Self::INIT => TextSegment];

impl TextSegment {
    /// A zero-width empty box segment.
    pub const INIT: Self = Self {
        span: TextLayoutSpan::INIT,
        kind: TextSegmentKind::Box,
    };

    /// Creates a text segment.
    #[must_use]
    pub const fn new(span: TextLayoutSpan, kind: TextSegmentKind) -> Self {
        Self { span, kind }
    }

    /// Creates a box segment from primitive indices and units.
    #[must_use]
    pub const fn box_from_prim(start: u32, end: u32, units: TextUnit) -> Self {
        Self::new(TextLayoutSpan::from_prim(start, end, units), Kind::Box)
    }

    /// Creates a glue segment from primitive indices and units.
    #[must_use]
    pub const fn glue_from_prim(start: u32, end: u32, units: TextUnit) -> Self {
        Self::new(TextLayoutSpan::from_prim(start, end, units), Kind::Glue)
    }

    /// Creates a hard-break segment from primitive indices.
    #[must_use]
    pub const fn hard_break_from_prim(start: u32, end: u32) -> Self {
        Self::new(TextLayoutSpan::from_prim(start, end, 0), Kind::HardBreak)
    }

    /// Returns whether this segment is a box.
    #[must_use]
    pub const fn is_box(self) -> bool {
        matches!(self.kind, Kind::Box)
    }

    /// Returns whether this segment is glue.
    #[must_use]
    pub const fn is_glue(self) -> bool {
        matches!(self.kind, Kind::Glue)
    }

    /// Returns whether this segment is a hard break.
    #[must_use]
    pub const fn is_hard_break(self) -> bool {
        matches!(self.kind, Kind::HardBreak)
    }

    /// Fills `out` with simple paragraph-oriented segments derived from chars.
    ///
    /// Current policy:
    /// - contiguous non-whitespace chars become [`TextSegmentKind::Box`]
    /// - contiguous inline whitespace becomes one [`TextSegmentKind::Glue`]
    /// - `\n` and `\r\n` become [`TextSegmentKind::HardBreak`]
    ///
    /// Returns the initialized prefix length.
    pub const fn segments_from_chars(text: &str, out: &mut [TextSegment]) -> usize {
        let (mut seg_len, mut char_i) = (0, 0);
        let (mut run_start, mut run_units) = (0, 0);
        let mut run_kind: Option<TextSegmentKind> = None;
        let mut chars = CharIter::<&str>::new(text);
        while let Some(ch) = chars.next_char() {
            if ch == '\n' || ch == '\r' {
                if let Some(kind) = run_kind.take() {
                    let span = TextLayoutSpan::from_prim(run_start, char_i, run_units);
                    is! { !push_segment(out, &mut seg_len, span, kind), return seg_len }
                }
                let break_start = char_i;
                char_i += 1;
                if ch == '\r' && matches!(chars.peek_char(), Some('\n')) {
                    let _ = chars.next_char();
                    char_i += 1;
                }
                let span = TextLayoutSpan::from_prim(break_start, char_i, 0);
                if !push_segment(out, &mut seg_len, span, Kind::HardBreak) {
                    return seg_len;
                }
                continue;
            }
            let kind = is! { ch.is_whitespace(), Kind::Glue, Kind::Box };
            match run_kind {
                None => {
                    run_start = char_i;
                    run_units = 1;
                    run_kind = Some(kind);
                }
                Some(prev) if prev.eq(kind) => {
                    is! { kind.is_box(), run_units += 1 }
                    // Glue stays width 1, regardless of how many whitespace chars it covers.
                }
                Some(prev) => {
                    let span = TextLayoutSpan::from_prim(run_start, char_i, run_units);
                    is! { !push_segment(out, &mut seg_len, span, prev), return seg_len }
                    run_start = char_i;
                    run_units = 1;
                    run_kind = Some(kind);
                }
            }
            char_i += 1;
        }
        if let Some(kind) = run_kind {
            let span = TextLayoutSpan::from_prim(run_start, char_i, run_units);
            let _ = push_segment(out, &mut seg_len, span, kind);
        }
        seg_len
    }
}

const fn push_segment(
    out: &mut [TextSegment],
    len: &mut usize,
    span: TextLayoutSpan,
    kind: TextSegmentKind,
) -> bool {
    is! { *len == out.len(), return false }
    out[*len] = TextSegment::new(span, kind);
    *len += 1;
    true
}
