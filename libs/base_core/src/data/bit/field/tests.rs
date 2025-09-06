// devela_base_core::data::bit::field::tests
//
//!
//
// TOC
// - bits
// - bitmask TODO
// - bitrange TODO

use super::bitfield;

#[test]
#[rustfmt::skip]
fn bits() {
    bitfield! { (extra) struct Bf(u8) {} }

    // ...

    assert_eq!(Bf::with_bits(10).bits(), 10);
    assert_eq!(Bf::with_bits(0).bits(), 0);

    assert_eq!(Bf::new_zeroed().bits(), 0);
    assert!(Bf::new_zeroed().is_empty());

    assert_eq!(Bf::new_oned().bits(), !0);
    assert!(Bf::new_oned().is_full());

    assert_eq!(Bf::with_bits(0b1111).count_ones(), 4);
    assert_eq!(Bf::new_zeroed().count_ones(), 0);

    assert_eq!(Bf::with_bits(0b1111).count_zeros(), u8::BITS - 4);
    assert_eq!(Bf::new_zeroed().count_zeros(), u8::BITS);

    assert!(Bf::new_zeroed().is_empty());
    assert!(!Bf::with_bits(1).is_empty());

    assert!(Bf::new_oned().is_full());
    assert!(!Bf::new_zeroed().is_full());

    assert!(Bf::with_bits(0b1000).is_checked_bit_set(3).unwrap());
    assert!(!Bf::new_zeroed().is_checked_bit_set(1).unwrap());
    assert!(Bf::with_bits(0b1000).is_bit_set(3));

    assert_eq!(Bf::with_bits(0b1010).get_checked_bit(1).unwrap().bits(), 2);
    assert_eq!(Bf::with_bits(0b1010).get_bit(1).bits(), 2);

    assert_eq!(Bf::with_bits(0b100).get_shifted_checked_bit(2).unwrap().bits(), 1);
    assert_eq!(Bf::with_bits(0b100).get_shifted_bit(2).bits(), 1);

    assert_eq!(Bf::new_zeroed().set_checked_bit(2).unwrap().bits(), 4);
    assert_eq!(Bf::new_zeroed().set_bit(2).bits(), 4);
    let mut b = Bf::new_zeroed();
    b.mut_set_checked_bit(2).unwrap();
    assert_eq!(b.bits(), 4);

    assert_eq!(Bf::with_bits(0b111).unset_checked_bit(1).unwrap().bits(), 0b101);
    assert_eq!(Bf::with_bits(0b111).unset_bit(1).bits(), 0b101);
    let mut b = Bf::with_bits(0b111);
    b.mut_unset_checked_bit(1).unwrap();
    assert_eq!(b.bits(), 0b101);

    assert_eq!(Bf::with_bits(0b1010).flip().bits(), 0b1111_0101);
    let mut b = Bf::with_bits(0b1010);
    b.mut_flip();
    assert_eq!(b.bits(), 0b1111_0101);
    assert_eq!(Bf::with_bits(0b10).flip_checked_bit(1).unwrap().bits(), 0b00);
    assert_eq!(Bf::with_bits(0b10).flip_bit(1).bits(), 0b00);

    // ...

    /* immutable ops */

    // set:
    let b0 = Bf::new_zeroed();
    assert_eq![0b_0000_0010, b0.set_checked_bit(1).unwrap().bits()];
    assert_eq![0b_0000_1100, b0.set_checked_bit(2).unwrap().set_checked_bit(3).unwrap().bits()];
    assert_eq![0b_0000_0010, b0.set_bit(1).bits()];
    assert_eq![0b_0000_1100, b0.set_bit(2).set_bit(3).bits()];
    // unset:
    let b1 = Bf::new_oned();
    assert_eq![0b_1111_1101, b1.unset_checked_bit(1).unwrap().bits()];
    assert_eq![0b_1111_0011, b1.unset_checked_bit(2).unwrap().unset_checked_bit(3).unwrap().bits()];
    assert_eq![0b_1111_1101, b1.unset_bit(1).bits()];
    assert_eq![0b_1111_0011, b1.unset_bit(2).unset_bit(3).bits()];
    // flip:
    let b2 = Bf::with_bits(0b_1111_0000);
    assert_eq![0b_1111_0010, b2.flip_checked_bit(1).unwrap().bits()];
    assert_eq![0b_1101_0100, b2.flip_checked_bit(2).unwrap().flip_checked_bit(5).unwrap().bits()];
    assert_eq![0b_1111_0010, b2.flip_bit(1).bits()];
    assert_eq![0b_1101_0100, b2.flip_bit(2).flip_bit(5).bits()];

    /* mutable ops */

    // set:
    let mut b0 = Bf::new_zeroed();
    assert![b0.mut_set_checked_bit(0).is_ok()]; assert_eq![0b_0000_0001, b0.bits()];
           b0.mut_set_bit(3); assert_eq![0b_0000_1001, b0.bits()];
    // unset:
    let mut b1 = Bf::new_oned();
    assert![b1.mut_unset_checked_bit(2).is_ok()]; assert_eq![0b_1111_1011, b1.bits()];
           b1.mut_unset_bit(3); assert_eq![0b_1111_0011, b1.bits()];
    // flip:
    let mut b2 = Bf::with_bits(0b_1111_0000);
    assert![b2.mut_flip_checked_bit(3).is_ok()]; assert_eq![0b_1111_1000, b2.bits()];
           b2.mut_flip_bit(4); assert_eq![0b_1110_1000, b2.bits()];

    /* check out of bounds */

    // immutable
    assert![b0.set_checked_bit(9).is_err()];
    assert![b0.unset_checked_bit(9).is_err()];
    assert![b0.flip_checked_bit(9).is_err()];
    // mutable
    assert![b0.mut_set_checked_bit(9).is_err()];
    assert![b0.mut_unset_checked_bit(9).is_err()];
    assert![b0.mut_flip_checked_bit(9).is_err()];

    // NOTE: unchecked panics are tested in devela::data::bit::tests
}

// #[test]
// fn bitmask() {
//     bitfield! { struct Bf(u8) {} }
//     let mut b0 = Bf::new();
//     let mut mask = Bf::new().set_unchecked(2);
// }

#[test]
fn bitrange() {
    bitfield! { (extra) struct Bf8(u8) {} }

    // assert_eq![0b_0000_1111, Bf8::mask_set_between_unchecked(0, 3)];

    // assert_eq![0b_0000_1111, Bf8::mask_set_between_unchecked2(0, 3)];
    // assert_eq![0b_0001_1000, Bf8::mask_set_between_unchecked2(3, 4)];
    // assert_eq![0b_0000_0000, Bf8::mask_set_between_unchecked2(4, 3)]; // IMPROVE?
}
