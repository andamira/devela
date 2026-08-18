// devela/src/data/store/pool/seq/_test.rs

use crate::PoolSeqExample as Pool;

#[test]
fn insert_access_remove_reuse() {
    let mut pool = Pool::<u8, 4, 16>::new_init();
    let a = pool.insert(b"abc").unwrap();
    let b = pool.insert(b"defg").unwrap();
    assert_eq!(pool.get(a), Some(b"abc".as_slice()));
    assert_eq!(pool.get(b), Some(b"defg".as_slice()));
    assert!(pool.remove(a));
    assert!(!pool.contains(a));
    let c = pool.insert(b"xy").unwrap();
    assert_eq!(pool.get(c), Some(b"xy".as_slice()));
}
#[test]
fn fragmentation_is_observable() {
    let mut pool = Pool::<u8, 4, 16>::new_init();
    let a = pool.insert(&[1; 4]).unwrap();
    let b = pool.insert(&[2; 4]).unwrap();
    let c = pool.insert(&[3; 4]).unwrap();
    assert!(pool.remove(b));
    assert_eq!(pool.cell_remaining(), 8);
    assert_eq!(pool.largest_free_span(), 4);
    assert!(pool.is_fragmented_for(6));
    assert!(pool.insert(&[9; 6]).is_none());
    // Removing the trailing allocation lowers the frontier and
    // absorbs the preceding free span.
    assert!(pool.remove(c));
    assert_eq!(pool.largest_free_span(), 12);
    assert!(!pool.is_fragmented_for(6));
    assert!(pool.insert(&[9; 6]).is_some());
    assert_eq!(pool.get(a), Some(&[1; 4][..]));
}
#[test]
fn truncate_and_shrink_are_distinct() {
    let mut pool = Pool::<u8, 2, 16>::new_init();
    let a = pool.insert(b"abcdefgh").unwrap();
    assert!(pool.truncate(a, 3));
    assert_eq!(pool.seq_len(a), Some(3));
    assert_eq!(pool.seq_capacity(a), Some(8));
    assert_eq!(pool.cell_len(), 3);
    assert_eq!(pool.allocated_cell_len(), 8);
    assert!(pool.shrink_to_fit(a));
    assert_eq!(pool.seq_capacity(a), Some(3));
    assert_eq!(pool.allocated_cell_len(), 3);
}
#[test]
fn stale_handle_is_rejected() {
    let mut pool = Pool::<u8, 1, 8>::new_init();
    let old = pool.insert(b"a").unwrap();
    assert!(pool.remove(old));
    let new = pool.insert(b"b").unwrap();
    assert_ne!(old, new);
    assert_eq!(pool.get(old), None);
    assert_eq!(pool.get(new), Some(b"b".as_slice()));
}
#[test]
fn push_can_relocate_without_changing_identity() {
    let mut pool = Pool::<u8, 4, 16>::new_init();
    let a = pool.insert(b"abc").unwrap();
    let b = pool.insert(b"WXYZ").unwrap();
    // `b` occupies the cells immediately after `a`,
    // so growing `a` requires relocation.
    assert_eq!(pool.push(a, b'd'), Ok(()));
    assert_eq!(pool.get(a), Some(b"abcd".as_slice()));
    assert_eq!(pool.get(b), Some(b"WXYZ".as_slice()));
    assert!(pool.contains(a));
    assert_eq!(pool.seq_len(a), Some(4));
    assert_eq!(pool.seq_capacity(a), Some(4));
}
#[test]
fn reserve_uses_existing_capacity() {
    let mut pool = Pool::<u8, 2, 16>::new_init();
    let a = pool.insert(b"abcdef").unwrap();
    assert!(pool.truncate(a, 3));
    assert_eq!(pool.seq_capacity(a), Some(6));
    assert_eq!(pool.seq_remaining(a), Some(3));
    assert!(pool.reserve_exact(a, 2));
    assert_eq!(pool.seq_capacity(a), Some(6));
    assert_eq!(pool.push(a, b'd'), Ok(()));
    assert_eq!(pool.get(a), Some(b"abcd".as_slice()));
}
#[test]
fn extend_is_all_or_nothing() {
    let mut pool = Pool::<u8, 2, 8>::new_init();
    let a = pool.insert(b"abcd").unwrap();
    let b = pool.insert(b"WXYZ").unwrap();
    assert!(!pool.extend_from_slice(a, b"ef"));
    assert_eq!(pool.get(a), Some(b"abcd".as_slice()));
    assert_eq!(pool.get(b), Some(b"WXYZ".as_slice()));
    assert_eq!(pool.cell_len(), 8);
}
#[test]
fn pop_preserves_reserved_capacity() {
    let mut pool = Pool::<u8, 1, 8>::new_init();
    let a = pool.insert(b"abcd").unwrap();
    assert_eq!(pool.pop(a), Some(b'd'));
    assert_eq!(pool.get(a), Some(b"abc".as_slice()));
    assert_eq!(pool.seq_len(a), Some(3));
    assert_eq!(pool.seq_capacity(a), Some(4));
    assert_eq!(pool.pop(a), Some(b'c'));
    assert_eq!(pool.pop(a), Some(b'b'));
    assert_eq!(pool.pop(a), Some(b'a'));
    assert_eq!(pool.pop(a), None);
    assert_eq!(pool.seq_capacity(a), Some(4));
}
#[test]
fn clear_resets_pool_and_invalidates_handles() {
    let mut pool = Pool::<u8, 3, 16>::new_init();
    let a = pool.insert(b"abc").unwrap();
    let b = pool.insert(b"defg").unwrap();
    pool.clear();
    assert!(pool.is_empty());
    assert_eq!(pool.cell_len(), 0);
    assert_eq!(pool.allocated_cell_len(), 0);
    assert_eq!(pool.cell_remaining(), 16);
    assert_eq!(pool.largest_free_span(), 16);
    assert!(!pool.contains(a));
    assert!(!pool.contains(b));
    assert_eq!(pool.get(a), None);
    assert_eq!(pool.get(b), None);
    let c = pool.insert(b"x").unwrap();
    assert_ne!(c, a);
    assert_eq!(pool.get(c), Some(b"x".as_slice()));
}
#[test]
fn compact_removes_fragmentation_but_preserves_capacity() {
    let mut pool = Pool::<u8, 4, 16>::new_init();
    let a = pool.insert(&[1; 4]).unwrap();
    let b = pool.insert(&[2; 4]).unwrap();
    let c = pool.insert(&[3; 4]).unwrap();
    let d = pool.insert(&[4; 4]).unwrap();
    assert!(pool.remove(b));
    assert!(pool.remove(d));
    assert_eq!(pool.cell_remaining(), 8);
    assert_eq!(pool.largest_free_span(), 4);
    assert!(pool.is_fragmented_for(6));
    assert!(!pool.can_insert(6));
    pool.compact();
    // Identity and contents survive relocation.
    assert_eq!(pool.get(a), Some(&[1; 4][..]));
    assert_eq!(pool.get(c), Some(&[3; 4][..]));
    // Reservations are preserved.
    assert_eq!(pool.seq_capacity(a), Some(4));
    assert_eq!(pool.seq_capacity(c), Some(4));
    assert_eq!(pool.allocated_cell_len(), 8);
    // All free space is now trailing and contiguous.
    assert_eq!(pool.largest_free_span(), 8);
    assert!(!pool.is_fragmented_for(6));
    assert!(pool.can_insert(6));
}
#[test]
fn pack_releases_all_sequence_slack() {
    let mut pool = Pool::<u8, 3, 20>::new_init();
    let a = pool.insert(b"abcdef").unwrap();
    let b = pool.insert(b"WXYZ").unwrap();
    assert!(pool.truncate(a, 3));
    assert_eq!(pool.pop(b), Some(b'Z'));
    assert_eq!(pool.get(a), Some(b"abc".as_slice()));
    assert_eq!(pool.get(b), Some(b"WXY".as_slice()));
    assert_eq!(pool.seq_capacity(a), Some(6));
    assert_eq!(pool.seq_capacity(b), Some(4));
    assert_eq!(pool.cell_len(), 6);
    assert_eq!(pool.allocated_cell_len(), 10);
    pool.pack();
    assert_eq!(pool.get(a), Some(b"abc".as_slice()));
    assert_eq!(pool.get(b), Some(b"WXY".as_slice()));
    assert_eq!(pool.seq_capacity(a), Some(3));
    assert_eq!(pool.seq_capacity(b), Some(3));
    assert_eq!(pool.cell_len(), 6);
    assert_eq!(pool.allocated_cell_len(), 6);
    assert_eq!(pool.cell_remaining(), 14);
    assert_eq!(pool.largest_free_span(), 14);
}
#[test]
fn compact_follows_physical_not_descriptor_order() {
    let mut pool = Pool::<u8, 4, 24>::new_init();
    let a = pool.insert(b"aaaa").unwrap(); // descriptor 0, physical first
    let b = pool.insert(b"bbbb").unwrap(); // descriptor 1
    let c = pool.insert(b"cccc").unwrap(); // descriptor 2
    // `a` cannot grow in place because `b` follows it,
    // so it relocates beyond `c`.
    assert!(pool.reserve_exact(a, 4));
    assert_eq!(pool.seq_capacity(a), Some(8));
    assert_eq!(pool.get(a), Some(b"aaaa".as_slice()));
    // Make another interior gap.
    assert!(pool.remove(b));
    pool.compact();
    // Slot order is now A,C while physical order was C,A before compaction.
    // Both must survive intact.
    assert_eq!(pool.get(a), Some(b"aaaa".as_slice()));
    assert_eq!(pool.get(c), Some(b"cccc".as_slice()));
    assert_eq!(pool.seq_capacity(a), Some(8));
    assert_eq!(pool.seq_capacity(c), Some(4));
}
