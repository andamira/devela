use crate::{ConstInit, Radix};

mod base16;
#[cfg(feature = "base32")]
mod base32;
#[cfg(feature = "base64")]
mod base64;

const ALL_BYTES: [u8; 256] = {
    let mut bytes = [0; 256];
    crate::whilst! { i in 0..256; { bytes[i] = i as u8; }}
    bytes
};
#[cfg(any(feature = "base32", feature = "base64"))]
fn without_padding(input: &[u8]) -> &[u8] {
    let mut len = input.len();
    while len > 0 && input[len - 1] == b'=' {
        len -= 1;
    }
    &input[..len]
}

#[test]
fn const_init() {
    assert_eq!(<Radix<16> as ConstInit>::INIT, Radix::<16>::HEX);
    #[cfg(feature = "base32")]
    assert_eq!(<Radix<32> as ConstInit>::INIT, Radix::<32>::STD);
    #[cfg(feature = "base64")]
    assert_eq!(<Radix<64> as ConstInit>::INIT, Radix::<64>::STD);
}
