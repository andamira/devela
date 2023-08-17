// devela::num::tests
//
//!
//

use super::*;

#[test]
fn non_range() {
    // a range that doesn't wrap around
    assert_eq!(NonRangeI8::<-10, 10>::INVALID_VALUES, 21);
    assert_eq!(NonRangeI8::<-10, 10>::VALID_VALUES, 235);
    assert!(NonRangeI8::<-10, 10>::new(11).is_some());
    assert!(NonRangeI8::<-10, 10>::new(-11).is_some());
    //
    assert!(NonRangeI8::<-10, 10>::new(1).is_none());
    assert!(NonRangeI8::<-10, 10>::new(10).is_none());
    assert!(NonRangeI8::<-10, 10>::new(-10).is_none());

    // a range that wraps around
    assert_eq!(NonRangeI8::<-121, 10>::INVALID_VALUES, 132);
    assert_eq!(NonRangeI8::<-121, 10>::VALID_VALUES, 124);
    assert!(NonRangeI8::<-121, 10>::new(11).is_some());
    assert!(NonRangeI8::<-121, 10>::new(-122).is_some());
    //

    // a range that includes all possible values except one
    assert_eq!(NonRangeI8::<-127, 127>::INVALID_VALUES, 255);
    assert_eq!(NonRangeI8::<-127, 127>::VALID_VALUES, 1);
    //
    assert!(NonRangeI8::<-127, 127>::new(-128).is_some());
    //
    assert!(NonRangeI8::<-127, 127>::new(-127).is_none());
    assert!(NonRangeI8::<-127, 127>::new(127).is_none());
    assert!(NonRangeI8::<-127, 127>::new(1).is_none());

    // checking the extremes
    assert_eq!(NonRangeI8::<0, 0>::INVALID_VALUES, 1);
    assert_eq!(NonRangeI8::<0, 0>::VALID_VALUES, 255);

    assert_eq!(NonRangeI8::<-128, 127>::INVALID_VALUES, 0); // wrapped 256
    assert_eq!(NonRangeI8::<-128, 127>::VALID_VALUES, 0);
}

#[test]
#[cfg(feature = "bytemuck")]
fn non_range_bytemuck() {
    use bytemuck::CheckedBitPattern;

    // a range that doesn't wrap around
    assert!(NonRangeI8::<-10, 10>::is_valid_bit_pattern(&-1));
    assert!(NonRangeI8::<-10, 10>::is_valid_bit_pattern(&21));
    //
    assert!(!NonRangeI8::<-10, 10>::is_valid_bit_pattern(&0));
    assert!(!NonRangeI8::<-10, 10>::is_valid_bit_pattern(&20));

    // a range that wraps around
    assert!(NonRangeI8::<-121, 10>::is_valid_bit_pattern(&-1));
    assert!(NonRangeI8::<-121, 10>::is_valid_bit_pattern(&-124));
    //
    assert!(!NonRangeI8::<-121, 10>::is_valid_bit_pattern(&0));
    assert!(!NonRangeI8::<-121, 10>::is_valid_bit_pattern(&-125));

    // a range that includes all possible values except one
    assert!(NonRangeI8::<-127, 127>::is_valid_bit_pattern(&-1));
    //
    assert!(!NonRangeI8::<-127, 127>::is_valid_bit_pattern(&0));
    assert!(!NonRangeI8::<-127, 127>::is_valid_bit_pattern(&1));
}
