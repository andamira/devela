// devela::media::color::namespace
//
//! Standalone color functions and constants.
//

#[allow(unused_imports)]
use crate::code::{iif, paste, sf, CONST};

#[cfg(all(_float_·, not(feature = "std")))]
#[allow(unused_imports, reason = "!std: powf, powi")]
use crate::num::ExtFloat;

/// Color namespace for constants and methods
pub struct Color;

#[doc = crate::doc_private!()]
///
///
/// # Args
/// $t:   the floating-point primitive
/// $cap: the capability feature that enables the given implementation. E.g "_f32".
macro_rules! color_gamma_fns {
    ($($t:ty : $cap:literal),+) => { $( color_gamma_fns![@$t:$cap]; )+ };
    (@$t:ty : $cap:literal) => { paste! {
        impl Color {
            #[doc = "Applies the `gamma` *(γ)* to a linear `" $t "` channel to make it non-linear."]
            ///
            /// # Algorithm
            /// $$
            /// \begin{align}
            /// \notag f_\text{apply}(c) = \begin{cases}
            /// 12.92c,
            ///   & \text{if } c <= 0.0031308 \cr
            /// 1.055c^{1/\gamma} - 0.055,
            ///   & \text{if } c > 0.0031308 \end{cases} \cr
            /// \end{align}
            /// $$
            #[cfg(any(feature = "std", feature = $cap))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = $cap))))]
            pub fn [<gamma_apply_ $t>](c: $t, gamma: $t) -> $t {
                iif![c <= 0.003_130_8; 12.92 * c; 1.055 * c.powf(1.0 / gamma) - 0.055]
            }

            #[doc = "Removes the `gamma` *(γ)* from a non-linear `" $t "` channel to make it linear."]
            ///
            /// # Algorithm
            /// $$
            /// \begin{align}
            /// \notag f_\text{remove}(c) = \begin{cases}
            /// c / 12.92,
            ///   & \normalsize\text{if } c <= 0.04045 \cr
            /// \left(\Large\frac{c+0.055}{1.055} - \normalsize 0.055\right)^{\gamma},
            ///   & \normalsize \text{if } c > 0.04045 \end{cases} \cr
            /// \end{align}
            /// $$
            #[cfg(any(feature = "std", feature = $cap))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "std", feature = $cap))))]
            pub fn [<gamma_remove_ $t>](c: $t, gamma: $t) -> $t {
                iif![c <= 0.040_45; c / 12.92; ((c + 0.055) / (1.055)).powf(gamma)]
            }
        }
    }};
}
color_gamma_fns![f32:"_float_f32", f64:"_float_f64"];
