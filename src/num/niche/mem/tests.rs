// devela::num::niche::tests
//
//!
//

#[allow(unused_imports)]
use super::*;

#[test]
fn non_value() {
    // a positive value
    type S1 = NonValueI8<120>;
    assert_eq!(S1::VALID_VALUES, u8::MAX);
    assert_eq!(S1::INVALID_VALUES, 1);
    for valid in (i8::MIN..=119).chain(121..=i8::MAX) {
        assert!(S1::new(valid).is_some());
    }
    assert!(S1::new(120).is_none());

    // a negative value
    type S2 = NonValueI8<-10>;
    for valid in (i8::MIN..=-11).chain(-9..=i8::MAX) {
        assert!(S2::new(valid).is_some());
    }
    assert!(S2::new(-10).is_none());
}
