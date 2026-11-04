//
//! Defines [`RasterIndexed8`] and [`RasterIndexed8Error`].
//

use crate::{Extent2, Palette, Position2, RasterByteSlice, RasterFormat, Slice};

#[doc = crate::_tags!(image color lifetime)]
/// A borrowed 8-bit indexed raster interpreted through a palette.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster", struct RasterIndexed8),
}]
/// The raster stores only indices; the palette supplies their color values.
/// Neither is copied or owned by this view. Changing palette entries does not
/// require changing the raster, and different palettes can interpret the same
/// indices. Color values may be [`RgbBytes`][crate::RgbBytes], [`Rgb8`][crate::Rgb8],
/// or any other element type. A view borrows its palette immutably; to edit
/// the palette, release the view and construct another afterward.
///
/// Coordinates use the canonical upper-left raster origin. The underlying
/// [`RasterLayout`][crate::RasterLayout] determines row stride and orientation.
/// Palette indices are zero-based; indices missing from the palette cannot be
/// resolved. The source must use [`RasterFormat::INDEXED8`].
///
/// # Example
/// ```
/// use devela::{Extent2, Palette, RasterIndexed8, RgbBytes};
///
/// let mut palette = Palette::new([
///     [0, 0, 0], [200, 40, 40], [40, 200, 40],
/// ]);
/// let indices = [0, 1, 2, 1];
/// let extent = Extent2::new([2, 2]);
/// let mut output: [RgbBytes; 4] = [[0; 3]; 4];
///
/// {
///     let raster = RasterIndexed8::dense(extent, &indices, palette.view()).unwrap();
///     raster.resolve_into(&mut output).unwrap();
/// }
/// palette.swap(1, 2); // the index buffer is unchanged
/// let raster = RasterIndexed8::dense(extent, &indices, palette.view()).unwrap();
/// raster.resolve_into(&mut output).unwrap();
/// assert_eq!(output[1], [40, 200, 40]);
/// ```
#[derive(Debug)]
#[must_use]
pub struct RasterIndexed8<'a, C> {
    raster: RasterByteSlice<&'a [u8]>,
    palette: Palette<&'a [C]>,
}

impl<C> Copy for RasterIndexed8<'_, C> {}
impl<C> Clone for RasterIndexed8<'_, C> {
    fn clone(&self) -> Self {
        *self
    }
}

#[allow(missing_docs)]
#[doc = crate::_tags!(image color error)]
/// Failure while expanding indexed pixels into caller-provided color storage.
#[doc = crate::_doc_meta!{location("media/visual/image/raster", enum RasterIndexed8Error)}]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterIndexed8Error {
    /// Pixel count cannot be represented by a machine-sized output length.
    PixelCountOverflow,
    /// The output slice cannot hold every resolved pixel.
    OutputTooSmall { required: usize, available: usize },
    /// A raster index is absent from the supplied palette.
    MissingColor { coord: Position2<u32>, index: u8 },
}

#[rustfmt::skip]
impl<'a, C> RasterIndexed8<'a, C> {
    /// Borrows an 8-bit indexed raster and a palette without allocating.
    ///
    /// Returns `None` if the source is not 8-bit indexed, or if its layout
    /// or backing bytes are invalid (even if constructed unchecked).
    /// Pixel indices themselves are checked when accessed or resolved, so the
    /// palette can contain fewer than 256 colors.
    pub const fn new(
        raster: RasterByteSlice<&'a [u8]>,
        palette: Palette<&'a [C]>,
    ) -> Option<Self> {
        let format = raster.format();
        if !format.is_indexed() || !matches!(format.bits_per_pixel(), Some(8)) {
            return None;
        }
        let Some(raster) = RasterByteSlice::new(format, raster.layout(), raster.bytes()) else {
            return None;
        };
        Some(Self { raster, palette })
    }

    /// Borrows a tightly packed, top-down 8-bit indexed raster.
    ///
    /// Returns `None` if the extent is not representable by the backing bytes.
    pub const fn dense(
        extent: Extent2<u32>,
        indices: &'a [u8],
        palette: Palette<&'a [C]>,
    ) -> Option<Self> {
        let Some(raster) = RasterByteSlice::dense(RasterFormat::INDEXED8, extent, indices)
            else { return None; };
        Self::new(raster, palette)
    }

    /// Returns the logical raster extent.
    pub const fn extent(&self) -> Extent2<u32> { self.raster.extent() }
    /// Returns the underlying byte-raster view.
    pub const fn raster(&self) -> RasterByteSlice<&'a [u8]> { self.raster }
    /// Returns the borrowed palette view.
    pub const fn palette(&self) -> Palette<&'a [C]> { self.palette }

    /// Returns the palette index stored at `coord`, or `None` outside the raster.
    ///
    /// The index need not identify a color in the palette.
    pub const fn index_at(&self, coord: Position2<u32>) -> Option<u8> {
        let Some(offset) = self.raster.layout().pixel_offset_bytes(coord) else { return None; };
        match Slice::get(self.raster.bytes(), offset) {
            Some(&index) => Some(index),
            None => None,
        }
    }

    /// Returns the color at `coord` from the current palette.
    ///
    /// Returns `None` if the coordinate is outside the raster or if the stored
    /// index has no corresponding palette entry. Use [`index_at`](Self::index_at)
    /// to distinguish these cases.
    pub const fn color_at(&self, coord: Position2<u32>) -> Option<&'a C> {
        let Some(index) = self.index_at(coord) else { return None; };
        self.palette.get(index as usize)
    }

    /// Expands all indexed pixels into a dense, top-down output slice.
    ///
    /// Writes one color per pixel in row-major order, ignoring any source row
    /// padding or reversal. Extra output elements are left unchanged. The palette
    /// determines the output color type; no color-space conversion is performed.
    /// Returns the number of colors written.
    ///
    /// # Errors
    /// Returns [`RasterIndexed8Error::OutputTooSmall`] for insufficient output,
    /// [`RasterIndexed8Error::PixelCountOverflow`] if the extent cannot fit in
    /// `usize`, or [`RasterIndexed8Error::MissingColor`] at the first absent
    /// palette entry. On a missing color, earlier pixels may already be written.
    pub fn resolve_into(&self, output: &mut [C]) -> Result<usize, RasterIndexed8Error>
    where C: Clone {
        let [width, height] = self.extent().dim;
        let Some(required) = (width as usize).checked_mul(height as usize) else {
            return Err(RasterIndexed8Error::PixelCountOverflow);
        };
        if output.len() < required {
            return Err(RasterIndexed8Error::OutputTooSmall {
                required, available: output.len(),
            });
        }
        let mut written = 0;
        let layout = self.raster.layout();
        let indices = self.raster.bytes();
        for y in 0..height {
            // Validated layouts have a representable offset for each stored row.
            let row = layout.row_offset_bytes(y).expect("validated indexed raster row");
            for x in 0..width {
                let index = indices[row + x as usize];
                let Some(color) = self.palette.get(index as usize) else {
                    return Err(RasterIndexed8Error::MissingColor {
                        coord: Position2::new([x, y]), index,
                    });
                };
                output[written] = C::clone(color);
                written += 1;
            }
        }
        Ok(written)
    }
}

#[cfg(test)]
mod _test {
    use super::{RasterIndexed8, RasterIndexed8Error as Error};
    use crate::{Boundary1d, Extent2, Palette, Position2, RasterByteSlice, RasterFormat};
    use crate::{RasterSlice, RasterView, RgbBytes};

    #[test]
    fn dense_and_palette_changes() {
        let mut palette = Palette::new([[0, 0, 0], [200, 40, 40], [40, 200, 40]]);
        let indices = [0, 1, 2, 1];
        let extent = Extent2::new([2, 2]);
        let mut output: [RgbBytes; 4] = [[0; 3]; 4];
        {
            let raster = RasterIndexed8::dense(extent, &indices, palette.view()).unwrap();
            assert_eq!(raster.index_at(Position2::new([1, 0])), Some(1));
            assert_eq!(raster.color_at(Position2::new([1, 0])), Some(&[200, 40, 40]));
            assert_eq!(raster.index_at(Position2::new([2, 0])), None);
            assert_eq!(raster.resolve_into(&mut output), Ok(4));
            assert_eq!(output, [[0, 0, 0], [200, 40, 40], [40, 200, 40], [200, 40, 40]]);
            // The resolved colors can be consumed as a normal typed raster.
            let resolved = RasterSlice::dense(RasterFormat::RGB8, extent, &output).unwrap();
            assert_eq!(resolved.raster_get(Position2::new([0, 1])), Some(&[40, 200, 40]));
        }
        palette.subpalette_mut(1..3).unwrap().rotate_left(1);
        let raster = RasterIndexed8::dense(extent, &indices, palette.view()).unwrap();
        assert_eq!(raster.resolve_into(&mut output), Ok(4));
        assert_eq!(output, [[0, 0, 0], [40, 200, 40], [200, 40, 40], [40, 200, 40]]);
        assert_eq!(indices, [0, 1, 2, 1]);
    }
    #[test]
    fn padded_bottom_up() {
        // The physical lower row precedes the upper row; each row has one pad byte.
        let bytes = [2, 1, 0, 255, 0, 1, 2];
        let extent = Extent2::new([3, 2]);
        let layout = crate::RasterLayout::interleaved(extent, 1, 4, Boundary1d::Lower);
        let source = RasterByteSlice::new(RasterFormat::INDEXED8, layout, &bytes).unwrap();
        let palette = Palette::new([[0u8; 3], [1u8; 3], [2u8; 3]]);
        let raster = RasterIndexed8::new(source, palette.view()).unwrap();
        assert_eq!(raster.index_at(Position2::new([0, 0])), Some(0));
        assert_eq!(raster.index_at(Position2::new([0, 1])), Some(2));
        let mut out = [[99u8; 3]; 7];
        assert_eq!(raster.resolve_into(&mut out), Ok(6));
        assert_eq!(out, [[0; 3], [1; 3], [2; 3], [2; 3], [1; 3], [0; 3], [99; 3]]);
    }
    #[test]
    fn failures_and_empty() {
        let extent = Extent2::new([1, 1]);
        let wrong_bytes = [0, 0, 0];
        let wrong = RasterByteSlice::dense(RasterFormat::RGB8, extent, &wrong_bytes).unwrap();
        let palette = Palette::new([[7u8; 3]]);
        assert!(RasterIndexed8::new(wrong, palette.view()).is_none());
        let short = [0];
        let layout = crate::RasterLayout::dense_interleaved(Extent2::new([2, 1]), 1).unwrap();
        let unchecked = RasterByteSlice::new_unchecked(RasterFormat::INDEXED8, layout, &short);
        assert!(RasterIndexed8::new(unchecked, palette.view()).is_none());
        let invalid_indices = [2];
        let raster = RasterIndexed8::dense(extent, &invalid_indices, palette.view()).unwrap();
        assert_eq!(raster.index_at(Position2::new([0, 0])), Some(2));
        assert_eq!(raster.color_at(Position2::new([0, 0])), None);
        let mut out = [[9u8; 3]; 1];
        assert_eq!(
            raster.resolve_into(&mut []),
            Err(Error::OutputTooSmall { required: 1, available: 0 })
        );
        assert_eq!(
            raster.resolve_into(&mut out),
            Err(Error::MissingColor { coord: Position2::new([0, 0]), index: 2 })
        );
        assert_eq!(out, [[9u8; 3]]);
        let empty_indices = [];
        let empty =
            RasterIndexed8::dense(Extent2::new([0, 0]), &empty_indices, palette.view()).unwrap();
        assert_eq!(empty.resolve_into(&mut out), Ok(0));
        assert_eq!(out, [[9u8; 3]]);
    }
    #[test]
    fn non_copy_colors() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Color(u8);
        let palette = Palette::new([Color(4), Color(8)]);
        let indices = [1, 0];
        let raster = RasterIndexed8::dense(Extent2::new([2, 1]), &indices, palette.view()).unwrap();
        let mut output = [Color(0), Color(0)];
        assert_eq!(raster.resolve_into(&mut output), Ok(2));
        assert_eq!(output, [Color(8), Color(4)]);
    }
}
