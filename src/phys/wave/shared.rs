// devela::phys::wave::shared

#[doc = crate::_TAG_WAVE!()]
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

#[doc = crate::_TAG_WAVE!()]
/// A Haar wavelet is a simple, piecewise-constant wavelet.
///
/// It is ideal for basic signal decomposition and testing.
///
/// - <https://en.wikipedia.org/wiki/Haar_wavelet>.
#[derive(Debug)]
pub struct WaveletHaar;
