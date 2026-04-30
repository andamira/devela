// devela::media::visual::image::raster::traits
//
//!
//
// TOC
// * typed-sample family
//   - trait RasterView
//   - trait RasterBuf
//   - trait Raster
// * byte-view family
//   - trait RasterViewBytes
//   - trait RasterBufBytes
// * typed packed adapter
//   - (trait RasterSampleBytes)
//   - (trait Sealed)
//   - trait RasterSamplePacked
//   - trait RasterViewPacked
//
/* depth vs storage width
   ---------------------------------------------------------------------------
   - `raster_depth()` is a compact logical depth value, kept as `u8` because it
     mirrors common backend/image depth fields such as X11 depth.
   - Logical depth may exclude padding bits. Example: a 32-bit stored XRGB pixel
     often has logical depth 24.
   - `raster_bits_per_pixel()` is stored pixel width and includes padding bits.
   - Stored widths are returned as `u16` to avoid truncation for large sample
     types such as `[u8; 32]`, even though real presentation backends usually
     stay far below that.
*/

#[cfg(feature = "unsafe_layout")]
use crate::MemPod;
use crate::{Extent2, is};

/* typed-sample family */

#[doc = crate::_tags!(image)]
/// A borrowed dense 2D raster view over contiguous samples.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This is the minimal read contract for typed raster consumption and export.
///
/// The samples form one row-major dense image.
/// It does not imply mutability, ownership, resizing, or any byte layout.
pub trait RasterView {
    /// The stored sample or packed pixel type.
    type Sample;

    /// Returns the logical raster extent in samples.
    fn raster_extent(&self) -> Extent2<u32>;
    /// Returns the dense sample slice in row-major order.
    fn raster_samples(&self) -> &[Self::Sample];
}

#[rustfmt::skip]
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

    /* provided */

    /// Fills the whole raster with one sample value.
    fn raster_fill(&mut self, sample: Self::Sample) where Self::Sample: Clone {
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

/* byte-view family */

#[doc = crate::_tags!(image)]
/// A borrowed dense 2D byte raster view with explicit row layout.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This is the safe byte-first bridge for presentation and backend upload.
///
/// It is for rasters whose consumer needs raw bytes and explicit scanline layout.
/// It is a sibling of [`RasterView`], not a replacement for it.
pub trait RasterViewBytes {
    /// Returns the logical raster extent in samples or pixels.
    ///
    /// This uses a distinct method name to avoid ambiguity with [`RasterView`]
    /// on types that implement both traits.
    fn raster_extent_bytes(&self) -> Extent2<u32>;
    /// Returns the logical raster depth in bits.
    ///
    /// This is a compact backend-style depth value. It may differ from the stored
    /// bits per pixel. For example, a 32-bit stored XRGB pixel may have depth 24.
    fn raster_depth(&self) -> u8;
    /// Returns the raw raster bytes.
    fn raster_bytes(&self) -> &[u8];
    /// Returns the stored bytes per scanline.
    fn raster_bytes_per_line(&self) -> usize;

    /* provided */

    /// Returns the total stored byte length.
    fn raster_len_bytes(&self) -> usize {
        self.raster_bytes().len()
    }
    /// Returns the stored bits per pixel, if known from the row layout.
    ///
    /// This is storage width, not logical depth.
    fn raster_bits_per_pixel_bytes(&self) -> Option<u16> {
        let [w, _] = self.raster_extent_bytes().dim;
        is! { w == 0, return None }
        let bits = self.raster_bytes_per_line().checked_mul(8)?;
        let bpp = bits / w as usize;
        u16::try_from(bpp).ok()
    }
}
impl<T> RasterViewBytes for T
where
    T: RasterViewPacked,
    T::Sample: RasterSampleBytes,
{
    fn raster_extent_bytes(&self) -> Extent2<u32> {
        self.raster_extent()
    }
    fn raster_depth(&self) -> u8 {
        RasterViewPacked::raster_depth(self)
    }
    fn raster_bytes(&self) -> &[u8] {
        <T::Sample as RasterSampleBytes>::slice_as_bytes(self.raster_samples())
    }
    fn raster_bytes_per_line(&self) -> usize {
        RasterViewPacked::raster_bytes_per_line(self)
    }
}

#[doc = crate::_tags!(image)]
/// Exclusive access to a dense 2D byte raster with explicit row layout.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This extends [`RasterViewBytes`] with direct mutable access to the backing
/// bytes used by a backend-native image layout.
pub trait RasterBufBytes: RasterViewBytes {
    /// Returns the raw raster bytes mutably.
    fn raster_bytes_mut(&mut self) -> &mut [u8];
}
impl<T> RasterBufBytes for T
where
    T: RasterBuf + RasterViewPacked,
    T::Sample: RasterSampleBytes,
{
    fn raster_bytes_mut(&mut self) -> &mut [u8] {
        <T::Sample as RasterSampleBytes>::slice_as_bytes_mut(self.raster_samples_mut())
    }
}

/* typed packed adapter */

/// Internal byte-conversion bridge for packed raster sample slices.
///
/// It centralizes how `&[Sample]` and `&mut [Sample]`
/// are exposed as byte slices for the public raster byte-view adapters.
///
/// In safe mode it is only implemented for byte-addressable sample layouts that
/// can be flattened without unsafe reinterpreting, i.e. `u8` and `[u8; N]`.
///
/// With `unsafe_layout`, any `MemPod` sample may also use this bridge.
#[rustfmt::skip]
trait RasterSampleBytes: RasterSamplePacked {
    fn slice_as_bytes(samples: &[Self]) -> &[u8] where Self: Sized;
    fn slice_as_bytes_mut(samples: &mut [Self]) -> &mut [u8] where Self: Sized;
}

#[rustfmt::skip]
#[cfg(not(feature = "unsafe_layout"))]
mod safe_impls {
    impl super::RasterSampleBytes for u8 {
        fn slice_as_bytes(samples: &[Self]) -> &[u8] { samples }
        fn slice_as_bytes_mut(samples: &mut [Self]) -> &mut [u8] { samples }
    }
    impl<const N: usize> super::RasterSampleBytes for [u8; N] {
        fn slice_as_bytes(samples: &[Self]) -> &[u8] { samples.as_flattened() }
        fn slice_as_bytes_mut(samples: &mut [Self]) -> &mut [u8] { samples.as_flattened_mut() }
    }
}
#[cfg(feature = "unsafe_layout")]
impl<T: MemPod> RasterSampleBytes for T {
    fn slice_as_bytes(samples: &[Self]) -> &[u8] {
        let len = core::mem::size_of_val(samples);
        unsafe { core::slice::from_raw_parts(samples.as_ptr().cast::<u8>(), len) }
    }
    fn slice_as_bytes_mut(samples: &mut [Self]) -> &mut [u8] {
        let len = core::mem::size_of_val(samples);
        unsafe { core::slice::from_raw_parts_mut(samples.as_mut_ptr().cast::<u8>(), len) }
    }
}

/// Marker trait to prevent downstream implementations of the `RasterSamplePacked` trait.
trait Sealed {}

#[cfg(feature = "unsafe_layout")]
impl<T: MemPod> Sealed for T {}

#[cfg(not(feature = "unsafe_layout"))]
impl Sealed for u8 {}
#[cfg(not(feature = "unsafe_layout"))]
impl<const N: usize> Sealed for [u8; N] {}

#[doc = crate::_tags!(image)]
/// Marker for packed sample types supported by safe byte reinterpretation.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This trait is sealed.
///
/// # Features
/// With the `unsafe_layout` feature enabled, any [`MemPod`] type also qualifies.
///
/// Otherwise it is implemented for selected primitive
/// and fixed-array sample types with known packed layouts.
#[expect(private_bounds, reason = "Sealed trait")]
pub trait RasterSamplePacked: Sealed {}

impl<T: Sealed> RasterSamplePacked for T {}

#[doc = crate::_tags!(image)]
/// A typed raster view whose packed samples can be exposed as bytes.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This bridges typed sample rasters into [`RasterViewBytes`] when the sample
/// layout is known to be safely byte-addressable.
///
/// It is intended for tightly packed sample rows by default.
/// Backends with padded scanlines may override
/// [`raster_bytes_per_line`][Self::raster_bytes_per_line].
pub trait RasterViewPacked: RasterView
where
    Self::Sample: RasterSamplePacked,
{
    /// Returns the logical storage depth in bits.
    ///
    /// This is a compact backend-style depth value, not necessarily the full
    /// stored bits per pixel. For example, a 32-bit stored pixel may have
    /// logical depth 24.
    fn raster_depth(&self) -> u8;

    /* provided */

    /// Returns the stored bits per pixel.
    ///
    /// This is derived from the packed sample type and includes padding bits.
    fn raster_bits_per_pixel(&self) -> u16 {
        (size_of::<Self::Sample>() * 8) as u16
    }
    /// Returns the stored bytes per pixel.
    fn raster_bytes_per_pixel(&self) -> usize {
        size_of::<Self::Sample>()
    }
    /// Returns the stored bytes per scanline.
    ///
    /// Default: tightly packed rows.
    fn raster_bytes_per_line(&self) -> usize {
        let [w, _h] = self.raster_extent().dim;
        w as usize * self.raster_bytes_per_pixel()
    }
}
