// devela_base::data::bit::wrapper::tests
//
// TOC
// - bit_mask_range
// - bit_ops

use crate::{Bitwise, Bitwise as Bg};

#[test] #[rustfmt::skip]
fn bit_mask_range() {
    assert_eq![0b_0000_0001, Bitwise::<u8>::mask_range(0, 0).0];
    assert_eq![0b_0000_0001, Bitwise::<u8>::mask_checked_range(0, 0).unwrap().0];
    assert_eq![0b_1000_0000, Bitwise::<u8>::mask_range(7, 7).0];
    assert_eq![0b_1000_0000, Bitwise::<u8>::mask_checked_range(7, 7).unwrap().0];
    assert_eq![0b_0111_1110, Bitwise::<u8>::mask_range(1, 6).0];
    assert_eq![0b_0111_1110, Bitwise::<u8>::mask_checked_range(1, 6).unwrap().0];

    debug_assert![Bitwise::<u8>::mask_checked_range(8, 8).is_err()];
    debug_assert![Bitwise::<u8>::mask_checked_range(0, 8).is_err()];
    debug_assert![Bitwise::<u8>::mask_checked_range(4, 1).is_err()];

    // NOTE: panics are tested in devela::data::bit::tests
}
#[test] #[rustfmt::skip]
fn bit_ops() {
    let b = Bg(0b_1111_0000_u8);
    assert_eq![0b_0011_0000, b.get_range(2, 5).0];
    assert_eq![0b_1111_1100, b.set_range(2, 5).0];
    assert_eq![0b_1100_0000, b.unset_range(2, 5).0];
    assert_eq![0b_1100_1100, b.flip_range(2, 5).0];

    // get/set value
    let b = Bg(0b_1000_0001_u8);
    assert_eq![0b_0000_0010, b.get_value_range(6, 7).0];
    assert_eq![0b_0000_0100, b.get_value_range(5, 7).0];
    assert_eq![0b_1000_1101, b.set_value_range(0b_11, 2, 5).0];
    assert_eq![0b_1011_0001, b.set_value_range(0b_11, 4, 5).0];
    assert_eq![0b_1011_0001, b.set_value_range(0b_111, 4, 5).0]; // value trimmed to range
    assert![b.set_checked_value_checked_range(0b_111, 4, 5).is_err()]; // err value overflows range

    // reverse
    let b = Bg(0b_1010_1010_u8);
    assert_eq!(0b_0101_1010, b.reverse_range(4, 7).0);
    //            ----
    let b = Bg(0b_0011_0001_u8);
    assert_eq!(0b_0000_1101, b.reverse_range(2, 5).0);
    //              --_--
    let b = Bg(0b_1000_1110_u8);
    assert_eq!(0b_1000_0111, b.reverse_range(0, 3).0);
    //                 ----

    let b = Bitwise(0b_1111_0000_u8);
    // count
    assert_eq![2, b.count_ones_range(3, 5)];
    assert_eq![1, b.count_zeros_range(3, 5)];
    // find first
    assert_eq![Some(4), b.find_first_one_range(3, 5)];
    assert_eq![None, b.find_first_one_range(0, 3)];
    assert_eq![Some(2), b.find_first_zero_range(2, 5)];
    assert_eq![None, b.find_first_zero_range(4, 7)];
    // find last
    assert_eq![Some(5), b.find_last_one_range(3, 5)];
    assert_eq![None, b.find_last_one_range(0, 3)];
    assert_eq![Some(3), b.find_last_zero_range(2, 5)];
    assert_eq![None, b.find_last_zero_range(4, 7)];
}
