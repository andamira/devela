// devela::data::codec::pack::compress::mode

#[doc = crate::_tags!(codec)]
/// The type of compression applied to data.
#[doc = crate::_doc_location!("data/codec")]
///
/// This enum is used to specify whether the compression algorithm prioritizes
/// retaining all original data or reducing file size, potentially at the cost
/// of data fidelity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CompressionMode {
    /// Compression that retains all original data.
    ///
    /// Commonly used when data accuracy is critical, such as with formats like
    /// PNG, ZIP, or FLAC. All original data can be perfectly restored.
    Lossless,

    /// Compression that sacrifices some data accuracy for reduced size.
    ///
    /// Used in cases where smaller size is more important than retaining all
    /// original data, such as with formats like JPEG, MP3, or MPEG.
    /// Some data is discarded to achieve a more compact representation.
    Lossy,
}
