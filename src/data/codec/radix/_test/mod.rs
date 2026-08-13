// devela/src/data/codec/radix/_test/mod.rs

use crate::{ConstInit, Radix};

mod base16;
mod base32;
mod base64;

const ALL_BYTES: [u8; 256] = {
    let mut bytes = [0; 256];
    crate::whilst! { i in 0..256; { bytes[i] = i as u8; }}
    bytes
};

#[test]
fn const_init() {
    assert_eq!(<Radix<16> as ConstInit>::INIT, Radix::<16>::HEX);
    assert_eq!(<Radix<32> as ConstInit>::INIT, Radix::<32>::STD);
    assert_eq!(<Radix<64> as ConstInit>::INIT, Radix::<64>::STD);
}
