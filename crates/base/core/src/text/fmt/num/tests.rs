// devela_base_core::text::fmt::num::tests

use super::{FmtNumConf as Conf, FmtNumGroup as Group, FmtNumSign as Sign, *};

/* float */

#[test] #[rustfmt::skip]
fn fmtnum_float() {
    let mut buf = [0u8; 8];

    let f = FmtNum(-1.2f32); assert_eq!(f.measure(1).total(), 4);
    let s = f.as_str_into(&mut buf, 1); assert_eq!(s, "-1.2");

    let f = FmtNum(1.2f32); assert_eq!(f.measure(0).total(), 1);
    let s = f.as_str_into(&mut buf, 0); assert_eq!(s, "1");

    let f = FmtNum(1.2f32); assert_eq!(f.measure(1).total(), 3);
    let s = f.as_str_into(&mut buf, 1); assert_eq!(s, "1.2");

    let f = FmtNum(1.2f32); assert_eq!(f.measure(2).total(), 4);
    let s = f.as_str_into(&mut buf, 2); assert_eq!(s, "1.20");
}

#[test] #[rustfmt::skip]
fn fmtnum_float_conf() {
    let mut buf = [0u8; 8];
    let mut c = Conf::new();

    let f = FmtNum(-1.2f32);
    let s = f.as_str_into_fmt(&mut buf, c); assert_eq!(s, "-1");
    c.set_fract(3);
    let s = f.as_str_into_fmt(&mut buf, c); assert_eq!(s, "-1.200");
    c.set_int(2);
    let s = f.as_str_into_fmt(&mut buf, c); assert_eq!(s, "-01.200");
    c.set_pad_sign(true);
    let s = f.as_str_into_fmt(&mut buf, c); assert_eq!(s, "-1.200");
    c.set_sign(Sign::Never);
    let s = f.as_str_into_fmt(&mut buf, c); assert_eq!(s, "01.200");
}

#[test]
fn fmtnum_float_group_basic() {
    let mut buf = [0u8; 32];
    let g = Group::new().with_left(3, '_').with_right(3, '_');
    let c = Conf::new_float(0, 6);

    let len = FmtNum(1234.56789_f64).write_group(&mut buf, 0, c, g);
    assert_eq!(&buf[..len], b"1_234.567_890");
}

/* integers */

#[test]
fn fmtnum_int_signed() {
    let mut buf = [0u8; 8];

    // unsigned
    let len = FmtNum(0i32).write(&mut buf, 0);
    assert_eq!(&buf[..len], b"0");

    let len = FmtNum(u8::MAX).write(&mut buf, 0);
    assert_eq!(&buf[..len], b"255");

    // signed
    let len = FmtNum(-12_i32).write(&mut buf, 0);
    assert_eq!(&buf[..len], b"-12");

    let len = FmtNum(i8::MIN).write(&mut buf, 0);
    assert_eq!(&buf[..len], b"-128");
}

#[test]
fn fmtnum_int_unsigned() {
    let mut buf = [0u8; 8];
    let len = FmtNum(255u8).write(&mut buf, 0);
    assert_eq!(&buf[..len], b"255");
}

#[test]
fn fmtnum_int_truncation() {
    let mut buf = [0u8; 2];
    let len = FmtNum(1234u32).write(&mut buf, 0);
    // digits not written, since there's not enough space for all four digits
    assert_eq!(len, 0);
    assert_eq!(&buf, b"\0\0");

    // For negative numbers the sign should not be written wither
    let len = FmtNum(-123i32).write(&mut buf, 0);
    assert_eq!(len, 0);
    assert_eq!(&buf, b"\0\0");
}

#[test] #[rustfmt::skip]
fn fmtnum_int_conf() {
    let mut buf = [0u8; 8];
    let mut c = Conf::new();

    let i = FmtNum(-42_i32);
    let s = i.as_str_into_fmt(&mut buf, c); assert_eq!(s, "-42");
    c.set_fract(3);
    let s = i.as_str_into_fmt(&mut buf, c); assert_eq!(s, "-42");
    c.set_int(4);
    let s = i.as_str_into_fmt(&mut buf, c); assert_eq!(s, "-0042");
    c.set_pad_sign(true);
    let s = i.as_str_into_fmt(&mut buf, c); assert_eq!(s, "-042");
    c.set_sign(Sign::Never);
    let s = i.as_str_into_fmt(&mut buf, c); assert_eq!(s, "0042");
}

/* grouping */

#[test]
fn fmtnum_int_group_basic() {
    let mut buf = [0u8; 16];
    let g = Group::new().with_left_len(3).with_left_sep('_');
    let c = Conf::new();

    let len = FmtNum(1234567_i32).write_group(&mut buf, 0, c, g);
    assert_eq!(&buf[..len], b"1_234_567");
}
#[test]
fn fmtnum_int_group_with_padding() {
    let mut buf = [0u8; 16];
    let g = Group::new().with_left_len(3).with_left_sep('_');
    let c = Conf::new().with_int(8);

    let len = FmtNum(42_i32).write_group(&mut buf, 0, c, g);
    // Left block width is 8, grouping-aware.
    assert_eq!(&buf[..len], b"0_000_042");
}
#[test]
fn fmtnum_int_group_negative() {
    let mut buf = [0u8; 16];
    let g = Group::new().with_left_len(3).with_left_sep('_');
    let c = Conf::new();

    let len = FmtNum(-123456_i32).write_group(&mut buf, 0, c, g);
    assert_eq!(&buf[..len], b"-123_456");
}
#[test]
fn fmtnum_int_group_exact_width() {
    let mut buf = [0u8; 16];
    let g = Group::new().with_left_len(3).with_left_sep('_');
    let c = Conf::new();

    let len = FmtNum(999_i32).write_group(&mut buf, 0, c, g);
    assert_eq!(&buf[..len], b"999");
}
#[test]
fn fmtnum_int_group_disabled() {
    let mut buf = [0u8; 16];
    let g = Group::new().with_left_len(3); // no separator
    let c = Conf::new();

    let len = FmtNum(123456_i32).write_group(&mut buf, 0, c, g);
    assert_eq!(&buf[..len], b"123456");
}

#[test]
fn fmtnum_uint_group_basic() {
    let mut buf = [0u8; 16];
    let g = Group::new().with_left_len(3).with_left_sep('_');
    let c = Conf::new();

    let len = FmtNum(1234567u32).write_group(&mut buf, 0, c, g);
    assert_eq!(&buf[..len], b"1_234_567");
}

#[test]
fn fmtnum_uint_group_with_padding() {
    let mut buf = [0u8; 16];
    let g = Group::new().with_left_len(3).with_left_sep('_');
    let c = Conf::new().with_int(8);

    let len = FmtNum(42u32).write_group(&mut buf, 0, c, g);
    assert_eq!(&buf[..len], b"0_000_042");
}
