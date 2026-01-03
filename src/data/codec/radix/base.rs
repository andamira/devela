// devela::data::codec::radix
//
//! Radix-based encodings.
//
// TOC
// - Base struct
// - type aliases
// - encoding schemes
// - implementations
// - trait impls
// - helpers
// - tests
//
// IMPROVE: simplify, leave just the LUT option
// RE-DESIGN to have 2 versions: Base & BaseAlloc, like Sort.

use crate::{_TAG_CODEC, ConstInit, PhantomData};

#[doc = _TAG_CODEC!()]
/// A compile-time configurable radix-based encoding scheme.
#[doc = crate::_doc_location!("data/codec")]
///
/// This struct defines the core behavior for various base encodings,
/// including [`Base16`], [`Base32`] and [`Base64`].
///
/// It converts **binary ↔ text** for data transmission/storage.
///
/// It is configurable via const generics to support different encoding
/// behaviors, such as lookup tables, padding, and case insensitivity.
///
/// # Type Parameters
/// - `RADIX`: The numeric radix of the encoding (e.g., 16, 32, 64).
/// - `LUT`: Whether to use a lookup table (`true`) for fast decoding
///   or a linear search (`false`) using less memory.
/// - `PAD`: Whether to use padding (`=`) for encoding output (used in Base32/Base64).
/// - `CASE`: Whether decoding should be case-insensitive (e.g., `true` for Crockford Base32).
/// - `CODE`: A marker type defining the specific encoding scheme (e.g., [`Rfc4648`], [`Crockford`]).
#[derive(Clone, Copy, Debug)]
pub struct Base<
    const RADIX: usize, // The numeric radix (16, 32, 58, 64, 85)
    const LUT: bool,    // Whether to use a lookup table for fast decoding.
    const PAD: bool,    // Whether to use padding for encoding (Base 32/64).
    const CASE: bool,   // Case-insensitive decoding.
    CODE,               // Encoding scheme marker type
> {
    _code: PhantomData<CODE>,
}

/* type aliases */

#[doc = _TAG_CODEC!()]
/// `Base16` standard encoding (hex), with linear search. Case-insensitive.
#[doc = crate::_doc_location!("data/codec")]
///
/// RFC 4648 describes Base16 as "the standard case-insensitive hex encoding".
pub type Base16 = Base<16, false, false, false, Rfc4648>;

#[doc = _TAG_CODEC!()]
/// `Base32` standard encoding, using LUT decoding. Case-sensitive.
#[doc = crate::_doc_location!("data/codec")]
///
/// RFC 4648 specifies Base32 as case-insensitive in decoding,
/// but encoded output is uppercase.
pub type Base32 = Base<32, true, false, false, Rfc4648>;

#[doc = _TAG_CODEC!()]
/// `Base32` encoding with padding (`=`) enabled, using LUT decoding.
#[doc = crate::_doc_location!("data/codec")]
///
/// Padding ensures the encoded length is always a multiple of 8 characters
pub type Base32Padded = Base<32, true, true, false, Rfc4648>;

#[doc = _TAG_CODEC!()]
/// `Base32` `Crockford` encoding. Case-insensitive, remaps `O → 0`, `I/L → 1`.
#[doc = crate::_doc_location!("data/codec")]
///
/// This variant is human-friendly and eliminates ambiguity in certain characters.
pub type Base32Crockford = Base<32, true, false, true, Crockford>;

#[doc = _TAG_CODEC!()]
/// `Base32Hex` uses RFC 4648 hex-encoding (`0-9 A-V` instead of `A-Z 2-7`).
#[doc = crate::_doc_location!("data/codec")]
///
/// Encoded data maintains its sort order when compared bit-wise.
pub type Base32Hex = Base<32, true, false, false, Rfc4648Hex>;

// /// Base58 standard, with LUT decoding.
// pub type Base58 = Base<58, true, false, false, false, Base58Std>;

#[doc = _TAG_CODEC!()]
/// `Base64` standard encoding, using LUT decoding. Case-sensitive.
#[doc = crate::_doc_location!("data/codec")]
///
/// RFC 4648 specifies that Base64 preserves case distinctions in encoding and decoding.
pub type Base64 = Base<64, true, false, false, Rfc4648>;

#[doc = _TAG_CODEC!()]
/// `Base64` encoding with padding (`=`) enabled, using LUT decoding.
#[doc = crate::_doc_location!("data/codec")]
///
/// Ensures encoded output length is always a multiple of 4 characters.
pub type Base64Padded = Base<64, true, true, false, Rfc4648>;

// /// Base85 standard, with LUT decoding.
// pub type Base85 = Base<85, true, false, false, false, Rfc4648>;

/* encoding schemes */

#[doc = _TAG_CODEC!()]
/// The `RFC 4648` standard encoding, used in [`Base16`], [`Base32`], and [`Base64`].
#[doc = crate::_doc_location!("data/codec")]
#[derive(Clone, Copy, Debug)]
pub struct Rfc4648;

#[doc = _TAG_CODEC!()]
/// The `RFC 4648` hexadecimal-variant encoding, used in [`Base32`].
#[doc = crate::_doc_location!("data/codec")]
#[derive(Clone, Copy, Debug)]
pub struct Rfc4648Hex;

#[doc = _TAG_CODEC!()]
/// The `Crockford` [`Base32`] encoding, case-insensitive, remaps certain characters.
#[doc = crate::_doc_location!("data/codec")]
#[derive(Clone, Copy, Debug)]
pub struct Crockford;

// /// The Bitcoin [`Base58`] encoding, which removes easily confused characters (0OIl).
// #[derive(Clone, Copy, Debug)]
// pub struct Base58Std;

// ///
// #[derive(Clone, Copy, Debug)]
// pub struct Ascii85;

/* implementations */

crate::sf! {
    // Base16
    impl<const LUT: bool, const PAD: bool, const CASE: bool, CODE>
        Base<16, LUT, PAD, CASE, CODE> {
        const ALPHABET: [u8; 16] = *b"0123456789ABCDEF";
        const fn remap(byte: u8) -> Option<u8> { Some(byte) } }
    impl<const PAD: bool, const CASE: bool>                  // CODE: Rfc4648
        Base<16, false, PAD, CASE, Rfc4648> {
        methods!(16, false, 4, Self); }                                        // LUT: false
    impl<const PAD: bool, const CASE: bool>
        Base<16, true, PAD, CASE, Rfc4648> {                                   // LUT: true
        build_lut!(16, Self::remap, Self::ALPHABET);
        methods!(16, true, 4, Self); }
    // impl<const PAD: bool, const CASE: bool>                  // CODE: Rfc4648Hex
    //     Base<16, false, PAD, CASE, Rfc4648Hex> { // (duplicated)
    //     methods!(16, false, 4, Self); }                                     // LUT: false
    // impl<const PAD: bool, const CASE: bool>
    //     Base<16, true, PAD, CASE, Rfc4648Hex> {                             // LUT: true
    //     build_lut!(16, Self::remap, Self::ALPHABET);
    //     methods!(16, true, 4, Self); }
    #[cfg(feature = "alloc")]
    impl<const PAD: bool, const CASE: bool>
        Base<16, false, PAD, CASE, Rfc4648> { methods!(@alloc 4, Self); }
    #[cfg(feature = "alloc")]
    impl<const PAD: bool, const CASE: bool>
        Base<16, true, PAD, CASE, Rfc4648> { methods!(@alloc 4, Self); }

    // # Base32
    impl<const LUT: bool, const PAD: bool, const CASE: bool>
        Base<32, LUT, PAD, CASE, Rfc4648> {                                    // CODE: Rfc4648
        const ALPHABET: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";       // -------------
        const fn remap(byte: u8) -> Option<u8> { Some(byte) } }
    impl<const PAD: bool, const CASE: bool>
        Base<32, false, PAD, CASE, Rfc4648> {                                  // LUT:false
        methods!(32, false, 5, Self); }
    impl<const PAD: bool, const CASE: bool>
        Base<32, true, PAD, CASE, Rfc4648> {                                   // LUT:true
        build_lut!(32, Self::remap, Self::ALPHABET);
        methods!(32, true, 5, Self); }
    #[cfg(feature = "alloc")]
    impl<const PAD: bool, const CASE: bool>
        Base<32, false, PAD, CASE, Rfc4648> { methods!(@alloc 5, Self); }
    #[cfg(feature = "alloc")]
    impl<const PAD: bool, const CASE: bool>
        Base<32, true, PAD, CASE, Rfc4648> { methods!(@alloc 5, Self); }

    impl<const LUT: bool, const PAD: bool, const CASE: bool>
        Base<32, LUT, PAD, CASE, Crockford> {                                  // CODE: Crockford
        const ALPHABET: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";       // ---------------
        const fn remap(byte: u8) -> Option<u8> {
            match byte { b'O' => Some(b'0'), b'I' | b'L' => Some(b'1'), _ => Some(byte) } } }
    impl<const PAD: bool, const CASE: bool>
        Base<32, false, PAD, CASE, Crockford> {                                // LUT:false
        methods!(32, false, 5, Self); }
    impl<const PAD: bool, const CASE: bool>
        Base<32, true, PAD, CASE, Crockford> {                                 // LUT:true
        build_lut!(32, Self::remap, Self::ALPHABET);
        methods!(32, true, 5, Self); }
    impl<const LUT: bool, const PAD: bool, const CASE: bool>
        Base<32, LUT, PAD, CASE, Rfc4648Hex> {                                 // CODE: Rfc4648Hex
        const ALPHABET: [u8; 32] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUV";       // ----------------
        const fn remap(byte: u8) -> Option<u8> { Some(byte) } }
    impl<const PAD: bool, const CASE: bool>
        Base<32, false, PAD, CASE, Rfc4648Hex> {                               // LUT:false
        methods!(32, false, 5, Self); }
    impl<const PAD: bool, const CASE: bool>
        Base<32, true, PAD, CASE, Rfc4648Hex> {                                // LUT:true
        build_lut!(32, Self::remap, Self::ALPHABET);
        methods!(32, true, 5, Self); }

    // // Base58
    // #[rustfmt::skip] impl<const LUT: bool, const PAD: bool, const CASE: bool, CODE>
    //     Base<58, LUT, PAD, CASE, CODE> {
    //     const ALPHABET: [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"; }
    // #[rustfmt::skip] impl<const LUT: bool, const PAD: bool, const CASE: bool>
    //     Base<58, LUT, PAD, CASE, Base58> {
    //     const fn remap(byte: u8) -> Option<u8> {
    //         match byte { b'0' | b'O' | b'I' | b'l' => None, _ => Some(byte) } } }
    // #[rustfmt::skip] impl<const PAD: bool, const CASE: bool>
    //     Base<58, true, PAD, CASE, Base58> { build_lut!(58, Self::remap, Self::ALPHABET); }

    // Base64
    impl<const LUT: bool, const PAD: bool, const CASE: bool>
        Base<64, LUT, PAD, CASE, Rfc4648> {                                    // CODE: Rfc4648
        const ALPHABET: [u8; 64] =                                             // -------------
            *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        const fn remap(byte: u8) -> Option<u8> { Some(byte) } }
    impl<const PAD: bool, const CASE: bool>
        Base<64, false, PAD, CASE, Rfc4648> {                                  // LUT:false
        methods!(64, false, 6, Self); }
    impl<const PAD: bool, const CASE: bool>
        Base<64, true, PAD, CASE, Rfc4648> {                                   // LUT:true
        build_lut!(64, Self::remap, Self::ALPHABET);
        methods!(64, true, 6, Self); }
    #[cfg(feature = "alloc")]
    impl<const PAD: bool, const CASE: bool>
        Base<64, false, PAD, CASE, Rfc4648> { methods!(@alloc 6, Self); }
    #[cfg(feature = "alloc")]
    impl<const PAD: bool, const CASE: bool>
        Base<64, true, PAD, CASE, Rfc4648> { methods!(@alloc 6, Self); }

    // // Base85
    // #[rustfmt::skip] impl<const LUT: bool, const PAD: bool, const CASE: bool, CODE>
    //     Base<85, LUT, PAD, CASE, CODE> {
    //     const ALPHABET: [u8; 85] =
    //         *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";
    //     const fn remap(byte: u8) -> Option<u8> { Some(byte) } }
    // #[rustfmt::skip] impl<const PAD: bool, const CASE: bool, CODE>
    //     Base<85, true, PAD, CASE, CODE> { build_lut!(58, Self::remap, Self::ALPHABET); }
}

/// # misc. methods
#[rustfmt::skip]
impl<const RADIX: usize, const LUT: bool, const PAD: bool, const CASE: bool, CODE>
    Base<RADIX, LUT, PAD, CASE, CODE> {
    /// Returns a new `Base` with the chosen generics.
    pub const fn new() -> Self {
        Self { _code: PhantomData }
    }
    /// Change the radix while keeping everything else the same.
    pub const fn with_radix<const NEW_RADIX: usize>(self)
        -> Base<NEW_RADIX, LUT, PAD, CASE, CODE> { Base { _code: PhantomData }
    }
    /// Toggle LUT usage while keeping everything else the same.
    pub const fn with_lut<const NEW_LUT: bool>(self)
        -> Base<RADIX, NEW_LUT, PAD, CASE, CODE> { Base { _code: PhantomData }
    }
    /// Toggle padding while keeping everything else the same.
    pub const fn with_pad<const NEW_PAD: bool>(self)
        -> Base<RADIX, LUT, NEW_PAD, CASE, CODE> { Base { _code: PhantomData }
    }
    /// Change the case sensitivity while keeping everything else the same.
    pub const fn with_case<const NEW_CASE: bool>(self)
        -> Base<RADIX, LUT, PAD, NEW_CASE, CODE> { Base { _code: PhantomData }
    }
    /// Change the encoding scheme while keeping everything else the same.
    pub const fn with_encoding<NewCode>(self)
        -> Base<RADIX, LUT, PAD, CASE, NewCode> { Base { _code: PhantomData }
    }
}

/* trait impls */

#[rustfmt::skip]
impl<
    const RADIX: usize, const LUT: bool, const PAD: bool, const CASE: bool, CODE,
    > ConstInit for Base<RADIX, LUT, PAD, CASE, CODE>
{
    const INIT: Self = Self::new();
}
#[rustfmt::skip]
impl<
    const RADIX: usize, const LUT: bool, const PAD: bool, const CASE: bool, CODE,
    > Default for Base<RADIX, LUT, PAD, CASE, CODE>
{
    fn default() -> Self { Self::new() }
}

/* helpers */

macro_rules! build_lut {
    ($radix:expr, $remap_fn:expr, $alphabet:expr) => {
        const LUT_TABLE: [u8; 256] = {
            let mut table = [255; 256]; // Default: invalid character
            let mut i = 0;
            while i < $radix {
                let base_char = $alphabet[i];
                // Apply remapping inside the LUT construction
                if let Some(mapped) = $remap_fn(base_char) {
                    table[mapped as usize] = i as u8;
                }
                // Store both uppercase and lowercase versions if CASE is true
                table[base_char as usize] = i as u8;
                if CASE {
                    table[base_char.to_ascii_lowercase() as usize] = i as u8;
                }
                i += 1;
            }
            table
        };
    };
}
use build_lut;

macro_rules! methods {
    ( // LUT: True
      $radix:expr, true, $chunk_bits:expr, $Self:ident) => {
        const fn decode_byte(byte: u8) -> Option<u8> {
            let decoded = $Self::LUT_TABLE[byte as usize];
            if decoded == 255 { None } else { Some(decoded) }
        }
        methods![@non_alloc $radix, $chunk_bits, $Self];
    };
    ( // LUT: False
      $radix:expr, false, $chunk_bits:expr, $Self:ident) => {
        const fn decode_byte(byte: u8) -> Option<u8> {
            let mut i = 0;
            while i < $radix {
                let mapped = $Self::remap($Self::ALPHABET[i]);
                if mapped.is_none() {
                    i += 1;
                    continue;
                } // Skip invalid character
                if byte == mapped.unwrap()
                    || (CASE && byte == mapped.unwrap().to_ascii_lowercase()) {
                    return Some(i as u8);
                }
                i += 1;
            }
            None
        }
        methods![@non_alloc $radix, $chunk_bits, $Self];
    };
    (@non_alloc $radix:expr, $chunk_bits:expr, $Self:ident) => {
        /// Returns the required output buffer size for encoding `input_len` bytes.
        pub const fn encoded_len(input_len: usize) -> usize {
            (input_len * 8).div_ceil($chunk_bits)
        }
        /// Returns the required output buffer size for encoding `input_len` bytes.
        pub const fn encoded_len_padded(input_len: usize) -> usize {
            let base_len = (input_len * 8).div_ceil($chunk_bits);
            if PAD {
                base_len.div_ceil(8) * 8 // Round up to nearest multiple of 8
            } else {
                base_len
            }
        }
        /// Returns the required output buffer size for decoding `input_len` Base32 characters.
        pub const fn decoded_len(input_len: usize) -> usize { (input_len * $chunk_bits) / 8 }
        /// Returns the required output buffer size for decoding `input_len` Base32 characters.
        ///
        /// Strips the padding. Otherwise `decoded_len` will calculate for the worst case.
        pub const fn decoded_len_stripped(input: &[u8]) -> usize {
            let mut length = input.len();
            while length > 0 && input[length - 1] == b'=' {
                length -= 1;
            }
            (length * $chunk_bits) / 8
        }

        /* decode / encode */

        /// Decodes `input` into `output`, returning the number of bytes written.
        /// Uses a LUT when `DEC_TABLE = true`, otherwise falls back to linear search.
        ///
        /// Returns `None` if it finds an invalid byte.
        /// # Panics
        /// Panics if `output` is too small.
        pub const fn decode_from_slice(input: &[u8], output: &mut [u8]) -> Option<usize> {
            let (mut buffer, mut bits_left, mut index, mut i): (u32, u8, usize, usize) = (0,0,0,0);
            while i < input.len() {
                let byte = input[i];
                if PAD && byte == b'=' { break; } // Ignore padding
                let Some(value) =  $Self::decode_byte(byte) else { return None };
                buffer = (buffer << $chunk_bits) | value as u32;
                bits_left += $chunk_bits;
                i += 1;
                while bits_left >= 8 {
                    output[index] = (buffer >> (bits_left - 8)) as u8;
                    bits_left -= 8;
                    index += 1;
                }
            }
            Some(index)
        }

        /// Encodes `input` into `output`, returning the number of bytes written.
        /// # Panics
        /// Panics if `output` is too small.
        pub const fn encode_to_slice(input: &[u8], output: &mut [u8]) -> usize {
            let (mut buffer, mut bits_left, mut index, mut i): (u32, u8, usize, usize) = (0,0,0,0);
            while i < input.len() {
                buffer = (buffer << 8) | input[i] as u32;
                bits_left += 8;
                i += 1;
                while bits_left >= $chunk_bits as u8 {
                    let char_index = ((buffer >> (bits_left - $chunk_bits as u8))
                        & ((1 << $chunk_bits) - 1)) as usize;
                    output[index] = $Self::ALPHABET[char_index];
                    bits_left -= $chunk_bits as u8;
                    index += 1;
                }
            }
            if bits_left > 0 {
                let char_index = ((buffer << ($chunk_bits as u8 - bits_left))
                    & ((1 << $chunk_bits) - 1)) as usize;
                output[index] = $Self::ALPHABET[char_index];
                index += 1;
            }
            if PAD {
                while index % 8 != 0 {
                    output[index] = b'=';
                    index += 1;
                }
            }
            index
        }
    };
    (@alloc $chunk_bits:expr, $Self:ident) => {
        /// Decodes `input` into a `Vec<u8>`,
        /// returns `None` if invalid characters are found.
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        pub fn decode(input: &[u8]) -> Option<$crate::Vec<u8>> {
            let mut output = $crate::Vec::with_capacity(Self::decoded_len(input.len()));
            let (mut buffer, mut bits_left) = (0, 0);
            for &byte in input {
                if PAD && byte == b'=' { break; } // Ignore padding
                let value = Self::decode_byte(byte)?;
                buffer = (buffer << $chunk_bits) | value as u32;
                bits_left += $chunk_bits;
                while bits_left >= 8 {
                    output.push((buffer >> (bits_left - 8)) as u8);
                    bits_left -= 8;
                }
            }
            Some(output)
        }
        /// Encodes `input` into a `String`.
        #[cfg(feature = "alloc")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        pub fn encode(input: &[u8]) -> $crate::String {
            let mut output = $crate::String::with_capacity($Self::encoded_len(input.len()));
            let (mut buffer, mut bits_left) = (0, 0);
            for &byte in input {
                buffer = (buffer << 8) | byte as u32;
                bits_left += 8;
                while bits_left >= $chunk_bits {
                    let index = ((buffer >> (bits_left - $chunk_bits))
                        & ((1 << $chunk_bits) - 1)) as usize;
                    output.push($Self::ALPHABET[index as usize] as char);
                    bits_left -= $chunk_bits;
                }
            }
            if bits_left > 0 {
                let index = ((buffer << ($chunk_bits - bits_left))
                    & ((1 << $chunk_bits) - 1)) as usize;
                output.push($Self::ALPHABET[index as usize] as char);
            }
            if PAD {
                while output.len() % (8 * $chunk_bits / 8) != 0 {
                    output.push('=');
                }
            }
            output
        }
    };
}
use methods;

#[cfg(test)]
mod tests_no_std {
    use super::*;

    #[test]
    fn base_reconstructors() {
        let base32 = Base32::new();
        let _base64 = base32.with_radix::<64>().with_pad::<true>();
    }

    /* base-16 */

    #[test]
    fn base16_rfc4648_lut() {
        pub type Base16 = Base<16, true, false, true, Rfc4648>; // LUT = true
        let mut encoded_buf = [0u8; 22];
        let encoded_len = Base16::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, b"68656C6C6F20776F726C64"];

        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base16::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }
    #[test]
    fn base16_rfc4648_nolut() {
        //default
        let mut encoded_buf = [0u8; 22];
        let encoded_len = Base16::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, b"68656C6C6F20776F726C64"];
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base16::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }

    /* base-32 */

    #[test]
    fn base32_rfc4648_lut() {
        let mut encoded_buf = [0u8; 18];
        let encoded_len = Base32::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, b"NBSWY3DPEB3W64TMMQ"];
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base32::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }
    #[test]
    fn base32_rfc4648_nolut() {
        pub type Base32 = Base<32, false, false, false, Rfc4648>;
        let mut encoded_buf = [0u8; 18];
        let encoded_len = Base32::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, b"NBSWY3DPEB3W64TMMQ"];
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base32::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }

    #[test]
    fn base32_rfc4648_padded() {
        pub type Base32 = Base<32, true, true, false, Rfc4648>; // PAD = true
        const ENC_REFERENCE: &[u8] = b"NBSWY3DPEB3W64TMMQ======";
        const DEC_LEN_PAD: usize = Base32::decoded_len_stripped(ENC_REFERENCE);

        let mut encoded_buf = [0u8; Base32::encoded_len_padded(b"hello world".len())];
        let encoded_len = Base32::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, ENC_REFERENCE];

        let mut decoded_buf = [0u8; DEC_LEN_PAD];
        let decoded_len = Base32::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }

    #[test]
    fn base32_rfc4648_case_sensitive() {
        pub type Base32 = Base<32, true, false, false, Rfc4648>; // CASE = false
        let encoded_lower = b"nbswy3dpeb3w64tmmq"; // Lowercase should fail
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base32::decode_from_slice(encoded_lower, &mut decoded_buf);
        assert_eq![decoded_len, None]; // Decoding should fail

        let encoded_lower = b"NBSWY3DPEB3W64TMMQ"; // Uppercase should succeed
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base32::decode_from_slice(encoded_lower, &mut decoded_buf);
        assert_eq![decoded_len, Some(11)];
        let decoded = &decoded_buf[..decoded_len.unwrap()];
        assert_eq![decoded, b"hello world"];
    }
    #[test]
    fn base32_crockford_case_insensitive() {
        pub type Base32 = Base<32, true, false, true, Crockford>; // CASE = true
        let encoded = b"NBSWY3DPEB3W64TMMQ"; // Uppercase encoding
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base32::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];

        let encoded_lower = b"nbswy3dpeb3w64tmmq"; // Lowercase encoding (should decode the same)
        let decoded_len = Base32::decode_from_slice(encoded_lower, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }

    #[test]
    fn base32hex_rfc4648() {
        pub type Base32H = Base<32, true, false, false, Rfc4648Hex>;
        let mut encoded_buf = [0u8; 18];
        let encoded_len = Base32H::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, b"D1IMOR3F41RMUSJCCG"];
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base32H::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }

    /* base-64 */

    #[test]
    fn base64_rfc4648_lut() {
        let mut encoded_buf = [0u8; 18];
        let encoded_len = Base64::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, b"aGVsbG8gd29ybGQ"];
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base64::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
        // assert_eq![core::str::from_utf8(decoded).unwrap(), "hello world"];
    }
    #[test]
    fn base64_rfc4648_nolut() {
        pub type Base64 = Base<64, false, false, false, Rfc4648>;
        let mut encoded_buf = [0u8; 18];
        let encoded_len = Base64::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, b"aGVsbG8gd29ybGQ"];
        let mut decoded_buf = [0u8; 11];
        let decoded_len = Base64::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }

    #[test]
    fn base64_rfc4648_padded() {
        pub type Base64 = Base<64, true, true, false, Rfc4648>; // PAD = true
        const ENC_REFERENCE: &[u8] = b"aGVsbG8gd29ybGQ=";
        const DEC_LEN_PAD: usize = Base64::decoded_len_stripped(ENC_REFERENCE);

        let mut encoded_buf = [0u8; Base64::encoded_len_padded(b"hello world".len())];
        let encoded_len = Base64::encode_to_slice(b"hello world", &mut encoded_buf);
        let encoded = &encoded_buf[..encoded_len];
        assert_eq![encoded, ENC_REFERENCE];

        let mut decoded_buf = [0u8; DEC_LEN_PAD];
        let decoded_len = Base64::decode_from_slice(encoded, &mut decoded_buf).unwrap();
        let decoded = &decoded_buf[..decoded_len];
        assert_eq![decoded, b"hello world"];
    }
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests_alloc {
    use super::*;
    #[test]
    fn base32_rfc4648_lut() {
        let encoded = Base32::encode(b"hello world");
        assert_eq![encoded.as_bytes(), b"NBSWY3DPEB3W64TMMQ"];
        let decoded = Base32::decode(encoded.as_bytes()).unwrap();
        assert_eq![&decoded, b"hello world"];
    }
    #[test]
    fn base32_rfc4648_nolut() {
        pub type Base32 = Base<32, false, false, false, Rfc4648>;
        let encoded = Base32::encode(b"hello world");
        assert_eq![encoded.as_bytes(), b"NBSWY3DPEB3W64TMMQ"];
        let decoded = Base32::decode(encoded.as_bytes()).unwrap();
        assert_eq![&decoded, b"hello world"];
    }
}
