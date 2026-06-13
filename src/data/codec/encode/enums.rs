// devela/src/data/codec/encode/enums.rs

#[doc = crate::_tags!(codec)]
/// The primary mode for data encoding.
#[doc = crate::_doc_meta!{location("data/codec")}]
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
