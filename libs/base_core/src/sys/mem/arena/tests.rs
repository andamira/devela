// devela_base_core::sys::mem::arena::bytes

use crate::{ArenaBytes, ArenaField, ArenaRecipe};

#[test]
fn push_and_read_bytes() {
    let mut a = ArenaBytes::<16>::new();
    let handle = a.push_bytes(&[1, 2, 3, 4]).unwrap();
    assert_eq!(handle.offset(), 0);
    assert_eq!(handle.len(), 4);
    assert_eq!(a.read_bytes(handle).unwrap(), &[1, 2, 3, 4]);
}

#[test]
fn replace_and_mutate_bytes() {
    let mut a = ArenaBytes::<8>::new();
    let h = a.push_bytes(&[9, 9]).unwrap();
    assert!(a.replace_bytes(h, &[7, 8]));
    assert_eq!(a.read_bytes(h).unwrap(), &[7, 8]);
    let dst = a.read_bytes_mut(h).unwrap();
    dst.copy_from_slice(&[5, 6]);
    assert_eq!(a.read_bytes(h).unwrap(), &[5, 6]);
}

#[test]
fn push_and_read_primitives() {
    let mut a = ArenaBytes::<32>::new();
    let h = a.push_u32(0x11223344).unwrap();
    assert_eq!(a.read_u32(h), Some(0x11223344));
    assert!(a.replace_u32(h, 0x55667788));
    assert_eq!(a.read_u32(h), Some(0x55667788));
}

#[test]
fn push_and_read_str() {
    let mut a = ArenaBytes::<32>::new();
    let h = a.push_str_u8("hi").unwrap();
    assert_eq!(a.read_str_u8(h), Some("hi"));
}

#[test]
fn bool_and_char() {
    let mut a = ArenaBytes::<16>::new();
    let hb = a.push_bool(true).unwrap();
    let hc = a.push_char('Z').unwrap();
    assert_eq!(a.read_bool(hb), Some(true));
    assert_eq!(a.read_char(hc), Some('Z'));
}

#[test]
fn pop_and_truncate() {
    let mut a = ArenaBytes::<8>::new();
    let h1 = a.push_bytes(&[1, 2]).unwrap();
    let h2 = a.push_bytes(&[3, 4]).unwrap();
    assert!(!a.truncate_last(h1));
    assert!(a.truncate_last(h2));
    assert_eq!(a.len(), h1.offset() + h1.len());
}

#[test]
fn capacity_and_remaining() {
    let a = ArenaBytes::<8>::new();
    assert_eq!(a.capacity(), 8);
    assert_eq!(a.remaining(), 8);
}

#[test]
fn handle_bounds_checks() {
    let mut a = ArenaBytes::<4>::new();
    assert!(a.push_bytes(&[1, 2, 3, 4]).is_some());
    assert!(a.push_byte(5).is_none()); // capacity overflow
}

#[test]
fn eq_bytes_and_replace_str() {
    let mut a = ArenaBytes::<32>::new();
    let h = a.push_str_u8("hi").unwrap();
    assert_eq!(a.read_str_u8(h), Some("hi"));
    assert!(a.replace_str_u8(h, "hi"));
    assert_eq!(a.read_str_u8(h), Some("hi"));

    let mut b = ArenaBytes::<32>::new();
    let _ = b.push_str_u8("hi");
    assert!(a == b);
}

// #[test]
// fn recipe_header_and_payload() {
//     let mut a = ArenaBytes::<64>::new();
//     // TODO
//     // let r = a.record_push(7, 1, b"abc").unwrap();
//     // let (ty, ver, len) = a.record_read_header(r).unwrap();
//     // assert_eq!((ty, ver, len), (7, 1, 3));
//     // assert_eq!(a.record_read_payload(r).unwrap(), b"abc");
// }
//
// #[test]
// fn recipe_field_views() {
//     // payload: [x: u8][y: u16][z: u8] = 1 + 2 + 1 = 4
//     const F: &[ArenaField] = &[
//         ArenaField { off: 0, len: 1 },
//         ArenaField { off: 1, len: 2 },
//         ArenaField { off: 3, len: 1 },
//     ];
//     const R: ArenaRecipe = ArenaRecipe { ty: 10, ver: 0, size: 4, fields: F };
//
//     let mut a = ArenaBytes::<32>::new();
//     // TODO
//     // let r = a.record_push(10, 0, &[0x01, 0x34, 0x12, 0xFF]).unwrap();
//     // assert!(ArenaRecipe::record_matches(r, &a, &R));
//     // assert_eq!(ArenaRecipe::record_view(&a, r, &R, 0).unwrap(), &[0x01]);
//     // assert_eq!(ArenaRecipe::record_view(&a, r, &R, 1).unwrap(), &[0x34, 0x12]);
//     // assert_eq!(ArenaRecipe::record_view(&a, r, &R, 2).unwrap(), &[0xFF]);
// }
