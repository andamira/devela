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
    /// The type used for color components.
    type Component;

    /// The number of color components.
    ///
    /// For example:
    /// - RGB returns `3`
    /// - Spectral data may return `n`
    const COLOR_COUNT: usize;

    /// Whether the color representation is linear or not.
    const COLOR_IS_LINEAR: bool;

    /// Writes the color components to a pre-allocated `buffer`.
    ///
    /// # Errors
    /// Returns [`NotEnoughSpace`] if the buffer size is less than `COLOR_COUNT`.
    fn color_components_write(&self, buffer: &mut [Self::Component]) -> Result<(), NotEnoughSpace>;

    /* non-required */

    /// Whether the color representation is linear or not.
    fn color_is_linear(&self) -> bool { Self::COLOR_IS_LINEAR }

    /// Returns the number of color components.
    fn color_component_count(&self) -> usize { Self::COLOR_COUNT }

    /// Returns a vector containing the color components.
    #[cfg(feature = "alloc")]
    fn color_components_vec(&self) -> Vec<Self::Component> where Self::Component: Default + Clone {
        let mut buffer = vec![Self::Component::default(); self.color_component_count()];
        self.color_components_write(&mut buffer);
        buffer
    }
}
