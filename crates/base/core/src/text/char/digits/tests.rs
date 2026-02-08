// devela_base_core::text::char::digits::tests

use super::Digits;

#[test]
fn count_digits() {
    assert_eq!(Digits(000_u32).count_digits10(), 1);
    assert_eq!(Digits(001_u32).count_digits10(), 1);
    assert_eq!(Digits(009_u32).count_digits10(), 1);
    assert_eq!(Digits(099_u32).count_digits10(), 2);
    assert_eq!(Digits(100_u32).count_digits10(), 3);
    assert_eq!(Digits(999_u32).count_digits10(), 3);

    assert_eq!(Digits(0x00_u32).count_digits16(), 1);
    assert_eq!(Digits(0x01_u32).count_digits16(), 1);
    assert_eq!(Digits(0x0F_u32).count_digits16(), 1);
    assert_eq!(Digits(0x10_u32).count_digits16(), 2);
    assert_eq!(Digits(0xFF_u32).count_digits16(), 2);
}

#[test]
fn digit_at_power10() {
    let d = Digits(123_u32);

    assert_eq!(d.digit_at_power10(1), b'3');
    assert_eq!(d.digit_at_power10(10), b'2');
    assert_eq!(d.digit_at_power10(100), b'1');

    // using a power that overflows the number of digits returns 0:
    assert_eq!(d.digit_at_power10(1_000), b'0');
    assert_eq!(d.digit_at_power10(10_000), b'0');
    assert_eq!(d.digit_at_power10(100_000), b'0');
    assert_eq!(d.digit_at_power10(1_000_000), b'0');

    // not using powers of 10 returns unexpected results:
    assert_eq!(d.digit_at_power10(2), b'1');
    assert_eq!(d.digit_at_power10(3), b'1');
    assert_eq!(d.digit_at_power10(4), b'0');
    assert_eq!(d.digit_at_power10(5), b'4');
    assert_eq!(d.digit_at_power10(6), b'0');
    assert_eq!(d.digit_at_power10(7), b'7');
    assert_eq!(d.digit_at_power10(8), b'5');
    assert_eq!(d.digit_at_power10(9), b'3');
    assert_eq!(d.digit_at_power10(11), b'1');
    assert_eq!(d.digit_at_power10(12), b'0');
}
#[test]
fn digit_at_power16() {
    let d = Digits(0x9AB_u32);

    assert_eq!(d.digit_at_power16(0x1), b'B');
    assert_eq!(d.digit_at_power16(0x10), b'A');
    assert_eq!(d.digit_at_power16(0x100), b'9');

    // using a power that overflows the number of digits returns 0:
    assert_eq!(d.digit_at_power16(0x1_000), b'0');
    assert_eq!(d.digit_at_power16(0x10_000), b'0');
    assert_eq!(d.digit_at_power16(0x100_000), b'0');
    assert_eq!(d.digit_at_power16(0x1_000_000), b'0');

    // not using powers of 16 returns unexpected results:
    assert_eq!(d.digit_at_power16(0x2), b'5');
    assert_eq!(d.digit_at_power16(0x3), b'9');
    assert_eq!(d.digit_at_power16(0x4), b'A');
    assert_eq!(d.digit_at_power16(0x5), b'F');
    assert_eq!(d.digit_at_power16(0x6), b'C');
    assert_eq!(d.digit_at_power16(0x7), b'1');
    assert_eq!(d.digit_at_power16(0x8), b'5');
    assert_eq!(d.digit_at_power16(0x9), b'3');
    assert_eq!(d.digit_at_power16(0xA), b'7');
    assert_eq!(d.digit_at_power16(0xB), b'1');
}
#[test]
fn digit_at_index10() {
    // TODO: digit_at_index10
}
#[test]
fn digit_at_index16() {
    let n = 0xAB_u8;

    assert_eq!(Digits(n).digit_at_index16(0), b'B');
    assert_eq!(Digits(n).digit_at_index16(1), b'A');

    // using an overflowing digit returns 0:
    assert_eq!(Digits(n).digit_at_index16(2), b'0');
    assert_eq!(Digits(n).digit_at_index16(200), b'0');

    // checked version returns None if requesting an implicit leading-0 digit.
    assert_eq!(Digits(n).digit_at_index16_checked(0), Some(b'B'));
    assert_eq!(Digits(n).digit_at_index16_checked(1), Some(b'A'));
    assert_eq!(Digits(n).digit_at_index16_checked(2), None);
}

#[test]
fn test_write_digits10_u64() {
    let mut buf = [0u8; 32];

    // Test edge cases
    assert_eq!(Digits(0u64).write_digits10(&mut buf, 0), 1);
    assert_eq!(&buf[0..1], b"0");

    assert_eq!(Digits(1u64).write_digits10(&mut buf, 0), 1);
    assert_eq!(&buf[0..1], b"1");

    assert_eq!(Digits(9u64).write_digits10(&mut buf, 0), 1);
    assert_eq!(&buf[0..1], b"9");

    // Test 2-digit numbers
    assert_eq!(Digits(10u64).write_digits10(&mut buf, 0), 2);
    assert_eq!(&buf[0..2], b"10");

    assert_eq!(Digits(99u64).write_digits10(&mut buf, 0), 2);
    assert_eq!(&buf[0..2], b"99");

    // Test 3-digit numbers
    assert_eq!(Digits(100u64).write_digits10(&mut buf, 0), 3);
    assert_eq!(&buf[0..3], b"100");

    assert_eq!(Digits(999u64).write_digits10(&mut buf, 0), 3);
    assert_eq!(&buf[0..3], b"999");

    // Test powers of 10
    assert_eq!(Digits(1_000u64).write_digits10(&mut buf, 0), 4);
    assert_eq!(&buf[0..4], b"1000");

    assert_eq!(Digits(10_000u64).write_digits10(&mut buf, 0), 5);
    assert_eq!(&buf[0..5], b"10000");

    assert_eq!(Digits(100_000u64).write_digits10(&mut buf, 0), 6);
    assert_eq!(&buf[0..6], b"100000");

    assert_eq!(Digits(1_000_000u64).write_digits10(&mut buf, 0), 7);
    assert_eq!(&buf[0..7], b"1000000");

    // Test large numbers
    assert_eq!(Digits(123_456_789u64).write_digits10(&mut buf, 0), 9);
    assert_eq!(&buf[0..9], b"123456789");

    assert_eq!(Digits(9_876_543_210u64).write_digits10(&mut buf, 0), 10);
    assert_eq!(&buf[0..10], b"9876543210");

    // Test u64::MAX (20 digits)
    assert_eq!(Digits(u64::MAX).write_digits10(&mut buf, 0), 20);
    assert_eq!(&buf[0..20], b"18446744073709551615");

    // Test with offsets
    assert_eq!(Digits(42u64).write_digits10(&mut buf, 5), 2);
    assert_eq!(&buf[5..7], b"42");

    // Test multiple writes
    let mut offset = 0;
    offset += Digits(123u64).write_digits10(&mut buf, offset);
    offset += Digits(456u64).write_digits10(&mut buf, offset);
    offset += Digits(789u64).write_digits10(&mut buf, offset);
    assert_eq!(offset, 9);
    assert_eq!(&buf[0..9], b"123456789");
}

#[test]
fn test_write_digits10_u64_buffer_bounds() {
    let mut buf = [0u8; 20];
    assert_eq!(Digits(100u64).write_digits10(&mut buf[0..2], 0), 0);
    assert_eq!(Digits(100u64).write_digits10(&mut buf[0..3], 0), 3);
    assert_eq!(Digits(100u64).write_digits10(&mut buf[0..4], 0), 3);
}

#[test]
fn test_write_digits10_u64_comprehensive() {
    let mut buf = [0u8; 32];
    // Test all digit lengths
    let test_cases = [
        (1u64, 1, "1"),
        (12u64, 2, "12"),
        (123u64, 3, "123"),
        (1234u64, 4, "1234"),
        (12345u64, 5, "12345"),
        (123456u64, 6, "123456"),
        (1234567u64, 7, "1234567"),
        (12345678u64, 8, "12345678"),
        (123456789u64, 9, "123456789"),
        (1234567890u64, 10, "1234567890"),
        (12345678901u64, 11, "12345678901"),
        (123456789012u64, 12, "123456789012"),
        (1234567890123u64, 13, "1234567890123"),
        (12345678901234u64, 14, "12345678901234"),
        (123456789012345u64, 15, "123456789012345"),
        (1234567890123456u64, 16, "1234567890123456"),
        (12345678901234567u64, 17, "12345678901234567"),
        (123456789012345678u64, 18, "123456789012345678"),
        (1234567890123456789u64, 19, "1234567890123456789"),
        (12345678901234567890u64, 20, "12345678901234567890"),
    ];
    for (num, expected_len, expected_str) in test_cases {
        buf.fill(0);
        let written = Digits(num).write_digits10(&mut buf, 0);
        assert_eq!(written, expected_len, "Failed for number: {}", num);
        assert_eq!(&buf[0..written], expected_str.as_bytes(), "Failed for number: {}", num);
    }
}
