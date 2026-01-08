// devela::phys::wave::shared

#[doc = crate::_tags!(wave)]
/// Distinguishes the role of a component in wavelet analysis.
#[doc = crate::_doc_location!("phys/wave")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WaveletUnitRole {
    /// Represents the base approximation in the wavelet transform,
    /// capturing the coarser features of the data.
    Scaling,
    /// Represents the detail component in the wavelet transform,
    /// capturing finer variations in the data.
    Wavelet,
}

#[doc = crate::_tags!(wave)]
/// A Haar wavelet is a simple, piecewise-constant wavelet.
#[doc = crate::_doc_location!("phys/wave")]
///
/// It is ideal for basic signal decomposition and testing.
///
/// - <https://en.wikipedia.org/wiki/Haar_wavelet>.
#[derive(Debug)]
pub struct WaveletHaar;
