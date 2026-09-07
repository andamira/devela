// devela/src/data/store/intern/string/_test.rs

use crate::{HasherFx, InternStringExample as Symbols};
// use crate::{InternStringSymbolExample as Symbol};

#[test]
fn existing_string_survives_capacity_exhaustion() {
    let mut i = Symbols::<1, 3, 1>::new();
    let a = i.intern("abc").unwrap();
    assert!(i.can_intern("abc"));
    assert!(!i.can_intern("x"));
    assert_eq!(i.intern("abc"), Some(a));
    assert_eq!(i.intern("x"), None);
    assert_eq!(i.len(), 1);
}
#[test]
fn clear_allows_symbol_reuse() {
    let mut i = Symbols::<2, 16, 2>::new();
    let old = i.intern("old").unwrap();
    i.clear();
    assert_eq!(i.get(old), None);
    let new = i.intern("new").unwrap();
    assert_eq!(old, new);
    assert_eq!(i.get(old), Some("new"));
}
#[test]
fn canonicalizes_equal_strings() {
    let mut i = Symbols::<8, 64, 16>::new();
    let a = i.intern("alpha").unwrap();
    let b = i.intern("alpha").unwrap();
    assert_eq!(a, b);
    assert_eq!(i.len(), 1);
    assert_eq!(i.byte_len(), 5);
    assert_eq!(i.get(a), Some("alpha"));
}
#[test]
fn distinct_strings_have_distinct_symbols() {
    let mut i = Symbols::<8, 64, 16>::new();
    let a = i.intern("alpha").unwrap();
    let b = i.intern("beta").unwrap();
    assert_ne!(a, b);
    assert_eq!(i.find("alpha"), Some(a));
    assert_eq!(i.find("beta"), Some(b));
}
#[test]
fn find_does_not_insert() {
    let mut i = Symbols::<8, 64, 16>::new();
    assert_eq!(i.find("alpha"), None);
    assert_eq!(i.len(), 0);
    let a = i.intern("alpha").unwrap();
    assert_eq!(i.find("alpha"), Some(a));
    assert_eq!(i.len(), 1);
}
#[test]
fn empty_string_is_canonical() {
    let mut i = Symbols::<4, 0, 8>::new();
    let a = i.intern("").unwrap();
    let b = i.intern("").unwrap();
    assert_eq!(a, b);
    assert_eq!(i.len(), 1);
    assert_eq!(i.byte_len(), 0);
}
#[test]
fn collisions_compare_strings_not_hashes() {
    const WORDS: &[&str] = &["a", "b", "c", "d", "e", "f", "g", "h"];
    const SLOTS: usize = 3;
    let mut pair = None;
    'outer: for (a_i, &a) in WORDS.iter().enumerate() {
        let ah = HasherFx::<usize>::hash_bytes(a.as_bytes()) % SLOTS;
        for &b in &WORDS[a_i + 1..] {
            let bh = HasherFx::<usize>::hash_bytes(b.as_bytes()) % SLOTS;
            if ah == bh {
                pair = Some((a, b));
                break 'outer;
            }
        }
    }
    let (a_str, b_str) = pair.unwrap();
    let mut i = Symbols::<3, 32, SLOTS>::new();
    let a = i.intern(a_str).unwrap();
    let b = i.intern(b_str).unwrap();
    assert_ne!(a, b);
    assert_eq!(i.find(a_str), Some(a));
    assert_eq!(i.find(b_str), Some(b));
    // Most important: canonicalizing again walks through the collision.
    assert_eq!(i.intern(a_str), Some(a));
    assert_eq!(i.intern(b_str), Some(b));
}
#[test]
fn full_probe_table_terminates() {
    let mut i = Symbols::<2, 32, 2>::new();
    i.intern("one").unwrap();
    i.intern("two").unwrap();
    assert_eq!(i.find("missing"), None);
    assert_eq!(i.intern("missing"), None);
}
#[test]
fn access_and_iteration() {
    let mut i = Symbols::<4, 32, 4>::new();
    let a = i.intern("alpha").unwrap();
    let b = i.intern("beta").unwrap();
    assert!(i.contains(a));
    assert!(i.contains(b));
    let mut strings = i.iter();
    assert_eq!(strings.next(), Some("alpha"));
    assert_eq!(strings.next(), Some("beta"));
    assert_eq!(strings.next(), None);
    let mut symbols = i.symbols();
    assert_eq!(symbols.next(), Some(a));
    assert_eq!(symbols.next(), Some(b));
    assert_eq!(symbols.next(), None);
    let mut entries = i.entries();
    assert_eq!(entries.next(), Some((a, "alpha")));
    assert_eq!(entries.next(), Some((b, "beta")));
    assert_eq!(entries.next(), None);
}

#[cfg(feature = "alloc")]
mod alloc {
    use crate::{InternStringAllocExample as Symbols, ToString};

    #[test]
    fn grows_from_empty_lookup_table() {
        let mut i = Symbols::new();
        assert_eq!(i.slot_capacity(), 0);
        let a = i.intern("alpha").unwrap();
        assert_eq!(i.slot_capacity(), 1);
        assert_eq!(i.get(a), Some("alpha"));
        assert_eq!(i.find("alpha"), Some(a));
    }
    #[test]
    fn lookup_growth_preserves_symbols() {
        let mut i = Symbols::with_capacity(1, 4, 1);
        let a = i.intern("a").unwrap();
        assert_eq!(i.slot_capacity(), 1);
        let b = i.intern("bb").unwrap();
        assert_eq!(i.slot_capacity(), 2);
        let c = i.intern("ccc").unwrap();
        assert_eq!(i.slot_capacity(), 4);
        assert_eq!(i.get(a), Some("a"));
        assert_eq!(i.get(b), Some("bb"));
        assert_eq!(i.get(c), Some("ccc"));
        assert_eq!(i.find("a"), Some(a));
        assert_eq!(i.find("bb"), Some(b));
        assert_eq!(i.find("ccc"), Some(c));
        // Canonicalization remains stable through rehashes.
        assert_eq!(i.intern("a"), Some(a));
        assert_eq!(i.intern("bb"), Some(b));
    }
    #[test]
    fn clear_retains_lookup_storage() {
        let mut i = Symbols::with_capacity(4, 32, 8);
        let old = i.intern("old").unwrap();
        assert_eq!(i.slot_capacity(), 8);
        i.clear();
        assert_eq!(i.len(), 0);
        assert_eq!(i.slot_capacity(), 8);
        assert!(!i.contains(old));
        let new = i.intern("new").unwrap();
        assert_eq!(old, new);
        assert_eq!(i.get(old), Some("new"));
    }
    #[test]
    fn duplicate_does_not_trigger_growth() {
        let mut i = Symbols::with_capacity(1, 3, 1);
        let a = i.intern("abc").unwrap();
        let slots = i.slot_capacity();
        assert_eq!(i.intern("abc"), Some(a));
        assert_eq!(i.slot_capacity(), slots);
    }
    #[test]
    fn alloc_reserves_max_index_for_empty_slot() {
        use crate::InternStringAllocExample as Symbols;
        assert_eq!(Symbols::MAX_CAPACITY, 254);
        let mut i = Symbols::new();
        for n in 0..Symbols::MAX_CAPACITY {
            assert!(i.intern(&n.to_string()).is_some());
        }
        assert_eq!(i.len(), Symbols::MAX_CAPACITY);
        assert!(i.intern("one-too-many").is_none());
    }
}
