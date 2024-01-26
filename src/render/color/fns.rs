// devela::render::color::fns
//
//! Standalone color functions and constants.

use crate::code::{iif, paste, sf};
#[allow(unused_imports)]
use crate::num::ExtFloat;

/// The coefficient used for calculating the red luminance.
pub const COLOR_LUMINANCE_RED: f32 = LUMINANCE_RED![];
sf! { macro_rules! LUMINANCE_RED { () => { 0.212639 }; } }
pub(crate) use LUMINANCE_RED;

/// The coefficient used for calculating the green luminance.
pub const COLOR_LUMINANCE_GREEN: f32 = LUMINANCE_GREEN![];
sf! { macro_rules! LUMINANCE_GREEN { () => { 0.715169 }; } }
pub(crate) use LUMINANCE_GREEN;

/// The coefficient used for calculating the blue luminance.
pub const COLOR_LUMINANCE_BLUE: f32 = LUMINANCE_BLUE![];
sf! { macro_rules! LUMINANCE_BLUE { () => { 0.072192 }; } }
pub(crate) use LUMINANCE_BLUE;

macro_rules! color_gamma_fns {
    ($($t:ty),+) => { $( color_gamma_fns![@$t]; )+ };
    (@$t:ty) => { paste! {
        #[doc = "Applies the `gamma` *(γ)* to a linear `" $t "` channel to make it non-linear."]
        ///
        /// # Algorithm
        /// $$ \large
        /// \begin{align}
        /// \notag f_\text{apply}(c) = \begin{cases}
        /// 12.92c,
        ///   & \text{if } c <= 0.0031308 \cr
        /// 1.055c^{1/\large\gamma} - 0.055,
        ///   & \text{if } c > 0.0031308 \end{cases} \cr
        /// \end{align}
        /// $$
        #[inline]
        pub fn [<color_gamma_apply_ $t>](c: $t, gamma: $t) -> $t {
            iif![c <= 0.0031308; 12.92 * c; 1.055 * c.powf(1.0 / gamma) - 0.055]
        }

        #[doc = "Removes the `gamma` *(γ)* from a non-linear `" $t "` channel to make it linear."]
        ///
        /// # Algorithm
        /// $$ \large
        /// \begin{align}
        /// \notag f_\text{remove}(c) = \begin{cases}
        /// \LARGE\frac{c}{12.92} \large,
        ///   & \text{if } c <= 0.04045 \cr
        /// \LARGE\left(\frac{c+0.055}{1.055} \large - 0.055\right)^\gamma,
        ///   & \text{if } c > 0.04045 \end{cases} \cr
        /// \end{align}
        /// $$
        #[inline]
        pub fn [<color_gamma_remove_ $t>](c: $t, gamma: $t) -> $t {
            iif![c <= 0.04045; c / 12.92; ((c + 0.055) / (1.055)).powf(gamma)]
        }
    }};
}
color_gamma_fns!(f32, f64);
