// devela/src/sys/os/browser/web/ui/canvas.rs
//
//! Defines [`WebCanvasUi`].
//

use crate::{UiDensity, UiDrawKind, UiDrawListView, UiRound, Web, is};

#[doc = crate::_tags!(ui web)]
/// A graphic-form UI adapter for an HTML canvas.
#[doc = crate::_doc_meta! {
    location("sys/os/browser/web"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(WebCanvasUi = 16|128; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(WebCanvasUi = 24|192; niche Option),
}]
///
/// Projects backend-neutral UI draw records into browser canvas operations.
///
/// Geometry is converted from logical UI space into canvas pixels
/// using the configured [`UiDensity`].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WebCanvasUi<'a> {
    selector: &'a str,
    density: UiDensity,
}

#[rustfmt::skip]
impl<'a> WebCanvasUi<'a> {
    /// Constructs an adapter for the canvas matching `selector`.
    pub const fn new(selector: &'a str, density: UiDensity) -> Self {
        Self { selector, density }
    }

    /// Returns the canvas selector.
    pub const fn selector(&self) -> &'a str { self.selector }

    /// Returns the logical-to-pixel density.
    pub const fn density(&self) -> UiDensity { self.density }
}

impl WebCanvasUi<'_> {
    /// Renders a painter-ordered UI draw list using `resolve` for colors.
    pub fn render_with<S, T>(
        &self,
        draws: &UiDrawListView<'_, S, T>,
        mut resolve: impl FnMut(&S) -> [u8; 4],
    ) where
        T: AsRef<str>,
    {
        Web::set_canvas(self.selector);

        // Canvas defaults to an alphabetic baseline, while `UiDraw::Text`
        // positions text from the logical top of its rectangle.
        let mut text_baseline_is_top = false;

        for draw in draws.iter() {
            let rect = self.density.rect_to_px(draw.rect(), UiRound::Nearest);
            match draw.kind() {
                UiDrawKind::RectFill { style } => {
                    render_fill(rect, resolve(style));
                }
                UiDrawKind::RectStroke { width, style } => {
                    let width = self.density.lunit_to_px(*width, UiRound::Nearest);
                    render_inner_stroke(rect, width, resolve(style));
                }
                UiDrawKind::Text { text, style } => {
                    if !text_baseline_is_top {
                        Web::set_text_baseline_top();
                        text_baseline_is_top = true;
                    }
                    render_text(rect, text.as_ref(), resolve(style));
                }
            }
        }
    }
    /// Renders draw records whose styles are RGBA byte arrays.
    pub fn render_rgba<T>(&self, draws: &UiDrawListView<'_, [u8; 4], T>)
    where
        T: AsRef<str>,
    {
        self.render_with(draws, |rgba| *rgba);
    }

    /// Renders draw records whose styles are RGB byte arrays.
    pub fn render_rgb<T>(&self, draws: &UiDrawListView<'_, [u8; 3], T>)
    where
        T: AsRef<str>,
    {
        self.render_with(draws, |&[r, g, b]| [r, g, b, 255]);
    }
}

/* private helpers */

type PixelRect = crate::RegionS2<i32>;

/// Renders a stroke entirely inside `rect`.
///
/// If the stroke consumes either complete dimension, the rectangle is filled.
fn render_inner_stroke(rect: PixelRect, width: i32, rgba: [u8; 4]) {
    let (x, y, w, h) = (rect.x(), rect.y(), rect.w(), rect.h());
    is! { width <= 0 || w <= 0 || h <= 0, return }
    // Once the inner opening has no positive width or height,
    // the stroke occupies the complete rectangle:
    let double_width = width.saturating_mul(2);
    if double_width >= w || double_width >= h {
        render_fill(rect, rgba);
        return;
    }
    Web::set_fill_rgba(rgba);
    let inner_y = y.saturating_add(width);
    let middle_h = h.saturating_sub(double_width);
    let bottom_y = y.saturating_add(h.saturating_sub(width));
    let right_x = x.saturating_add(w.saturating_sub(width));
    // Horizontal edges span the complete width:
    fill_pixel_rect(PixelRect::from_xy_wh(x, y, w, width));
    fill_pixel_rect(PixelRect::from_xy_wh(x, bottom_y, w, width));
    // Vertical edges occupy only the remaining middle section,
    // avoiding redundant drawing over the corners:
    fill_pixel_rect(PixelRect::from_xy_wh(x, inner_y, width, middle_h));
    fill_pixel_rect(PixelRect::from_xy_wh(right_x, inner_y, width, middle_h));
}

/// Renders a filled pixel rectangle.
fn render_fill(rect: PixelRect, rgba: [u8; 4]) {
    is! { rect.w() <= 0 || rect.h() <= 0, return }
    Web::set_fill_rgba(rgba);
    fill_pixel_rect(rect);
}

/// Renders one unwrapped text run from the rectangle's top-start position.
///
/// Text clipping to the rectangle is not yet performed.
fn render_text(rect: PixelRect, text: &str, rgba: [u8; 4]) {
    is! { text.is_empty() || rect.w() <= 0 || rect.h() <= 0, return }
    Web::set_fill_rgba(rgba);
    Web::fill_text(text, f64::from(rect.x()), f64::from(rect.y()));
}

/// Sends one integer pixel rectangle to the native canvas API.
fn fill_pixel_rect(rect: PixelRect) {
    Web::fill_rect(
        f64::from(rect.x()),
        f64::from(rect.y()),
        f64::from(rect.w()),
        f64::from(rect.h()),
    );
}
