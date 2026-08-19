// devela/src/data/store/pool/item/_test.rs

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
#[test]
fn shared_iterator_tracks_exact_limits() {
    let mut pool = Pool::<u8, 6>::new();
    let a = pool.insert(10).unwrap();
    let _b = pool.insert(20).unwrap();
    let c = pool.insert(30).unwrap();
    let _d = pool.insert(40).unwrap();
    assert_eq!(pool.remove(a), Some(10));
    assert_eq!(pool.remove(c), Some(30));
    let mut iter = pool.iter();
    assert_eq!(iter.len(), 2);
    assert!(!iter.is_empty());
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(&40));
    assert_eq!(iter.len(), 0);
    assert!(iter.is_empty());
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.len(), 0);
}
#[test]
fn mutable_iterator_yields_disjoint_values_and_tracks_limits() {
    let mut pool = Pool::<u8, 6>::new();
    let a = pool.insert(10).unwrap();
    let b = pool.insert(20).unwrap();
    let c = pool.insert(30).unwrap();
    let d = pool.insert(40).unwrap();
    assert_eq!(pool.remove(a), Some(10));
    assert_eq!(pool.remove(c), Some(30));
    {
        let mut iter = pool.iter_mut();
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.size_hint(), (2, Some(2)));
        let first = iter.next().unwrap();
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.size_hint(), (1, Some(1)));
        let second = iter.next().unwrap();
        assert_eq!(iter.len(), 0);
        assert!(iter.is_empty());
        // Both exclusive references coexist and target different slots.
        assert!(!core::ptr::eq(first, second));
        *first += 1;
        *second += 2;
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }
    assert_eq!(pool.get(b), Some(&21));
    assert_eq!(pool.get(d), Some(&42));
}
const CONST_ITER_MUTATED: [u8; 2] = {
    let mut pool = Pool::<u8, 4>::new();
    let a = match pool.insert_copy(1) {
        Ok(handle) => handle,
        Err(_) => panic!("unexpected full pool"),
    };
    let b = match pool.insert_copy(2) {
        Ok(handle) => handle,
        Err(_) => panic!("unexpected full pool"),
    };
    let c = match pool.insert_copy(3) {
        Ok(handle) => handle,
        Err(_) => panic!("unexpected full pool"),
    };
    match pool.remove(b) {
        Some(2) => {}
        _ => panic!("unexpected removal"),
    }
    {
        let mut iter = pool.iter_mut();
        while let Some(value) = iter.next() {
            *value += 10;
        }
    }
    [
        match pool.get(a) {
            Some(value) => *value,
            None => panic!("missing first value"),
        },
        match pool.get(c) {
            Some(value) => *value,
            None => panic!("missing second value"),
        },
    ]
};
#[test]
fn mutable_iteration_works_during_const_evaluation() {
    assert_eq!(CONST_ITER_MUTATED, [11, 13]);
}
#[test]
fn reference_into_iteration_skips_vacant_slots() {
    let mut pool = Pool::<u8, 4>::new();
    let a = pool.insert(1).unwrap();
    let b = pool.insert(2).unwrap();
    let c = pool.insert(3).unwrap();
    assert_eq!(pool.remove(b), Some(2));
    let mut sum = 0;
    let mut count = 0;
    for value in &pool {
        sum += *value;
        count += 1;
    }
    assert_eq!(sum, 4);
    assert_eq!(count, 2);
    let mut count = 0;
    for value in &mut pool {
        *value += 10;
        count += 1;
    }
    assert_eq!(count, 2);
    assert_eq!(pool.get(a), Some(&11));
    assert_eq!(pool.get(c), Some(&13));
}

#[cfg(feature = "alloc")]
mod alloc {
    use crate::PoolAllocExample as Pool;

    #[test]
    fn allocated_pool_grows_reuses_and_clears() {
        let mut pool = Pool::<u8>::new();
        assert_eq!(pool.len(), 0);
        assert_eq!(pool.slot_count(), 0);
        assert_eq!(pool.remaining(), pool.capacity());
        assert!(!pool.is_full());
        let a = pool.insert(1).unwrap();
        let b = pool.insert(2).unwrap();
        assert_eq!(pool.slot_count(), 2);
        assert_eq!(pool.remaining(), pool.capacity() - pool.len());
        assert_eq!(pool.remove(a), Some(1));
        let slots = pool.slot_count();
        let c = pool.insert(3).unwrap();
        assert_eq!(pool.slot_count(), slots);
        assert_eq!(a.index_prim(), c.index_prim());
        assert_ne!(a.generation_prim(), c.generation_prim());
        assert_eq!(pool.get(a), None);
        pool.clear();
        assert_eq!(pool.len(), 0);
        assert_eq!(pool.slot_count(), slots);
        assert_eq!(pool.get(b), None);
        assert_eq!(pool.get(c), None);
    }
    #[test]
    #[should_panic(expected = "exceeds its index representation")]
    fn with_capacity_rejects_unrepresentable_capacity() {
        let _ = Pool::<u8>::with_capacity(256);
    }
}
