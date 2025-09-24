// devela_base_core::sys::mem::slice::namespace::tests

use crate::Slice;

#[test]
fn copy_from_slice() {
    // Basic copy
    let mut dst = [0u8; 4];
    Slice::copy_from_slice(&mut dst, &[1, 2, 3, 4]);
    assert_eq!(dst, [1, 2, 3, 4]);
    // Empty slices
    let mut empty_dest = [0u8; 0];
    Slice::<u8>::copy_from_slice(&mut empty_dest, &[]);
    assert_eq!(empty_dest, [0u8; 0]);
    // Const context (compile test)
    const fn _const_test() {
        let mut dst = [0u8; 2];
        Slice::copy_from_slice(&mut dst, &[1, 2]);
    }
}
#[test]
#[should_panic]
fn copy_from_slice_panic() {
    let mut dst = [0u8; 3];
    Slice::copy_from_slice(&mut dst, &[1, 2, 3, 4]); // length mismatch
}
#[test]
fn copy_array_at() {
    // Offset copy
    let mut dst = [0u8; 5];
    Slice::<u8>::copy_array_at(&mut dst, &[1, 2], 2);
    assert_eq!(dst, [0, 0, 1, 2, 0]);
    // Full copy
    let mut dst = [0u8; 3];
    Slice::<u8>::copy_array_at(&mut dst, &[1, 2, 3], 0);
    assert_eq!(dst, [1, 2, 3]);
}
#[test]
fn copied_array_at() {
    // Offset copy
    let result = Slice::<u8>::copied_array_at([0u8; 5], &[1, 2], 2);
    assert_eq!(result, [0, 0, 1, 2, 0]);
    // Full copy
    let result = Slice::<u8>::copied_array_at([0u8; 3], &[1, 2, 3], 0);
    assert_eq!(result, [1, 2, 3]);
    // Edge cases
    assert_eq!(Slice::<u8>::copied_array_at([1u8; 3], &[], 1), [1, 1, 1]);
    assert_eq!(Slice::<u8>::copied_array_at([0u8; 3], &[1, 2], 1), [0, 1, 2]);
    // Const context (compile test)
    const fn _const_test() -> [u8; 2] {
        Slice::<u8>::copied_array_at([0u8; 2], &[1, 2], 0)
    }
}
#[test]
#[should_panic]
fn copied_array_at_panic_overflow() {
    Slice::<u8>::copied_array_at([0u8; 3], &[1, 2, 3, 4], 0); // overflow
}
#[test]
#[should_panic]
fn copied_array_at_panic_overflow_offset() {
    Slice::<u8>::copied_array_at([0u8; 3], &[1, 2], 2); // offset overflow
}

#[test]
fn eq_slices() {
    assert![Slice::<u8>::eq(&[1, 2], &[1, 2])];
    assert![Slice::<&[u8]>::eq(&[&[1, 2], &[3, 4]], &[&[1, 2], &[3, 4]])];

    assert![Slice::<u8>::eq(&[1, 2], &[1, 2])];
    assert![Slice::<&[u8]>::eq(&[&[1, 2], &[3, 4]], &[&[1, 2], &[3, 4]])];

    assert![Slice::<char>::eq(&['a', 'b'], &['a', 'b'])];
    assert![Slice::<&[char]>::eq(&[&['a', 'b'], &['c', 'd']], &[&['a', 'b'], &['c', 'd']])];

    assert![Slice::<&str>::eq(&"ab", "ab")];
    assert![Slice::<&[&str]>::eq(&["ab", "cd"], &["ab", "cd"])];
}

/* byte slices */

#[test]
fn trim_leading() {
    let data = b"0000123";

    assert_eq!(Slice::trim_leading(data, b'0'), b"123");

    assert_eq!(Slice::trim_leading_keep(data, b'0', 2), b"00123");

    assert_eq!(Slice::trim_leading_min_len(b"000", b'0', 1), b"0");
    assert_eq!(Slice::trim_leading_min_len(data, b'0', 0), b"123"); // min
    assert_eq!(Slice::trim_leading_min_len(data, b'0', 5), b"00123");
    assert_eq!(Slice::trim_leading_min_len(data, b'0', 20), b"0000123"); // max
}
#[test]
fn trim_edges() {
    let data = b"000123000";

    // left bias
    assert_eq!(Slice::trim_edges_min_len_left(data, b'0', 4), b"0123");
    assert_eq!(Slice::trim_edges_min_len_left(data, b'0', 5), b"01230");
    assert_eq!(Slice::trim_edges_min_len_left(data, b'0', 6), b"001230");
    assert_eq!(Slice::trim_edges_min_len_left(data, b'0', 7), b"0012300");

    // right bias
    assert_eq!(Slice::trim_edges_min_len_right(data, b'0', 4), b"1230");
    assert_eq!(Slice::trim_edges_min_len_right(data, b'0', 5), b"01230");
    assert_eq!(Slice::trim_edges_min_len_right(data, b'0', 6), b"012300");
    assert_eq!(Slice::trim_edges_min_len_right(data, b'0', 7), b"0012300");
}

#[test]
fn replace() {
    let mut data = [0, 0, 0, 0, 1, 2, 3];

    Slice::replace_leading(&mut data, 0, 48);
    assert_eq!(&data, &[48, 48, 48, 48, 1, 2, 3]);
}
