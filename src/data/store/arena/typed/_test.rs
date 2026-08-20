// devela/src/data/store/arena/typed/_test.rs

use crate::{ArenaExample as Arena, ArenaHandleExample as Handle};

#[test]
fn static_insert_access_and_iteration() {
    let mut arena = Arena::<i32, 4>::new();
    let a = arena.insert(10).unwrap();
    let b = arena.insert(20).unwrap();
    assert_eq!(a.get_index_usize(), Ok(0));
    assert_eq!(b.get_index_usize(), Ok(1));
    assert!(arena.contains(a));
    assert_eq!(arena.get(a), Some(&10));
    {
        let mut iter = arena.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), None);
    }
    *arena.get_mut(b).unwrap() = 25;
    assert_eq!(arena.get(b), Some(&25));
}
#[test]
fn static_get2_mut_preserves_handle_order() {
    let mut arena = Arena::<i32, 4>::new();
    let a = arena.insert(1).unwrap();
    let b = arena.insert(2).unwrap();
    let (b_value, a_value) = arena.get2_mut(b, a).unwrap();
    *b_value = 20;
    *a_value = 10;
    assert_eq!(arena.get(a), Some(&10));
    assert_eq!(arena.get(b), Some(&20));
    assert!(arena.get2_mut(a, a).is_none());
}
#[test]
fn static_mark_rollback_and_index_reuse() {
    let mut arena = Arena::<&str, 4>::new();
    let kept = arena.insert("kept").unwrap();
    let mark = arena.mark();
    let old = arena.insert("old").unwrap();
    assert!(arena.rollback(mark));
    assert_eq!(arena.get(kept), Some(&"kept"));
    assert!(!arena.contains(old));
    let new = arena.insert("new").unwrap();
    assert_eq!(old, new);
    assert_eq!(arena.get(old), Some(&"new"));
}
#[test]
fn static_rejects_future_mark() {
    let mut a = Arena::<i32, 4>::new();
    let mut b = Arena::<i32, 4>::new();
    a.insert(1).unwrap();
    b.insert(1).unwrap();
    b.insert(2).unwrap();
    let future = b.mark();
    assert!(!a.rollback(future));
    assert_eq!(a.len(), 1);
}
#[test]
fn static_clear_and_copy_variants() {
    const RESULT: (usize, bool) = {
        let mut arena = Arena::<i32, 4>::new();
        let _ = arena.insert_copy(1);
        let mark = arena.mark();
        let _ = arena.insert_copy(2);
        let rolled = arena.rollback_copy(mark);
        arena.clear_copy();
        (arena.len(), rolled)
    };
    assert_eq!(RESULT, (0, true));
    #[derive(Debug)]
    struct NeedsDrop;
    impl Drop for NeedsDrop {
        fn drop(&mut self) {}
    }
    let mut arena = Arena::<NeedsDrop, 4>::new();
    arena.insert(NeedsDrop).unwrap();
    arena.insert(NeedsDrop).unwrap();
    arena.clear();
    assert!(arena.is_empty());
}
#[test]
fn static_handle_bounds() {
    let mut arena = Arena::<i32, 2>::new();
    let _ = arena.insert(1).unwrap();
    assert!(arena.get(Handle::try_from_usize(1).unwrap()).is_none());
    let _ = arena.insert(2).unwrap();
    assert!(arena.insert(3).is_err());
}
#[test]
fn static_max_capacity_matches_frontier_repr() {
    assert_eq!(Arena::<(), 1>::MAX_CAPACITY, 254);
    let mut arena = Arena::<(), 254>::new();
    for _ in 0..254 {
        assert!(arena.insert(()).is_ok());
    }
    assert_eq!(arena.len(), 254);
    assert!(arena.is_full());
    assert!(arena.insert(()).is_err());
}

/**
```compile_fail, E0080
# use devela::ArenaExample as Arena;
let _ = Arena::<(), 255>::new();
```
**/
#[allow(dead_code)]
fn static_rejects_unrepresentable_frontier() {}

#[test]
fn static_handle_and_mark_are_compact() {
    use crate::ArenaMarkExample as Mark;
    assert_eq!(size_of::<Handle>(), 1);
    assert_eq!(size_of::<Mark>(), 1);
    assert_eq!(size_of::<Option<Handle>>(), 1);
    assert_eq!(size_of::<Option<Mark>>(), 1);
}

#[cfg(feature = "alloc")]
mod alloc {
    #[cfg(feature = "alloc")]
    crate::arena! {
        [index: u8 + crate::NonMaxU8;]
        ArenaAllocMarked: alloc;
        ArenaAllocMarkedHandle;
        ArenaAllocMarkedMark;
    }

    #[test]
    fn alloc_insert_get2_mut_clear_and_reuse() {
        use crate::ArenaAllocExample as ArenaAlloc;
        let mut arena = ArenaAlloc::<i32>::with_capacity(2);
        let a = arena.insert(1).unwrap();
        let old = arena.insert(2).unwrap();
        let (a_value, old_value) = arena.get2_mut(a, old).unwrap();
        *a_value = 10;
        *old_value = 20;
        {
            let mut iter = arena.iter();
            assert_eq!(iter.next(), Some(&10));
            assert_eq!(iter.next(), Some(&20));
            assert_eq!(iter.next(), None);
        }
        arena.clear();
        assert!(!arena.contains(old));
        let _ = arena.insert(20).unwrap();
        let new = arena.insert(30).unwrap();
        assert_eq!(old, new);
        assert_eq!(arena.get(old), Some(&30));
    }
    #[test]
    fn alloc_mark_rollback_and_reuse() {
        let mut arena = ArenaAllocMarked::<i32>::with_capacity(2);
        let kept = arena.insert(10).unwrap();
        let mark = arena.mark();
        let old = arena.insert(20).unwrap();
        assert!(arena.rollback(mark));
        assert_eq!(arena.get(kept), Some(&10));
        assert!(!arena.contains(old));
        let new = arena.insert(30).unwrap();
        assert_eq!(old, new);
        assert_eq!(arena.get(old), Some(&30));
    }
}
