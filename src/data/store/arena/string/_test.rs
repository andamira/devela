// devela/src/data/store/arena/string/_test.rs

use crate::{NonMaxU8, NonMaxU16, arena_string};

arena_string! {
    [
        index: u8 + NonMaxU8;
        cursor: u16 + NonMaxU16;
    ]

    pub Strings;
    pub StringId;
    pub StringMark;
}

#[test]
fn basic() {
    let mut s = Strings::<8, 32>::new();
    let a = s.insert("hello").unwrap();
    let b = s.insert("λ").unwrap();
    assert_eq!(s.get(a), Some("hello"));
    assert_eq!(s.get(b), Some("λ"));
    assert_eq!(s.len(), 2);
    assert_eq!(s.byte_len(), 7);
    assert_eq!(s.as_bytes(), "helloλ".as_bytes());
}
#[test]
fn duplicates_have_distinct_identity() {
    let mut s = Strings::<4, 16>::new();
    let a = s.insert("x").unwrap();
    let b = s.insert("x").unwrap();
    assert_ne!(a, b);
    assert_eq!(s.get(a), Some("x"));
    assert_eq!(s.get(b), Some("x"));
}
#[test]
fn empty_strings_use_identity_but_no_bytes() {
    let mut s = Strings::<4, 0>::new();
    let a = s.insert("").unwrap();
    let b = s.insert("").unwrap();
    assert_ne!(a, b);
    assert_eq!(s.len(), 2);
    assert_eq!(s.byte_len(), 0);
}
#[test]
fn string_and_byte_capacity_are_independent() {
    let mut s = Strings::<4, 1>::new();
    assert!(s.insert("x").is_some());
    assert_eq!(s.byte_remaining(), 0);
    // Still legal: an empty string needs an identity, but no bytes.
    assert!(s.insert("").is_some());
    assert!(s.insert("y").is_none());
}
#[test]
fn rollback_restores_both_frontiers() {
    let mut s = Strings::<8, 32>::new();
    let a = s.insert("one").unwrap();
    let mark = s.mark();
    let old = s.insert("two").unwrap();
    assert_eq!(s.as_bytes(), b"onetwo");
    assert!(s.rollback(mark));
    assert_eq!(s.as_bytes(), b"one");
    assert!(!s.contains(old));
    let new = s.insert("x").unwrap();
    // Arena coordinates can be reused.
    assert_eq!(old, new);
    assert_eq!(s.get(new), Some("x"));
    assert_eq!(s.get(a), Some("one"));
    assert_eq!(s.as_bytes(), b"onex");
}

#[cfg(feature = "alloc")]
mod alloc {
    use super::*;

    arena_string! {
        [
            index: u8 + NonMaxU8;
            cursor: u16 + NonMaxU16;
        ]
        pub StringsAlloc: alloc;
        pub StringAllocId;
        pub StringAllocMark;
    }

    #[test]
    fn alloc_grows_beyond_initial_capacity() {
        let mut s = StringsAlloc::with_capacity(1, 1);
        let a = s.insert("hello").unwrap();
        let b = s.insert("λ-world").unwrap();
        assert_eq!(s.get(a), Some("hello"));
        assert_eq!(s.get(b), Some("λ-world"));
        assert_eq!(s.len(), 2);
        assert_eq!(s.as_bytes(), "helloλ-world".as_bytes());
    }
    #[test]
    fn alloc_rollback_restores_both_frontiers() {
        let mut s = StringsAlloc::new();
        let a = s.insert("one").unwrap();
        let mark = s.mark();
        let old = s.insert("two").unwrap();
        assert!(s.rollback(mark));
        assert_eq!(s.get(a), Some("one"));
        assert!(!s.contains(old));
        assert_eq!(s.as_bytes(), b"one");
        let new = s.insert("x").unwrap();
        assert_eq!(old, new);
        assert_eq!(s.as_bytes(), b"onex");
    }
    #[test]
    fn alloc_uses_entire_index_domain() {
        let mut s = StringsAlloc::new();
        // NonMaxU8 represents 0..=254, therefore all 255 values
        // can serve as allocating-arena string IDs.
        for _ in 0..255 {
            assert!(s.insert("").is_some());
        }
        assert_eq!(s.len(), 255);
        assert!(s.is_full());
        assert!(s.insert("").is_none());
    }
}
