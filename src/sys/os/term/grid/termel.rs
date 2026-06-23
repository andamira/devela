// devela/src/sys/os/term/grid/termel.rs
//
//! Defines [`Termel`].
//!
//! A termel is a terminal-flavored text element:
//! a [`Textel`] plus terminal style and colors.
//!
//! The wrapped [`Textel`] carries the textual value and optional metadata.
//! The `Termel` layer adds terminal-compatible appearance, while remaining
//! usable by non-terminal renderers that want terminal-like grid semantics.
//
// TOC
// - struct Termel
// - impl constructors and acccesors
// - impl conversions from tuples
// - tests

use crate::{TermColor, TermColors, TermStyle, Textel};

#[doc = crate::_tags!(term text layout color data_structure)]
/// A terminal-flavored textual element.
#[doc = crate::_doc_meta!{location("sys/os/term/grid")}]
///
/// A termel wraps a [`Textel`] and adds terminal style and colors. The wrapped
/// textel carries both the textual value and optional representation metadata.
///
/// `M` can describe representation-specific state such as width, occupancy,
/// continuation cells, or backend-specific placement data.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Termel<T, S = TermStyle, C = TermColors, M = ()> {
    textel: Textel<T, M>,
    style: S,
    colors: C,
}
/* constructors */
impl<T, S, C> Termel<T, S, C, ()> {
    /// Creates a terminal element from its components, without metadata.
    pub const fn new(textel: Textel<T>, style: S, colors: C) -> Self {
        Self::from_textel(textel, style, colors)
    }
    /// Creates a terminal element from a textual value and its appearance, without metadata.
    pub const fn from_value(value: T, style: S, colors: C) -> Self {
        Self::new(Textel::new(value), style, colors)
    }
}
impl<T, S: Default, C: Default> Termel<T, S, C> {
    /// Creates a terminal element with default style and colors.
    pub fn plain(value: T) -> Self {
        Self::from_value(value, S::default(), C::default())
    }
    /// Creates a terminal element from a textel with default appearance.
    pub fn plain_textel(textel: Textel<T>) -> Self {
        Self::new(textel, S::default(), C::default())
    }
}
impl<T> Termel<T> {
    /// Creates a terminal element with terminal-default colors and no styles or metadata.
    pub const fn plain_const(value: T) -> Self {
        Self::from_value(value, TermStyle::new(), TermColors::DEFAULT)
    }
}
// constructors and accessors
#[rustfmt::skip]
impl<T, S, C, M> Termel<T, S, C, M> {
    /// Creates a terminal element from a textel and its appearance.
    pub const fn from_textel(textel: Textel<T, M>, style: S, colors: C) -> Self {
        Self { textel, style, colors }
    }
    /// Creates a terminal element from a textual value, appearance, and metadata.
    pub const fn from_value_meta(value: T, style: S, colors: C, meta: M) -> Self {
        Self::from_textel(Textel::new_meta(value, meta), style, colors)
    }

    /// Returns the textual element.
    pub const fn textel(&self) -> &Textel<T, M> { &self.textel }
    /// Returns an exclusive reference to the textual element.
    pub const fn textel_mut(&mut self) -> &mut Textel<T, M> { &mut self.textel }
    /// Returns the textual value.
    #[must_use]
    pub const fn value(&self) -> &T { self.textel.value() }
    /// Returns an exclusive reference to the textual value.
    #[must_use]
    pub const fn value_mut(&mut self) -> &mut T { self.textel.value_mut() }
    /// Returns the metadata.
    #[must_use]
    pub const fn meta(&self) -> &M { self.textel.meta() }
    /// Returns an exclusive reference to the metadata.
    #[must_use]
    pub const fn meta_mut(&mut self) -> &mut M { self.textel.meta_mut() }
    /// Returns the style.
    #[must_use]
    pub const fn style(&self) -> &S { &self.style }
    /// Returns an exclusive reference to the style.
    #[must_use]
    pub const fn style_mut(&mut self) -> &mut S { &mut self.style }
    /// Returns the colors.
    #[must_use]
    pub const fn colors(&self) -> &C { &self.colors }
    /// Returns an exclusive reference to the colors.
    #[must_use]
    pub const fn colors_mut(&mut self) -> &mut C { &mut self.colors }
    /// Returns this element with the textual element replaced.
    pub const fn with_textel<U, N>(self, textel: Textel<U, N>) -> Termel<U, S, C, N>
    where Self: Copy {
        Termel::from_textel(textel, self.style, self.colors)
    }
    /// Returns this element with the textual value replaced, preserving metadata.
    pub const fn with_value<U>(self, value: U) -> Termel<U, S, C, M> where Self: Copy, M: Copy {
        Termel::from_textel(Textel::new_meta(value, *self.textel.meta()), self.style, self.colors)
    }
    /// Returns this element with the style replaced.
    pub const fn with_style<U>(self, style: U) -> Termel<T, U, C, M> where Self: Copy {
        Termel::from_textel(self.textel, style, self.colors)
    }
    /// Returns this element with the colors replaced.
    pub const fn with_colors<U>(self, colors: U) -> Termel<T, S, U, M> where Self: Copy {
        Termel::from_textel(self.textel, self.style, colors)
    }
    /// Returns this element with the metadata replaced, preserving the textual value.
    pub const fn with_meta<N>(self, meta: N) -> Termel<T, S, C, N> where Self: Copy, T: Copy {
        Termel::from_textel(Textel::new_meta(*self.textel.value(), meta), self.style, self.colors)
    }
    /// Replaces the textual element.
    pub const fn set_textel(&mut self, textel: Textel<T, M>) where Textel<T, M>: Copy {
        self.textel = textel;
    }
    /// Replaces the textual value, preserving metadata.
    pub const fn set_value(&mut self, value: T) where T: Copy {
        *self.textel.value_mut() = value;
    }
    /// Replaces the metadata, preserving the textual value.
    pub const fn set_meta(&mut self, meta: M) where M: Copy {
        *self.textel.meta_mut() = meta;
    }
    /// Replaces the style.
    pub const fn set_style(&mut self, style: S) where S: Copy {
        self.style = style;
    }
    /// Replaces the colors.
    pub const fn set_colors(&mut self, colors: C) where C: Copy {
        self.colors = colors;
    }
    /// Consumes the terminal element and returns its direct components.
    pub fn into_parts(self) -> (Textel<T, M>, S, C) {
        (self.textel, self.style, self.colors)
    }
    /// Consumes the terminal element and returns its value, style, colors, and metadata.
    pub fn into_value_parts(self) -> (T, S, C, M) {
        let (value, meta) = self.textel.into_parts();
        (value, self.style, self.colors, meta)
    }
}
/// # Methods from TermColors
#[rustfmt::skip]
impl<T, S, M> Termel<T, S, TermColors, M> {
    /// Returns the foreground color.
    pub const fn fg(self) -> TermColor where Self: Copy { self.colors.fg() }
    /// Returns the background color.
    pub const fn bg(self) -> TermColor where Self: Copy { self.colors.bg() }
    /// Replaces the foreground color.
    pub const fn with_fg(self, fg: TermColor) -> Termel<T, S, TermColors, M> where Self: Copy {
        Termel::from_textel(self.textel, self.style, self.colors.with_fg(fg))
    }
    /// Replaces the background color.
    pub const fn with_bg(self, bg: TermColor) -> Termel<T, S, TermColors, M> where Self: Copy {
        Termel::from_textel(self.textel, self.style, self.colors.with_bg(bg))
    }
}

/* conversions from tuples */

impl<V, S, C, M> From<(Textel<V, M>, S, C)> for Termel<V, S, C, M> {
    fn from((textel, style, colors): (Textel<V, M>, S, C)) -> Self {
        Self::from_textel(textel, style, colors)
    }
}
impl<V, S, C, M> From<(V, S, C, M)> for Termel<V, S, C, M> {
    fn from((value, style, colors, meta): (V, S, C, M)) -> Self {
        Self::from_value_meta(value, style, colors, meta)
    }
}
impl<V, S, C> From<(V, S, C)> for Termel<V, S, C, ()> {
    fn from((value, style, colors): (V, S, C)) -> Self {
        Self::from_value(value, style, colors)
    }
}
impl<V, C> From<(V, C)> for Termel<V, TermStyle, C, ()> {
    fn from((value, colors): (V, C)) -> Self {
        Self::from_value(value, TermStyle::new(), colors)
    }
}
impl<V, S> From<(V, S)> for Termel<V, S, TermColors, ()> {
    fn from((value, style): (V, S)) -> Self {
        Self::from_value(value, style, TermColors::DEFAULT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TermStyleExt;

    #[test]
    fn construct_and_replace() {
        let termel = Termel::from_value(
            'A',
            TermStyle::BOLD,
            TermColors::new(TermColor::indexed(7), TermColor::indexed(0)),
        );
        assert_eq!(*termel.textel().value(), 'A');
        assert_eq!(*termel.style(), TermStyle::BOLD);
        assert_eq!(termel.colors().fg(), TermColor::indexed(7));
        assert_eq!(termel.colors().bg(), TermColor::indexed(0));
        let termel =
            termel.with_value('B').with_style(TermStyle::ITALIC).with_fg(TermColor::indexed(2));
        assert_eq!(*termel.textel().value(), 'B');
        assert_eq!(*termel.style(), TermStyle::ITALIC);
        assert_eq!(termel.colors().fg(), TermColor::indexed(2));
    }
    #[test]
    fn parts_roundtrip() {
        let termel = Termel::new(Textel::new(b'X'), TermStyle::UNDERLINE, TermColors::DEFAULT);
        let (textel, style, colors) = termel.into_parts();
        assert_eq!(textel, Textel::new(b'X'));
        assert_eq!(style, TermStyle::UNDERLINE);
        assert_eq!(colors, TermColors::DEFAULT);
    }
    #[test]
    fn compact_combinations() {
        type Tiny = Termel<u8, (), u16>;
        type Rich = Termel<u32, TermStyleExt, TermColors>;
        assert!(size_of::<Tiny>() <= 4);
        assert!(size_of::<Rich>() <= 16);
    }
}
