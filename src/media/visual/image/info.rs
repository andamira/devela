// devela/src/media/visual/image/info.rs
//
//! Defines [`ImageInfo`], [`ImageFrameInfo`], [`ImageFrameSpan`].
//

use crate::{Extent2, RasterFormat};

#[doc = crate::_tags!(image)]
/// Basic metadata for one raster image.
#[doc = crate::_doc_meta!{
    location("media/visual/image"),
    test_size_of(ImageInfo = 12 | 96),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageInfo {
    /// Image extent in pixels.
    pub extent: Extent2<u32>,

    /// Raster sample layout and color interpretation.
    pub format: RasterFormat,
}

#[doc = crate::_tags!(image)]
/// Basic metadata for one image frame.
#[doc = crate::_doc_meta!{
    location("media/visual/image"),
    test_size_of(ImageFrameInfo = 16 | 128),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageFrameInfo {
    /// Image metadata for this frame.
    pub image: ImageInfo,
    /// Zero-based frame index.
    pub index: u32,
}

#[doc = crate::_tags!(image)]
/// Metadata and byte span for one encoded image frame.
#[doc = crate::_doc_meta!{
    location("media/visual/image"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(ImageFrameSpan = 24 | 192),
    #[cfg(target_pointer_width = "64")]
    test_size_of(ImageFrameSpan = 32 | 256),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageFrameSpan {
    /// Frame metadata.
    frame: ImageFrameInfo,
    /// First byte of the encoded frame.
    offset: usize,
    /// Number of bytes occupied by the encoded frame.
    len: usize,
}
#[rustfmt::skip]
impl ImageFrameSpan {
    /* constructors */

    /// Creates a checked encoded-frame span.
    #[must_use]
    pub const fn new(frame: ImageFrameInfo, offset: usize, len: usize) -> Option<Self> {
        if offset.checked_add(len).is_some() { Some(Self { frame, offset, len }) } else { None }
    }
    /// Creates a span from an inclusive start and exclusive end.
    #[must_use]
    pub const fn from_range(frame: ImageFrameInfo, offset: usize, end: usize) -> Option<Self> {
        if end >= offset { Some(Self { frame, offset, len: end - offset }) } else { None }
    }
    /// Creates a span without checking its arithmetic invariant.
    ///
    /// Invalid spans may overflow in [`Self::end`].
    #[must_use]
    #[expect(unused, reason = "WIP")]
    pub(crate) const fn new_unchecked(frame: ImageFrameInfo, offset: usize, len: usize) -> Self {
        Self { frame, offset, len }
    }

    /* queries */

    /// Returns the frame metadata.
    #[must_use]
    pub const fn frame(self) -> ImageFrameInfo { self.frame }
    /// Returns the first byte of the encoded frame.
    #[must_use]
    pub const fn offset(self) -> usize { self.offset }
    /// Returns the encoded frame length in bytes.
    #[must_use]
    pub const fn len(self) -> usize { self.len }
    /// Returns whether the encoded frame is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool { self.len == 0 }
    /// Returns the first byte after this encoded frame.
    #[must_use]
    pub const fn end(self) -> usize { self.offset + self.len }
    /// Returns the first byte after this encoded frame, if representable.
    #[must_use]
    pub const fn checked_end(self) -> Option<usize> { self.offset.checked_add(self.len) }
}
