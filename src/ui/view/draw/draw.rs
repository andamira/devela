// devela/src/ui/view/draw/draw.rs
//
//! Defines [`UiDraw`], [`UiDrawKind`].

use crate::{Lunit, UiRect};

// MAYBE
// /// A draw record borrowing its text from a string slice.
// pub type UiDrawRef<'a, S> = UiDraw<S, &'a str>;

#[doc = crate::_tags!(ui)]
/// A resolved backend-neutral draw record in logical UI space.
#[doc = crate::_doc_meta! { location("ui/view") }]
///
/// A draw record describes one spatial presentation operation produced during
/// UI view projection. Its rectangle is fully resolved and expressed in [`Lunit`]s.
///
/// Concrete presenters project the logical rectangle into terminal cells,
/// output pixels, document geometry, message structure,
/// or another supported presentation space.
///
/// Draw records are interpreted in their surrounding [`UiDrawList`] order.
/// When records overlap, later records are presented over earlier records.
///
/// `S` is the caller-defined presentation style payload.
///
/// `T` is the caller-defined text payload. Common choices include `&str` for
/// borrowed frame data and an owned string or text handle for retained output.
///
/// [`UiDrawList`]: crate::UiDrawList
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiDraw<S, T = &'static str> {
    rect: UiRect,
    kind: UiDrawKind<S, T>,
}

#[rustfmt::skip]
impl<S, T> UiDraw<S, T> {
    /* constructors */

    /// Constructs a draw record from its rectangle and operation.
    pub const fn new(rect: UiRect, kind: UiDrawKind<S, T>) -> Self {
        Self { rect, kind }
    }
    /// Constructs a filled-rectangle draw record.
    pub const fn rect_fill(rect: UiRect, style: S) -> Self {
        Self::new(rect, UiDrawKind::RectFill { style })
    }
    /// Constructs a stroked-rectangle draw record.
    pub const fn rect_stroke(rect: UiRect, width: Lunit, style: S) -> Self {
        Self::new(rect, UiDrawKind::RectStroke { width, style })
    }
    /// Constructs a text draw record.
    pub const fn text(rect: UiRect, text: T, style: S) -> Self {
        Self::new(rect, UiDrawKind::Text { text, style })
    }

    /* queries */

    /// Returns the logical draw rectangle.
    pub const fn rect(&self) -> UiRect { self.rect }
    /// Returns the draw operation and its payload.
    pub const fn kind(&self) -> &UiDrawKind<S, T> { &self.kind }

    /* modifiers */

    /// Returns this draw record with another rectangle.
    pub fn with_rect(self, rect: UiRect) -> Self {
        Self { rect, ..self }
    }
    /// Returns this draw record with another rectangle.
    pub const fn with_rect_const(self, rect: UiRect) -> Self where Self: Copy {
        Self { rect, ..self }
    }

    /* conversion */

    /// Returns the rectangle and operation as separate values.
    pub fn into_parts(self) -> (UiRect, UiDrawKind<S, T>) {
        (self.rect, self.kind)
    }
    /// Returns the rectangle and operation as separate values.
    pub const fn into_parts_const(self) -> (UiRect, UiDrawKind<S, T>) where Self: Copy {
        (self.rect, self.kind)
    }
}

#[doc = crate::_tags!(ui)]
/// Operation and payload of a [`UiDraw`].
#[doc = crate::_doc_meta! { location("ui/view") }]
///
/// Geometry common to every operation is stored by [`UiDraw`]. This enum
/// contains only the operation-specific data required by the presenter.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UiDrawKind<S, T> {
    /// Fills the complete logical draw rectangle.
    RectFill {
        /// Presentation style.
        style: S,
    },
    /// Strokes the inside boundary of the logical draw rectangle.
    ///
    /// The stroke remains within the rectangle.
    /// A zero width produces no visible stroke.
    RectStroke {
        /// Logical stroke width.
        width: Lunit,
        /// Presentation style.
        style: S,
    },
    /// Presents one unwrapped text run from the logical start of the rectangle.
    ///
    /// The rectangle defines the available presentation region. A presenter may
    /// clip or truncate text that does not fit, according to its presentation form.
    Text {
        /// Text payload.
        text: T,
        /// Presentation style.
        style: S,
    },
}

impl<S, T> UiDrawKind<S, T> {
    /// Returns the presentation style.
    pub const fn style(&self) -> &S {
        match self {
            Self::RectFill { style }
            | Self::RectStroke { style, .. }
            | Self::Text { style, .. } => style,
        }
    }
    /// Returns the text payload, when this is a text operation.
    pub const fn text(&self) -> Option<&T> {
        match self {
            Self::Text { text, .. } => Some(text),
            _ => None,
        }
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{Lunit, UiExt, UiPos, UiRect};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum Style {
        Background,
        Border,
        Text,
    }
    fn rect(x: i32, y: i32, width: i32, height: i32) -> UiRect {
        UiRect::new(
            UiPos::new([Lunit::new(x), Lunit::new(y)]),
            UiExt::new([Lunit::new(width), Lunit::new(height)]),
        )
    }

    #[test]
    fn constructors() {
        let area = rect(1, 2, 30, 10);
        let fill: UiDraw<_, &str> = UiDraw::rect_fill(area, Style::Background);
        assert_eq!(fill.rect(), area);
        assert_eq!(fill.kind(), &UiDrawKind::RectFill { style: Style::Background });
        let stroke: UiDraw<_, &str> = UiDraw::rect_stroke(area, Lunit::new(1), Style::Border);
        assert_eq!(stroke.rect(), area);
        assert_eq!(
            stroke.kind(),
            &UiDrawKind::RectStroke { width: Lunit::new(1), style: Style::Border }
        );
        let text = UiDraw::text(area, "hello", Style::Text);
        assert_eq!(text.rect(), area);
        assert_eq!(text.kind(), &UiDrawKind::Text { text: "hello", style: Style::Text });
    }
    #[test]
    fn kind_queries() {
        let area = rect(0, 0, 10, 2);
        let fill: UiDraw<Style, &str> = UiDraw::rect_fill(area, Style::Background);
        assert_eq!(fill.kind().style(), &Style::Background);
        assert_eq!(fill.kind().text(), None);
        let text = UiDraw::text(area, "value", Style::Text);
        assert_eq!(text.kind().style(), &Style::Text);
        assert_eq!(text.kind().text(), Some(&"value"));
    }
    #[test]
    fn with_rect_supports_non_copy_payloads() {
        #[derive(Debug, PartialEq, Eq)]
        struct TextPayload(&'static str);
        let first = rect(0, 0, 10, 2);
        let second = rect(4, 5, 20, 3);
        let draw = UiDraw::text(first, TextPayload("hello"), Style::Text);
        let draw = draw.with_rect(second);
        assert_eq!(draw.rect(), second);
        assert_eq!(
            draw.kind(),
            &UiDrawKind::Text { text: TextPayload("hello"), style: Style::Text }
        );
    }
    #[test]
    fn into_parts() {
        let area = rect(2, 3, 12, 4);
        let draw = UiDraw::text(area, "parts", Style::Text);
        let (draw_rect, kind) = draw.into_parts();
        assert_eq!(draw_rect, area);
        assert_eq!(kind, UiDrawKind::Text { text: "parts", style: Style::Text });
    }
}
