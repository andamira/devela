// devela::data::bit::ops::tests
//
// TOC
// - bit_mask_range
// - bit_ops

use super::*;

#[test] #[rustfmt::skip]
fn bit_mask_range() {
    assert_eq![0b_0000_0001, Bits::<u8>::mask_range(0, 0).0];
    assert_eq![0b_0000_0001, Bits::<u8>::mask_checked_range(0, 0).unwrap().0];
    assert_eq![0b_1000_0000, Bits::<u8>::mask_range(7, 7).0];
    assert_eq![0b_1000_0000, Bits::<u8>::mask_checked_range(7, 7).unwrap().0];
    assert_eq![0b_0111_1110, Bits::<u8>::mask_range(1, 6).0];
    assert_eq![0b_0111_1110, Bits::<u8>::mask_checked_range(1, 6).unwrap().0];

    debug_assert![Bits::<u8>::mask_checked_range(8, 8).is_err()];
    debug_assert![Bits::<u8>::mask_checked_range(0, 8).is_err()];
    debug_assert![Bits::<u8>::mask_checked_range(4, 1).is_err()];
    #[cfg(feature = "std")]
    {
        use std::panic::catch_unwind;
        debug_assert![catch_unwind(|| { let _ = Bits::<u8>::mask_range(8, 8); }).is_err()];
        debug_assert![catch_unwind(|| { let _ = Bits::<u8>::mask_range(0, 8); }).is_err()];
        debug_assert![catch_unwind(|| { let _ = Bits::<u8>::mask_range(4, 1); }).is_err()];
    }
}
#[test] #[rustfmt::skip]
fn bit_ops() {
    let bits = Bits(0b_1111_0000_u8);
    assert_eq![0b_0011_0000, bits.get_range(2, 5).0];
    assert_eq![0b_0000_1100, bits.get_shifted_range(2, 5).0];
    assert_eq![0b_1111_1100, bits.set_range(2, 5).0];
    assert_eq![0b_1100_0000, bits.unset_range(2, 5).0];
    assert_eq![0b_1100_1100, bits.flip_range(2, 5).0];

    // reverse
    let bits = Bits(0b__1010__1010_u8);
    assert_eq!(0b__0101__1010, bits.reverse_range(4, 7).0);
    let bits = Bits(0b_00__1100__01_u8);
    assert_eq!(0b_00__0011__01, bits.reverse_range(2, 5).0);
    let bits = Bits(0b_1000__1110__u8);
    assert_eq!(0b_1000__0111__, bits.reverse_range(0, 3).0);

    let bits = Bits(0b_1111_0000_u8);
    // count
    assert_eq![2, bits.count_ones_range(3, 5)];
    assert_eq![1, bits.count_zeros_range(3, 5)];
    // find first
    assert_eq![Some(4), bits.find_first_one_range(3, 5)];
    assert_eq![None, bits.find_first_one_range(0, 3)];
    assert_eq![Some(2), bits.find_first_zero_range(2, 5)];
    assert_eq![None, bits.find_first_zero_range(4, 7)];
    // find last
    assert_eq![Some(5), bits.find_last_one_range(3, 5)];
    assert_eq![None, bits.find_last_one_range(0, 3)];
    assert_eq![Some(3), bits.find_last_zero_range(2, 5)];
    assert_eq![None, bits.find_last_zero_range(4, 7)];
}
