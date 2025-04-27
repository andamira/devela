// devela::media::color::color
//
//! Defines the [`Color`] trait.
//

use crate::NotEnoughSpace;
#[cfg(feature = "alloc")]
use crate::{Vec, vec_ as vec};

/// Base trait for general color data representation.
///
/// Provides a core interface for working with color data across different
/// formats and models, supporting both practical and scientific applications.
#[rustfmt::skip]
pub trait Color {
    /// The type of a single color component (e.g., `u8`, `f32`).
    type Component;

    /// The bit depth of each color component (e.g., `8` for `u8`, `32` for `f32`).
    const COLOR_BITS: usize;

    /// The number of color components (channels) in the representation.
    ///
    /// Examples:
    /// - RGB: `3`
    /// - RGBA: `4`
    /// - Spectral data: Arbitrary `n` (e.g., `31` for 400..=700nm in 10nm steps)
    const COLOR_COUNT: usize;

    /// Whether the color has an alpha component are integer types (e.g., `u8`, `u16`).
    const COLOR_HAS_ALPHA: bool;

    /// Whether the color components are integer types (e.g., `u8`, `u16`).
    ///
    /// If `false`, the components are floating-point (e.g., `f32`, `f64`).
    const COLOR_IS_INT: bool;

    /// Whether the color space is linear (as opposed to non-linear, e.g., sRGB).
    ///
    /// - Linear colors are physically accurate for lighting/blending math.
    /// - Non-linear colors (e.g., sRGB) are gamma-encoded for storage/display.
    const COLOR_IS_LINEAR: bool;

    /// Whether the color uses premultiplied alpha (vs. straight/unassociated alpha).
    ///
    /// - Premultiplied alpha (`true`) means each RGB component is scaled by the alpha value.
    /// - Straight alpha (`false`) means RGB components are independent of alpha.
    ///
    /// Note: Only relevant for alpha-enabled formats (e.g., `COLOR_COUNT > 3`).
    const COLOR_IS_PREMUL: bool;

    /// Writes the color components to a pre-allocated `buffer`.
    ///
    /// # Errors
    /// Returns [`NotEnoughSpace`] if the buffer size is less than `COLOR_COUNT`.
    fn color_components_write(&self, buffer: &mut [Self::Component]) -> Result<(), NotEnoughSpace>;

    /* non-required */

    #[inline(always)]
    /// Returns the bit depth of each color component (e.g., 8 for `u8`, 32 for `f32`).
    fn color_bits(&self) -> usize { Self::COLOR_BITS }

    #[inline(always)]
    /// Returns the number of color components (channels).
    fn color_count(&self) -> usize { Self::COLOR_COUNT }

    #[inline(always)]
    /// Returns `true` if the color has an alpha component.
    fn color_has_alpha(&self) -> bool { Self::COLOR_HAS_ALPHA }

    #[inline(always)]
    /// Returns `true` if the color uses integer components (e.g., `u8`, `u16`).
    fn color_is_int(&self) -> bool { Self::COLOR_IS_INT }

    #[inline(always)]
    /// Returns `true` if the color is in a linear space (not gamma-encoded like sRGB).
    fn color_is_linear(&self) -> bool { Self::COLOR_IS_LINEAR }

    #[inline(always)]
    /// Returns `true` if the color uses premultiplied alpha.
    fn color_is_premul(&self) -> bool { Self::COLOR_IS_PREMUL }

    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    /// Returns a vector containing the color components.
    fn color_components_vec(&self) -> Vec<Self::Component> where Self::Component: Default + Clone {
        let mut buffer = vec![Self::Component::default(); self.color_component_count()];
        self.color_components_write(&mut buffer);
        buffer
    }
}
