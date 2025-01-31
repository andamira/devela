// devela::media::color::base
//
//!
//

#[cfg(feature = "alloc")]
use crate::{vec_ as vec, Vec};

/// Base trait for general color data representation.
///
/// Provides a core interface for working with color data across different
/// formats and models, supporting both practical and scientific applications.
pub trait ColorBase {
    /// The type used for color components.
    type Component;

    /// Returns the number of color components.
    ///
    /// For example:
    /// - RGB returns `3`
    /// - Spectral data may return `n`
    fn color_component_count(&self) -> usize;

    /// Writes the color components to a pre-allocated buffer.
    ///
    /// # Panics
    /// Panics if the buffer size is less than `color_component_count()`.
    // TODO: Return Error
    fn color_components_write(&self, buffer: &mut [Self::Component]);

    /// Returns a vector containing the color components.
    #[cfg(feature = "alloc")]
    fn color_components_vec(&self) -> Vec<Self::Component>
    where
        Self::Component: Default + Clone,
    {
        let mut buffer = vec![Self::Component::default(); self.color_component_count()];
        self.color_components_write(&mut buffer);
        buffer
    }
}
