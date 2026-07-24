// devela/src/data/layout/buffer/ring/_test.rs

use super::*;
use crate::{ConstInit, const_assert};

#[derive(Debug, PartialEq, Eq)]
struct Item(u8);
impl ConstInit for Item {
    const INIT: Self = Self(0);
}

mod array {
    use super::*;

    type RingArray = BufferRingStaticExample<i32, [i32; 4]>;
    crate::items! {
        #[derive(Debug, Default, PartialEq, Eq)]
        struct Token(i32);
        type RingToken = BufferRingStaticExample<Token, [Token; 4]>;
    }
    fn assert_ring_eq(buf: &RingArray, expected: &[i32]) {
        assert_eq!(buf.len_prim() as usize, expected.len());
        assert!(buf.iter().copied().eq(expected.iter().copied()));
    }

    #[test]
    const fn array_ring_pop_noncopy_is_const() {
        const RING_POP: (Option<Item>, usize) = {
            let mut ring =
                BufferRingStaticExample::<Item, [Item; 2]>::from_array_full([Item(10), Item(20)]);
            let value = ring.pop_back();
            (value, ring.len().get() as usize)
        };
        const_assert!(eq RING_POP.0.unwrap().0, Item(20).0);
        const_assert!(eq RING_POP.1, 1);
    }
    #[test]
    fn basic_fifo() {
        let mut buf = RingArray::new_init();
        assert_eq!(buf.push_back(10), Ok(()));
        assert_eq!(buf.push_back(20), Ok(()));
        assert_eq!(buf.pop_front_copy(), Some(10));
        assert_eq!(buf.pop_front_copy(), Some(20));
        assert_eq!(buf.pop_front_copy(), None);
    }
    #[test]
    fn wraparound_order() {
        let mut buf = RingArray::new_init();
        assert_eq!(buf.push_back_slice_copy(&[1, 2, 3]), 3);
        assert_eq!(buf.pop_front_copy(), Some(1));
        assert_eq!(buf.push_back_slice_copy(&[4, 5]), 2);
        assert_ring_eq(&buf, &[2, 3, 4, 5]);
    }
    #[test]
    fn push_front_slice_order() {
        let mut buf = RingArray::new_init();
        assert_eq!(buf.push_back_slice_copy(&[10, 20]), 2);
        assert_eq!(buf.push_front_slice_copy(&[1, 2]), 2);
        assert_ring_eq(&buf, &[1, 2, 10, 20]);
    }
    #[test]
    fn pop_init_moves_value() {
        let mut buf = RingArray::new_init();
        assert_eq!(buf.push_back_slice_copy(&[1, 2, 3]), 3);
        assert_eq!(buf.pop_front(), Some(1));
        assert_eq!(buf.pop_back(), Some(3));
        assert_ring_eq(&buf, &[2]);
    }
    #[test]
    fn swap_remove_init() {
        let mut buf = RingArray::new_init();
        assert_eq!(buf.push_back_slice_copy(&[1, 2, 3, 4]), 4);
        assert_eq!(buf.swap_remove_copy_prim(1).unwrap(), Some(2));
        // Order is intentionally not preserved.
        assert_ring_eq(&buf, &[1, 4, 3]);
    }
    #[test]
    fn from_array_ring() {
        let buf = RingArray::from_array_ring_prim([30, 40, 10, 20], 2, 4).unwrap();
        assert_ring_eq(&buf, &[10, 20, 30, 40]);
    }
    #[test]
    fn pop_default_non_copy() {
        let mut buf = RingToken::from_array_full([Token(1), Token(2), Token(3), Token(4)]);
        assert_eq!(buf.pop_front_default(), Some(Token(1)));
        assert_eq!(buf.pop_back_default(), Some(Token(4)));
        assert_eq!(buf.get_prim(0).unwrap(), Some(&Token(2)));
        assert_eq!(buf.get_prim(1).unwrap(), Some(&Token(3)));
        assert_eq!(buf.get_prim(2).unwrap(), None);
    }
    #[test]
    fn swap_remove_default_non_copy() {
        let mut buf = RingToken::from_array_full([Token(1), Token(2), Token(3), Token(4)]);
        assert_eq!(buf.swap_remove_default_prim(1).unwrap(), Some(Token(2)));
        // Order is intentionally not preserved.
        assert_eq!(buf.get_prim(0).unwrap(), Some(&Token(1)));
        assert_eq!(buf.get_prim(1).unwrap(), Some(&Token(4)));
        assert_eq!(buf.get_prim(2).unwrap(), Some(&Token(3)));
        assert_eq!(buf.get_prim(3).unwrap(), None);
    }
}

mod option {
    use super::*;

    type RingOption = BufferRingStaticExample<i32, [Option<i32>; 4]>;

    fn assert_ring_option_eq(buf: &RingOption, expected: &[i32]) {
        assert_eq!(buf.len_prim() as usize, expected.len());
        assert!(buf.iter().copied().eq(expected.iter().copied()));
    }

    #[test]
    fn basic_fifo() {
        let mut buf = RingOption::new_empty();
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
    fn wraparound_order() {
        let mut buf = RingOption::new_empty();
        assert_eq!(buf.push_back(1), Ok(()));
        assert_eq!(buf.push_back(2), Ok(()));
        assert_eq!(buf.push_back(3), Ok(()));
        assert_eq!(buf.pop_front(), Some(1));
        assert_eq!(buf.push_back(4), Ok(()));
        assert_eq!(buf.push_back(5), Ok(()));
        assert!(buf.is_full());
        assert_ring_option_eq(&buf, &[2, 3, 4, 5]);
        assert_eq!(buf.pop_front(), Some(2));
        assert_eq!(buf.pop_back(), Some(5));
        assert_ring_option_eq(&buf, &[3, 4]);
    }
    #[test]
    fn as_slices_wraparound() {
        let mut buf = RingOption::new_empty();
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
    fn from_array_ring() {
        let buf =
            RingOption::from_array_ring_prim([Some(30), None, Some(10), Some(20)], 2, 3).unwrap();
        assert_ring_option_eq(&buf, &[10, 20, 30]);
    }
    #[test]
    fn from_array_ring_rejects_wrong_occupancy() {
        assert!(
            RingOption::from_array_ring_prim([Some(30), Some(99), Some(10), Some(20)], 2, 3)
                .is_none()
        );
    }
    #[test]
    fn push_back_slice_wraparound() {
        let mut buf = RingOption::new_empty();
        buf.push_back_slice_copy(&[1, 2, 3]);
        assert_eq!(buf.pop_front(), Some(1));
        assert_eq!(buf.push_back_slice_copy(&[4, 5, 6]), 2);
        assert_ring_option_eq(&buf, &[2, 3, 4, 5]);
        assert!(buf.is_full());
    }
    #[test]
    fn push_back_slice_copy_exact() {
        let mut buf = RingOption::new_empty();
        assert_eq!(buf.push_back_slice_copy_exact(&[1, 2, 3]), Ok(()));
        assert_eq!(buf.push_back_slice_copy_exact(&[4, 5]), Err(1));
        assert_ring_option_eq(&buf, &[1, 2, 3]);
    }
    #[test]
    fn push_front_slice_wraparound() {
        let mut buf = RingOption::new_empty();
        assert_eq!(buf.push_back_slice_copy(&[10, 20]), 2);
        assert_eq!(buf.push_front_slice_copy(&[1, 2, 3]), 2);
        assert_ring_option_eq(&buf, &[1, 2, 10, 20]);
    }
    #[test]
    fn swap_remove() {
        let mut buf = RingOption::new_empty();
        buf.push_back_slice_copy(&[1, 2, 3, 4]);
        assert_eq!(buf.swap_remove_prim(1), Ok(Some(2)));
        assert_eq!(buf.len_prim(), 3);
        // Order is intentionally not preserved.
        assert_ring_option_eq(&buf, &[1, 4, 3]);
    }
    #[test]
    fn clear_resets_head_after_wrap() {
        let mut buf = RingOption::new_empty();
        buf.push_back_slice_copy(&[1, 2, 3]);
        assert_eq!(buf.pop_front(), Some(1));
        buf.push_back(4).unwrap();
        buf.clear();
        assert!(buf.is_empty());
        buf.push_back(9).unwrap();
        assert_eq!(buf.pop_front(), Some(9));
    }
}
