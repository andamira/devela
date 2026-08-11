// devela/src/data/id/uuid/_test.rs

use super::*;

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
