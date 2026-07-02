// devela/ui/layout/_tests.rs

use super::*;

#[test]
fn lunit_checked_arithmetic() {
    assert_eq!(Lunit::new(2).che_add(Lunit::new(3)), Some(Lunit::new(5)));
    assert_eq!(Lunit::MAX.che_add(Lunit::new(1)), None);
    assert_eq!(Lunit::new(5).che_sub(Lunit::new(3)), Some(Lunit::new(2)));
    assert_eq!(Lunit::new(3).che_sub(Lunit::new(5)), Some(Lunit::new(-2)));
}
#[test]
fn lunit_saturating_arithmetic() {
    assert_eq!(Lunit::MAX.sat_add(Lunit::new(1)), Lunit::MAX);
    assert_eq!(Lunit::new(0).sat_sub(Lunit::new(1)), Lunit::new(-1));
}
#[test]
fn lunit_zero_floor_arithmetic() {
    assert_eq!(Lunit::new(0).sat_sub_floor_zero(Lunit::new(1)), Lunit::ZERO);
    assert_eq!(Lunit::new(3).sat_sub_floor_zero(Lunit::new(5)), Lunit::ZERO);
}
#[test]
fn layout1d_remaining_fit_and_overflow() {
    let l = Layout1d::new(Lunit::new(10), Lunit::new(7));
    assert_eq!(l.avail(), Lunit::new(10));
    assert_eq!(l.used(), Lunit::new(7));
    assert_eq!(l.remaining(), Lunit::new(3));
    assert!(l.fits(Lunit::new(10)));
    assert!(!l.fits(Lunit::new(11)));
    assert_eq!(l.overflow(Lunit::new(14)), Lunit::new(4));
}
#[test]
fn layout1d_clamps_used_to_available() {
    let l = Layout1d::new_clamped(Lunit::new(10), Lunit::new(14));
    assert_eq!(l.used(), Lunit::new(10));
    assert_eq!(l.remaining(), Lunit::ZERO);
}
#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn layout1d_new_panics_when_used_exceeds_available() {
    let _ = Layout1d::new(Lunit::new(10), Lunit::new(11));
}
