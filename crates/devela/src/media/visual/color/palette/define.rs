//
//! Defines [`Palette`].
//

use crate::{Index, IndexMut, Range, Slice, is, unwrap};

#[doc = crate::_tags!(color)]
/// An ordered palette of colors with caller-chosen storage.
///
/// An array owns a fixed-size palette; a shared or mutable slice provides a
/// borrowed view. Color values can be raw arrays such as [`RgbBytes`][crate::RgbBytes]
/// or semantic types such as [`Rgb8`][crate::Rgb8]. Wrapping storage and selecting
/// subpalettes require no allocation.
///
/// Palette positions are zero-based. Subpalettes borrow contiguous regions
/// without copying colors, and their indices start at zero. Rotating a palette
/// changes the colors assigned to positions, not indices stored elsewhere
/// (for example, in an indexed image).
///
/// # Example
/// ```
/// use devela::{Palette, RgbBytes};
///
/// const BASE: Palette<[RgbBytes; 4]> = Palette::new([
///     [0, 0, 0], [80, 40, 120], [180, 80, 80], [255, 240, 180],
/// ]);
/// let mut colors = BASE;
/// colors.subpalette_mut(1..4).unwrap().rotate_left(1);
/// assert_eq!(colors[1], [180, 80, 80]);
/// ```
#[doc = crate::_doc_meta!{ location("media/visual/color", struct Palette), }]
#[repr(transparent)]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Palette<S> {
    colors: S,
}

#[rustfmt::skip]
impl<S> Palette<S> {
    /// Wraps the given storage by value, without allocating.
    pub const fn new(colors: S) -> Self { Self { colors } }

    /// Returns a shared reference to the underlying storage.
    pub const fn storage(&self) -> &S { &self.colors }

    /// Returns a mutable reference to the underlying storage.
    pub const fn storage_mut(&mut self) -> &mut S { &mut self.colors }

    /// Consumes the palette and returns its storage.
    pub fn into_storage(self) -> S { self.colors }

    /// Consumes the palette and returns its `Copy` storage in a const context.
    pub const fn into_storage_copy(self) -> S where S: Copy { self.colors }
}

impl<S: Index<usize>> Index<usize> for Palette<S> {
    type Output = S::Output;
    fn index(&self, index: usize) -> &Self::Output {
        &self.colors[index]
    }
}
impl<S: IndexMut<usize>> IndexMut<usize> for Palette<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.colors[index]
    }
}

/* fixed-size, owned storage */

impl<C, const N: usize> Palette<[C; N]> {
    /// Returns the number of colors in the palette.
    pub const fn len(&self) -> usize {
        N
    }
    /// Returns `true` if the palette contains no colors.
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
    /// Returns a shared slice of the colors.
    pub const fn as_slice(&self) -> &[C] {
        &self.colors
    }
    /// Returns a mutable slice of the colors.
    pub const fn as_mut_slice(&mut self) -> &mut [C] {
        &mut self.colors
    }
    /// Returns a reference to the color at `index`, if present.
    pub const fn get(&self, index: usize) -> Option<&C> {
        if index < N { Some(&self.colors[index]) } else { None }
    }
    /// Returns a mutable reference to the color at `index`, if present.
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut C> {
        if index < N { Some(&mut self.colors[index]) } else { None }
    }
    /// Returns a shared view of the entire palette without copying colors.
    pub const fn view(&self) -> Palette<&[C]> {
        Palette::new(&self.colors)
    }
    /// Returns a mutable view of the entire palette without copying colors.
    pub const fn view_mut(&mut self) -> Palette<&mut [C]> {
        Palette::new(&mut self.colors)
    }

    /// Borrows a contiguous subpalette selected by a half-open range.
    ///
    /// Accepts both `core::ops::Range` and `core::range::Range`.
    /// Returns `None` for invalid bounds; empty ranges are valid.
    /// For const contexts, use [`subpalette_bounds`](Self::subpalette_bounds).
    pub fn subpalette(&self, range: impl Into<Range<usize>>) -> Option<Palette<&[C]>> {
        let range = range.into();
        self.subpalette_bounds(range.start, range.end)
    }
    /// Borrows a subpalette within the half-open bounds `start..end`.
    ///
    /// Returns `None` unless `start <= end <= self.len()`; empty ranges are valid.
    /// Indices in the returned subpalette start at zero.
    pub const fn subpalette_bounds(&self, start: usize, end: usize) -> Option<Palette<&[C]>> {
        unwrap![=some_map Slice::range_checked(&self.colors, start, end), |c| Palette::new(c)]
    }

    /// Mutably borrows a contiguous subpalette selected by a half-open range.
    ///
    /// Accepts both `core::ops::Range` and `core::range::Range`.
    /// Returns `None` for invalid bounds; empty ranges are valid.
    /// For const contexts, use [`subpalette_bounds_mut`](Self::subpalette_bounds_mut).
    pub fn subpalette_mut(&mut self, range: impl Into<Range<usize>>) -> Option<Palette<&mut [C]>> {
        let range = range.into();
        self.subpalette_bounds_mut(range.start, range.end)
    }
    /// Mutably borrows a subpalette within the half-open bounds `start..end`.
    ///
    /// Returns `None` unless `start <= end <= self.len()`; empty ranges are valid.
    /// Indices in the returned subpalette start at zero.
    pub const fn subpalette_bounds_mut(
        &mut self,
        start: usize,
        end: usize,
    ) -> Option<Palette<&mut [C]>> {
        unwrap![=some_map
            Slice::range_mut_checked(&mut self.colors, start, end), |c| Palette::new(c)]
    }

    /// Exchanges two colors. Returns `false` if either index is out of bounds.
    pub const fn swap(&mut self, a: usize, b: usize) -> bool {
        self.view_mut().swap(a, b)
    }
    /// Rotates the colors left by `steps` positions, modulo the palette length.
    ///
    /// Does nothing if the palette is empty.
    pub const fn rotate_left(&mut self, steps: usize) {
        self.view_mut().rotate_left(steps);
    }
    /// Rotates the colors right by `steps` positions, modulo the palette length.
    ///
    /// Does nothing if the palette is empty.
    pub const fn rotate_right(&mut self, steps: usize) {
        self.view_mut().rotate_right(steps);
    }
}

/* shared, borrowed storage */

impl<'a, C> Palette<&'a [C]> {
    /// Returns the number of colors in the palette.
    pub const fn len(&self) -> usize {
        self.colors.len()
    }
    /// Returns `true` if the palette contains no colors.
    pub const fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }
    /// Returns a shared slice of the colors.
    pub const fn as_slice(&self) -> &'a [C] {
        self.colors
    }
    /// Returns a reference to the color at `index`, if present.
    pub const fn get(&self, index: usize) -> Option<&'a C> {
        if index < self.colors.len() { Some(&self.colors[index]) } else { None }
    }
    /// Borrows a contiguous subpalette selected by a half-open range.
    ///
    /// Accepts both `core::ops::Range` and `core::range::Range`.
    /// Returns `None` for invalid bounds; empty ranges are valid.
    /// For const contexts, use [`subpalette_bounds`](Self::subpalette_bounds).
    pub fn subpalette(&self, range: impl Into<Range<usize>>) -> Option<Palette<&[C]>> {
        let range = range.into();
        self.subpalette_bounds(range.start, range.end)
    }
    /// Borrows a subpalette within the half-open bounds `start..end`.
    ///
    /// Returns `None` unless `start <= end <= self.len()`; empty ranges are valid.
    /// Indices in the returned subpalette start at zero.
    pub const fn subpalette_bounds(&self, start: usize, end: usize) -> Option<Palette<&[C]>> {
        unwrap![=some_map Slice::range_checked(&self.colors, start, end), |c| Palette::new(c)]
    }
}

/* exclusive, borrowed storage */

impl<C> Palette<&mut [C]> {
    /// Returns the number of colors in the palette.
    pub const fn len(&self) -> usize {
        self.colors.len()
    }
    /// Returns `true` if the palette contains no colors.
    pub const fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }
    /// Returns a shared slice of the colors.
    pub const fn as_slice(&self) -> &[C] {
        self.colors
    }
    /// Returns a mutable slice of the colors.
    pub const fn as_mut_slice(&mut self) -> &mut [C] {
        self.colors
    }
    /// Returns a reference to the color at `index`, if present.
    pub const fn get(&self, index: usize) -> Option<&C> {
        if index < self.colors.len() { Some(&self.colors[index]) } else { None }
    }
    /// Returns a mutable reference to the color at `index`, if present.
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut C> {
        if index < self.colors.len() { Some(&mut self.colors[index]) } else { None }
    }

    /// Borrows a contiguous subpalette selected by a half-open range.
    ///
    /// Accepts both `core::ops::Range` and `core::range::Range`.
    /// Returns `None` for invalid bounds; empty ranges are valid.
    /// For const contexts, use [`subpalette_bounds`](Self::subpalette_bounds).
    pub fn subpalette(&self, range: impl Into<Range<usize>>) -> Option<Palette<&[C]>> {
        let range = range.into();
        self.subpalette_bounds(range.start, range.end)
    }
    /// Borrows a subpalette within the half-open bounds `start..end`.
    ///
    /// Returns `None` unless `start <= end <= self.len()`; empty ranges are valid.
    /// Indices in the returned subpalette start at zero.
    pub const fn subpalette_bounds(&self, start: usize, end: usize) -> Option<Palette<&[C]>> {
        unwrap![=some_map Slice::range_checked(&self.colors, start, end), |c| Palette::new(c)]
    }

    /// Mutably borrows a contiguous subpalette selected by a half-open range.
    ///
    /// Accepts both `core::ops::Range` and `core::range::Range`.
    /// Returns `None` for invalid bounds; empty ranges are valid.
    /// For const contexts, use [`subpalette_bounds_mut`](Self::subpalette_bounds_mut).
    pub fn subpalette_mut(&mut self, range: impl Into<Range<usize>>) -> Option<Palette<&mut [C]>> {
        let range = range.into();
        self.subpalette_bounds_mut(range.start, range.end)
    }
    /// Mutably borrows a subpalette within the half-open bounds `start..end`.
    ///
    /// Returns `None` unless `start <= end <= self.len()`; empty ranges are valid.
    /// Indices in the returned subpalette start at zero.
    pub const fn subpalette_bounds_mut(
        &mut self,
        start: usize,
        end: usize,
    ) -> Option<Palette<&mut [C]>> {
        unwrap![=some_map
            Slice::range_mut_checked(&mut self.colors, start, end), |c| Palette::new(c)]
    }

    /// Exchanges two colors. Returns `false` if either index is out of bounds.
    pub const fn swap(&mut self, a: usize, b: usize) -> bool {
        is! { a >= self.len() || b >= self.len(), return false }
        self.colors.swap(a, b);
        true
    }
    /// Rotates the colors left by `steps` positions, modulo the palette length.
    ///
    /// Does nothing if the palette is empty.
    pub const fn rotate_left(&mut self, steps: usize) {
        let len = self.colors.len();
        is! { len != 0, self.colors.rotate_left(steps % len) }
    }
    /// Rotates the colors right by `steps` positions, modulo the palette length.
    ///
    /// Does nothing if the palette is empty.
    pub const fn rotate_right(&mut self, steps: usize) {
        let len = self.colors.len();
        is! { len != 0, self.colors.rotate_right(steps % len) }
    }
}

#[cfg(test)]
mod _test {
    use super::Palette;
    use crate::{Rgb8, RgbBytes};

    const STATIC: Palette<[RgbBytes; 4]> =
        Palette::new([[0, 0, 0], [10, 20, 30], [40, 50, 60], [70, 80, 90]]);
    const LENGTH: usize = STATIC.len();
    const FIRST: RgbBytes = *STATIC.get(0).unwrap();

    #[test]
    fn const_and_borrowed_access() {
        assert_eq!(LENGTH, 4);
        assert_eq!(FIRST, [0, 0, 0]);
        let view = STATIC.view();
        assert_eq!(view.len(), 4);
        assert_eq!(view.get(2), Some(&[40, 50, 60]));
        assert_eq!(view.get(4), None);
        assert_eq!(view.subpalette(1..3).unwrap().as_slice(), &[[10, 20, 30], [40, 50, 60]]);
        assert!(view.subpalette(3..2).is_none());
        assert!(view.subpalette(0..5).is_none());
        assert!(view.subpalette(4..4).unwrap().is_empty());
    }
    #[test]
    fn edit_and_cycle_part() {
        let mut palette = STATIC;
        palette[0] = [1, 2, 3];
        {
            let mut group = palette.subpalette_mut(1..4).unwrap();
            assert!(group.swap(0, 2));
            assert!(!group.swap(0, 3));
            group.rotate_left(1);
            group.rotate_right(4); // equivalent to one step right
        }
        assert_eq!(palette.as_slice(), &[[1, 2, 3], [70, 80, 90], [40, 50, 60], [10, 20, 30]]);
        palette.rotate_left(6); // 6 % 4 = 2
        assert_eq!(palette[0], [40, 50, 60]);
        palette.rotate_right(6);
        assert_eq!(palette[0], [1, 2, 3]);
    }
    #[test]
    fn borrowed_mutation_and_other_color_values() {
        let mut colors = [Rgb8::new(1, 2, 3), Rgb8::new(4, 5, 6)];
        let mut palette = Palette::new(&mut colors[..]);
        assert_eq!(palette.get(0), Some(&Rgb8::new(1, 2, 3)));
        *palette.get_mut(0).unwrap() = Rgb8::new(7, 8, 9);
        palette.rotate_left(1);
        assert_eq!(colors, [Rgb8::new(4, 5, 6), Rgb8::new(7, 8, 9)]);
        // Rotation operates on elements, without requiring `Copy`.
        #[derive(Debug, PartialEq)]
        struct NonCopy(u8);
        let mut objects = Palette::new([NonCopy(1), NonCopy(2), NonCopy(3)]);
        objects.rotate_right(1);
        assert_eq!(objects.into_storage(), [NonCopy(3), NonCopy(1), NonCopy(2)]);
        let mut empty: Palette<[u8; 0]> = Palette::new([]);
        empty.rotate_left(usize::MAX);
        empty.rotate_right(usize::MAX);
        assert!(!empty.swap(0, 0));
    }
}
