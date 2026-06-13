// devela/src/sys/mem/view/slice/tests.rs

use crate::{
    IteratorLending, IteratorLendingDoubleEnded, IteratorLendingExactSize, IteratorLendingPeek,
    SliceIter, SliceIterMut,
};

#[test]
fn slice_iter_peek_respects_back() {
    let array = [1, 2];
    let mut it = SliceIter::new(&array);
    assert_eq!(it.len(), 2);
    assert_eq!(it.peek(), Some(&1));
    assert_eq!(it.next_back(), Some(&2));
    assert_eq!(it.len(), 1);
    assert_eq!(it.peek(), Some(&1));
    assert_eq!(it.next_back(), Some(&1));
    assert_eq!(it.len(), 0);
    assert_eq!(it.peek(), None);
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}
#[test]
fn slice_iter_mixed_front_back_order() {
    let array = [10, 20, 30, 40];
    let mut it = SliceIter::new(&array);
    assert_eq!(it.size_hint(), (4, Some(4)));
    assert_eq!(it.peek(), Some(&10));
    assert_eq!(it.next(), Some(&10));
    assert_eq!(it.size_hint(), (3, Some(3)));
    assert_eq!(it.next_back(), Some(&40));
    assert_eq!(it.peek(), Some(&20));
    assert_eq!(it.size_hint(), (2, Some(2)));
    assert_eq!(it.next_back(), Some(&30));
    assert_eq!(it.next(), Some(&20));
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert!(it.is_empty());
    assert_eq!(it.peek(), None);
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}
#[test]
fn slice_iter_empty() {
    let array: [u8; 0] = [];
    let mut it = SliceIter::new(&array);
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert!(it.is_empty());
    assert_eq!(it.peek(), None);
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}
#[test]
fn slice_iter_mut_mixed_front_back_mutation() {
    let mut array = [1, 2, 3, 4];
    {
        let mut it = SliceIterMut::new(&mut array);
        assert_eq!(it.size_hint(), (4, Some(4)));
        *it.next().unwrap() += 10; // 1 -> 11
        assert_eq!(it.size_hint(), (3, Some(3)));
        *it.next_back().unwrap() += 40; // 4 -> 44
        assert_eq!(it.size_hint(), (2, Some(2)));
        *it.next_back().unwrap() += 30; // 3 -> 33
        assert_eq!(it.size_hint(), (1, Some(1)));
        *it.next().unwrap() += 20; // 2 -> 22
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.is_empty());
        assert!(it.next().is_none());
        assert!(it.next_back().is_none());
    }
    assert_eq!(array, [11, 22, 33, 44]);
}
#[test]
fn slice_iter_mut_empty() {
    let mut array: [u8; 0] = [];
    let mut it = SliceIterMut::new(&mut array);
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert!(it.is_empty());
    assert!(it.next().is_none());
    assert!(it.next_back().is_none());
}
