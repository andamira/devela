// devela::num::wave
//
//! Wavelets.
//

#[cfg(test)]
mod tests;

#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "_float_f64"))] // NOTE: not documented
crate::items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    mod alloc;
    pub use alloc::*;
}
pub(crate) mod all {
    #[doc(inline)]
    pub use super::*;
}

/// Distinguishes the role of a component in wavelet analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WaveletUnitRole {
    /// Represents the base approximation in the wavelet transform,
    /// capturing the coarser features of the data.
    Scaling,
    /// Represents the detail component in the wavelet transform,
    /// capturing finer variations in the data.
    Wavelet,
}

/// A Haar wavelet is a simple, piecewise-constant wavelet.
///
/// It is ideal for basic signal decomposition and testing.
///
/// - <https://en.wikipedia.org/wiki/Haar_wavelet>.
pub struct WaveletHaar;
