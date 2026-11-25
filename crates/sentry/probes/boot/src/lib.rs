//! Downstream probe for root-independent expression macros.
//!
//! Check this package in isolation so other sentry packages do not unify
//! additional devela features into the zero-domain configuration.

#![no_std]
#![forbid(unsafe_code)]

const fn from_root() -> usize {
    use devela::{is, lets, punroll, sf, whilst};

    lets![values = punroll![4 [] |i| i + 1], mut sum = 0];
    whilst![value in &values; sum += *value];
    sf! { is![sum == 10, sum, 0] }
}

const fn from_all() -> usize {
    use devela::all::{is, lets, punroll, sf, whilst};

    lets![(a, b) = punroll![2 () |i| i + 1], mut sum = a + b];
    sf! { whilst![i in 0..3; sum += i]; }
    is![let Some(value) = Some(sum), value, 0]
}

const _: () = {
    assert!(from_root() == 10);
    assert!(from_all() == 6);
};

#[test]
fn expression_macros() {
    assert_eq!(from_root(), 10);
    assert_eq!(from_all(), 6);
}
