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
fn push_filled_and_zeroed() {
    let mut a = Arena::<8>::new();
    let hf = a.push_filled(3, 0xAB).unwrap();
    let hz = a.push_zeroed(2).unwrap();
    assert_eq!(a.read_bytes(hf), Some(&[0xAB, 0xAB, 0xAB][..]));
    assert_eq!(a.read_bytes(hz), Some(&[0, 0][..]));
    assert_eq!(a.as_bytes(), &[0xAB, 0xAB, 0xAB, 0, 0]);
}
#[test]
fn push_filled_is_atomic() {
    let mut a = Arena::<4>::new();
    let h = a.push_filled(3, 7).unwrap();
    assert!(a.push_filled(2, 9).is_none());
    assert_eq!(a.len(), 3);
    assert_eq!(a.as_bytes(), &[7, 7, 7]);
    assert_eq!(a.read_bytes(h), Some(&[7, 7, 7][..]));
}
#[test]
fn push_and_read_bytes() {
    let mut a = Arena::<16>::new();
    let handle = a.push_bytes(&[1, 2, 3, 4]).unwrap();
    assert_eq!(handle.get_offset_prim(), 0);
    assert_eq!(handle.get_len_prim(), 4);
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
    assert_eq!(a.len(), h1.get_offset_prim() + h1.get_len_prim());
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
#[test]
fn strings() {
    let mut a = Arena::<16>::new();
    let h = a.push_str("café").unwrap();
    // The handle covers exactly the UTF-8 bytes: no framing bytes.
    assert_eq!(h.get_len_prim(), 5);
    assert_eq!(a.as_bytes(), "café".as_bytes());
    assert_eq!(a.read_str(h), Some("café"));
    // Safe mutable string access preserves UTF-8.
    a.read_str_mut(h).unwrap().make_ascii_uppercase();
    assert_eq!(a.read_str(h), Some("CAFé"));
    // Replacement is span-sized, in bytes.
    assert!(a.replace_str(h, "niño"));
    assert_eq!(a.read_str(h), Some("niño"));
    assert!(!a.replace_str(h, "longer"));
    assert_eq!(a.read_str(h), Some("niño"));
}
#[test]
fn strings_reject_invalid_utf8() {
    let mut a = Arena::<8>::new();
    let h = a.push_str("é").unwrap();
    assert_eq!(a.read_str(h), Some("é"));
    // Raw byte access can invalidate a previously valid string span.
    a.read_bytes_mut(h).unwrap()[0] = 0xFF;
    assert_eq!(a.read_str(h), None);
    assert!(a.read_str_mut(h).is_none());
}
#[test]
fn push_str_is_atomic() {
    let mut a = Arena::<4>::new();
    let h = a.push_str("ab").unwrap();
    // "€" needs 3 bytes, but only 2 remain.
    assert!(a.push_str("€").is_none());
    assert_eq!(a.len(), 2);
    assert_eq!(a.as_bytes(), b"ab");
    assert_eq!(a.read_str(h), Some("ab"));
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
    fn filled_and_zeroed_grow() {
        let mut a = Arena::with_capacity(1);
        let hf = a.push_filled(3, 7).unwrap();
        let hz = a.push_zeroed(2).unwrap();
        assert_eq!(a.read_bytes(hf), Some(&[7, 7, 7][..]));
        assert_eq!(a.read_bytes(hz), Some(&[0, 0][..]));
        assert_eq!(a.as_bytes(), &[7, 7, 7, 0, 0]);
    }
    #[test]
    fn filled_push_is_atomic_at_cursor_limit() {
        let mut a = Arena::new();
        let h = a.push_filled(253, 7).unwrap();
        assert!(a.push_zeroed(2).is_none());
        assert_eq!(a.len(), 253);
        assert_eq!(a.read_bytes(h).unwrap().len(), 253);
        assert!(a.read_bytes(h).unwrap().iter().all(|&b| b == 7));
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
    #[test]
    fn strings() {
        let mut a = Arena::with_capacity(1);
        let h = a.push_str("café").unwrap(); // grows/reallocates
        assert_eq!(h.get_len_prim(), 5);
        assert_eq!(a.as_bytes(), "café".as_bytes());
        assert_eq!(a.read_str(h), Some("café"));
        a.read_str_mut(h).unwrap().make_ascii_uppercase();
        assert_eq!(a.read_str(h), Some("CAFé"));
        assert!(a.replace_str(h, "niño"));
        assert_eq!(a.read_str(h), Some("niño"));
        assert!(!a.replace_str(h, "longer"));
        assert_eq!(a.read_str(h), Some("niño"));
    }
}
