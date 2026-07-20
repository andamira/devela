// devela/src/sys/device/display/x11/ui/surface.rs
//
//! Defines [`XSurfaceUi`].
//

use crate::{FontBitmapWord, RegionS2, XError, XSurfaceFrame, is};
use crate::{UiDensity, UiDrawKind, UiDrawListView, UiRound};

#[doc = crate::_tags!(ui unix)]
/// A graphic-form UI adapter for X11 pixel surfaces.
#[doc = crate::_doc_meta! {
    location("sys/device/display/x11"),
    #[cfg(target_pointer_width = "64")]
    test_size_of(XSurfaceUi = 12|96),
}]
/// Projects backend-neutral UI drawing records directly into an
/// [`XSurfaceFrame`].
///
/// Geometry is converted from logical UI space into pixels using the configured
/// [`UiDensity`]. Text is rasterized with a caller-provided [`FontBitmapWord`].
///
/// The current implementation supports little-endian, 32-bit stored XRGB surfaces.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XSurfaceUi {
    density: UiDensity,
    text_scale: u8,
}

#[rustfmt::skip]
impl XSurfaceUi {
    /// Constructs an X11 surface UI adapter.
    ///
    /// # Panics
    /// Panics if `text_scale` is zero.
    pub const fn new(density: UiDensity, text_scale: u8) -> Self {
        assert!(text_scale != 0, "text scale must be non-zero");
        Self { density, text_scale }
    }

    /// Returns the logical-to-pixel density.
    pub const fn density(&self) -> UiDensity { self.density }

    #[must_use] /// Returns the bitmap-font scale.
    pub const fn text_scale(&self) -> u8 { self.text_scale }
}

impl XSurfaceUi {
    /// Renders a painter-ordered UI draw list into `surface`.
    pub fn render_with<S, T, G>(
        &self,
        draws: &UiDrawListView<'_, S, T>,
        surface: &mut XSurfaceFrame<'_>,
        font: &FontBitmapWord<'_, G>,
        mut resolve: impl FnMut(&S) -> [u8; 4],
    ) -> Result<(), XError>
    where
        T: AsRef<str>,
        G: Copy + Into<u64>,
    {
        let mut pixels = Xrgb8888Surface::new(surface)?;

        for draw in draws.iter() {
            let rect = self.density.rect_to_px(draw.rect(), UiRound::Nearest);

            match draw.kind() {
                UiDrawKind::RectFill { style } => pixels.fill_rect(rect, resolve(style)),
                UiDrawKind::RectStroke { width, style } => {
                    let width = self.density.lunit_to_px(*width, UiRound::Nearest);
                    render_inner_stroke(&mut pixels, rect, width, resolve(style));
                }
                UiDrawKind::Text { text, style } => {
                    render_text(
                        &mut pixels,
                        rect,
                        text.as_ref(),
                        resolve(style),
                        font,
                        self.text_scale,
                    );
                }
            }
        }

        Ok(())
    }
}

/* private pixel surface */

type PixelRect = RegionS2<i32>;

struct Xrgb8888Surface<'a> {
    width: usize,
    height: usize,
    stride: usize,
    bytes: &'a mut [u8],
}

impl<'a> Xrgb8888Surface<'a> {
    fn new(frame: &'a mut XSurfaceFrame<'_>) -> Result<Self, XError> {
        if cfg!(target_endian = "big") {
            return Err(XError::Other("XSurfaceUi does not yet support big-endian XRGB surfaces"));
        }
        if frame.bits_per_pixel() != 32 {
            return Err(XError::Other("XSurfaceUi currently requires 32 stored bits per pixel"));
        }
        let width = frame.width() as usize;
        let height = frame.height() as usize;
        let stride = frame.bytes_per_line() as usize;
        let required =
            stride.checked_mul(height).ok_or(XError::Other("X11 surface byte length overflow"))?;

        let bytes = frame.bytes_mut();
        is![bytes.len() < required, return Err(XError::Other("X11 surface storage is too short"))];
        Ok(Self {
            width,
            height,
            stride,
            bytes: &mut bytes[..required],
        })
    }

    fn fill_rect(&mut self, rect: PixelRect, rgba: [u8; 4]) {
        is! { rect.w() <= 0 || rect.h() <= 0, return }
        let width = self.width as i32;
        let height = self.height as i32;
        let x0 = rect.x().clamp(0, width);
        let y0 = rect.y().clamp(0, height);
        let x1 = rect.x().saturating_add(rect.w()).clamp(0, width);
        let y1 = rect.y().saturating_add(rect.h()).clamp(0, height);
        is! { x0 >= x1 || y0 >= y1, return }
        for y in y0..y1 {
            for x in x0..x1 {
                self.put_pixel(x, y, rgba);
            }
        }
    }

    fn put_pixel(&mut self, x: i32, y: i32, [r, g, b, a]: [u8; 4]) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height || a == 0 {
            return;
        }
        let offset = y as usize * self.stride + x as usize * 4;
        if a == u8::MAX {
            self.bytes[offset] = b;
            self.bytes[offset + 1] = g;
            self.bytes[offset + 2] = r;
        } else {
            self.bytes[offset] = blend_channel(self.bytes[offset], b, a);
            self.bytes[offset + 1] = blend_channel(self.bytes[offset + 1], g, a);
            self.bytes[offset + 2] = blend_channel(self.bytes[offset + 2], r, a);
        }
        self.bytes[offset + 3] = 0; // Unused X component in XRGB8888
    }
}

fn blend_channel(under: u8, over: u8, alpha: u8) -> u8 {
    let alpha = alpha as u16;
    let inverse = u8::MAX as u16 - alpha;
    ((over as u16 * alpha + under as u16 * inverse + 127) / 255) as u8
}

/* drawing helpers */

fn render_inner_stroke(
    pixels: &mut Xrgb8888Surface<'_>,
    rect: PixelRect,
    width: i32,
    rgba: [u8; 4],
) {
    let (x, y, w, h) = (rect.x(), rect.y(), rect.w(), rect.h());
    is! { width <= 0 || w <= 0 || h <= 0, return }
    let double_width = width.saturating_mul(2);
    if double_width >= w || double_width >= h {
        pixels.fill_rect(rect, rgba);
        return;
    }
    let middle_y = y.saturating_add(width);
    let middle_h = h.saturating_sub(double_width);
    let bottom_y = y.saturating_add(h.saturating_sub(width));
    let right_x = x.saturating_add(w.saturating_sub(width));
    pixels.fill_rect(PixelRect::from_xy_wh(x, y, w, width), rgba);
    pixels.fill_rect(PixelRect::from_xy_wh(x, bottom_y, w, width), rgba);
    pixels.fill_rect(PixelRect::from_xy_wh(x, middle_y, width, middle_h), rgba);
    pixels.fill_rect(PixelRect::from_xy_wh(right_x, middle_y, width, middle_h), rgba);
}

/// Renders one top-aligned, unwrapped bitmap-font text run.
///
/// Text is clipped to both `rect` and the X11 surface. Line breaks terminate
/// the run.
fn render_text<G>(
    pixels: &mut Xrgb8888Surface<'_>,
    rect: PixelRect,
    text: &str,
    rgba: [u8; 4],
    font: &FontBitmapWord<'_, G>,
    scale: u8,
) where
    G: Copy + Into<u64>,
{
    is! { text.is_empty() || rect.w() <= 0 || rect.h() <= 0, return }
    let scale = scale as i32;
    let glyph_width = font.width() as usize;
    let glyph_height = font.height() as usize;
    let advance = (font.advance_x() as i32).saturating_mul(scale);
    let right = rect.x().saturating_add(rect.w());
    let bottom = rect.y().saturating_add(rect.h());
    let mut pen_x = rect.x();
    for ch in text.chars() {
        is! { matches!(ch, '\r' | '\n'), break }
        is! { pen_x >= right, break }
        let glyph = font.glyph(ch).or_else(|| font.glyph('?'));
        if let Some(glyph) = glyph {
            let glyph: u64 = glyph.into();
            for row in 0..glyph_height {
                for col in 0..glyph_width {
                    let bit = row * glyph_width + col;
                    is! { glyph & (1_u64 << bit) == 0, continue }
                    let glyph_x = pen_x.saturating_add((col as i32).saturating_mul(scale));
                    let glyph_y = rect.y().saturating_add((row as i32).saturating_mul(scale));
                    for sy in 0..scale {
                        let y = glyph_y.saturating_add(sy);
                        is! { y < rect.y() || y >= bottom, continue }
                        for sx in 0..scale {
                            let x = glyph_x.saturating_add(sx);
                            is! { x < rect.x() || x >= right, continue }
                            pixels.put_pixel(x, y, rgba);
                        }
                    }
                }
            }
        }
        pen_x = pen_x.saturating_add(advance);
    }
}
