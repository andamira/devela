// devela::sys::mem::size::bit::tests

#![allow(unused)]

use crate::{BitSized, ByteSized, Mem};

#[test]
fn bitsized_queries_concrete_primitives() {
    assert_eq!(<bool as BitSized<1>>::BIT_SIZE, 1);
    assert_eq!(<bool as BitSized<1>>::MIN_BYTE_SIZE, 1);

    assert_eq!(<u8 as BitSized<8>>::BIT_SIZE, 8);
    assert_eq!(<u8 as BitSized<8>>::MIN_BYTE_SIZE, 1);

    assert_eq!(<u16 as BitSized<16>>::BIT_SIZE, 16);
    assert_eq!(<u16 as BitSized<16>>::MIN_BYTE_SIZE, 2);

    assert_eq!(<u16 as ByteSized>::BYTE_SIZE, 2);
}

#[test]
fn bitsized_methods_work_from_values() {
    let a = true;
    let b = 0_u8;
    let c = 0_u32;

    assert_eq!(a.bit_size(), 1);
    assert_eq!(a.min_byte_size(), 1);

    assert_eq!(b.bit_size(), 8);
    assert_eq!(b.min_byte_size(), 1);

    assert_eq!(c.bit_size(), 32);
    assert_eq!(c.min_byte_size(), 4);
}

const fn packed_bytes<T, const LEN: usize, const COUNT: usize>() -> usize
where
    T: BitSized<LEN>,
{
    Mem::bytes_from_bits(LEN * COUNT)
}

#[test]
fn bitsized_supports_packing_calculations() {
    assert_eq!(packed_bytes::<bool, 1, 8>(), 1);
    assert_eq!(packed_bytes::<bool, 1, 9>(), 2);
    assert_eq!(packed_bytes::<u8, 8, 3>(), 3);
    assert_eq!(packed_bytes::<u16, 16, 3>(), 6);
}

const fn fits_in_bits<T, const LEN: usize, const MAX: usize>() -> bool
where
    T: BitSized<LEN>,
{
    LEN <= MAX
}

#[test]
fn bitsized_can_check_capacity_as_a_value() {
    assert!(fits_in_bits::<u8, 8, 8>());
    assert!(fits_in_bits::<u8, 8, 16>());
    assert!(!fits_in_bits::<u16, 16, 8>());
}

crate::bitfield! {
    struct TestHeader(u16) {
        KIND = 0..=3;
        LEN  = 4..=11;
        FLAG = 12;
    }
}

#[test]
fn bitfield_can_advertise_storage_bit_size() {
    assert_eq!(<TestHeader as BitSized<13>>::BIT_SIZE, 13);
    assert_eq!(<TestHeader as BitSized<13>>::MIN_BYTE_SIZE, 2);

    let h = TestHeader::new().with_kind(3).with_len(120).with_flag(1);
    assert_eq!(h.bit_size(), 13);
    assert_eq!(h.min_byte_size(), 2);
}
