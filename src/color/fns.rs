// devela::color::fns

#[cfg(any(feature = "std", feature = "libm"))]
use crate::math::FloatExt;
use crate::meta::{iif, paste};

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
        #[cfg(any(feature = "std", feature = "libm"))]
        #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
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
        #[cfg(any(feature = "std", feature = "libm"))]
        #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
        pub fn [<color_gamma_remove_ $t>](c: $t, gamma: $t) -> $t {
            iif![c <= 0.04045; c / 12.92; ((c + 0.055) / (1.055)).powf(gamma)]
        }
    }};
}
color_gamma_fns!(f32, f64);
