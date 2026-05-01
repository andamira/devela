// devela::data::codec::crypto::_helper
//
//! Defines [`CryptoError`].
//

pub(crate) const fn _hex<const N: usize>(s: &str) -> [u8; N] {
    use devela::{Base, Rfc4648};
    type Hex = Base<16, false, false, true, Rfc4648>;
    let input = s.as_bytes();
    assert!(input.len() == N * 2);
    let mut out = [0u8; N];
    let written = match Hex::decode_from_slice(input, &mut out) {
        Some(written) => written,
        None => panic!("invalid hex"),
    };
    assert!(written == N);
    out
}
