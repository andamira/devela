// devela/src/data/store/arena/bytes/_test.rs

use crate::ArenaBytesExample as Arena;

crate::arena_bytes! {
    [cursor: u8 + crate::NonMaxU8;]
    _ArenaNoMark;
    _ArenaNoMarkHandle;
}
#[test]
fn works_without_marks() {
    let mut a = _ArenaNoMark::<4>::new();
    let h = a.push_byte(7).unwrap();
    assert_eq!(a.read_byte(h), Some(7));
    a.clear();
    assert!(a.is_empty());
}
#[test]
fn push_and_read_bytes() {
    let mut a = Arena::<16>::new();
    let handle = a.push_bytes(&[1, 2, 3, 4]).unwrap();
    assert_eq!(handle.offset_prim(), 0);
    assert_eq!(handle.len_prim(), 4);
    assert_eq!(a.read_bytes(handle).unwrap(), &[1, 2, 3, 4]);
}
#[test]
fn replace_and_mutate_bytes() {
    let mut a = Arena::<8>::new();
    let h = a.push_bytes(&[9, 9]).unwrap();
    assert!(a.replace_bytes(h, &[7, 8]));
    assert_eq!(a.read_bytes(h).unwrap(), &[7, 8]);
    let dst = a.read_bytes_mut(h).unwrap();
    dst.copy_from_slice(&[5, 6]);
    assert_eq!(a.read_bytes(h).unwrap(), &[5, 6]);
}
#[test]
fn push_and_read_primitives() {
    let mut a = Arena::<32>::new();
    let h = a.push_u32(0x11223344).unwrap();
    assert_eq!(a.read_u32(h), Some(0x11223344));
    assert!(a.replace_u32(h, 0x55667788));
    assert_eq!(a.read_u32(h), Some(0x55667788));
}
#[test]
fn push_and_read_str() {
    let mut a = Arena::<32>::new();
    let h = a.push_str_u8("hi").unwrap();
    assert_eq!(a.read_str_u8(h), Some("hi"));
}
#[test]
fn bool_and_char() {
    let mut a = Arena::<16>::new();
    let hb = a.push_bool(true).unwrap();
    let hc = a.push_char('Z').unwrap();
    assert_eq!(a.read_bool(hb), Some(true));
    assert_eq!(a.read_char(hc), Some('Z'));
}
#[test]
fn pop_and_truncate() {
    let mut a = Arena::<8>::new();
    let h1 = a.push_bytes(&[1, 2]).unwrap();
    let h2 = a.push_bytes(&[3, 4]).unwrap();
    assert!(!a.truncate_last(h1));
    assert!(a.truncate_last(h2));
    assert_eq!(a.len(), h1.offset_prim() + h1.len_prim());
}
#[test]
fn capacity_and_remaining() {
    let a = Arena::<8>::new();
    assert_eq!(a.capacity(), 8);
    assert_eq!(a.remaining(), 8);
}
#[test]
fn handle_bounds_checks() {
    let mut a = Arena::<4>::new();
    assert!(a.push_bytes(&[1, 2, 3, 4]).is_some());
    assert!(a.push_byte(5).is_none()); // capacity overflow
}
#[test]
fn eq_bytes_and_replace_str() {
    let mut a = Arena::<32>::new();
    let h = a.push_str_u8("hi").unwrap();
    assert_eq!(a.read_str_u8(h), Some("hi"));
    assert!(a.replace_str_u8(h, "hi"));
    assert_eq!(a.read_str_u8(h), Some("hi"));
    let mut b = Arena::<32>::new();
    let _ = b.push_str_u8("hi");
    assert!(a == b);
}
#[test]
fn rejects_oversized_primitive_span() {
    let mut a = Arena::<8>::new();
    let h = a.push_bytes(&[1, 2, 3, 4, 5]).unwrap();
    assert_eq!(a.read_u32(h), None);
    assert!(!a.replace_u32(h, 7));
}
#[test]
fn rejects_invalid_bool_encoding() {
    let mut a = Arena::<4>::new();
    let h = a.push_byte(2).unwrap();
    assert_eq!(a.read_bool(h), None);
}
