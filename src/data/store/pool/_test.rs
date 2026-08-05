// devela/src/data/store/pool/_test.rs

use crate::{PoolExample as Pool, PoolHandleExample as Handle};

#[test]
fn empty_and_capacity() {
    let pool = Pool::<u8, 3>::new();
    assert_eq!(pool.capacity(), 3);
    assert_eq!(pool.len(), 0);
    assert_eq!(pool.remaining(), 3);
    assert!(pool.is_empty());
    assert!(!pool.is_full());
}
#[test]
fn insertion_and_access() {
    let mut pool = Pool::<&str, 2>::new();
    let a = pool.insert("a").unwrap();
    let b = pool.insert("b").unwrap();
    assert_eq!(pool.get(a), Some(&"a"));
    assert_eq!(pool.get(b), Some(&"b"));
    assert!(pool.contains(a));
    assert!(pool.is_full());
    assert_eq!(pool.insert("c"), Err("c"));
}
#[test]
fn removal_preserves_unrelated_handles() {
    let mut pool = Pool::<&str, 3>::new();
    let a = pool.insert("a").unwrap();
    let b = pool.insert("b").unwrap();
    assert_eq!(pool.remove(a), Some("a"));
    assert_eq!(pool.get(a), None);
    assert_eq!(pool.get(b), Some(&"b"));
    assert_eq!(pool.remove(a), None);
}
#[test]
fn reuse_invalidates_the_previous_handle() {
    let mut pool = Pool::<&str, 1>::new();
    let old = pool.insert("old").unwrap();
    assert_eq!(pool.remove(old), Some("old"));
    let new = pool.insert("new").unwrap();
    assert_eq!(old.index_prim(), new.index_prim());
    assert_ne!(old.generation_prim(), new.generation_prim());
    assert_eq!(pool.get(old), None);
    assert_eq!(pool.remove(old), None);
    assert_eq!(pool.get(new), Some(&"new"));
}
#[test]
fn zero_capacity() {
    let mut pool = Pool::<u8, 0>::new();
    assert!(pool.is_empty());
    assert!(pool.is_full());
    assert_eq!(pool.insert(7), Err(7));
}
#[test]
fn clear_invalidates_live_handles() {
    let mut pool = Pool::<&str, 3>::new();
    let a = pool.insert("a").unwrap();
    let b = pool.insert("b").unwrap();
    pool.clear();
    assert!(pool.is_empty());
    assert_eq!(pool.get(a), None);
    assert_eq!(pool.get(b), None);
    let c = pool.insert("c").unwrap();
    assert_eq!(pool.get(c), Some(&"c"));
    assert_eq!(pool.get(a), None);
}
#[test]
fn replacement_preserves_the_handle() {
    let mut pool = Pool::<&str, 2>::new();
    let handle = pool.insert("old").unwrap();
    assert_eq!(pool.replace(handle, "new"), Ok("old"));
    assert_eq!(pool.get(handle), Some(&"new"));
    assert_eq!(pool.remove(handle), Some("new"));
    assert_eq!(pool.replace(handle, "unused"), Err("unused"));
}
#[test]
fn two_mutable_values_are_distinct_and_ordered() {
    let mut pool = Pool::<&str, 3>::new();
    let a = pool.insert("a").unwrap();
    let b = pool.insert("b").unwrap();
    {
        let (b_value, a_value) = pool.get2_mut(b, a).unwrap();
        *b_value = "B";
        *a_value = "A";
    }
    assert_eq!(pool.get(a), Some(&"A"));
    assert_eq!(pool.get(b), Some(&"B"));
    assert!(pool.get2_mut(a, a).is_none());
    pool.remove(a);
    assert!(pool.get2_mut(a, b).is_none());
}
#[test]
fn value_iteration_skips_vacant_slots() {
    let mut pool = Pool::<u8, 4>::new();
    let a = pool.insert(1).unwrap();
    let b = pool.insert(2).unwrap();
    let c = pool.insert(3).unwrap();
    assert_eq!(pool.remove(b), Some(2));
    assert_eq!(pool.iter().copied().sum::<u8>(), 4);
    for value in pool.iter_mut() {
        *value += 10;
    }
    assert_eq!(pool.get(a), Some(&11));
    assert_eq!(pool.get(c), Some(&13));
}
#[test]
fn handles_and_entries_resolve() {
    let mut pool = Pool::<&str, 4>::new();
    let a = pool.insert("a").unwrap();
    let b = pool.insert("b").unwrap();
    let c = pool.insert("c").unwrap();
    pool.remove(b);
    assert_eq!(pool.handles().count(), 2);
    assert!(pool.handles().all(|handle| pool.contains(handle)));
    let mut entries = 0;
    for (handle, value) in pool.entries() {
        assert_eq!(pool.get(handle), Some(value));
        entries += 1;
    }
    assert_eq!(entries, pool.len());
    assert!(pool.contains(a));
    assert!(pool.contains(c));
}
#[test]
fn stale_relationship_does_not_resolve_to_replacement() {
    #[derive(Clone, Copy, Debug)]
    struct Node {
        parent: Option<Handle>,
    }
    let mut pool = Pool::<Node, 2>::new();
    let parent = pool.insert(Node { parent: None }).unwrap();
    let child = pool.insert(Node { parent: Some(parent) }).unwrap();
    assert!(pool.remove(parent).is_some());
    let replacement = pool.insert(Node { parent: None }).unwrap();
    assert_eq!(parent.index_prim(), replacement.index_prim());
    assert_ne!(parent.generation_prim(), replacement.generation_prim());
    let stale_parent = pool.get(child).unwrap().parent.unwrap();
    assert!(pool.get(stale_parent).is_none());
}
