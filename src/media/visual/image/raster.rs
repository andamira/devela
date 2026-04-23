// devela::media::visual::image::raster
//
//! Defines  [`RasterView`], [`RasterBuf`], [`RasterResize`].
//

use crate::Extent2;

#[doc = crate::_tags!(image)]
/// A borrowed dense 2D raster view over contiguous samples.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This is the minimal read contract for pixel presentation and export.
///
/// The samples form one row-major dense image.
/// It does not imply mutability, ownership, or resizing.
pub trait RasterView {
    /// The stored sample or packed pixel type.
    type Sample;

    /// Returns the logical raster extent in samples.
    fn raster_extent(&self) -> Extent2<u32>;

    /// Returns the dense sample slice in row-major order.
    fn raster_samples(&self) -> &[Self::Sample];
}

#[doc = crate::_tags!(image)]
/// Exclusive access to a dense 2D raster over contiguous samples.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This extends [`RasterView`] with direct mutable access to the dense sample
/// storage.
///
/// It is suitable for drawing into existing raster memory.
/// It does not imply retained ownership or successful resizing.
pub trait RasterBuf: RasterView {
    /// Returns the dense mutable sample slice in row-major order.
    fn raster_samples_mut(&mut self) -> &mut [Self::Sample];

    /// Fills the whole raster with one sample value.
    fn raster_fill(&mut self, sample: Self::Sample)
    where
        Self::Sample: Clone,
    {
        self.raster_samples_mut().fill(sample);
    }
}

#[doc = crate::_tags!(image)]
/// A retained dense raster that owns its sample storage.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This is the practical CPU-side raster contract for surfaces that keep pixels
/// across frames and may need to reshape their backing memory.
///
/// It includes creation, direct mutable access, and reshaping.
pub trait Raster: RasterBuf {
    /// The error produced when a target extent cannot be realized.
    type ResizeError;

    /// Creates a raster with the given extent, filled with `fill`.
    fn raster_new(extent: Extent2<u32>, fill: Self::Sample) -> Result<Self, Self::ResizeError>
    where
        Self: Sized,
        Self::Sample: Clone;

    /// Reshapes the raster to `extent`, filling newly exposed samples with `fill`.
    fn raster_resize(
        &mut self,
        extent: Extent2<u32>,
        fill: Self::Sample,
    ) -> Result<(), Self::ResizeError>
    where
        Self::Sample: Clone;
}
