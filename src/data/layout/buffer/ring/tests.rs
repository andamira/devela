// devela::data::layout::buffer::ring::tests

use super::*;

type Ring = BufferRingStaticExample<i32, [Option<i32>; 4]>;

#[test]
fn option_basic_fifo() {
    let mut buf = Ring::new();
    assert!(buf.is_empty());
    assert_eq!(buf.capacity_prim(), 4);
    assert_eq!(buf.push_back(10), Ok(()));
    assert_eq!(buf.push_back(20), Ok(()));
    assert_eq!(buf.len_prim(), 2);
    assert_eq!(buf.peek_front(), Some(&10));
    assert_eq!(buf.peek_back(), Some(&20));
    assert_eq!(buf.pop_front(), Some(10));
    assert_eq!(buf.pop_front(), Some(20));
    assert_eq!(buf.pop_front(), None);
    assert!(buf.is_empty());
}
#[test]
#[cfg(feature = "alloc")]
fn option_wraparound_order() {
    let mut buf = Ring::new();
    assert_eq!(buf.push_back(1), Ok(()));
    assert_eq!(buf.push_back(2), Ok(()));
    assert_eq!(buf.push_back(3), Ok(()));
    assert_eq!(buf.pop_front(), Some(1));
    assert_eq!(buf.push_back(4), Ok(()));
    assert_eq!(buf.push_back(5), Ok(()));
    assert!(buf.is_full());
    assert_eq!(buf.iter().copied().collect::<crate::Vec<_>>(), [2, 3, 4, 5]);
    assert_eq!(buf.pop_front(), Some(2));
    assert_eq!(buf.pop_back(), Some(5));
    assert_eq!(buf.iter().copied().collect::<crate::Vec<_>>(), [3, 4]);
}
#[test]
fn option_as_slices_wraparound() {
    let mut buf = Ring::new();
    for n in [1, 2, 3] {
        buf.push_back(n).unwrap();
    }
    assert_eq!(buf.pop_front(), Some(1));
    buf.push_back(4).unwrap();
    buf.push_back(5).unwrap();
    let (a, b) = buf.as_slices();
    assert_eq!(a, &[Some(2), Some(3), Some(4)]);
    assert_eq!(b, &[Some(5)]);
}
#[test]
#[cfg(feature = "alloc")]
fn option_from_array_ring() {
    let buf = Ring::from_array_ring_prim([Some(30), None, Some(10), Some(20)], 2, 3).unwrap();
    assert_eq!(buf.iter().copied().collect::<crate::Vec<_>>(), [10, 20, 30]);
}
#[test]
fn option_from_array_ring_rejects_wrong_occupancy() {
    assert!(Ring::from_array_ring_prim([Some(30), Some(99), Some(10), Some(20)], 2, 3).is_none());
}
#[test]
#[cfg(feature = "alloc")]
fn option_push_back_slice_wraparound() {
    let mut buf = Ring::new();
    buf.push_back_slice_copy(&[1, 2, 3]);
    assert_eq!(buf.pop_front(), Some(1));
    assert_eq!(buf.push_back_slice_copy(&[4, 5, 6]), 2);
    assert_eq!(buf.iter().copied().collect::<crate::Vec<_>>(), [2, 3, 4, 5]);
    assert!(buf.is_full());
}
#[test]
#[cfg(feature = "alloc")]
fn option_push_back_slice_copy_exact() {
    let mut buf = Ring::new();
    assert_eq!(buf.push_back_slice_copy_exact(&[1, 2, 3]), Ok(()));
    assert_eq!(buf.push_back_slice_copy_exact(&[4, 5]), Err(1));
    assert_eq!(buf.iter().copied().collect::<crate::Vec<_>>(), [1, 2, 3]);
}
#[test]
#[cfg(feature = "alloc")]
fn option_push_front_slice_wraparound() {
    let mut buf = Ring::new();
    assert_eq!(buf.push_back_slice_copy(&[10, 20]), 2);
    assert_eq!(buf.push_front_slice_copy(&[1, 2, 3]), 2);
    assert_eq!(buf.iter().copied().collect::<crate::Vec<_>>(), [1, 2, 10, 20]);
}
#[test]
#[cfg(feature = "alloc")]
fn option_swap_remove() {
    let mut buf = Ring::new();
    buf.push_back_slice_copy(&[1, 2, 3, 4]);
    assert_eq!(buf.swap_remove_prim(1), Ok(Some(2)));
    assert_eq!(buf.len_prim(), 3);
    // Order is intentionally not preserved.
    assert_eq!(buf.iter().copied().collect::<crate::Vec<_>>(), [1, 4, 3]);
}
#[test]
fn option_clear_resets_head_after_wrap() {
    let mut buf = Ring::new();
    buf.push_back_slice_copy(&[1, 2, 3]);
    assert_eq!(buf.pop_front(), Some(1));
    buf.push_back(4).unwrap();
    buf.clear();
    assert!(buf.is_empty());
    buf.push_back(9).unwrap();
    assert_eq!(buf.pop_front(), Some(9));
}
