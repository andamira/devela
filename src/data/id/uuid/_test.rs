// devela/src/data/id/uuid/_test.rs

use super::*;
use crate::{Pcg32, TextParseErrorKind, assert_matches, const_assert, format_buf};

#[test]
#[rustfmt::skip]
fn representation() {
    let bytes = [
        0x00, 0x11, 0x22, 0x33,
        0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xAA, 0xBB,
        0xCC, 0xDD, 0xEE, 0xFF,
    ];
    let uuid = Uuid::from_bytes(bytes);
    assert_eq!(uuid.as_bytes(), &bytes);
    assert_eq!(uuid.into_bytes(), bytes);
    let value = 0x00112233445566778899AABBCCDDEEFF_u128;
    assert_eq!(Uuid::from_u128(value), uuid);
    assert_eq!(uuid.as_u128(), value);
}
#[test]
fn special_values() {
    assert!(Uuid::NIL.is_nil());
    assert!(!Uuid::NIL.is_max());
    assert_eq!(Uuid::NIL.variant(), UuidVariant::Ncs);
    assert!(Uuid::MAX.is_max());
    assert!(!Uuid::MAX.is_nil());
    assert_eq!(Uuid::MAX.variant(), UuidVariant::Future);
}
#[test]
fn variants() {
    fn with_variant(byte: u8) -> Uuid {
        let mut bytes = [0; 16];
        bytes[8] = byte;
        Uuid::from_bytes(bytes)
    }
    assert_eq!(with_variant(0x00).variant(), UuidVariant::Ncs);
    assert_eq!(with_variant(0x7F).variant(), UuidVariant::Ncs);
    assert_eq!(with_variant(0x80).variant(), UuidVariant::Ietf);
    assert_eq!(with_variant(0xBF).variant(), UuidVariant::Ietf);
    assert_eq!(with_variant(0xC0).variant(), UuidVariant::Microsoft);
    assert_eq!(with_variant(0xDF).variant(), UuidVariant::Microsoft);
    assert_eq!(with_variant(0xE0).variant(), UuidVariant::Future);
    assert_eq!(with_variant(0xFF).variant(), UuidVariant::Future);
}
#[test]
fn versions() {
    let mut bytes = [0; 16];
    // IETF variant, version 4.
    bytes[6] = 0x40;
    bytes[8] = 0x80;
    let uuid = Uuid::from_bytes(bytes);
    assert_eq!(uuid.version_number(), Some(4));
    assert_eq!(uuid.version(), Some(UuidVersion::V4));
    // Reserved future version.
    bytes[6] = 0x90;
    let uuid = Uuid::from_bytes(bytes);
    assert_eq!(uuid.version_number(), Some(9));
    assert_eq!(uuid.version(), None);
    // Same nibble, but not an IETF UUID.
    bytes[8] = 0x00;
    let uuid = Uuid::from_bytes(bytes);
    assert_eq!(uuid.version_number(), None);
    assert_eq!(uuid.version(), None);
}
#[test]
fn non_nil_construction() {
    assert_eq!(UuidNonNil::from_u128(0), None);
    assert_eq!(UuidNonNil::from_bytes([0; 16]), None);
    assert_eq!(UuidNonNil::from_uuid(Uuid::NIL), None);
    let uuid = UuidNonNil::from_u128(1).unwrap();
    assert_eq!(uuid.as_u128(), 1);
    assert_eq!(uuid.into_bytes(), 1_u128.to_be_bytes());
    assert_eq!(uuid.into_uuid(), Uuid::from_u128(1));
}
#[test]
fn non_nil_max() {
    assert_eq!(UuidNonNil::MAX.as_u128(), u128::MAX);
    assert!(UuidNonNil::MAX.is_max());
    assert_eq!(UuidNonNil::MAX.into_uuid(), Uuid::MAX);
}
#[test]
fn non_nil_niche() {
    assert_eq!(size_of::<UuidNonNil>(), 16);
    assert_eq!(size_of::<Option<UuidNonNil>>(), 16);
}
#[test]
fn non_nil_classification() {
    let mut bytes = [0; 16];
    bytes[6] = 0x70; // V7
    bytes[8] = 0x80; // IETF
    let uuid = UuidNonNil::from_bytes(bytes).unwrap();
    assert_eq!(uuid.variant(), UuidVariant::Ietf);
    assert_eq!(uuid.version_number(), Some(7));
    assert_eq!(uuid.version(), Some(UuidVersion::V7));
}
#[test]
fn text_parse() {
    let expected = Uuid::from_u128(0xf81d4fae_7dec_11d0_a765_00a0c91e6bf6);
    assert_eq!(Uuid::parse_str("f81d4fae-7dec-11d0-a765-00a0c91e6bf6"), Ok(expected));
    assert_eq!(Uuid::parse_str("F81D4fAe-7DEC-11d0-A765-00A0c91E6BF6"), Ok(expected));
    assert_eq!("f81d4fae-7dec-11d0-a765-00a0c91e6bf6".parse::<Uuid>(), Ok(expected));
}
#[test]
fn text_parse_errors() {
    let err = Uuid::parse_str("").unwrap_err();
    assert_matches!(err.kind, TextParseErrorKind::UnexpectedEof);
    assert_eq!(err.at.index.as_usize(), 0);
    let err = Uuid::parse_str("f81d4fae_7dec-11d0-a765-00a0c91e6bf6").unwrap_err();
    assert_matches!(
        err.kind,
        TextParseErrorKind::UnexpectedByte { expected: b'-', found: Some(b'_') }
    );
    assert_eq!(err.at.index.as_usize(), 8);
    let err = Uuid::parse_str("g81d4fae-7dec-11d0-a765-00a0c91e6bf6").unwrap_err();
    assert_matches!(err.kind, TextParseErrorKind::InvalidDigit);
    assert_eq!(err.at.index.as_usize(), 0);
    let err = Uuid::parse_str("f81d4fae-7dec-11d0-a765-00a0c91e6bf6x").unwrap_err();
    assert_matches!(err.kind, TextParseErrorKind::TrailingInput);
    assert_eq!(err.at.index.as_usize(), Uuid::STR_LEN);
}
#[test]
fn text_roundtrip() {
    let uuid = Uuid::from_u128(0xf81d4fae_7dec_11d0_a765_00a0c91e6bf6);
    let mut buf = [0u8; 36];
    let string = format_buf!(&mut buf, "{uuid}").unwrap();
    assert_eq!(Uuid::parse_str(string), Ok(uuid));
}
#[test]
fn text_format() {
    let uuid = Uuid::from_u128(0xf81d4fae_7dec_11d0_a765_00a0c91e6bf6);
    let mut buf = [0; Uuid::STR_LEN];
    assert_eq!(uuid.as_str_into(&mut buf), Some("f81d4fae-7dec-11d0-a765-00a0c91e6bf6"),);
    let mut short = [0; Uuid::STR_LEN - 1];
    assert_eq!(uuid.as_str_into(&mut short), None);
}
// RFC 9562 Appendix A.3
#[test]
#[rustfmt::skip]
fn v4_rfc9562_vector() {
    let random = [
        0x91, 0x91, 0x08, 0xF7,
        0x52, 0xD1, 0x33, 0x20,
        0x5B, 0xAC, 0xF8, 0x47,
        0xDB, 0x41, 0x48, 0xA8,
    ];
    let uuid = Uuid::from_random_v4(random);
    assert_eq!(
        uuid.into_bytes(),
        [
            0x91, 0x91, 0x08, 0xF7,
            0x52, 0xD1, 0x43, 0x20,
            0x9B, 0xAC, 0xF8, 0x47,
            0xDB, 0x41, 0x48, 0xA8,
        ]
    );
    assert_eq!(uuid.variant(), UuidVariant::Ietf);
    assert_eq!(uuid.version(), Some(UuidVersion::V4));
}
// RFC 9562 Appendix A.6
#[test]
#[rustfmt::skip]
fn v7_rfc9562_vector() {
    let random = [
        0x0C, 0xC3,
        0x18, 0xC4, 0xDC, 0x0C,
        0x0C, 0x07, 0x39, 0x8F,
    ];
    let uuid =
        Uuid::from_random_v7(1_645_557_742_000, random).unwrap();
    assert_eq!(
        uuid.into_bytes(),
        [
            0x01, 0x7F, 0x22, 0xE2,
            0x79, 0xB0, 0x7C, 0xC3,
            0x98, 0xC4, 0xDC, 0x0C,
            0x0C, 0x07, 0x39, 0x8F,
        ]
    );
    assert_eq!(uuid.variant(), UuidVariant::Ietf);
    assert_eq!(uuid.version(), Some(UuidVersion::V7));
}
#[test]
fn v7_timestamp_bounds() {
    let random = [0; 10];
    assert!(Uuid::from_random_v7(0xFFFF_FFFF_FFFF, random).is_some());
    assert!(Uuid::from_random_v7(0x1_0000_0000_0000, random).is_none());
}
#[test]
fn v7_timestamp() {
    const TS: u64 = 1_645_557_742_000;
    let uuid = Uuid::from_random_v7(TS, [0; 10]).unwrap();
    assert_eq!(uuid.unix_ts_ms_v7(), Some(TS));
    assert_eq!(Uuid::from_random_v4([0; 16]).unix_ts_ms_v7(), None);
}
#[test]
fn invalid_v7_timestamp_does_not_advance_pcg32() {
    let mut rng = Pcg32::new(1, 2);
    let state = rng.inner_state();
    assert_eq!(Uuid::from_pcg32_v7(0x1_0000_0000_0000, &mut rng), None);
    assert_eq!(rng.inner_state(), state);
}
#[test]
fn pcg32_generation() {
    let mut rng = Pcg32::new(1, 2);
    let v4 = Uuid::from_pcg32_v4(&mut rng);
    assert_eq!(v4.variant(), UuidVariant::Ietf);
    assert_eq!(v4.version(), Some(UuidVersion::V4));
    let v7 = Uuid::from_pcg32_v7(1234, &mut rng).unwrap();
    assert_eq!(v7.variant(), UuidVariant::Ietf);
    assert_eq!(v7.version(), Some(UuidVersion::V7));
    assert_eq!(v7.unix_ts_ms_v7(), Some(1234));
}
#[test]
const fn pcg32_generation_const() {
    const PCG_UUID_V4: Uuid = {
        let mut rng = Pcg32::new(1, 2);
        Uuid::from_pcg32_v4(&mut rng)
    };
    const_assert!(eq PCG_UUID_V4.version().unwrap().number(), UuidVersion::V4.number());
}
