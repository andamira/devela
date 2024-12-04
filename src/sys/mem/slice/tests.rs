// devela::sys::mem::slice::tests
//
// TODO: test panics

use crate::Slice;

#[test]
fn range_to() {
    assert_eq!(Slice::range_to(&[1, 2, 3], 2), &[1, 2][..]);
    assert_eq!(Slice::range_to(&[1, 2, 3], 0), &[] as &[i32]);
    assert_eq!(Slice::range_to_checked(&[1, 2, 3], 2), Some(&[1, 2][..]));
    assert_eq!(Slice::range_to_checked(&[1, 2, 3], 0), Some(&[] as &[i32]));
    assert_eq!(Slice::range_to_checked(&[1, 2, 3], 5), None);
    //
    assert_eq!(Slice::range_to_mut(&mut [1, 2, 3], 2), &mut [1, 2][..]);
    assert_eq!(Slice::range_to_mut(&mut [1, 2, 3], 0), &mut [] as &mut [i32]);
    assert_eq!(Slice::range_to_mut_checked(&mut [1, 2, 3], 2), Some(&mut [1, 2][..]));
    assert_eq!(Slice::range_to_mut_checked(&mut [1, 2, 3], 0), Some(&mut [] as &mut [i32]));
    assert_eq!(Slice::range_to_mut_checked(&mut [1, 2, 3], 5), None);
}

#[test]
fn range_from() {
    assert_eq!(Slice::range_from(&[1, 2, 3], 1), &[2, 3][..]);
    assert_eq!(Slice::range_from(&[1, 2, 3], 3), &[] as &[i32]);
    assert_eq!(Slice::range_from_checked(&[1, 2, 3], 1), Some(&[2, 3][..]));
    assert_eq!(Slice::range_from_checked(&[1, 2, 3], 3), Some(&[] as &[i32]));
    assert_eq!(Slice::range_from_checked(&[1, 2, 3], 5), None);
    //
    assert_eq!(Slice::range_from_mut(&mut [1, 2, 3], 1), &mut [2, 3][..]);
    assert_eq!(Slice::range_from_mut(&mut [1, 2, 3], 3), &mut [] as &mut [i32]);
    assert_eq!(Slice::range_from_mut_checked(&mut [1, 2, 3], 1), Some(&mut [2, 3][..]));
    assert_eq!(Slice::range_from_mut_checked(&mut [1, 2, 3], 3), Some(&mut [] as &mut [i32]));
    assert_eq!(Slice::range_from_mut_checked(&mut [1, 2, 3], 5), None);
}

#[test]
fn range() {
    assert_eq!(Slice::range(&[1, 2, 3], 1, 3), &[2, 3][..]);
    assert_eq!(Slice::range(&[1, 2, 3], 0, 0), &[] as &[i32]);
    assert_eq!(Slice::range_checked(&[1, 2, 3], 1, 3), Some(&[2, 3][..]));
    assert_eq!(Slice::range_checked(&[1, 2, 3], 0, 0), Some(&[] as &[i32]));
    assert_eq!(Slice::range_checked(&[1, 2, 3], 2, 1), None);
    assert_eq!(Slice::range_checked(&[1, 2, 3], 0, 5), None);
    //
    assert_eq!(Slice::range_mut(&mut [1, 2, 3], 1, 3), &mut [2, 3][..]);
    assert_eq!(Slice::range_mut(&mut [1, 2, 3], 0, 0), &mut [] as &mut [i32]);
    assert_eq!(Slice::range_mut_checked(&mut [1, 2, 3], 1, 3), Some(&mut [2, 3][..]));
    assert_eq!(Slice::range_mut_checked(&mut [1, 2, 3], 0, 0), Some(&mut [] as &mut [i32]));
    assert_eq!(Slice::range_mut_checked(&mut [1, 2, 3], 2, 1), None);
    assert_eq!(Slice::range_mut_checked(&mut [1, 2, 3], 0, 5), None);
}

#[test]
fn take_first() {
    assert_eq!(Slice::take_first(&[1, 2, 3], 2), &[1, 2][..]);
    assert_eq!(Slice::take_first(&[1, 2, 3], 0), &[] as &[i32]);
    assert_eq!(Slice::take_first_checked(&[1, 2, 3], 2), Some(&[1, 2][..]));
    assert_eq!(Slice::take_first_checked(&[1, 2, 3], 0), Some(&[] as &[i32]));
    assert_eq!(Slice::take_first_checked(&[1, 2, 3], 5), None);
    //
    assert_eq!(Slice::take_first_mut(&mut [1, 2, 3], 2), &mut [1, 2][..]);
    assert_eq!(Slice::take_first_mut(&mut [1, 2, 3], 0), &mut [] as &mut [i32]);
    assert_eq!(Slice::take_first_mut_checked(&mut [1, 2, 3], 2), Some(&mut [1, 2][..]));
    assert_eq!(Slice::take_first_mut_checked(&mut [1, 2, 3], 0), Some(&mut [] as &mut [i32]));
    assert_eq!(Slice::take_first_mut_checked(&mut [1, 2, 3], 5), None);
}

#[test]
fn take_last() {
    assert_eq!(Slice::take_last(&[1, 2, 3], 2), &[2, 3][..]);
    assert_eq!(Slice::take_last(&[1, 2, 3], 3), &[1, 2, 3][..]);
    assert_eq!(Slice::take_last_checked(&[1, 2, 3], 2), Some(&[2, 3][..]));
    assert_eq!(Slice::take_last_checked(&[1, 2, 3], 3), Some(&[1, 2, 3][..]));
    assert_eq!(Slice::take_last_checked(&[1, 2, 3], 5), None);
    //
    assert_eq!(Slice::take_last_mut(&mut [1, 2, 3], 2), &mut [2, 3][..]);
    assert_eq!(Slice::take_last_mut(&mut [1, 2, 3], 3), &mut [1, 2, 3][..]);
    assert_eq!(Slice::take_last_mut_checked(&mut [1, 2, 3], 2), Some(&mut [2, 3][..]));
    assert_eq!(Slice::take_last_mut_checked(&mut [1, 2, 3], 3), Some(&mut [1, 2, 3][..]));
    assert_eq!(Slice::take_last_mut_checked(&mut [1, 2, 3], 5), None);
}

#[test]
fn take_omit_last() {
    assert_eq!(Slice::take_omit_last(&[1, 2, 3], 1), &[1, 2][..]);
    assert_eq!(Slice::take_omit_last(&[1, 2, 3], 3), &[] as &[i32]);
    assert_eq!(Slice::take_omit_last_checked(&[1, 2, 3], 1), Some(&[1, 2][..]));
    assert_eq!(Slice::take_omit_last_checked(&[1, 2, 3], 3), Some(&[] as &[i32]));
    assert_eq!(Slice::take_omit_last_checked(&[1, 2, 3], 5), None);
    //
    assert_eq!(Slice::take_omit_last_mut(&mut [1, 2, 3], 1), &mut [1, 2][..]);
    assert_eq!(Slice::take_omit_last_mut(&mut [1, 2, 3], 3), &mut [] as &mut [i32]);
    assert_eq!(Slice::take_omit_last_mut_checked(&mut [1, 2, 3], 1), Some(&mut [1, 2][..]));
    assert_eq!(Slice::take_omit_last_mut_checked(&mut [1, 2, 3], 3), Some(&mut [] as &mut [i32]));
    assert_eq!(Slice::take_omit_last_mut_checked(&mut [1, 2, 3], 5), None);
}
