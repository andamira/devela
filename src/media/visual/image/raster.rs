// devela::media::visual::image::raster
//
//! Defines [`RasterView`].
//

use crate::Extent2;

#[doc = crate::_tags!(image)]
/// A borrowed dense 2D raster view over contiguous samples.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// The meaning of [`Sample`][Self::Sample] is intentionally left open.
/// Typical choices include packed pixels such as `u32`, grayscale bytes,
/// indexed values, or small typed pixel structs.
///
/// Interleaved multi-channel byte buffers such as RGB or RGBA `&[u8]`
/// are not fully described by this trait alone.
pub trait RasterView {
    /// The storage element yielded by [`raster_samples`][Self::raster_samples].
    type Sample;

    /// Returns the raster extent in sample coordinates.
    fn raster_extent(&self) -> Extent2<u32>;

    /// Returns the contiguous backing samples in row-major order.
    ///
    /// This trait assumes a dense raster with no explicit stride metadata.
    fn raster_samples(&self) -> &[Self::Sample];
}
