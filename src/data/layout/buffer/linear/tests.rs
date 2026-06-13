// devela/src/data/layout/buffer/linear/tests.rs

use super::*;

type Linear = BufferLinearStaticExample<i32, [i32; 4]>;
type LinearOption = BufferLinearStaticExample<i32, [Option<i32>; 4]>;
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
type LinearUninit = BufferLinearStaticExample<i32, [crate::MaybeUninit<i32>; 4]>;

#[test]
fn array() {
    let mut buf = Linear::new_init();
    buf.push_back(10).unwrap();
    buf.push_back(20).unwrap();
    assert_eq!(buf.as_slice(), &[10, 20]);
}

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

#[test]
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
fn uninit() {
    let mut buf = LinearUninit::new();
    buf.push_back(10).unwrap();
    buf.push_back(20).unwrap();
    assert_eq!(buf.as_slice(), &[10, 20]);
}
