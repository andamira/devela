// devela::data::codec::bit::set::tests

#![allow(unused)]

crate::set! {
    struct TestSet(u16) {
        A = 0; B = 1; C = 2; D = 3;
        AB = 0, 1;
        BC = 1..=2;
        AD = 0, 3;
        MIXED = 0, 3..=5, 7;
    }
}
#[test]
fn generated_bitsized_uses_highest_declared_bit_width() {
    use crate::BitSized;
    assert_eq!(TestSet::_MIXED_BITS, 5);
    assert_eq!(TestSet::new().bit_size(), 8);
}
#[test]
fn formatting_uses_declared_domain_width() {
    use crate::format_buf;
    let mut buf = [0u8; 32];
    let mixed = TestSet::MIXED;

    let debug = format_buf![&mut buf, "{:?}", mixed];
    assert_eq!(debug, Ok("TestSet(0b10111001)"));

    let display = format_buf![&mut buf, "{}", mixed];
    assert_eq!(display, Ok("0b10111001"));

    let binary = format_buf![&mut buf, "{:b}", mixed];
    assert_eq!(binary, Ok("10111001"));
    let binary_pretty = format_buf![&mut buf, "{:#b}", mixed];
    assert_eq!(binary_pretty, Ok("0b10111001"));

    let octal = format_buf![&mut buf, "{:o}", mixed];
    assert_eq!(octal, Ok("271"));
    let octal_pretty = format_buf![&mut buf, "{:#o}", mixed];
    assert_eq!(octal_pretty, Ok("0o271"));

    let lower_hex = format_buf![&mut buf, "{:x}", mixed];
    assert_eq!(lower_hex, Ok("b9"));
    let lower_hex_pretty = format_buf![&mut buf, "{:#x}", mixed];
    assert_eq!(lower_hex_pretty, Ok("0xb9"));
    let upper_hex = format_buf![&mut buf, "{:X}", mixed];
    assert_eq!(upper_hex, Ok("B9"));
    let upper_hex_pretty = format_buf![&mut buf, "{:#X}", mixed];
    assert_eq!(upper_hex_pretty, Ok("0xB9"));

    use crate::{DebugExt, ReprMode};
    let named = format_buf![&mut buf, "{:?}", TestSet::MIXED.debug_with(&ReprMode::Named)];
    assert_eq!(named, Ok("TestSet(A | D)"));
    let raw = format_buf![&mut buf, "{:?}", TestSet::MIXED.debug_with(&ReprMode::Raw)];
    assert_eq!(raw, Ok("TestSet(0b10111001)"));
    let raw_named = format_buf![&mut buf, "{:?}", TestSet::MIXED.debug_with(&ReprMode::RawNamed)];
    assert_eq!(raw_named, Ok("TestSet(0b10111001; A | D)"));
}
#[test]
fn constants_accept_bits_and_ranges() {
    assert_eq!(TestSet::A.bits(), 0b0000_0001);
    assert_eq!(TestSet::B.bits(), 0b0000_0010);
    assert_eq!(TestSet::AB.bits(), 0b0000_0011);
    assert_eq!(TestSet::BC.bits(), 0b0000_0110);
    assert_eq!(TestSet::AD.bits(), 0b0000_1001);
    assert_eq!(TestSet::MIXED.bits(), 0b1011_1001);
}
#[test]
fn mathematical_set_operations() {
    let a = TestSet::AB; // 0, 1
    let b = TestSet::BC; // 1, 2
    assert_eq!(a.union(b).bits(), 0b0000_0111);
    assert_eq!(a.intersection(b).bits(), 0b0000_0010);
    assert_eq!(a.difference(b).bits(), 0b0000_0001);
    assert_eq!(b.difference(a).bits(), 0b0000_0100);
    assert_eq!(a.symmetric_difference(b).bits(), 0b0000_0101);
    assert!(a.intersects(b));
    assert!(!TestSet::A.intersects(TestSet::C));
    assert!(TestSet::A.is_subset(a));
    assert!(a.is_superset(TestSet::A));
    assert!(a.contains(TestSet::A));
    assert!(!a.is_subset(TestSet::A));
    assert!(!TestSet::A.is_superset(a));
    assert!(!TestSet::A.contains(a));
}
#[test]
fn mutating_core_operations_match_set_operations() {
    let mut s = TestSet::A;
    s.insert(TestSet::C);
    assert_eq!(s.bits(), TestSet::A.union(TestSet::C).bits());
    s.toggle(TestSet::A);
    assert_eq!(s.bits(), TestSet::C.bits());
    s.remove(TestSet::C);
    assert!(s.is_empty());
}
#[test]
fn generic_convenience_methods_work() {
    let s = TestSet::new().with(TestSet::A).with_if(true, TestSet::C).with_if(false, TestSet::D);
    assert!(s.has(TestSet::A));
    assert!(s.contains(TestSet::C));
    assert!(!s.has(TestSet::D));
    assert_eq!(s.without(TestSet::A).bits(), TestSet::C.bits());
    assert_eq!(s.toggled(TestSet::C).bits(), TestSet::A.bits());
}
#[test]
fn per_constant_methods_work_for_single_bit_constants() {
    let mut s = TestSet::new();
    assert!(!s.has_a());
    assert!(!s.contains_a());
    assert!(!s.intersects_a());
    s.set_a();
    assert!(s.has_a());
    assert!(s.contains_a());
    assert!(s.intersects_a());
    s.set_b_if(true);
    s.set_c_if(false);
    assert!(s.has_b());
    assert!(!s.has_c());
    assert_eq!(s.without_a().bits(), TestSet::B.bits());
    s.unset_b();
    assert!(!s.is_empty());
    s.unset_a();
    assert!(s.is_empty());
}
#[test]
fn per_constant_methods_work_for_grouped_constants() {
    let s = TestSet::AB;
    assert!(s.contains_ab());
    assert!(s.intersects_ab());
    assert!(!TestSet::A.contains_ab());
    assert!(TestSet::A.intersects_ab());
    assert_eq!(TestSet::new().with_ab().bits(), TestSet::AB.bits());
    assert_eq!(TestSet::AB.without_b().bits(), TestSet::A.bits());
    let mut s = TestSet::new();
    s.set_mixed();
    assert!(s.contains_mixed());
    s.unset_ad();
    assert_eq!(s.bits(), TestSet::MIXED.without(TestSet::AD).bits());
}
#[test]
fn generated_metadata_counts_bits() {
    assert_eq!(TestSet::_A_BITS, 1);
    assert_eq!(TestSet::_B_BITS, 1);
    assert_eq!(TestSet::_AB_BITS, 2);
    assert_eq!(TestSet::_BC_BITS, 2);
    assert_eq!(TestSet::_AD_BITS, 2);
    assert_eq!(TestSet::_MIXED_BITS, 5);
}
#[test]
fn only_single_bit_constants_get_has_aliases() {
    let _ = TestSet::A.has_a();
    let _ = TestSet::B.has_b();
}

// The following tests intentionally should not compile
/**
```compile_fail
devela::set! { struct TestSet(u16) { A = 0; B = 1; C = 2; AB = 0, 1; BC = 1..=2; } }
let _ = TestSet::AB.has_ab();
```
```compile_fail
devela::set! { struct TestSet(u16) { A = 0; B = 1; C = 2; AB = 0, 1; BC = 1..=2; } }
let _ = TestSet::BC.has_bc();
```
**/
fn grouped_constants_do_not_have_has_aliases() {}
