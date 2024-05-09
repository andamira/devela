// devela::text::fmt::num_to_str::tests

use super::NumToStr;

#[test]
fn num_to_str_convenience() {
    let mut buffer = [0u8; 20];
    assert_eq!("256123", 256123.to_str_base(10, &mut buffer));
}

#[test]
#[should_panic]
fn num_to_str_base10_u8_array_too_small() {
    let mut buffer = [0u8; 2];
    let _ = 0u8.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base10_u8_array_just_right() {
    let mut buffer = [0u8; 3];
    let _ = 0u8.to_bytes_base(10, &mut buffer);
}

#[test]
#[should_panic]
fn num_to_str_base10_i8_array_too_small() {
    let mut buffer = [0u8; 3];
    let _ = 0i8.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base10_i8_array_just_right() {
    let mut buffer = [0u8; 4];
    assert_eq!((-127i8).to_bytes_base(10, &mut buffer), b"-127");
}

#[test]
#[should_panic]
fn num_to_str_base10_i16_array_too_small() {
    let mut buffer = [0u8; 5];
    let _ = 0i16.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base10_i16_array_just_right() {
    let mut buffer = [0u8; 6];
    assert_eq!((-12768i16).to_bytes_base(10, &mut buffer), b"-12768");
}

#[test]
#[should_panic]
fn num_to_str_base10_u16_array_too_small() {
    let mut buffer = [0u8; 4];
    let _ = 0u16.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base10_u16_array_just_right() {
    let mut buffer = [0u8; 5];
    let _ = 0u16.to_bytes_base(10, &mut buffer);
}

#[test]
#[should_panic]
fn num_to_str_base10_i32_array_too_small() {
    let mut buffer = [0u8; 10];
    let _ = 0i32.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base10_i32_array_just_right() {
    let mut buffer = [0u8; 11];
    let _ = 0i32.to_bytes_base(10, &mut buffer);
}

#[test]
#[should_panic]
fn num_to_str_base10_u32_array_too_small() {
    let mut buffer = [0u8; 9];
    let _ = 0u32.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base10_u32_array_just_right() {
    let mut buffer = [0u8; 10];
    let _ = 0u32.to_bytes_base(10, &mut buffer);
}

#[test]
#[should_panic]
fn num_to_str_base10_i64_array_too_small() {
    let mut buffer = [0u8; 19];
    let _ = 0i64.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base10_i64_array_just_right() {
    let mut buffer = [0u8; 20];
    let _ = 0i64.to_bytes_base(10, &mut buffer);
}

#[test]
#[should_panic]
fn num_to_str_base10_u64_array_too_small() {
    let mut buffer = [0u8; 19];
    let _ = 0u64.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base10_u64_array_just_right() {
    let mut buffer = [0u8; 20];
    let _ = 0u64.to_bytes_base(10, &mut buffer);
}

#[test]
fn num_to_str_base8_min_signed_number() {
    let mut buffer = [0u8; 30];
    assert_eq!((-128i8).to_bytes_base(8, &mut buffer), b"-200");
    assert_eq!((-32768i16).to_bytes_base(8, &mut buffer), b"-100000");
    assert_eq!(
        (-2147483648i32).to_bytes_base(8, &mut buffer),
        b"-20000000000"
    );
    assert_eq!(
        (-9223372036854775808i64).to_bytes_base(8, &mut buffer),
        b"-1000000000000000000000"
    );
}

#[test]
fn num_to_str_base16_min_signed_number() {
    let mut buffer = [0u8; 20];
    assert_eq!((-128i8).to_bytes_base(16, &mut buffer), b"-80");
    assert_eq!((-32768i16).to_bytes_base(16, &mut buffer), b"-8000");
    assert_eq!(
        (-2147483648i32).to_bytes_base(16, &mut buffer),
        b"-80000000"
    );
    assert_eq!(
        (-9223372036854775808i64).to_bytes_base(16, &mut buffer),
        b"-8000000000000000"
    );
}
