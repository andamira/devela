// devela/src/data/codec/radix/define.rs
//
//! Defines [`Radix`].
//

#[doc = crate::_tags!(codec)]
/// A configurable radix-based binary-to-text codec.
#[doc = crate::_doc_meta!{
    location("data/codec", struct Radix),
    test_size_of(Radix<16> = 1|8; niche !Option),
}]
/// `Radix<BASE>` groups binary-to-text codecs by numeric base, with associated
/// constants selecting a concrete encoding configuration.
///
/// Operations are allocation-free, use caller-provided output storage, and
/// are available in `const` contexts.
///
/// # Supported codecs
///
/// - `Radix<16>`
///   - `HEX`: RFC 4648 Base16, uppercase output.
///   - `HEX_LOWER`: RFC 4648 Base16, lowercase output.
/// - `Radix<32>`
///   - `STD`, `STD_UNPADDED`: RFC 4648 Base32.
///   - `HEX`, `HEX_UNPADDED`: RFC 4648 Base32hex.
///   - `CROCKFORD`: Crockford Base32 / Base32 for Humans.
/// - `Radix<64>`
///   - `STD`, `STD_UNPADDED`: RFC 4648 Base64.
///   - `URL`, `URL_UNPADDED`: RFC 4648 Base64url.
///
/// # Methods
///
/// Each supported radix provides:
///
/// - `encode_to_slice`: encodes bytes into caller-provided ASCII storage.
/// - `decode_from_slice`: decodes the selected canonical representation.
/// - `decode_array`: decodes into an exact-size byte array.
///
/// Base32 and Base64 also provide:
///
/// - `decode_from_slice_relaxed`: accepts equivalent relaxed input forms.
/// - `decode_array_relaxed`: the exact-size array counterpart.
///
/// # Example
///
/// ```
/// use devela::Radix;
///
/// let mut encoded = [0; 4];
/// let len = Radix::<64>::URL_UNPADDED
///     .encode_to_slice(&[0xfb, 0xff], &mut encoded)
///     .unwrap();
///
/// assert_eq!(&encoded[..len], b"-_8");
///
/// let decoded = Radix::<64>::URL_UNPADDED
///     .decode_array::<2>(&encoded[..len])
///     .unwrap();
///
/// assert_eq!(decoded, [0xfb, 0xff]);
/// ```
///
/// # References
///
/// - [RFC 4648: Base16, Base32, Base32hex, Base64 and Base64url]
/// - [Base32 for Humans], an active specification of Crockford Base32.
///
/// [RFC 4648: Base16, Base32, Base32hex, Base64 and Base64url]:
///     https://www.rfc-editor.org/rfc/rfc4648.html
/// [Base32 for Humans]: https://datatracker.ietf.org/doc/draft-crockford-davis-base32-for-humans/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Radix<const BASE: u8> {
    pub(super) cfg: u8,
}

impl<const BASE: u8> Radix<BASE> {
    /// The numeric base.
    pub const BASE: u8 = BASE;

    pub(super) const fn configured(cfg: u8) -> Self {
        Self { cfg }
    }
}
