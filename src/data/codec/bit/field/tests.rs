// devela::data::codec::bit::field::tests

#![allow(unused)]

use super::*;

bitfield! {
    struct TestBitfield(u16) {
        a = 0;
        b = 1..=3;
        c = 4..=7;
        D = 8..=15; // check uppercase field
    }
}

#[test]
fn field_metadata() {
    assert_eq![1, TestBitfield::_b_START];
    assert_eq![3, TestBitfield::_b_END];
    assert_eq![3, TestBitfield::_b_WIDTH];
    assert_eq![0b0000_1110, TestBitfield::_b_MASK];
    assert_eq![0b0000_0111, TestBitfield::_b_MAX];

    assert_eq![4, TestBitfield::_c_START];
    assert_eq![7, TestBitfield::_c_END];
    assert_eq![4, TestBitfield::_c_WIDTH];
    assert_eq![0b1111_0000, TestBitfield::_c_MASK];
    assert_eq![0b0000_1111, TestBitfield::_c_MAX];
}

#[test]
fn bitfield_gets_and_sets_fields() {
    let f = TestBitfield::new().with_a(1).with_b(0b101).with_c(0b1100).with_d(0xAB);
    assert_eq!(f.get_a(), 1);
    assert_eq!(f.get_b(), 0b101);
    assert_eq!(f.get_c(), 0b1100);
    assert_eq!(f.get_d(), 0xAB);
    assert_eq!(TestBitfield::a_mask(), 0b0000_0000_0000_0001);
    assert_eq!(TestBitfield::b_mask(), 0b0000_0000_0000_1110);
    assert_eq!(TestBitfield::c_mask(), 0b0000_0000_1111_0000);
    assert_eq!(TestBitfield::d_mask(), 0b1111_1111_0000_0000);
}
#[test]
fn bitfield_mutates_fields() {
    let mut f = TestBitfield::new();
    f.set_b(0b111);
    assert_eq!(f.get_b(), 0b111);
    f.set_b(0b010);
    assert_eq!(f.get_b(), 0b010);
    f.clear_b();
    assert_eq!(f.get_b(), 0);
    assert_eq!(f.bits(), 0);
}
#[test]
fn bitfield_overwrites_only_target_range() {
    let f = TestBitfield::new().with_b(0b111).with_c(0b1111).with_b(0b001);
    assert_eq!(f.get_b(), 0b001);
    assert_eq!(f.get_c(), 0b1111);
}
#[test]
fn bitfield_checked_operations_validate_capacity() {
    crate::bitfield! {
        struct CheckedHeader(u16) {
            KIND = 0..=3;
            TWO  = 4..=5;
            FLAG = 6;
        }
    }
    let mut h = CheckedHeader::new();

    assert!(h.try_set_kind(15).is_ok());
    assert_eq!(h.get_kind(), 15);
    assert!(h.try_set_kind(16).is_err());
    assert_eq!(h.get_kind(), 15);

    assert!(h.try_set_two(3).is_ok());
    assert_eq!(h.get_two(), 3);
    assert!(h.try_set_two(4).is_err());
    assert_eq!(h.get_two(), 3);

    assert!(h.try_set_flag(1).is_ok());
    assert_eq!(h.get_flag(), 1);
    assert!(h.try_set_flag(2).is_err());
    assert_eq!(h.get_flag(), 1);

    assert!(CheckedHeader::new().try_with_kind(15).is_ok());
    assert!(CheckedHeader::new().try_with_kind(16).is_err());
    assert!(CheckedHeader::new().try_with_two(3).is_ok());
    assert!(CheckedHeader::new().try_with_two(4).is_err());
}
#[test]
fn bitfield_uppercase_fields_generate_lowercase_methods() {
    crate::bitfield! {
        struct CaseHeader(u8) {
            KIND = 0..=3;
            IS_SET = 4;
        }
    }
    let h = CaseHeader::new().with_kind(7).with_is_set(1);
    assert_eq!(h.get_kind(), 7);
    assert_eq!(h.get_is_set(), 1);
}
