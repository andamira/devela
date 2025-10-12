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
