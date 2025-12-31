// devela::data::codec::types

#[doc = crate::_TAG_CODEC!()]
/// The primary mode for data encoding.
///
/// This enum is used to guide encoding/decoding strategies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EncodingMode {
    /// Text-based and intended to be human-readable.
    ///
    /// Often used for formats like TOML, JSON, XML, etc., where readability
    /// and editability are prioritized over storage or transmission efficiency.
    Textual,

    /// Binary-based, optimized for machine processing.
    ///
    /// Typically used in high-performance or low-level contexts where
    /// efficiency, compactness, and speed are more important than readability.
    /// Examples include formats like Protobuf and Bincode.
    Binary,
}

#[doc = crate::_TAG_CODEC!()]
/// The type of compression applied to data.
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
