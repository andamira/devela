// devela::data::codec::encode::traits
//
//! Define `Encodable`, `EncodableLen`.
//

use crate::{EncodeLen, IoResult, IoWrite};

/// A type that can be encoded into an I/O writer.
///
/// See also [`Decodable`].
pub trait Encodable<W: IoWrite> {
    /// Encodes `self` into the given `writer`, returning the bytes written.
    ///
    /// # Errors
    /// Returns [`IoError`][crate::IoError] if encoding fails.
    fn encode(&self, writer: &mut W) -> IoResult<usize>;
}

/// A type that can compute the size of its encoded form without actual encoding.
///
/// This trait is automatically implemented for all types that implement
/// [`Encodable`] with [`EncodeLen`].
///
/// See [`EncodeLen`] for details on length-based encoding.
pub trait EncodableLen: Encodable<EncodeLen> {
    /// Computes the size of `self` when encoded.
    ///
    /// This method simulates encoding without writing any data,
    /// allowing for preallocation or buffer sizing.
    fn encoded_size(&self) -> IoResult<usize> {
        let mut encoder = EncodeLen::new();
        self.encode(&mut encoder)?;
        Ok(encoder.size())
    }
}
impl<T: Encodable<EncodeLen>> EncodableLen for T {}
