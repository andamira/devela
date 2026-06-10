// devela::sys::os::term::grid::termel
//
//! Defines [`Termel`].
//
// TOC
// - struct Termel
// - impl constructors and acccesors
// - impl conversions from tuples
// - tests

use crate::{TermColor, TermColors, TermStyle, Textel};

#[doc = crate::_tags!(term text color data_structure)]
/// A textual element with terminal style and colors.
#[doc = crate::_doc_meta!{location("sys/os/term/grid")}]
///
/// `M` carries optional representation-specific metadata,
/// such as cell width or occupancy state, and defaults to `()`.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Termel<T, S = TermStyle, C = TermColors, M = ()> {
    textel: Textel<T>,
    style: S,
    colors: C,
    meta: M,
}
/* constructors */
impl<T, S, C> Termel<T, S, C, ()> {
    /// Creates a terminal element from its components, without metadata.
    pub const fn new(textel: Textel<T>, style: S, colors: C) -> Self {
        Self::new_meta(textel, style, colors, ())
    }
    /// Creates a terminal element from a textual value and its appearance, without metadata,
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
    /// Creates a terminal element with terminal-default colors and with no styles or metadata.
    pub const fn plain_const(value: T) -> Self {
        Self::from_value(value, TermStyle::new(), TermColors::DEFAULT)
    }
}
// constructors and accessors
#[rustfmt::skip]
impl<T, S, C, M> Termel<T, S, C, M> {
    /// Creates a terminal element from its components.
    pub const fn new_meta(textel: Textel<T>, style: S, colors: C, meta: M) -> Self {
        Self { textel, style, colors, meta }
    }
    /// Creates a terminal element from a textual value and its appearance.
    pub const fn from_value_meta(value: T, style: S, colors: C, meta: M) -> Self {
        Self::new_meta(Textel::new(value), style, colors, meta)
    }

    /// Returns the textual element.
    pub const fn textel(&self) -> &Textel<T> { &self.textel }
    /// Returns an exclusive reference to the textual element.
    pub const fn textel_mut(&mut self) -> &mut Textel<T> { &mut self.textel }

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

    /// Returns the metadata.
    #[must_use]
    pub const fn meta(&self) -> &M { &self.meta }
    /// Returns an exclusive reference to the metadata.
    #[must_use]
    pub const fn meta_mut(&mut self) -> &mut M { &mut self.meta }

    /// Returns this element with the textual element replaced.
    pub const fn with_textel<U>(self, textel: Textel<U>) -> Termel<U, S, C, M> where Self: Copy {
        Termel::new_meta(textel, self.style, self.colors, self.meta)
    }
    /// Returns this element with the textual value replaced.
    pub const fn with_value<U>(self, value: U) -> Termel<U, S, C, M> where Self: Copy {
        self.with_textel(Textel::new(value))
    }
    /// Returns this element with the style replaced.
    pub const fn with_style<U>(self, style: U) -> Termel<T, U, C, M> where Self: Copy {
        Termel::new_meta(self.textel, style, self.colors, self.meta)
    }
    /// Returns this element with the colors replaced.
    pub const fn with_colors<U>(self, colors: U) -> Termel<T, S, U, M> where Self: Copy {
        Termel::new_meta(self.textel, self.style, colors, self.meta)
    }
    /// Returns this element with the metadata replaced.
    pub const fn with_meta<U>(self, meta: U) -> Termel<T, S, C, U> where Self: Copy {
        Termel::new_meta(self.textel, self.style, self.colors, meta)
    }

    /// Replaces the textual element.
    pub const fn set_textel(&mut self, textel: Textel<T>) where T: Copy { self.textel = textel; }

    /// Replaces the textual value.
    pub const fn set_value(&mut self, value: T) where T: Copy { self.textel = Textel::new(value); }

    /// Replaces the style.
    pub const fn set_style(&mut self, style: S) where S: Copy { self.style = style; }

    /// Replaces the colors.
    pub const fn set_colors(&mut self, colors: C) where C: Copy { self.colors = colors; }

    /// Replaces the metadata.
    pub const fn set_meta(&mut self, meta: M) where M: Copy, { self.meta = meta; }

    /// Consumes the terminal element and returns its components.
    pub fn into_parts(self) -> (Textel<T>, S, C, M) {
        (self.textel, self.style, self.colors, self.meta) }
}
/// # Methods from TermColors
#[rustfmt::skip]
impl<T, S, M> Termel<T, S, TermColors, M> {
    /// Returns the foreground color.
    pub const fn fg(self) -> TermColor where Self: Copy { self.colors.fg() }
    /// Returns the the background color.
    pub const fn bg(self) -> TermColor where Self: Copy { self.colors.bg() }

    /// Replaces the foreground color.
    pub const fn with_fg(self, fg: TermColor) -> Termel<T, S, TermColors, M> where Self: Copy {
        Termel::new_meta(self.textel, self.style, self.colors.with_fg(fg), self.meta)
    }
    /// Replaces the background color.
    pub const fn with_bg(self, bg: TermColor) -> Termel<T, S, TermColors, M> where Self: Copy {
        Termel::new_meta(self.textel, self.style, self.colors.with_bg(bg), self.meta)
    }
}

/* conversions from tuples */

type Vscm<V, S, C, M> = (V, S, C, M);
impl<V, S, C, M> From<Vscm<V, S, C, M>> for Termel<V, S, C, M> {
    fn from(vscm: Vscm<V, S, C, M>) -> Self {
        let (v, s, c, m) = vscm;
        Self::from_value_meta(v, s, c, m)
    }
}
type Vsc<V, S, C> = (V, S, C);
impl<V, S, C> From<Vsc<V, S, C>> for Termel<V, S, C, ()> {
    fn from(vsc: Vsc<V, S, C>) -> Self {
        let (v, s, c) = vsc;
        Self::from_value(v, s, c)
    }
}
type Vc<V, C> = (V, C);
impl<V, C> From<Vc<V, C>> for Termel<V, TermStyle, C, ()> {
    fn from(vc: Vc<V, C>) -> Self {
        let (v, c) = vc;
        Self::from_value(v, TermStyle::new(), c)
    }
}
type Vs<V, S> = (V, S);
impl<V, S> From<Vs<V, S>> for Termel<V, S, TermColors, ()> {
    fn from(vs: Vs<V, S>) -> Self {
        let (v, s) = vs;
        Self::from_value(v, s, TermColors::DEFAULT)
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
        let (textel, style, colors, meta) = termel.into_parts();
        assert_eq!(meta, ());
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
