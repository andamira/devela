// devela/src/data/value/value/_test.rs

use super::*;
use crate::ValueKind4;

#[test]
fn value8_immediates() {
    assert_eq!(Value8::BITS, 8);
    assert_eq!(Value8::KIND_BITS, 4);
    assert_eq!(Value8::PAYLOAD_BITS, 4);
    assert_eq!(Value8::PAYLOAD_MAX, 15);
    assert_eq!(Value8::NIL.kind(), ValueKind4::Nil);
    assert_eq!(Value8::NIL.payload(), 0);
    assert_eq!(Value8::FALSE.as_bool(), Some(false));
    assert_eq!(Value8::TRUE.as_bool(), Some(true));
    for value in 0..=Value8::UINT_MAX {
        assert_eq!(Value8::try_from_uint(value).unwrap().as_uint(), Some(value));
    }
    for value in Value8::INT_MIN..=Value8::INT_MAX {
        assert_eq!(Value8::try_from_int(value).unwrap().as_int(), Some(value));
    }
    assert_eq!(Value8::try_from_int(-9), None);
    assert_eq!(Value8::try_from_int(8), None);
    assert_eq!(Value8::try_from_uint(16), None);
}
#[test]
fn value8_immediate_kind_projection() {
    let value = Value8::try_from_int(3).unwrap();
    assert_eq!(value.as_int(), Some(3));
    assert_eq!(value.as_uint(), None);
    assert_eq!(value.as_bool(), None);
    assert_eq!(value.as_char(), None);
}
#[test]
fn value8_canonical_parts_roundtrip() {
    let mut valid = 0;
    for code in 0..16 {
        let kind = ValueKind4::from_code(code).unwrap();
        for payload in 0..=Value8::PAYLOAD_MAX {
            let value = Value8::try_from_parts(kind, payload);
            if let Some(value) = value {
                valid += 1;
                assert_eq!(value.kind(), kind);
                assert_eq!(value.payload(), payload);
                let (kind2, payload2) = value.into_parts();
                assert_eq!((kind2, payload2), (kind, payload));
                assert_eq!(Value8::try_from_parts(kind2, payload2), Some(value));
            }
        }
    }
    // Nil: 1
    // Bool: 2
    // Int + UInt + Char: 3 × 16
    // ten token kinds: 10 × 16
    // Float: 0
    assert_eq!(valid, 211);
}
#[test]
fn chars_by_grade() {
    assert!(Value8::try_from_char('\u{000F}').is_some());
    assert!(Value8::try_from_char('\u{0010}').is_none());
    assert!(Value16::try_from_char('\u{0FFF}').is_some());
    assert!(Value16::try_from_char('\u{1000}').is_none());
    assert_eq!(Value32::try_from_char('\u{10FFFF}').unwrap().as_char(), Some('\u{10FFFF}'));
}
#[test]
fn char_canonical_boundaries() {
    assert!(Value32::try_from_parts(ValueKind4::Char, 0xD7FF).is_some());
    assert!(Value32::try_from_parts(ValueKind4::Char, 0xD800).is_none());
    assert!(Value32::try_from_parts(ValueKind4::Char, 0xDFFF).is_none());
    assert!(Value32::try_from_parts(ValueKind4::Char, 0xE000).is_some());
    assert!(Value32::try_from_parts(ValueKind4::Char, 0x10_FFFF).is_some());
    assert!(Value32::try_from_parts(ValueKind4::Char, 0x11_0000).is_none());
}
#[test]
fn value8_word_roundtrip() {
    let mut admitted = 0;
    for raw in u8::MIN..=u8::MAX {
        let kind = ValueKind4::from_code(raw >> Value8::KIND_SHIFT).unwrap();
        let payload = raw & Value8::PAYLOAD_MASK;
        let expected = Value8::try_from_parts(kind, payload);
        assert_eq!(Value8::try_from_raw(raw).ok(), expected);
        if let Ok(value) = Value8::try_from_raw(raw) {
            admitted += 1;
            assert_eq!(value.raw(), raw);
            assert_eq!(Value8::try_from_raw(value.raw()), Ok(value));
        }
    }
    assert_eq!(admitted, 211);
}
