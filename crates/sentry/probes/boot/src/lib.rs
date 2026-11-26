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

// All four UTF-8 widths, fixed and dynamic indexed operations, and the public
// boot paths must work with no domain roots selected.
const fn indexed() -> [u8; 12] {
    use devela::boot::{read_at, write_at};
    let mut bytes = [0u8; 12];
    let mut pos = 0;
    write_at!(bytes, +=pos, #'A', #'µ', #'€', #'𐀀', @2 *b"ok");
    assert!(pos == bytes.len());
    let mut dst = [0u8; 12];
    let head = read_at!(bytes, 0, @4);
    let mut off = 0;
    devela::all::write_at!(dst, +=off, @4 head);
    let mut tail = [0u8; 8];
    assert!(devela::read_at!(bytes, 4, @tail) == 12);
    devela::write_at!(dst, +=off, @tail);
    assert!(off == 12);
    dst
}

const _: () = {
    use devela::boot::rust::NonZeroU8;
    assert!(core::mem::size_of::<NonZeroU8>() == 1);
    let bytes = indexed();
    assert!(bytes[0] == b'A');
    assert!(bytes[11] == b'k');
};

#[test]
fn indexed_and_public_boot() {
    assert_eq!(&indexed(), "Aµ€𐀀ok".as_bytes());
    assert_eq!(devela::boot::is![true, 1, 2], 1);
}

#[test]
fn grouped_boot() {
    use devela::boot::{
        expr::is,
        synth::{CONST, items},
    };
    CONST! { VALUE = 7 }
    items! { const N: u8 = VALUE!(); }
    mod exports {
        use devela::boot::synth::mods_out;
        mod value {
            pub const VALUE: u8 = 7;
        }
        mods_out! { _mods { pub use super::value::VALUE; } }
    }
    assert_eq!(is![true, N, 0], exports::VALUE);
    assert_eq!(devela::boot::is![true, N, 0], N);
}

// External policy expansion remains available without yard or any domain root.
devela::_devela_policy! {
    item_gate unsafe:"unsafe_hint",
    const POLICY_VALUE: u8 = 7;
}

const _: () = {
    assert!(devela::unwrap![some Some(POLICY_VALUE)] == 7);
    assert!(devela::all::unwrap![ok Ok::<_, ()>(7)] == 7);
    assert!(devela::boot::unwrap![some_or None::<u8>, 7] == 7);
    assert!(devela::boot::expr::unwrap![sok Some(Ok::<_, ()>(7))] == 7);
    // SAFETY: the selected variants are present.
    assert!(devela::all_::_boot::unwrap![some_guaranteed_or_ub Some(7)] == 7);
    assert!(devela::boot::unwrap![ok_guaranteed_or_ub Ok::<_, ()>(7)] == 7);
    assert!(devela::unwrap![sok_guaranteed_or_ub Some(Ok::<_, ()>(7))] == 7);
};

#[test]
fn flat_boot() {
    use devela::all_::_boot::*;
    CONST! { VALUE = 7 }
    let n = unwrap![some NonZeroU8::new(VALUE!())];
    assert_eq!(n.get(), 7);
}
