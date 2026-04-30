// devela::media::visual::image::raster::borrow
//
//! Borrowed concrete raster views.
//
// TOC
// - definitions
// - impl methods
// - impl traits
// - inner helpers

use crate::{
    Extent2, RasterBuf, RasterBufBytes, RasterFormat, RasterLayout, RasterSamplePacked, RasterView,
    RasterViewBytes, RasterViewPacked, is,
};

/* definitions */

#[doc = crate::_tags!(image lifetime)]
/// Borrowed dense raster view over typed samples.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This is the concrete borrowed form of [`RasterView`].
///
/// It carries a [`RasterFormat`], a [`RasterLayout`], and a shared sample slice.
/// The typed view is accepted only when the layout is dense and the stored pixel
/// width matches the sample type.
///
/// For raw backend-facing bytes, use [`RasterBytesRef`].
#[derive(Clone, Copy, Debug)]
pub struct RasterRef<'a, T> {
    format: RasterFormat,
    layout: RasterLayout,
    samples: &'a [T],
}

#[doc = crate::_tags!(image lifetime)]
/// Borrowed mutable dense raster view over typed samples.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This is the concrete borrowed form of [`RasterBuf`].
///
/// It gives exclusive access to dense row-major sample storage without implying
/// ownership or resizing.
#[derive(Debug)]
pub struct RasterMut<'a, T> {
    format: RasterFormat,
    layout: RasterLayout,
    samples: &'a mut [T],
}

#[doc = crate::_tags!(image lifetime)]
/// Borrowed byte raster view with explicit row layout.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This is the concrete borrowed form of [`RasterViewBytes`].
///
/// It is the safe byte-first bridge for codecs, presentation backends, and
/// foreign image surfaces. The layout may include row padding.
#[derive(Clone, Copy, Debug)]
pub struct RasterBytesRef<'a> {
    format: RasterFormat,
    layout: RasterLayout,
    bytes: &'a [u8],
}

#[doc = crate::_tags!(image lifetime)]
/// Borrowed mutable byte raster view with explicit row layout.
#[doc = crate::_doc_location!("media/visual/image")]
///
/// This is the concrete borrowed form of [`RasterBufBytes`].
///
/// It gives exclusive access to backend-native byte storage without implying
/// ownership or resizing.
#[derive(Debug)]
pub struct RasterBytesMut<'a> {
    format: RasterFormat,
    layout: RasterLayout,
    bytes: &'a mut [u8],
}

/* impl methods */

#[rustfmt::skip]
impl<'a, T> RasterRef<'a, T> {
    /// Creates a borrowed typed raster view from an explicit dense layout.
    ///
    /// Returns `None` if the layout is not dense, if the stored pixel width does
    /// not match `T`, or if the slice is too short for the layout.
    pub const fn new(format: RasterFormat, layout: RasterLayout, samples: &'a [T]) -> Option<Self> {
        is! { !layout.is_dense(), return None }
        is! { layout.bytes_per_pixel as usize != size_of::<T>(), return None }
        let Some(min_len) = layout.min_len_bytes() else { return None; };
        let needed = min_len / size_of::<T>();
        is! { samples.len() < needed, return None }
        Some(Self { format, layout, samples })
    }
    /// Creates a borrowed typed raster view from an explicit layout without checking invariants.
    ///
    /// This is semantically unchecked but memory-safe.
    pub const fn new_unchecked(format: RasterFormat, layout: RasterLayout, samples: &'a [T])
        -> Self { Self { format, layout, samples } }
    /// Creates a borrowed dense typed raster view.
    pub const fn dense(format: RasterFormat, extent: Extent2<u32>, samples: &'a [T])
        -> Option<Self> {
        let Some(layout) = RasterLayout::dense_interleaved(extent, size_of::<T>() as u8)
            else { return None; };
        Self::new(format, layout, samples)
    }
    /// Returns the raster format.
    pub const fn format(&self) -> RasterFormat { self.format }
    /// Returns the raster layout.
    pub const fn layout(&self) -> RasterLayout { self.layout }
    /// Returns the logical extent.
    pub const fn extent(&self) -> Extent2<u32> { self.layout.extent }
    /// Returns the borrowed sample slice.
    pub const fn samples(&self) -> &'a [T] { self.samples }
}
#[rustfmt::skip]
impl<'a, T> RasterMut<'a, T> {
    /// Creates a borrowed mutable typed raster view from an explicit dense layout.
    ///
    /// Returns `None` if the layout is not dense, if the stored pixel width does
    /// not match `T`, or if the slice is too short for the layout.
    pub const fn new(format: RasterFormat, layout: RasterLayout, samples: &'a mut [T])
        -> Option<Self> {
        if !layout.is_dense() { return None; }
        if layout.bytes_per_pixel as usize != size_of::<T>() { return None; }
        let Some(min_len) = layout.min_len_bytes() else { return None; };
        let needed = min_len / size_of::<T>();
        if samples.len() < needed { return None; }
        Some(Self { format, layout, samples })
    }
    /// Creates a borrowed mutable typed raster view from an explicit layout without checking invariants.
    ///
    /// This is semantically unchecked but memory-safe.
    pub const fn new_unchecked(format: RasterFormat, layout: RasterLayout, samples: &'a mut [T])
        -> Self { Self { format, layout, samples } }
    /// Creates a borrowed mutable dense typed raster view.
    pub const fn dense(format: RasterFormat, extent: Extent2<u32>, samples: &'a mut [T])
        -> Option<Self> {
        let Some(layout) = RasterLayout::dense_interleaved(extent, size_of::<T>() as u8)
            else { return None; };
        Self::new(format, layout, samples)
    }

    /// Returns the raster format.
    pub const fn format(&self) -> RasterFormat { self.format }
    /// Returns the raster layout.
    pub const fn layout(&self) -> RasterLayout { self.layout }
    /// Returns the logical extent.
    pub const fn extent(&self) -> Extent2<u32> { self.layout.extent }
    /// Returns the borrowed sample slice.
    pub const fn samples(&self) -> &[T] { self.samples }
    /// Returns the exclusively borrowed sample slice.
    pub const fn samples_mut(&mut self) -> &mut [T] { self.samples }
    /// Returns itself as non-mutable.
    pub const fn as_ref(&self) -> RasterRef<'_, T> {
        RasterRef { format: self.format, layout: self.layout, samples: self.samples }
    }
}
#[rustfmt::skip]
impl<'a> RasterBytesRef<'a> {
    /// Creates a borrowed byte raster view from an explicit layout.
    ///
    /// Returns `None` if the layout is not dense, if the stored pixel width does
    /// not match `T`, or if the slice is too short for the layout.
    pub const fn new(format: RasterFormat, layout: RasterLayout, bytes: &'a [u8])
        -> Option<Self> {
        is! { raster_depth_u8(format).is_none(), return None }
        let Some(min_len) = layout.min_len_bytes() else { return None; };
        is! { bytes.len() < min_len, return None }
        Some(Self { format, layout, bytes })
    }
    /// Creates a borrowed byte raster view without checking length.
    ///
    /// This is semantically unchecked but memory-safe.
    pub const fn new_unchecked(format: RasterFormat, layout: RasterLayout, bytes: &'a [u8])
        -> Self { Self { format, layout, bytes } }
    /// Creates a borrowed dense byte raster view.
    pub const fn dense(format: RasterFormat, extent: Extent2<u32>, bytes: &'a [u8])
        -> Option<Self> {
        let Some(bytes_per_pixel) = raster_bytes_per_pixel_u8(format) else { return None; };
        let Some(layout) = RasterLayout::dense_interleaved(extent, bytes_per_pixel) else {
            return None;
        };
        Self::new(format, layout, bytes)
    }

    /// Returns the raster format.
    pub const fn format(&self) -> RasterFormat { self.format }
    /// Returns the raster layout.
    pub const fn layout(&self) -> RasterLayout { self.layout }
    /// Returns the logical extent.
    pub const fn extent(&self) -> Extent2<u32> { self.layout.extent }
    /// Returns the borrowed byte slice.
    pub const fn bytes(&self) -> &'a [u8] { self.bytes }
}
#[rustfmt::skip]
impl<'a> RasterBytesMut<'a> {
    /// Creates a borrowed mutable byte raster view from an explicit layout.
    ///
    /// Returns `None` if the layout is not dense, if the stored pixel width does
    /// not match `T`, or if the slice is too short for the layout.
    pub const fn new(format: RasterFormat, layout: RasterLayout, bytes: &'a mut [u8])
        -> Option<Self> {
        is! { raster_depth_u8(format).is_none(), return None }
        let Some(min_len) = layout.min_len_bytes() else { return None; };
        is! { bytes.len() < min_len, return None }
        Some(Self { format, layout, bytes })
    }
    /// Creates a borrowed mutable byte raster view without checking length.
    ///
    /// This is semantically unchecked but memory-safe.
    pub const fn new_unchecked(format: RasterFormat, layout: RasterLayout, bytes: &'a mut [u8])
        -> Self { Self { format, layout, bytes } }
    /// Creates a borrowed mutable dense byte raster view.
    pub const fn dense(format: RasterFormat, extent: Extent2<u32>, bytes: &'a mut [u8])
        -> Option<Self> {
        let Some(bytes_per_pixel) = raster_bytes_per_pixel_u8(format) else { return None; };
        let Some(layout) = RasterLayout::dense_interleaved(extent, bytes_per_pixel) else {
            return None;
        };
        Self::new(format, layout, bytes)
    }

    /// Returns the raster format.
    pub const fn format(&self) -> RasterFormat { self.format }
    /// Returns the raster layout.
    pub const fn layout(&self) -> RasterLayout { self.layout }
    /// Returns the logical extent.
    pub const fn extent(&self) -> Extent2<u32> { self.layout.extent }
    /// Returns the borrowed byte slice.
    pub fn bytes(&self) -> &[u8] { self.bytes }
    /// Returns the exclusively borrowed byte slice.
    pub fn bytes_mut(&mut self) -> &mut [u8] { self.bytes }
    /// Returns itself as non-mutable.
    pub fn as_ref(&self) -> RasterBytesRef<'_> {
        RasterBytesRef { format: self.format, layout: self.layout, bytes: self.bytes }
    }
}

/* impl traits */

impl<T> RasterView for RasterRef<'_, T> {
    type Sample = T;
    fn raster_extent(&self) -> Extent2<u32> {
        self.layout.extent
    }
    fn raster_samples(&self) -> &[T] {
        self.samples
    }
}
impl<T> RasterView for RasterMut<'_, T> {
    type Sample = T;
    fn raster_extent(&self) -> Extent2<u32> {
        self.layout.extent
    }
    fn raster_samples(&self) -> &[T] {
        self.samples
    }
}
impl<T> RasterBuf for RasterMut<'_, T> {
    fn raster_samples_mut(&mut self) -> &mut [T] {
        self.samples
    }
}
impl RasterViewBytes for RasterBytesRef<'_> {
    fn raster_extent_bytes(&self) -> Extent2<u32> {
        self.layout.extent
    }
    fn raster_depth(&self) -> u8 {
        raster_depth_u8(self.format).expect("Raster format must have a valid u8 depth")
    }
    fn raster_bytes(&self) -> &[u8] {
        self.bytes
    }
    fn raster_bytes_per_line(&self) -> usize {
        self.layout.bytes_per_line
    }
}
impl RasterViewBytes for RasterBytesMut<'_> {
    fn raster_extent_bytes(&self) -> Extent2<u32> {
        self.layout.extent
    }
    fn raster_depth(&self) -> u8 {
        raster_depth_u8(self.format).expect("Raster format must have a valid u8 depth")
    }
    fn raster_bytes(&self) -> &[u8] {
        self.bytes
    }
    fn raster_bytes_per_line(&self) -> usize {
        self.layout.bytes_per_line
    }
}
impl RasterBufBytes for RasterBytesMut<'_> {
    fn raster_bytes_mut(&mut self) -> &mut [u8] {
        self.bytes
    }
}
impl<T: RasterSamplePacked> RasterViewPacked for RasterRef<'_, T> {
    fn raster_depth(&self) -> u8 {
        raster_depth_u8(self.format).expect("Raster format must have a valid u8 depth")
    }
    fn raster_bytes_per_line(&self) -> usize {
        self.layout.bytes_per_line
    }
}
impl<T: RasterSamplePacked> RasterViewPacked for RasterMut<'_, T> {
    fn raster_depth(&self) -> u8 {
        raster_depth_u8(self.format).expect("Raster format must have a valid u8 depth")
    }
    fn raster_bytes_per_line(&self) -> usize {
        self.layout.bytes_per_line
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
