// devela::media::color::gamma
//
//! Defines [`Gamma`].
//

#[cfg(any(feature = "std", _float··))]
use crate::is;
#[cfg(_float··)]
#[allow(unused_imports)]
use crate::{ExtFloat, Float};

/// Gamma correction curves.
///
/// Used for encoding and decoding linear luminance or tristimulus values
/// via power-law transformations (e.g. $v^γ$ and $v^{(1/γ)}$).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Gamma<T> {
    /// The gamma exponent (`γ`) used in the encoding/decoding transform.
    pub exp: T,
}
impl<T> Gamma<T> {
    /// Constructs a new gamma curve with the given exponent.
    pub const fn new(gamma: T) -> Self {
        Self { exp: gamma }
    }
}

// Implement gamma-related low-level methods. Directly operate over floating-points.
macro_rules! impl_gamma {
    () => { impl_gamma![gamma f32:"_float_f32", f64:"_float_f64"]; };
    ( gamma $( $T:ty : $bound:literal ),+) => { $( impl_gamma![@gamma $T:$bound]; )+ };
    (@gamma    $T:ty : $bound:literal  ) => {
        /// Gamma encoding, decoding, and sRGB transfer functions for floating-point values.
        impl Gamma<$T> {
            /* basic gamma */

            /// Encodes the given linear `v`alue using this gamma: $v^{(1/γ)}$.
            ///
            /// Performs basic gamma encoding (power-law).
            #[cfg(any(feature = "std", feature = $bound))]
            #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = $bound))))]
            pub fn encode(self, v: $T) -> $T { v.powf(self.exp.recip()) }
            /// Compile-time [`encode`](#method.encode).
            #[cfg(feature = $bound)]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $bound)))]
            pub const fn const_encode(self, v: $T) -> $T {
                let terms = Float(v).exp_series_terms();
                Float(v).powf_series(self.exp.recip(), terms).0
            }

            /// Decodes the given gamma-encoded `v`alue using this gamma: $v^γ$ .
            ///
            /// Performs basic gamma decoding (power-law inverse).
            #[cfg(any(feature = "std", feature = $bound))]
            #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = $bound))))]
            pub fn decode(self, v: $T) -> $T { v.powf(self.exp) }
            /// Compile-time [`decode`](#method.decode).
            #[cfg(feature = $bound)]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $bound)))]
            pub const fn const_decode(self, v: $T) -> $T {
                let terms = Float(v).exp_series_terms();
                Float(v).powf_series(self.exp, terms).0
            }

            /* srgb gamma */

            /// Encodes the given `v`alue using the sRGB transfer function.
            ///
            /// Applies a piecewise curve based on this gamma (typically 2.4).
            ///
            /// # Algorithm
            /// $$
            /// f_\text{encode}(c) = \begin{cases}
            /// 12.92c, & \text{if } c <= 0.0031308 \cr
            /// 1.055c^{1/\gamma} - 0.055, & \text{if } c > 0.0031308
            /// \end{cases}
            /// $$
            #[cfg(any(feature = "std", feature = $bound))]
            #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = $bound))))]
            pub fn encode_srgb(self, v: $T) -> $T {
                is![v <= 0.003_130_8; 12.92 * v; 1.055 * v.powf(self.exp.recip()) - 0.055]
            }
            /// Compile-time [`encode_srgb`](#method.encode_srgb).
            #[cfg(feature = $bound)]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $bound)))]
            pub const fn const_encode_srgb(self, v: $T) -> $T {
                if v <= 0.003_130_8 { 12.92 * v }
                else {
                    let terms = Float(v).exp_series_terms();
                    1.055 * Float(v).powf_series(self.exp.recip(), terms).0 - 0.055
                }
            }

            /// Decodes the given `v`alue using the sRGB inverse transfer function.
            ///
            /// Applies the inverse piecewise curve based on this gamma (typically 2.4).
            ///
            /// # Algorithm
            /// $$
            /// \notag f_\text{decode}(c) = \begin{cases}
            /// c / 12.92, & \normalsize\text{if } c <= 0.04045 \cr
            /// \left(\Large\frac{c + 0.055}{1.055}\right)^\gamma
            ///   & \normalsize \text{if } c > 0.04045
            /// \end{cases}
            /// $$
            #[cfg(any(feature = "std", feature = $bound))]
            #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = $bound))))]
            pub fn decode_srgb(self, v: $T) -> $T {
                is![v <= 0.040_45; v / 12.92; ((v + 0.055) / (1.055)).powf(self.exp)]
            }
            #[cfg(feature = $bound)]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $bound)))]
            /// Compile-time [`decode_srgb`](#method.decode_srgb).
            pub const fn const_decode_srgb(self, v: $T) -> $T {
                if v <= 0.040_45 { v / 12.92 } else {
                    let terms = Float(v).exp_series_terms();
                    Float((v + 0.055) / (1.055)).powf_series(self.exp, terms).0
                }
            }
        }
        /// Weighted RGB → luma conversion utilities using standard coefficients.
        impl Gamma<$T> {
            /// R′G′B′ coefficients for computing sRGB luma (same as Rec. 709).
            pub const SRGB: [$T; 3] = Self::REC_709;

            /// Typical gamma value for sRGB/Rec.709 (≈2.2)
            pub const SRGB_GAMMA: $T = 2.2;

            /// Threshold for sRGB linear segment (0.0031308)
            pub const SRGB_LINEAR_THRESHOLD: $T = 0.003_130_8;

            /// R′G′B′ coefficients for computing luma using [Rec. 709].
            ///
            /// [Rec. 709]: https://en.wikipedia.org/wiki/Rec._709
            pub const REC_709: [$T; 3] = [0.212_639, 0.715_169, 0.072_192];

            /// R′G′B′ coefficients for computing luma using [Rec. 601].
            ///
            /// [Rec. 601]: https://en.wikipedia.org/wiki/Rec._601
            pub const REC_601: [$T; 3] = [0.299, 0.587, 0.114];

            /// R′G′B′ coefficients for computing luma using [Rec. 2020].
            ///
            /// [Rec. 2020]: https://en.wikipedia.org/wiki/Rec._2020
            pub const REC_2020: [$T; 3] = [0.2627, 0.6780, 0.0593];

            /// Typical gamma value for Rec.1886 (≈2.4)
            ///
            /// [Rec. 1886]: https://en.wikipedia.org/wiki/ITU-R_BT.1886
            pub const REC_1886_GAMMA: $T = 2.4;

            /// CIE lightness transition point (216/24389 ≈ 0.008856)
            pub const CIE_E: $T = 216.0 / 24_389.0;

            /// CIE lightness linear coefficient (24389/27 ≈ 903.3)
            pub const CIE_K: $T = 24_389.0 / 27.0;

            /* RGB weighted luminance */

            /// Computes luma from R′G′B′ using the given `[kr, kg, kb]` weights.
            pub const fn compute_luma(rgb: [$T; 3], weights: [$T; 3]) -> $T {
                let [r, g, b] = rgb;
                let [kr, kg, kb] = weights;
                kr * r + kg * g + kb * b
            }

            /// Computes luma from R′G′B′ using Rec. 709 coefficients.
            pub const fn luma_srgb(rgb: [$T; 3]) -> $T {
                Self::compute_luma(rgb, Self::SRGB)
            }
            /// Computes luma from R′G′B′ using Rec. 709 coefficients.
            pub const fn luma_rec_709(rgb: [$T; 3]) -> $T {
                Self::compute_luma(rgb, Self::REC_709)
            }
            /// Computes luma from R′G′B′ using Rec. 601 coefficients.
            pub const fn luma_rec_601(rgb: [$T; 3]) -> $T {
                Self::compute_luma(rgb, Self::REC_601)
            }
            /// Computes luma from R′G′B′ using Rec. 2020 coefficients.
            pub const fn luma_rec_2020(rgb: [$T; 3]) -> $T {
                Self::compute_luma(rgb, Self::REC_2020)
            }

            /* ... */

            /// Converts linear luminance to CIE lightness (L*)
            #[cfg(any(feature = "std", feature = $bound))]
            #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = $bound))))]
            pub fn luminance_to_lightness(y: $T) -> $T {
                is![y <= Self::CIE_E; Self::CIE_K * y; 116.0 * y.cbrt() - 16.0]
            }
            /// Compile-time [`luminance_to_lightness`](#method.luminance_to_lightness).
            #[cfg(feature = $bound)]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $bound)))]
            pub const fn const_luminance_to_lightness(y: $T) -> $T {
                is![y <= Self::CIE_E; Self::CIE_K * y; 116.0 * Float(y).cbrt_nr().0 - 16.0]
            }

            /// Converts CIE lightness (L*) to linear luminance
            #[cfg(any(feature = "std", feature = $bound))]
            #[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", feature = $bound))))]
            pub fn lightness_to_luminance(l_star: $T) -> $T {
                is![l_star <= 8.0; l_star / Self::CIE_K; ((l_star + 16.0) / 116.0).powi(3)]
            }
            /// Compile-time [`lightness_to_luminance`](#method.lightness_to_luminance).
            #[cfg(feature = $bound)]
            #[cfg_attr(nightly_doc, doc(cfg(feature = $bound)))]
            pub const fn const_lightness_to_luminance(l_star: $T) -> $T {
                if l_star <= 8.0 { l_star / Self::CIE_K }
                else { Float((l_star + 16.0) / 116.0).const_powi(3).0 }
            }
        }
    };
}
use impl_gamma;
impl_gamma!();
