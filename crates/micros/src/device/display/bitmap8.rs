//
//! Defines [`BitmapPage8`].
//

use crate::{is, unwrap, whilst};

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
        match W.checked_mul(H.div_ceil(Self::PAGE_HEIGHT)) {
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
    pub const fn width(&self) -> usize {
        W
    }
    /// Returns the framebuffer height in pixels.
    #[must_use]
    pub const fn height(&self) -> usize {
        H
    }
    /// Returns the number of 8-pixel pages.
    #[must_use]
    pub const fn page_count(&self) -> usize {
        H.div_ceil(Self::PAGE_HEIGHT)
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
    const fn pixel_location(x: usize, y: usize) -> Option<(usize, u8)> {
        is! { x >= W || y >= H, return None }
        let page = y / Self::PAGE_HEIGHT;
        let bit = y % Self::PAGE_HEIGHT;
        Some((page * W + x, 1 << bit))
    }
    /// Returns the pixel at `(x, y)`, or `None` when outside the framebuffer.
    #[must_use]
    pub const fn get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        let (index, mask) = unwrap![some? Self::pixel_location(x, y)];
        Some(self.bytes[index] & mask != 0)
    }
    /// Sets the pixel at `(x, y)`.
    ///
    /// Returns `false` if the coordinate lies outside the framebuffer.
    pub const fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
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
    pub const fn draw_hline(&mut self, x: usize, y: usize, len: usize, value: bool) {
        is! { y >= H || x >= W, return }
        let end = match x.checked_add(len) {
            Some(end) if end < W => end,
            _ => W,
        };
        whilst! { px in x,..end; {
            self.set_pixel(px, y, value);
        }}
    }
    /// Draws a clipped vertical line.
    pub const fn draw_vline(&mut self, x: usize, y: usize, len: usize, value: bool) {
        is! { x >= W || y >= H, return }
        let end = match y.checked_add(len) {
            Some(end) if end < H => end,
            _ => H,
        };
        whilst! { py in y,..end; {
            self.set_pixel(x, py, value);
        }}
    }

    /// Draws the outline of a clipped rectangle.
    pub const fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        value: bool,
    ) {
        is! { width == 0 || height == 0 || x >= W || y >= H, return }
        let right = match x.checked_add(width - 1) {
            Some(right) if right < W => right,
            _ => W - 1,
        };
        let bottom = match y.checked_add(height - 1) {
            Some(bottom) if bottom < H => bottom,
            _ => H - 1,
        };
        self.draw_hline(x, y, right - x + 1, value);
        is! { bottom != y, self.draw_hline(x, bottom, right - x + 1, value) }
        self.draw_vline(x, y, bottom - y + 1, value);
        is! { right != x, self.draw_vline(right, y, bottom - y + 1, value) }
    }
    /// Fills a clipped rectangle.
    pub const fn fill_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        value: bool,
    ) {
        is! { width == 0 || height == 0 || x >= W || y >= H, return }
        let end = match y.checked_add(height) {
            Some(end) if end < H => end,
            _ => H,
        };
        whilst! { py in y,..end; {
            self.draw_hline(x, py, width, value);
        }}
    }
}
