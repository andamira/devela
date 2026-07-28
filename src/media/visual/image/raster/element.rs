// devela/src/media/visual/image/raster/element.rs
//
//! Defines [`RasterElement`].

use crate::{Coverage8, Position2};

#[doc = crate::_tags!(image quant)]
/// One rasterization output element for a single raster cell.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RasterElement = 12 ),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RasterElement = 24 ),
}]
/// A `RasterElement` associates a canonical raster cell coordinate
/// with a quantized coverage value.
///
/// It is the minimal destination-independent output
/// of foundational rasterization algorithms:
/// - `coord` identifies the target raster cell in logical raster-grid space.
/// - `coverage` expresses how much of that cell's sampling footprint is covered.
///
/// Grid containment is checked separately by the consumer
/// or by clipped rasterization routines.
///
/// See also [`Coverage8`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterElement {
    coord: Position2<usize>,
    coverage: Coverage8,
}

impl RasterElement {
    #[must_use]
    /// Creates a raster element from a coordinate and coverage.
    pub const fn new(coord: Position2<usize>, coverage: Coverage8) -> Self {
        Self { coord, coverage }
    }
    #[must_use]
    /// Creates a fully covered raster element.
    pub const fn full(coord: Position2<usize>) -> Self {
        Self { coord, coverage: Coverage8::FULL }
    }
    #[must_use]
    /// Returns the target raster-cell coordinate.
    pub const fn coord(self) -> Position2<usize> {
        self.coord
    }
    #[must_use]
    /// Returns the quantized raster coverage.
    pub const fn coverage(self) -> Coverage8 {
        self.coverage
    }
    #[must_use]
    /// Returns the coordinate and coverage as a pair.
    pub const fn into_parts(self) -> (Position2<usize>, Coverage8) {
        (self.coord, self.coverage)
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{Slice, const_assert};

    const P0: Position2<usize> = Position2::new([0, 0]);
    const P1: Position2<usize> = Position2::new([3, 7]);

    const E0: RasterElement = RasterElement::new(P0, Coverage8::ZERO);
    const E1: RasterElement = RasterElement::new(P1, Coverage8::new(128));
    const E2: RasterElement = RasterElement::full(P1);

    #[test]
    const fn constructors_and_accessors_are_const() {
        const_assert!(Slice::<usize>::eq(&E0.coord().dim, &P0.dim));
        const_assert!(eq E0.coverage().get(), 0);
        const_assert!(Slice::<usize>::eq(&E1.coord().dim, &P1.dim));
        const_assert!(eq E1.coverage().get(), 128);
        const_assert!(Slice::<usize>::eq(&E2.coord().dim, &P1.dim));
        const_assert!(eq E2.coverage().get(), 255);
        const COORD_COVERAGE: (Position2<usize>, Coverage8) = E1.into_parts();
        const_assert!(Slice::<usize>::eq(&COORD_COVERAGE.0.dim, &P1.dim));
        const_assert!(eq COORD_COVERAGE.1.get(), 128);
    }
    #[test]
    fn full_sets_full_coverage() {
        let e = RasterElement::full(Position2::new([5, 9]));
        assert_eq!(e.coord(), Position2::new([5, 9]));
        assert_eq!(e.coverage(), Coverage8::FULL);
    }
    #[test]
    fn equality_depends_on_both_fields() {
        let a = RasterElement::new(Position2::new([1, 2]), Coverage8::new(10));
        let b = RasterElement::new(Position2::new([1, 2]), Coverage8::new(10));
        let c = RasterElement::new(Position2::new([1, 2]), Coverage8::new(11));
        let d = RasterElement::new(Position2::new([2, 2]), Coverage8::new(10));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }
}
