// devela/src/text/layout/textel.rs
//
//! Defines [`Textel`], [`TextelWidth`], [`TextelWidthMode`].
//
//! A textel is a discrete textual element for cell-like lines, grids, or canvases.
//!
//! It stores a textual value together with optional metadata. The metadata can
//! describe representation details such as width, occupancy, continuation state,
//! or backend-specific placement information.
//!
//! [`TextelWidth`] and [`TextelWidthMode`] describe how much cell-like inline
//! extent textual units occupy before or during layout/rendering.
//

use crate::{_impl_init, TextUnit, enumint};

#[doc = crate::_tags!(text layout data_structure)]
/// A discrete textual element for cell-like lines, grids, or canvases.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(Textel<u8, ()> = 1|8),
}]
/// `T` is the textual value. `M` is optional representation metadata, such as
/// width, occupancy, continuation state, or backend-specific placement information.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Textel<T, M = ()> {
    value: T,
    meta: M,
}
impl<T> Textel<T, ()> {
    /// Creates a textual element without metadata.
    pub const fn new(value: T) -> Self {
        Self { value, meta: () }
    }
}

#[rustfmt::skip]
impl<T, M> Textel<T, M> {
    /// Creates a textual element with metadata.
    pub const fn new_meta(value: T, meta: M) -> Self { Self { value, meta } }

    /// Returns shared references to the value and metadata.
    #[must_use]
    pub const fn parts(&self) -> (&T, &M) { (&self.value, &self.meta) }
    /// Returns exclusive references to the value and metadata.
    #[must_use]
    pub const fn parts_mut(&mut self) -> (&mut T, &mut M) {
        (&mut self.value, &mut self.meta)
    }
    /// Consumes the textel and returns its value and metadata.
    #[must_use]
    pub fn into_parts(self) -> (T, M) { (self.value, self.meta) }

    /// Returns a shared reference to the contained value.
    #[must_use]
    pub const fn value(&self) -> &T { &self.value }
    /// Returns an exclusive reference to the contained value.
    #[must_use]
    pub const fn value_mut(&mut self) -> &mut T { &mut self.value }
    /// Consumes the textel and returns its contained value.
    #[must_use]
    pub fn into_value(self) -> T { self.value }
    /// Returns this textel with a different value.
    #[must_use]
    pub fn with_value<U>(self, value: U) -> Textel<U, M> {
        Textel::new_meta(value, self.meta)
    }
    /// Maps the contained value.
    pub fn map_value<U>(self, f: impl FnOnce(T) -> U) -> Textel<U, M> {
        Textel::new_meta(f(self.value), self.meta)
    }

    /// Returns a shared reference to the contained metadata.
    #[must_use]
    pub const fn meta(&self) -> &M { &self.meta }
    /// Returns an exclusive reference to the contained metadata.
    #[must_use]
    pub const fn meta_mut(&mut self) -> &mut M { &mut self.meta }
    /// Consumes the textel and returns its contained metadata.
    #[must_use]
    pub fn into_meta(self) -> M { self.meta }
    /// Returns this textel with different metadata.
    #[must_use]
    pub fn with_meta<N>(self, meta: N) -> Textel<T, N> {
        Textel::new_meta(self.value, meta)
    }
    /// Maps the contained metadata.
    pub fn map_meta<N>(self, f: impl FnOnce(M) -> N) -> Textel<T, N> {
        Textel::new_meta(self.value, f(self.meta))
    }
}

impl<T> From<T> for Textel<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
impl<T, M> From<(T, M)> for Textel<T, M> {
    fn from((value, meta): (T, M)) -> Self {
        Self::new_meta(value, meta)
    }
}

enumint! {
    #[doc = crate::_tags!(text layout quant)]
    /// Cell-like inline extent occupied by a textel.
    #[doc = crate::_doc_meta!{
        location("text/layout"),
        test_size_of(TextelWidth = 1|8; niche Option),
    }]
    /// This is a compact bounded measure, suitable for terminal-like grids,
    /// fixed-cell text canvases, and layout policies that need to distinguish
    /// zero-width, narrow, wide, and custom-width text.
    pub TextelWidth, u8, 0, 63
}
impl TextelWidth {
    /// No occupied inline cells.
    pub const ZERO: Self = Self::P0;

    /// One occupied inline cell.
    ///
    /// This is the usual width of a narrow textual unit.
    pub const ONE: Self = Self::P1;

    /// Two occupied inline cells.
    ///
    /// The usual width of a wide textual unit,
    /// such as many East Asian characters and emoji in terminal-like cell grids.
    pub const TWO: Self = Self::P2;

    /// Returns this width as a layout unit.
    pub const fn to_unit(self) -> TextUnit {
        self.get() as TextUnit
    }
}
_impl_init![Self::ZERO => TextelWidth];
impl Default for TextelWidth {
    /// Returns zero occupied inline cells.
    fn default() -> Self {
        Self::ZERO
    }
}

#[doc = crate::_tags!(text layout)]
/// Width policy for deriving textel inline extent.
#[doc = crate::_doc_meta!{
    location("text/layout"),
    test_size_of(TextelWidthMode = 1|8; niche Option),
}]
/// This controls how much cell-like inline space a textual unit occupies.
/// The result can be used when producing layout symbols, textels,
/// or terminal-like grid cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextelWidthMode {
    /// Every textual unit occupies one inline cell.
    #[default]
    Mono,

    /// Every textual unit occupies the given inline width.
    Fixed(TextelWidth),

    /// Unicode East Asian Width based policy.
    EastAsian,

    /// Devela's default terminal-like width policy.
    Terminal,
}
_impl_init![Self::Mono => TextelWidthMode];
