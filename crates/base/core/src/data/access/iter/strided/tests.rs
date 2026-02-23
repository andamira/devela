// devela_base_core::data::access::iter::strided::tests

crate::iter_strided! {
    /// 8-bit strided iterator for testing.
    struct StridedIterU8: ref (u8)
}

const DATA_U8: [u8; 256] = {
    let mut arr = [0u8; 256];
    crate::whilst! { i in 0..256;  { arr[i] = i as u8; }}
    arr
};

#[test]
fn basic_forward() {
    let data = [0, 1, 2, 3, 4, 5];
    let mut it = StridedIterU8::from_count(&data, 1, 3, 2);
    assert_eq!(it.len(), 3);
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&5));
    assert_eq!(it.next(), None);
    assert_eq!(it.len(), 0);
}

#[test]
fn basic_backward() {
    let data = [0, 1, 2, 3, 4, 5];
    let mut it = StridedIterU8::from_count(&data, 1, 3, 2);
    assert_eq!(it.next_back(), Some(&5));
    assert_eq!(it.next_back(), Some(&3));
    assert_eq!(it.next_back(), Some(&1));
    assert_eq!(it.next_back(), None);
}

#[test]
fn forward_backward_mix() {
    let data = [10, 11, 12, 13, 14];
    let mut it = StridedIterU8::from_bounds(&data, 0, 4, 1);
    assert_eq!(it.next(), Some(&10));
    assert_eq!(it.next_back(), Some(&14));
    assert_eq!(it.next(), Some(&11));
    assert_eq!(it.next_back(), Some(&13));
    assert_eq!(it.next(), Some(&12));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn empty_when_front_gt_back() {
    let data = [1, 2, 3];
    let mut it = StridedIterU8::from_bounds(&data, 2, 1, 1);
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn single_element() {
    let data = [42];
    let mut it = StridedIterU8::from_bounds(&data, 0, 0, 1);
    assert_eq!(it.next(), Some(&42));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn max_u8_range_stride_1() {
    let mut it = StridedIterU8::from_bounds(&DATA_U8, 0, 255, 1);

    for i in 0..=255 {
        assert_eq!(it.next(), Some(&i));
    }
    assert_eq!(it.next(), None);
}

#[test]
fn stride_two_full_range() {
    let mut it = StridedIterU8::from_bounds(&DATA_U8, 0, 254, 2);

    for i in (0..=254).step_by(2) {
        assert_eq!(it.next(), Some(&i));
    }
    assert_eq!(it.next(), None);
}

#[test]
fn sentinel_no_wrap_after_last() {
    let data = [0, 1, 2];
    let mut it = StridedIterU8::from_bounds(&data, 0, 2, 2);
    assert_eq!(it.next(), Some(&0));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), None);

    // ensure no revival
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn len_matches_iteration() {
    let data = &DATA_U8[..=50];
    let mut it = StridedIterU8::from_bounds(&data, 5, 45, 5);

    let expected = it.len() as usize;
    let mut count = 0;
    while it.next().is_some() {
        count += 1;
    }
    assert_eq!(count, expected);
    assert_eq!(it.len(), 0);
}

#[test]
#[should_panic]
fn stride_zero_panics() {
    let data = [1, 2, 3];
    let _ = StridedIterU8::from_count(&data, 0, 1, 0);
}
