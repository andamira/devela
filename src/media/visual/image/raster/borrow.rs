// devela/src/media/visual/image/raster/borrow.rs
//
//! Defines [`RasterSlice`] and [`RasterByteSlice`].
//
// TOC
// - definitions
// - impl methods
// - impl traits
// - inner helpers

use crate::{Boundary1d, Extent2, PhantomData, is};
use crate::{RasterBuf, RasterBufBytes, RasterFormat, RasterLayout, RasterSamplePacked};
use crate::{RasterView, RasterViewBytes, RasterViewPacked};

/* RasterSlice */

#[doc = crate::_tags!(image lifetime)]
/// Borrowed dense raster view over typed samples.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster", struct RasterSlice),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RasterSlice<u32, &[u32]> = 28|224),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RasterSlice<u32, &[u32]> = 40|320),
}]
/// This is the concrete borrowed form of [`RasterView`].
///
/// It carries a [`RasterFormat`], a [`RasterLayout`], and a sample slice.
/// The typed view is accepted only when the layout is dense
/// and the stored pixel width matches the sample type.
///
/// It gives access to dense row-major sample storage
/// without implying ownership or resizing.
///
/// For raw backend-facing bytes, use [`RasterByteSlice`].
#[derive(Clone, Copy, Debug)]
pub struct RasterSlice<T, B> {
    format: RasterFormat,
    layout: RasterLayout,
    samples: B,
    _sample: PhantomData<fn() -> T>,
}

#[rustfmt::skip]
impl<T, B> RasterSlice<T, B> {
    /// Returns the raster format.
    pub const fn format(&self) -> RasterFormat { self.format }
    /// Returns the raster layout.
    pub const fn layout(&self) -> RasterLayout { self.layout }
    /// Returns the logical extent.
    pub const fn extent(&self) -> Extent2<u32> { self.layout.extent }
}
#[rustfmt::skip]
impl<'a, T> RasterSlice<T, &'a [T]> {
    /// Creates a borrowed typed raster from an explicit canonical layout.
    ///
    /// Returns `None` if:
    /// - `T` has zero size or is too large for `RasterLayout`,
    /// - the layout is not dense and upper-first,
    /// - the format, layout, and sample type disagree on stored pixel width,
    /// - the format has no supported logical depth,
    /// - the extent overflows `usize`,
    /// - or the sample slice is too short.
    pub const fn new(format: RasterFormat, layout: RasterLayout, samples: &'a [T]) -> Option<Self> {
        let Some(needed) = raster_typed_len::<T>(format, layout) else { return None; };
        if samples.len() < needed { return None; }
        let (samples, _) = samples.split_at(needed);
        Some(Self { format, layout, samples, _sample: PhantomData })
    }
    /// Creates a borrowed typed raster view from an explicit dense layout
    /// without checking invariants.
    ///
    /// This is semantically unchecked but memory-safe.
    pub const fn new_unchecked(format: RasterFormat, layout: RasterLayout, samples: &'a [T])
        -> Self { Self { format, layout, samples, _sample: PhantomData }
    }
    /// Creates a borrowed dense typed raster.
    pub const fn dense(format: RasterFormat, extent: Extent2<u32>, samples: &'a [T])
        -> Option<Self> {
        let Some(bytes_per_pixel) = raster_sample_bytes_u8::<T>() else { return None; };
        let Some(layout) = RasterLayout::dense_interleaved(extent, bytes_per_pixel)
            else { return None; };
        Self::new(format, layout, samples)
    }

    /// Returns the borrowed sample slice.
    pub const fn samples(&self) -> &[T] { self.samples }
}
#[rustfmt::skip]
impl<'a, T> RasterSlice<T, &'a mut [T]> {
    /// Creates a borrowed mutable typed raster from an explicit canonical layout.
    ///
    /// Returns `None` if:
    /// - `T` has zero size or is too large for `RasterLayout`,
    /// - the layout is not dense and upper-first,
    /// - the format, layout, and sample type disagree on stored pixel width,
    /// - the format has no supported logical depth,
    /// - the extent overflows `usize`,
    /// - or the sample slice is too short.
    pub const fn new_mut(format: RasterFormat, layout: RasterLayout, samples: &'a mut [T])
        -> Option<Self> {
        let Some(needed) = raster_typed_len::<T>(format, layout) else { return None; };
        if samples.len() < needed { return None; }
        let (samples, _) = samples.split_at_mut(needed);
        Some(Self { format, layout, samples, _sample: PhantomData })
    }
    /// Creates a borrowed mutable typed raster view from an explicit dense layout
    /// without checking invariants.
    ///
    /// This is semantically unchecked but memory-safe.
    pub const fn new_mut_unchecked(format: RasterFormat, layout: RasterLayout, samples: &'a mut [T])
        -> Self { Self { format, layout, samples, _sample: PhantomData }
    }
    /// Creates a borrowed mutable dense typed raster.
    pub const fn dense_mut(format: RasterFormat, extent: Extent2<u32>, samples: &'a mut [T])
        -> Option<Self> {
        let Some(bytes_per_pixel) = raster_sample_bytes_u8::<T>() else { return None; };
        let Some(layout) = RasterLayout::dense_interleaved(extent, bytes_per_pixel)
            else { return None; };
        Self::new_mut(format, layout, samples)
    }

    /// Returns the borrowed sample slice.
    pub const fn samples(&self) -> &[T] { self.samples }
    /// Returns the exclusively borrowed sample slice.
    pub const fn samples_mut(&mut self) -> &mut [T] { self.samples }
    /// Returns itself as non-mutable.
    pub const fn as_ref(&self) -> RasterSlice<T, &[T]> {
        RasterSlice {
            format: self.format, layout: self.layout, samples: self.samples, _sample: PhantomData,
        }
    }
}

impl<T, B: AsRef<[T]>> RasterView for RasterSlice<T, B> {
    type Sample = T;
    fn raster_extent(&self) -> Extent2<u32> {
        self.layout.extent
    }
    fn raster_samples(&self) -> &[T] {
        self.samples.as_ref()
    }
}
impl<T, B: AsRef<[T]> + AsMut<[T]>> RasterBuf for RasterSlice<T, B> {
    fn raster_samples_mut(&mut self) -> &mut [T] {
        self.samples.as_mut()
    }
}
impl<T: RasterSamplePacked, B: AsRef<[T]>> RasterViewPacked for RasterSlice<T, B> {
    fn raster_depth(&self) -> u8 {
        raster_depth_u8(self.format).expect("Raster format must have a valid u8 depth")
    }
    fn raster_bytes_per_line(&self) -> usize {
        self.layout.bytes_per_line as usize
    }
}

/* RasterByteSlice */

#[doc = crate::_tags!(image lifetime)]
/// Borrowed byte raster view with explicit row layout.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster", struct RasterByteSlice),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RasterByteSlice<&[u8]> = 28|224),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RasterByteSlice<&[u8]> = 40|320),
}]
/// This is the concrete borrowed form of [`RasterViewBytes`].
///
/// It is the safe byte-first bridge for codecs, presentation backends,
/// and foreign image surfaces. The layout may include row padding.
///
/// It gives access to backend-native byte storage
/// without implying ownership or resizing.
#[derive(Clone, Copy, Debug)]
pub struct RasterByteSlice<B> {
    format: RasterFormat,
    layout: RasterLayout,
    bytes: B,
}
#[rustfmt::skip]
impl<B> RasterByteSlice<B> {
    /// Returns the raster format.
    pub const fn format(&self) -> RasterFormat { self.format }
    /// Returns the raster layout.
    pub const fn layout(&self) -> RasterLayout { self.layout }
    /// Returns the logical extent.
    pub const fn extent(&self) -> Extent2<u32> { self.layout.extent }
}
#[rustfmt::skip]
impl<'a> RasterByteSlice<&'a [u8]> {
    /// Creates a borrowed byte raster from an explicit layout.
    ///
    /// Returns `None` if:
    /// - the format has no supported depth or stored byte width,
    /// - the format and layout disagree on stored pixel width,
    /// - the layout has an invalid row stride,
    /// - the required byte length overflows `usize`,
    /// - or the byte slice is too short.
    pub const fn new(format: RasterFormat, layout: RasterLayout, bytes: &'a [u8]) -> Option<Self> {
        let Some(min_len) = raster_byte_len(format, layout) else { return None; };
        if bytes.len() < min_len { return None; }
        Some(Self { format, layout, bytes })
    }
    /// Creates a borrowed byte raster view without checking length.
    ///
    /// This is semantically unchecked but memory-safe.
    pub const fn new_unchecked(format: RasterFormat, layout: RasterLayout, bytes: &'a [u8])
        -> Self { Self { format, layout, bytes }
    }
    /// Creates a borrowed dense byte raster view.
    pub const fn dense(format: RasterFormat, extent: Extent2<u32>, bytes: &'a [u8])
        -> Option<Self> {
        let Some(bytes_per_pixel) = raster_bytes_per_pixel_u8(format) else { return None; };
        let Some(layout) = RasterLayout::dense_interleaved(extent, bytes_per_pixel) else {
            return None;
        };
        Self::new(format, layout, bytes)
    }

    /// Returns the borrowed byte slice.
    pub const fn bytes(&self) -> &'a [u8] { self.bytes }
}
#[rustfmt::skip]
impl<'a> RasterByteSlice<&'a mut [u8]> {
    /// Creates a borrowed mutable byte raster from an explicit layout.
    ///
    /// Returns `None` if:
    /// - the format has no supported depth or stored byte width,
    /// - the format and layout disagree on stored pixel width,
    /// - the layout has an invalid row stride,
    /// - the required byte length overflows `usize`,
    /// - or the byte slice is too short.
    pub const fn new_mut(format: RasterFormat, layout: RasterLayout, bytes: &'a mut [u8])
        -> Option<Self> {
        let Some(min_len) = raster_byte_len(format, layout) else { return None; };
        if bytes.len() < min_len { return None; }
        Some(Self { format, layout, bytes })
    }

    /// Creates a borrowed mutable byte raster view without checking length.
    ///
    /// This is semantically unchecked but memory-safe.
    pub const fn new_mut_unchecked(format: RasterFormat, layout: RasterLayout, bytes: &'a mut [u8])
        -> Self { Self { format, layout, bytes }
    }
    /// Creates a borrowed mutable dense byte raster view.
    pub const fn dense_mut(format: RasterFormat, extent: Extent2<u32>, bytes: &'a mut [u8])
        -> Option<Self> {
        let Some(bytes_per_pixel) = raster_bytes_per_pixel_u8(format) else { return None; };
        let Some(layout) = RasterLayout::dense_interleaved(extent, bytes_per_pixel) else {
            return None;
        };
        Self::new_mut(format, layout, bytes)
    }

    /// Returns the borrowed byte slice.
    pub const fn bytes(&self) -> &[u8] { self.bytes }
    /// Returns the exclusively borrowed byte slice.
    pub const fn bytes_mut(&mut self) -> &mut [u8] { self.bytes }
    /// Returns itself as non-mutable.
    pub const fn as_ref(&self) -> RasterByteSlice<&[u8]> {
        RasterByteSlice { format: self.format, layout: self.layout, bytes: self.bytes }
    }
}

impl<B: AsRef<[u8]>> RasterViewBytes for RasterByteSlice<B> {
    fn raster_extent_bytes(&self) -> Extent2<u32> {
        self.layout.extent
    }
    fn raster_depth(&self) -> u8 {
        raster_depth_u8(self.format).expect("Raster format must have a valid u8 depth")
    }
    fn raster_bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }
    fn raster_bytes_per_pixel_bytes(&self) -> usize {
        self.layout.bytes_per_pixel as usize
    }
    fn raster_bytes_per_line(&self) -> usize {
        self.layout.bytes_per_line as usize
    }
    fn raster_row_start_bytes(&self) -> Boundary1d {
        self.layout.row_start
    }
}
impl<B: AsRef<[u8]> + AsMut<[u8]>> RasterBufBytes for RasterByteSlice<B> {
    fn raster_bytes_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut()
    }
}

/* inner helpers */

const fn raster_depth_u8(format: RasterFormat) -> Option<u8> {
    match format.depth_bits() {
        Some(bits) if bits <= u8::MAX as u16 => Some(bits as u8),
        _ => None,
    }
}
const fn raster_bytes_per_pixel_u8(format: RasterFormat) -> Option<u8> {
    match format.stored_bytes_per_pixel() {
        Some(bytes) if bytes <= u8::MAX as u16 => Some(bytes as u8),
        _ => None,
    }
}
/// Returns the non-zero stored size of `T` when representable by `RasterLayout`.
const fn raster_sample_bytes_u8<T>() -> Option<u8> {
    let bytes = size_of::<T>();
    is! { bytes == 0 || bytes > u8::MAX as usize, None, Some(bytes as u8) }
}
/// Validates a canonical typed raster and returns its exact sample length.
const fn raster_typed_len<T>(format: RasterFormat, layout: RasterLayout) -> Option<usize> {
    let Some(sample_bytes) = raster_sample_bytes_u8::<T>() else {
        return None;
    };
    is! { !layout.is_dense(), return None } // RasterView exposes dense typed samples
    // RasterView has no orientation query,
    // so typed samples use one canonical logical order: upper row first.
    is! { !matches!(layout.row_start, Boundary1d::Upper), return None }
    is! { layout.bytes_per_pixel != sample_bytes, return None }
    is! { !raster_format_matches_layout(format, layout), return None }
    raster_sample_len(layout.extent)
}
/// Validates a byte raster and returns its minimum required byte length.
const fn raster_byte_len(format: RasterFormat, layout: RasterLayout) -> Option<usize> {
    is! { !raster_format_matches_layout(format, layout), return None }
    layout.min_len_bytes() // also validates row stride through RasterLayout::is_valid()
}
/// Returns whether the format and layout describe the same stored pixel width.
///
/// This also ensures that the logical depth can be represented by the
/// `u8` returned from the raster byte/packed traits.
const fn raster_format_matches_layout(format: RasterFormat, layout: RasterLayout) -> bool {
    let Some(_depth) = raster_depth_u8(format) else {
        return false;
    };
    let Some(bytes_per_pixel) = raster_bytes_per_pixel_u8(format) else {
        return false;
    };
    bytes_per_pixel == layout.bytes_per_pixel
}
/// Returns the number of logical samples in an extent.
const fn raster_sample_len(extent: Extent2<u32>) -> Option<usize> {
    let [width, height] = extent.dim;
    (width as usize).checked_mul(height as usize)
}
