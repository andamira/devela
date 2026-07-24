// devela/src/data/layout/buffer/linear/_test.rs

use super::*;
use crate::{ConstInit, const_assert, niche};

#[derive(Debug, PartialEq, Eq)]
struct Item(u8);
impl ConstInit for Item {
    const INIT: Self = Self(0);
}

mod array {
    use super::*;

    type Linear = BufferLinearStaticExample<i32, [i32; 4]>;

    #[test]
    fn array() {
        let mut buf = Linear::new_init();
        buf.push_back(10).unwrap();
        buf.push_back(20).unwrap();
        assert_eq!(buf.as_slice(), &[10, 20]);
    }
    #[test]
    const fn array_pop_noncopy_is_const() {
        const LINEAR_POP: (Option<Item>, usize) = {
            let mut buf = BufferLinearStaticExample::<Item, [Item; 3]>::from_array_clamped(
                [Item(10), Item(20), Item(99)],
                niche!(2u8; != MAX),
            );
            let value = buf.pop_back();
            (value, buf.len().get() as usize)
        };
        const_assert!(eq LINEAR_POP.0.unwrap().0, Item(20).0);
        const_assert!(eq LINEAR_POP.1, 1);
    }
    #[test]
    fn array_pop_with_custom_replacement() {
        let mut buf = BufferLinearStaticExample::<Item, [Item; 3]>::from_array_clamped(
            [Item(10), Item(20), Item(99)],
            niche!(2u8; != MAX),
        );
        assert_eq!(buf.pop_back_with(Item(7)), Some(Item(20)));
        assert_eq!(buf.len(), niche!(1u8; != MAX));
    }
}

mod option {
    use super::*;

    type LinearOption = BufferLinearStaticExample<i32, [Option<i32>; 4]>;

    #[test]
    fn option_push_slice_and_swap_remove() {
        let mut buf = LinearOption::new();
        assert_eq!(buf.push_slice_copy(&[1, 2, 3]), 3);
        assert_eq!(buf.push_slice_copy_exact(&[4]), Ok(()));
        assert_eq!(buf.push_slice_copy_exact(&[5]), Err(0));
        assert_eq!(buf.swap_remove_prim(1), Ok(Some(2)));
        assert_eq!(buf.as_slice(), &[Some(1), Some(4), Some(3)]);
    }
    #[test]
    fn option_truncate_and_clear() {
        let mut buf = LinearOption::new();
        buf.push_slice_copy(&[1, 2, 3, 4]);
        let _ = buf.truncate_prim(2);
        assert_eq!(buf.as_slice(), &[Some(1), Some(2)]);
        buf.clear();
        assert!(buf.is_empty());
        assert_eq!(buf.as_slice(), &[]);
    }
}

#[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
mod uninit {
    use super::*;

    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
    type LinearUninit = BufferLinearStaticExample<i32, [crate::MaybeUninit<i32>; 4]>;

    #[test]
    fn uninit() {
        let mut buf = LinearUninit::new();
        buf.push_back(10).unwrap();
        buf.push_back(20).unwrap();
        assert_eq!(buf.as_slice(), &[10, 20]);
    }
}
