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

#[cfg(feature = "alloc")]
mod alloc {
    use crate::ArenaBytesAllocExample as Arena;

    crate::arena_bytes! {
        [cursor: u8 + crate::NonMaxU8;]
        _ArenaAllocNoMark: alloc;
        _ArenaAllocNoMarkHandle;
    }
    #[test]
    fn works_without_marks() {
        let mut a = _ArenaAllocNoMark::new();
        let h = a.push_byte(7).unwrap();
        assert_eq!(a.read_byte(h), Some(7));
        a.clear();
        assert!(a.is_empty());
    }
    #[test]
    fn grows() {
        let mut a = Arena::with_capacity(1);
        let h = a.push_bytes(&[1, 2, 3, 4]).unwrap();
        assert_eq!(a.read_bytes(h), Some(&[1, 2, 3, 4][..]));
        assert_eq!(a.len(), 4);
    }
    #[test]
    fn cursor_limit() {
        let mut a = Arena::new();
        assert!(a.push_bytes(&[0; 254]).is_some());
        assert!(a.is_full());
        assert!(a.push_byte(0).is_none());
    }
    #[test]
    fn mark_survives_reallocation() {
        let mut a = Arena::with_capacity(1);
        let _ = a.push_bytes(&[1, 2]).unwrap();
        let mark = a.mark();
        let _ = a.push_bytes(&[3, 4, 5, 6, 7, 8]).unwrap();
        assert!(a.rollback(mark));
        assert_eq!(a.as_bytes(), &[1, 2]);
    }
    #[test]
    fn remaining_is_physical_capacity() {
        let a = Arena::new();
        assert_eq!(a.remaining(), a.capacity());
        assert!(!a.is_full());
    }
    #[test]
    fn rejects_invalid_primitive_encodings() {
        let mut a = Arena::new();
        let h = a.push_bytes(&[1, 2, 3, 4, 5]).unwrap();
        assert_eq!(a.read_u32(h), None);
        assert!(!a.replace_u32(h, 7));
        let h = a.push_byte(2).unwrap();
        assert_eq!(a.read_bool(h), None);
    }
    #[test]
    fn strings_are_transactional() {
        let mut a = Arena::new();
        // Leave exactly one representable byte at the coordinate frontier.
        let h = a.push_bytes(&[0; 253]).unwrap();
        // A u8-prefixed "a" requires two bytes. Neither the prefix nor
        // the payload may be written when the complete span cannot fit.
        assert!(a.push_str_u8("a").is_none());
        assert_eq!(a.len(), 253);
        assert_eq!(a.read_bytes(h), Some(&[0; 253][..]));
        assert_eq!(a.as_bytes(), &[0; 253]);
    }
    #[test]
    fn strings_read_and_replace() {
        let mut a = Arena::new();
        let h = a.push_str_u16("hi").unwrap();
        assert_eq!(a.read_str_u16(h), Some("hi"));
        assert!(a.replace_str_u16(h, "yo"));
        assert_eq!(a.read_str_u16(h), Some("yo"));
        assert!(!a.replace_str_u16(h, "longer"));
        assert_eq!(a.read_str_u16(h), Some("yo"));
    }
    #[test]
    fn views_pop_and_truncate() {
        let mut a = Arena::new();
        let h1 = a.push_bytes(&[1, 2]).unwrap();
        let h2 = a.push_bytes(&[3, 4]).unwrap();
        let h3 = a.push_bytes(&[5, 6]).unwrap();
        assert_eq!(a.view_bytes(h1, 3), Some(&[1, 2, 3, 4, 5, 6][..]));
        a.view_bytes_mut(h1, 3).unwrap()[3] = 9;
        assert_eq!(a.as_bytes(), &[1, 2, 3, 9, 5, 6]);
        assert!(!a.truncate_last(h1));
        let mut out = [0; 2];
        assert!(a.pop_into(h3, &mut out));
        assert_eq!(out, [5, 6]);
        assert_eq!(a.as_bytes(), &[1, 2, 3, 9]);
        assert!(a.truncate_last(h2));
        assert_eq!(a.as_bytes(), &[1, 2]);
    }
}
