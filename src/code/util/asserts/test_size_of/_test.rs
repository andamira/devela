// devela/src/code/util/asserts/test_size_of/_test.rs

use crate::{NonZeroU8, test_size_of};

struct TestSizeOfUnit;

#[allow(dead_code)]
struct TestSizeOfTuple(u8, u16);

type TestSizeOfNonZeroU8 = NonZeroU8;

/* generated tests */

test_size_of![TestSizeOfUnit = 0];
test_size_of![TestSizeOfNonZeroU8 = 1; niche Option];

test_size_of![test_size_of_explicit_ident: TestSizeOfTuple = 4];
test_size_of![test_size_of_explicit_ident_bits: TestSizeOfTuple = 4|32];
test_size_of![test_size_of_array: [u8; 4] = 4];
test_size_of![test_size_of_array_bits: [u8; 4] = 4|32];
test_size_of![test_size_of_option_nonzero_u8: Option<NonZeroU8> = 1];
test_size_of![test_size_of_explicit_niche_option: NonZeroU8 = 1; niche Option];
test_size_of![test_size_of_explicit_niche_not_option: u8 = 1; niche !Option];

/* assertion mode */

#[test]
fn assert_one() {
    test_size_of![assert u8 = 1];
    test_size_of![assert u8 = 1|8];
    test_size_of![assert u16 = 2];
    test_size_of![assert u16 = 2|16];
    test_size_of![assert [u8; 4] = 4];
    test_size_of![assert [u8; 4] = 4|32];
    test_size_of![assert Option<NonZeroU8> = 1];
}
#[test]
fn assert_niche_option() {
    test_size_of![assert NonZeroU8 = 1; niche Option];
    test_size_of![assert NonZeroU8 = 1|8; niche Option];
    test_size_of![assert TestSizeOfNonZeroU8 = 1; niche Option];
}
#[test]
fn assert_niche_not_option() {
    test_size_of![assert u8 = 1; niche !Option];
    test_size_of![assert u8 = 1|8; niche !Option];
}
#[test]
fn assert_batch() {
    test_size_of![assert {
        u8 = 1;
        u16 = 2;
        u32 = 4;
        [u8; 8] = 8;
        Option<NonZeroU8> = 1;
    }];
}
#[test]
fn assert_batch_bits() {
    test_size_of![assert {
        u8 = 1|8;
        u16 = 2|16;
        u32 = 4|32;
        [u8; 8] = 8|64;
        Option<NonZeroU8> = 1|8;
    }];
}
#[test]
fn assert_batch_niche_option() {
    test_size_of![assert {
        NonZeroU8 = 1; niche Option;
        TestSizeOfNonZeroU8 = 1|8; niche Option;
    }];
}
#[test]
fn assert_batch_niche_both() {
    test_size_of![assert {
        u8 = 1|8; niche !Option;
        NonZeroU8 = 1|8; niche Option;
    }];
}
#[test]
#[should_panic(expected = "size_of::<u16>() mismatch")]
fn assert_panics() {
    test_size_of![assert u16 = 1];
}
#[test]
#[should_panic(expected = "size_of::<u16>() bit mismatch")]
fn assert_bit_mismatch_panics() {
    test_size_of![assert u16 = 2|15];
}
#[test]
#[should_panic(expected = "expected: same stack size")]
fn assert_niche_option_panics() {
    test_size_of![assert u8 = 1; niche Option];
}
#[test]
#[should_panic(expected = "expected: different stack sizes")]
fn assert_niche_not_option_panics() {
    test_size_of![assert NonZeroU8 = 1; niche !Option];
}
#[test]
#[should_panic(expected = "size_of::<u16>() = 2 bytes (16 bits)")]
fn probe_panics_with_measured_size() {
    test_size_of![probe u16];
}

/* check_into mode */
#[test]
fn check_into_ok() {
    let mut buf = [0; 128];
    let (ok, ty, msg) = test_size_of![check_into &mut buf; u16 = 2];
    assert!(ok);
    assert_eq!(ty, "u16");
    assert_eq!(msg, "");
}
#[test]
fn check_into_ok_with_bits() {
    let mut buf = [0; 128];
    let (ok, ty, msg) = test_size_of![check_into &mut buf; u16 = 2|16];
    assert!(ok);
    assert_eq!(ty, "u16");
    assert_eq!(msg, "");
}
#[test]
fn check_into_ok_with_niche_option() {
    let mut buf = [0; 128];
    let (ok, ty, msg) = test_size_of![check_into &mut buf; NonZeroU8 = 1|8; niche Option];
    assert!(ok);
    assert!(ty.contains("NonZero"));
    assert_eq!(msg, "");
}
#[test]
fn check_into_byte_mismatch() {
    let mut buf = [0; 256];
    let (ok, ty, msg) = test_size_of![check_into &mut buf; u16 = 1];
    assert!(!ok);
    assert_eq!(ty, "u16");
    assert!(msg.contains("size_of::<u16>() byte mismatch"));
    assert!(msg.contains("actual:   2 bytes (16 bits)"));
    assert!(msg.contains("expected: 1 bytes (8 bits)"));
}
#[test]
fn check_into_bit_mismatch() {
    let mut buf = [0; 256];
    let (ok, ty, msg) = test_size_of![check_into &mut buf; u16 = 2|15];
    assert!(!ok);
    assert_eq!(ty, "u16");
    assert!(msg.contains("size_of::<u16>() bit mismatch"));
    assert!(msg.contains("actual:   16 bits"));
    assert!(msg.contains("expected: 15 bits"));
}
#[test]
fn check_into_ok_with_niche_not_option() {
    let mut buf = [0; 128];
    let (ok, ty, msg) = test_size_of![check_into &mut buf; u8 = 1|8; niche !Option];
    assert!(ok);
    assert_eq!(ty, "u8");
    assert_eq!(msg, "");
}
#[test]
fn check_into_niche_option_mismatch() {
    let mut buf = [0; 256];
    let (ok, ty, msg) = test_size_of![check_into &mut buf; u8 = 1|8; niche Option];
    assert!(!ok);
    assert_eq!(ty, "u8");
    assert!(msg.contains("Option size-preservation mismatch for `u8`"));
    assert!(msg.contains("size_of::<Option<u8>>()"));
    assert!(msg.contains("expected: same stack size"));
}
#[test]
fn check_into_niche_not_option_mismatch() {
    let mut buf = [0; 256];
    let (ok, ty, msg) = test_size_of![check_into &mut buf; NonZeroU8 = 1|8; niche !Option];
    assert!(!ok);
    assert_eq!(ty, core::any::type_name::<NonZeroU8>());
    assert!(msg.contains("Option size-preservation mismatch"));
    assert!(msg.contains("expected: different stack sizes"));
}
#[test]
fn check_into_truncates_diagnostic() {
    let mut buf = [0; 16];
    let buf_len = buf.len();
    let (ok, ty, msg) = test_size_of![check_into &mut buf; u16 = 1];
    assert!(!ok);
    assert_eq!(ty, "u16");
    assert!(!msg.is_empty());
    assert!(msg.len() <= buf_len);
}
/* line_into mode */
#[test]
fn line_into_ok() {
    let mut buf = [0; 128];
    let (ok, line) = test_size_of![line_into &mut buf; u16 = 2|16];
    assert!(ok);
    assert_eq!(line, "ok: size_of::<u16>() = 2 bytes (16 bits)");
}
#[test]
fn line_into_failure() {
    let mut buf = [0; 256];
    let (ok, line) = test_size_of![line_into &mut buf; u16 = 1];
    assert!(!ok);
    assert!(line.contains("size_of::<u16>() byte mismatch"));
    assert!(line.contains("actual:   2 bytes (16 bits)"));
}
#[test]
fn line_into_ok_with_niche_not_option() {
    let mut buf = [0; 128];
    let (ok, line) = test_size_of![line_into &mut buf; u8 = 1|8; niche !Option];
    assert!(ok);
    assert_eq!(line, "ok: size_of::<u8>() = 1 bytes (8 bits)");
}
#[test]
fn line_into_niche_not_option_mismatch() {
    let mut buf = [0; 256];
    let (ok, line) = test_size_of![line_into &mut buf; NonZeroU8 = 1|8; niche !Option];
    assert!(!ok);
    assert!(line.contains("expected: different stack sizes"));
}
