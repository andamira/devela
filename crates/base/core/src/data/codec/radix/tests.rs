// devela_base_core::data::codec::radix::tests

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
