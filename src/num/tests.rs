// devela::num::tests
//
//!
//

use super::*;

#[test]
fn non_range() {
    // a range that doesn't wrap around
    assert_eq!(NonRangeI8::<-10, 10>::VALID_VALUES, 235);
    assert_eq!(NonRangeI8::<-10, 10>::INVALID_VALUES, 21);
    assert!(NonRangeI8::<-10, 10>::new(11).is_some());
    assert!(NonRangeI8::<-10, 10>::new(-11).is_some());
    //
    assert!(NonRangeI8::<-10, 10>::new(1).is_none());
    assert!(NonRangeI8::<-10, 10>::new(10).is_none());
    assert!(NonRangeI8::<-10, 10>::new(-10).is_none());

    // a range that wraps around
    assert_eq!(NonRangeI8::<-121, 10>::VALID_VALUES, 124);
    assert_eq!(NonRangeI8::<-121, 10>::INVALID_VALUES, 132);
    assert!(NonRangeI8::<-121, 10>::new(11).is_some());
    assert!(NonRangeI8::<-121, 10>::new(-122).is_some());
    //

    // a range that includes all possible values except one
    assert_eq!(NonRangeI8::<-127, 127>::VALID_VALUES, 1);
    assert_eq!(NonRangeI8::<-127, 127>::INVALID_VALUES, 255);
    //
    assert!(NonRangeI8::<-127, 127>::new(-128).is_some());
    //
    assert!(NonRangeI8::<-127, 127>::new(-127).is_none());
    assert!(NonRangeI8::<-127, 127>::new(127).is_none());
    assert!(NonRangeI8::<-127, 127>::new(1).is_none());

    // checking the extremes
    assert_eq!(NonRangeI8::<0, 0>::VALID_VALUES, 255);
    assert_eq!(NonRangeI8::<0, 0>::INVALID_VALUES, 1);

    assert_eq!(NonRangeI8::<{ i8::MIN }, { i8::MAX }>::VALID_VALUES, 0);
    assert_eq!(NonRangeI8::<{ i8::MIN }, { i8::MAX }>::INVALID_VALUES, 0); // wrapped 256
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

#[test]
fn range() {
    // a positive range
    type R1 = RangeI8<1, 10>;
    assert_eq!(R1::VALID_VALUES, 10);
    assert_eq!(R1::INVALID_VALUES, 246);
    assert!(R1::new(1).is_some());
    assert!(R1::new(10).is_some());
    //
    assert!(R1::new(0).is_none());
    assert!(R1::new(11).is_none());

    // a range from negative to positve
    type R2 = RangeI8<-10, 10>;
    assert_eq!(R2::VALID_VALUES, 21);
    assert_eq!(R2::INVALID_VALUES, 235);
    assert!(R2::new(-10).is_some());
    assert!(R2::new(0).is_some());
    assert!(R2::new(10).is_some());
    //
    assert!(R2::new(11).is_none());
    assert!(R2::new(-11).is_none());

    // a range that includes all possible values except one
    type R3 = RangeI8<-127, 127>;
    assert_eq!(R3::VALID_VALUES, 255);
    assert_eq!(R3::INVALID_VALUES, 1);
    for valid in -127..=127 {
        assert![R3::new(valid).is_some()];
    }
    //
    assert!(R3::new(-128).is_none());

    // a range that includes only one possible value
    type R4 = RangeI8<0, 0>;
    assert_eq!(R4::VALID_VALUES, 1);
    assert_eq!(R4::INVALID_VALUES, 255);
    assert!(R4::new(0).is_some());
    //
    for invalid in (i8::MIN..=-1).chain(1..=i8::MAX) {
        assert!(R4::new(invalid).is_none());
    }

    // checking the extreme
    type R5 = RangeI8<{ i8::MIN }, { i8::MAX }>;
    assert_eq!(R5::VALID_VALUES, 0); // wrapped 256
    assert_eq!(R5::INVALID_VALUES, 0);
}

#[test]
#[cfg(feature = "bytemuck")]
fn range_bytemuck() {
    use bytemuck::CheckedBitPattern;

    // a positive range
    type R1 = RangeI8<1, 10>;
    assert!(R1::is_valid_bit_pattern(&1));
    assert!(R1::is_valid_bit_pattern(&10));
    //
    assert!(!R1::is_valid_bit_pattern(&0));
    assert!(!R1::is_valid_bit_pattern(&-1));
    assert!(!R1::is_valid_bit_pattern(&11));

    // a range from negative to positve
    type R2 = RangeI8<-10, 10>;
    assert!(R2::is_valid_bit_pattern(&-16));
    assert!(R2::is_valid_bit_pattern(&15));
    //
    assert!(!R2::is_valid_bit_pattern(&0));
    assert!(!R2::is_valid_bit_pattern(&-17));
    assert!(!R2::is_valid_bit_pattern(&16));

    // a range that includes all possible values except one
    type R3 = RangeI8<-127, 127>;
    for valid in (i8::MIN..=-1).chain(1..=i8::MAX) {
        assert![R3::is_valid_bit_pattern(&valid)];
    }
    //
    assert!(!R3::is_valid_bit_pattern(&0));

    // a range that includes only one possible value
    type R4 = RangeI8<0, 0>;
    assert!(R4::is_valid_bit_pattern(&-1));
    //
    for invalid in (i8::MIN..=-2).chain(0..=i8::MAX) {
        assert!(!R4::is_valid_bit_pattern(&invalid));
    }
}
