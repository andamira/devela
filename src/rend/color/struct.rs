// devela::rend::color::fns
//
//! Standalone color functions and constants.
//

#[allow(unused_imports)]
use crate::code::{iif, paste, sf};

#[cfg(all(_float_·, not(feature = "std")))]
#[allow(unused_imports, reason = "!std: powf, powi")]
use crate::num::ExtFloat;

/// color namespace for constants and methods
pub struct COLOR;

sf! { macro_rules! LUMINANCE_RED   { () => { 0.212639 }; } pub(crate) use LUMINANCE_RED;   }
sf! { macro_rules! LUMINANCE_GREEN { () => { 0.715169 }; } pub(crate) use LUMINANCE_GREEN; }
sf! { macro_rules! LUMINANCE_BLUE  { () => { 0.072192 }; } pub(crate) use LUMINANCE_BLUE;  }

impl COLOR {
    /* constants */

    /// The coefficient used for calculating the red luminance.
    pub const LUMINANCE_RED: f32 = LUMINANCE_RED![];

    /// The coefficient used for calculating the green luminance.
    pub const LUMINANCE_GREEN: f32 = LUMINANCE_GREEN![];

    /// The coefficient used for calculating the blue luminance.
    pub const LUMINANCE_BLUE: f32 = LUMINANCE_BLUE![];
}

// $t:   the floating-point primitive
// $cap: the capability feature that enables the given implementation. E.g "_f32".
macro_rules! color_gamma_fns {
    ($($t:ty : $cap:literal),+) => { $( color_gamma_fns![@$t:$cap]; )+ };
    (@$t:ty : $cap:literal) => { paste! {
        impl COLOR {
            #[doc = "Applies the `gamma` *(γ)* to a linear `" $t "` channel to make it non-linear."]
            ///
            /// # Algorithm
            /// $$
            /// \begin{align}
            /// \notag f_\text{apply}(c) = \begin{cases}
            /// 12.92c,
            ///   & \text{if } c <= 0.0031308 \cr
            /// 1.055c^{1/\large\gamma} - 0.055,
            ///   & \text{if } c > 0.0031308 \end{cases} \cr
            /// \end{align}
            /// $$
            #[inline]
            #[cfg(any(feature = "std", feature = $cap))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = $cap))))]
            pub fn [<color_gamma_apply_ $t>](c: $t, gamma: $t) -> $t {
                iif![c <= 0.0031308; 12.92 * c; 1.055 * c.powf(1.0 / gamma) - 0.055]
            }

            #[doc = "Removes the `gamma` *(γ)* from a non-linear `" $t "` channel to make it linear."]
            ///
            /// # Algorithm
            /// $$
            /// \begin{align}
            /// \notag f_\text{remove}(c) = \begin{cases}
            /// \large\frac{c}{12.92}
            ///   & \text{if } c <= 0.04045 \cr
            /// \left(\large\frac{c+0.055}{1.055} - 0.055\right)^\large\gamma,
            ///   & \text{if } c > 0.04045 \end{cases} \cr
            /// \end{align}
            /// $$
            #[inline]
            #[cfg(any(feature = "std", feature = $cap))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = $cap))))]
            pub fn [<color_gamma_remove_ $t>](c: $t, gamma: $t) -> $t {
                iif![c <= 0.04045; c / 12.92; ((c + 0.055) / (1.055)).powf(gamma)]
            }
        }
    }};
}
color_gamma_fns![f32:"_float_f32", f64:"_float_f64"];
