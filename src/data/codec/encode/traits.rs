// devela::data::codec::encode::traits
//
//! Define `Encodable`, `EncodableLen`.
//

use crate::{_TAG_CODEC, CodecLen, IoRead, IoResult, IoWrite};

#[doc = _TAG_CODEC!()]
/// A type that can be decoded from an I/O reader.
#[doc = crate::_doc!(location: "data/codec")]
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

#[doc = _TAG_CODEC!()]
/// A type that can be encoded into an I/O writer.
#[doc = crate::_doc!(location: "data/codec")]
///
/// See also [`Decodable`].
#[doc = crate::_doc!(vendor: "encode")]
pub trait Encodable<W: IoWrite> {
    /// Encodes `self` into the given `writer`, returning the bytes written.
    ///
    /// # Errors
    /// Returns [`IoError`][crate::IoError] if encoding fails.
    fn encode(&self, writer: &mut W) -> IoResult<usize>;
}

#[doc = _TAG_CODEC!()]
/// A type that can compute the size of its encoded form without actual encoding.
#[doc = crate::_doc!(location: "data/codec")]
///
/// This trait is automatically implemented for all types that implement
/// [`Encodable`] with [`CodecLen`].
///
/// See [`CodecLen`] for details on length-based encoding.
#[doc = crate::_doc!(vendor: "encode")]
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
