// devela::data::codec::encode::traits
//
//! Define `Encodable`, `EncodableLen`.
//

use crate::{CodecLen, IoRead, IoResult, IoWrite};

/// A type that can be decoded from an I/O reader.
///
/// See also [`Encodable`].
pub trait Decodable<R: IoRead>: Sized {
    /// The type produced when decoding.
    type Output;

    /// Decodes `Self` from the given `reader`, returning the decoded value.
    ///
    /// # Errors
    /// Returns an [`IoError`][crate::IoError] if decoding fails.
    fn decode(reader: &mut R) -> IoResult<Self::Output>;
}

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
/// [`Encodable`] with [`CodecLen`].
///
/// See [`CodecLen`] for details on length-based encoding.
pub trait EncodableLen: Encodable<CodecLen> {
    /// Computes the size of `self` when encoded.
    ///
    /// This method simulates encoding without writing any data,
    /// allowing for preallocation or buffer sizing.
    fn encoded_size(&self) -> IoResult<usize> {
        let mut encoder = CodecLen::new();
        self.encode(&mut encoder)?;
        Ok(encoder.size())
    }
}
impl<T: Encodable<CodecLen>> EncodableLen for T {}
