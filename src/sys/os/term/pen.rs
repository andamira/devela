// devela/src/sys/os/term/pen.rs
//
//! Defines [`TermPen`].
//

use crate::{TermColor, TermColors, TermStyle, Termel};

#[doc = crate::_tags!(term text color)]
/// Reusable terminal style and colors for constructing elements.
#[doc = crate::_doc_meta!{
    location("sys/os/term/grid"),
    test_size_of(__: TermPen<devela::TermStyle, devela::TermColors> = 16|128)
}]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TermPen<S = TermStyle, C = TermColors> {
    style: S,
    colors: C,
}

impl TermPen {
    /// A pen with no styles and terminal-default colors.
    pub const PLAIN: Self = Self::new(TermStyle::new(), TermColors::DEFAULT);
}
#[rustfmt::skip]
impl<S, C> TermPen<S, C> {
    /// Creates a terminal pen from its style and colors.
    pub const fn new(style: S, colors: C) -> Self {
        Self { style, colors }
    }
    /// Returns the style.
    #[must_use]
    pub const fn style(&self) -> &S { &self.style }
    /// Returns the colors.
    #[must_use]
    pub const fn colors(&self) -> &C { &self.colors }

    /// Returns this pen with its style replaced.
    pub const fn with_style<U>(self, style: U) -> TermPen<U, C> where Self: Copy {
        TermPen::new(style, self.colors)
    }
    /// Returns this pen with its colors replaced.
    pub const fn with_colors<U>(self, colors: U) -> TermPen<S, U> where Self: Copy {
        TermPen::new(self.style, colors)
    }

    /// Replaces the style.
    pub const fn set_style(&mut self, style: S) where S: Copy { self.style = style; }
    /// Replaces the colors.
    pub const fn set_colors(&mut self, colors: C) where C: Copy { self.colors = colors; }

    /// Creates a terminal element using this pen.
    pub const fn termel<T>(self, value: T) -> Termel<T, S, C> where Self: Copy {
        Termel::from_value(value, self.style, self.colors)
    }
}

/// # Methods from TermColors
#[rustfmt::skip]
impl<S> TermPen<S, TermColors> {
    /// Returns the foreground color.
    pub const fn fg(self) -> TermColor where Self: Copy { self.colors.fg() }
    /// Returns the the background color.
    pub const fn bg(self) -> TermColor where Self: Copy { self.colors.bg() }

    /// Replaces the foreground color.
    pub const fn with_fg(self, fg: TermColor) -> TermPen<S, TermColors> where Self: Copy {
        TermPen::new(self.style, self.colors.with_fg(fg))
    }
    /// Replaces the background color.
    pub const fn with_bg(self, bg: TermColor) -> TermPen<S, TermColors> where Self: Copy {
        TermPen::new(self.style, self.colors.with_bg(bg))
    }
}
