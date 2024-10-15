// devela::rend::color::trait
//
//!
//

/// Common color trait.
pub trait Color {
    /// The type of the inner color components.
    type Inner: Copy + PartialEq;

    /* methods */

    /// Returns the red luminosity.
    fn color_red(&self) -> Self::Inner;

    /// Returns the green luminosity.
    fn color_green(&self) -> Self::Inner;

    /// Returns the blue luminosity.
    fn color_blue(&self) -> Self::Inner;

    /// Returns the alpha luminosity.
    fn color_alpha(&self) -> Self::Inner;

    /// Returns the overall luminosity.
    //
    // If the color is not in [`Oklab32`] or [`Oklch32`] format,
    // it will be converted to `Oklab32` for the operation.
    fn color_luminosity(&self) -> Self::Inner;

    /// Returns the hue.
    //
    // If the color is not in [`Oklch32`] format
    // it will be converted to it for the operation.
    fn color_hue(&self) -> Self::Inner;

    /* conversions */

    /// Returns the 3 components, without alpha.
    fn color_to_array3(&self) -> [Self::Inner; 3];

    /// Returns the 4 components, with alpha.
    ///
    /// If the specific color type has no alpha the maximum value is returned.
    fn color_to_array4(&self) -> [Self::Inner; 4];
}
