//
//! Defines [`BitmapPage8`].
//

#[cfg(feature = "draw")]
use crate::{Canvas, Infallible, Position2, RegionS2};
use crate::{Cmp, Extent2, is, unwrap, whilst};

#[doc = crate::_tags!(hw image)]
/// A 1-bit bitmap packed into horizontal 8-pixel pages.
#[doc = crate::_doc_meta!{
    location("device/display", struct Ssd13xxI2c),
    test_size_of(BitmapPage8<40, 40, 1600> = 1600; niche !Option),
}]
/// Storage proceeds left-to-right within each page, then top-to-bottom
/// across pages. Each byte represents eight vertically adjacent pixels,
/// with bit 0 at the top of the page and bit 7 at the bottom.
///
/// `N` must equal `W * ceil(H / 8)`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitmapPage8<const W: usize, const H: usize, const N: usize> {
    bytes: [u8; N],
}

impl<const W: usize, const H: usize, const N: usize> BitmapPage8<W, H, N> {
    /// Number of vertical pixels stored in one page byte.
    pub const PAGE_HEIGHT: usize = 8;

    /// Returns whether the compile-time storage length matches the geometry.
    #[must_use]
    pub const fn is_valid_layout() -> bool {
        W <= u32::MAX as usize
            && H <= u32::MAX as usize
            && match W.checked_mul(H.div_ceil(Self::PAGE_HEIGHT)) {
                Some(required) => required == N,
                None => false,
            }
    }

    /// Creates an empty framebuffer with every pixel cleared.
    ///
    /// # Panics
    /// Panics if `N` does not match the storage required by `W` and `H`.
    #[must_use]
    pub const fn new() -> Self {
        assert!(Self::is_valid_layout(), "BitmapPage8 storage does not match its geometry");
        Self { bytes: [0; N] }
    }

    /// Returns the framebuffer width in pixels.
    #[must_use]
    pub const fn width(&self) -> u32 {
        W as u32
    }
    /// Returns the framebuffer height in pixels.
    #[must_use]
    pub const fn height(&self) -> u32 {
        H as u32
    }
    /// Returns the logical pixel extent.
    #[must_use]
    pub const fn extent(&self) -> Extent2<u32> {
        Extent2::new([W as u32, H as u32])
    }

    /// Returns the number of 8-pixel pages.
    #[must_use]
    pub const fn page_count(&self) -> usize {
        H.div_ceil(Self::PAGE_HEIGHT)
    }
    /// Returns the page-packed storage length in bytes.
    #[must_use]
    pub const fn byte_len(&self) -> usize {
        N
    }

    /// Returns the page-packed framebuffer bytes.
    #[must_use]
    pub const fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Returns the page-packed framebuffer bytes mutably.
    #[must_use]
    pub const fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

/// # Drawing
impl<const W: usize, const H: usize, const N: usize> BitmapPage8<W, H, N> {
    const fn pixel_location(x: u32, y: u32) -> Option<(usize, u8)> {
        is! { x >= W as u32 || y >= H as u32, return None }
        let (x, y) = (x as usize, y as usize);
        let (page, bit) = (y / Self::PAGE_HEIGHT, y % Self::PAGE_HEIGHT);
        Some((page * W + x, 1 << bit))
    }

    /// Returns the pixel at `(x, y)`, or `None` when outside the framebuffer.
    #[must_use]
    pub const fn get_pixel(&self, x: u32, y: u32) -> Option<bool> {
        let (index, mask) = unwrap![some? Self::pixel_location(x, y)];
        Some(self.bytes[index] & mask != 0)
    }
    /// Sets the pixel at `(x, y)`.
    ///
    /// Returns `false` if the coordinate lies outside the framebuffer.
    pub const fn set_pixel(&mut self, x: u32, y: u32, value: bool) -> bool {
        let (index, mask) = unwrap![some_or Self::pixel_location(x, y), return false];
        is! { value, self.bytes[index] |= mask, self.bytes[index] &= !mask }
        true
    }

    /// Sets every visible pixel to `value`.
    pub const fn clear(&mut self, value: bool) {
        let byte = if value { 0xff } else { 0x00 };
        whilst! { i in 0..N; {
            self.bytes[i] = byte;
        }}
        if value {
            let used = H % Self::PAGE_HEIGHT;
            if used != 0 {
                let mask = (1u8 << used) - 1;
                let last_page = self.page_count() - 1;
                let start = last_page * W;
                whilst! { x in 0..W; {
                    self.bytes[start + x] &= mask;
                }}
            }
        }
    }
    /// Draws a clipped horizontal line.
    pub const fn draw_hline(&mut self, x: u32, y: u32, len: u32, value: bool) {
        is! { y >= self.height() || x >= self.width(), return }
        let end = Cmp(x.saturating_add(len)).min(self.width());
        whilst! { px in x,..end; {
            self.set_pixel(px, y, value);
        }}
    }
    /// Draws a clipped vertical line.
    pub const fn draw_vline(&mut self, x: u32, y: u32, len: u32, value: bool) {
        is! { x >= self.width() || y >= self.height(), return }
        let end = Cmp(y.saturating_add(len)).min(self.height());
        whilst! { py in y,..end; {
            self.set_pixel(x, py, value);
        }}
    }

    /// Draws the outline of a clipped rectangle.
    pub const fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, value: bool) {
        is! { x >= self.width() || y >= self.height() || width == 0 || height == 0, return }
        let right = Cmp(x.saturating_add(width)).min(self.width());
        let bottom = Cmp(y.saturating_add(height)).min(self.height());
        self.draw_hline(x, y, right - x + 1, value);
        is! { bottom != y, self.draw_hline(x, bottom, right - x + 1, value) }
        self.draw_vline(x, y, bottom - y + 1, value);
        is! { right != x, self.draw_vline(right, y, bottom - y + 1, value) }
    }
    /// Fills a clipped rectangle.
    pub const fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, value: bool) {
        is! { x >= self.width() || y >= self.height() || width == 0 || height == 0, return }
        let bottom = Cmp(y.saturating_add(height)).min(self.height());
        whilst! { py in y,..bottom; {
            self.draw_hline(x, py, width, value);
        }}
    }
}

#[cfg(feature = "draw")]
impl<const W: usize, const H: usize, const N: usize> Canvas for BitmapPage8<W, H, N> {
    type Unit = u32;
    type Color = bool;
    type Error = Infallible;

    fn canvas_extent(&self) -> Extent2<u32> {
        self.extent()
    }
    fn canvas_clear(&mut self, color: bool) -> Result<(), Infallible> {
        self.clear(color);
        Ok(())
    }
    fn canvas_set_color(&mut self, pos: Position2<u32>, color: bool) -> Result<(), Infallible> {
        let [x, y] = pos.dim;
        self.set_pixel(x, y, color);
        Ok(())
    }
    fn canvas_fill_region(&mut self, region: RegionS2<u32>, color: bool) -> Result<(), Infallible> {
        self.fill_rect(region.x(), region.y(), region.w(), region.h(), color);
        Ok(())
    }
}
