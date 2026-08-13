// devela/src/data/codec/radix/_test/base16.rs

use super::*;

const fn hex<const N: usize>(s: &str) -> [u8; N] {
    match Radix::<16>::HEX.decode_array(s.as_bytes()) {
        Some(bytes) => bytes,
        None => panic!("invalid hexadecimal"),
    }
}

#[test]
fn base() {
    assert_eq!(Radix::<16>::BASE, 16);
}
#[test]
fn encode_known_vectors() {
    let vectors: &[(&[u8], &[u8])] = &[
        (b"", b""),
        (b"f", b"66"),
        (b"fo", b"666F"),
        (b"foo", b"666F6F"),
        (b"foob", b"666F6F62"),
        (b"fooba", b"666F6F6261"),
        (b"foobar", b"666F6F626172"),
    ];
    for &(input, expected) in vectors {
        let mut output = [0; 32];
        let written = Radix::<16>::HEX.encode_to_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn encode_lowercase() {
    let mut output = [0; 12];
    let written = Radix::<16>::HEX_LOWER.encode_to_slice(b"foobar", &mut output).unwrap();
    assert_eq!(written, 12);
    assert_eq!(&output, b"666f6f626172");
}
#[test]
fn decode_known_vectors() {
    let vectors: &[(&[u8], &[u8])] = &[
        (b"", b""),
        (b"66", b"f"),
        (b"666F", b"fo"),
        (b"666F6F", b"foo"),
        (b"666F6F62", b"foob"),
        (b"666F6F6261", b"fooba"),
        (b"666F6F626172", b"foobar"),
    ];
    for &(input, expected) in vectors {
        let mut output = [0; 16];
        let written = Radix::<16>::HEX.decode_from_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn decode_accepts_both_cases() {
    let expected = b"\xab\xcd\xef";
    for input in [b"ABCDEF".as_slice(), b"abcdef".as_slice(), b"AbCdEf".as_slice()] {
        let mut output = [0; 3];
        let written = Radix::<16>::HEX.decode_from_slice(input, &mut output).unwrap();
        assert_eq!(written, 3);
        assert_eq!(&output, expected);
    }
}
#[test]
fn both_configs_decode_identically() {
    let input = b"aBcDeF";
    let mut upper = [0; 3];
    let mut lower = [0; 3];
    assert_eq!(Radix::<16>::HEX.decode_from_slice(input, &mut upper), Some(3));
    assert_eq!(Radix::<16>::HEX_LOWER.decode_from_slice(input, &mut lower), Some(3));
    assert_eq!(upper, lower);
}
#[test]
fn decode_array_exact() {
    assert_eq!(Radix::<16>::HEX.decode_array::<3>(b"1234ab"), Some([0x12, 0x34, 0xab]));
    assert_eq!(Radix::<16>::HEX.decode_array::<2>(b"1234ab"), None);
    assert_eq!(Radix::<16>::HEX.decode_array::<4>(b"1234ab"), None);
}
#[test]
fn rejects_invalid_input() {
    let mut output = [0; 8];
    // Odd number of hexadecimal digits.
    assert_eq!(Radix::<16>::HEX.decode_from_slice(b"123", &mut output), None);
    // Non-hexadecimal characters.
    assert_eq!(Radix::<16>::HEX.decode_from_slice(b"12xz", &mut output), None);
    assert_eq!(Radix::<16>::HEX.decode_from_slice(b"12 3", &mut output), None);
}
#[test]
fn rejects_small_output() {
    let mut encoded = [0; 3];
    assert_eq!(Radix::<16>::HEX.encode_to_slice(b"ab", &mut encoded), None);
    let mut decoded = [0; 1];
    assert_eq!(Radix::<16>::HEX.decode_from_slice(b"abcd", &mut decoded), None);
}
#[test]
fn allows_larger_output() {
    let mut encoded = [0xff; 8];
    let written = Radix::<16>::HEX.encode_to_slice(b"ab", &mut encoded).unwrap();
    assert_eq!(written, 4);
    assert_eq!(&encoded[..4], b"6162");
    assert_eq!(&encoded[4..], &[0xff; 4]);
    let mut decoded = [0xff; 4];
    let written = Radix::<16>::HEX.decode_from_slice(b"6162", &mut decoded).unwrap();
    assert_eq!(written, 2);
    assert_eq!(&decoded[..2], b"ab");
    assert_eq!(&decoded[2..], &[0xff; 2]);
}
#[test]
fn roundtrip() {
    for radix in [Radix::<16>::HEX, Radix::<16>::HEX_LOWER] {
        let mut encoded = [0; 512];
        let mut decoded = [0; 256];
        assert_eq!(radix.encode_to_slice(&ALL_BYTES, &mut encoded), Some(encoded.len()));
        assert_eq!(radix.decode_from_slice(&encoded, &mut decoded), Some(decoded.len()));
        assert_eq!(decoded, ALL_BYTES);
    }
}
#[test]
fn helper() {
    const VALUE: [u8; 20] = hex("b617318655057264e28bc0b6fb378c8ef146be00");
    assert_eq!(
        VALUE,
        [
            0xb6, 0x17, 0x31, 0x86, 0x55, 0x05, 0x72, 0x64, 0xe2, 0x8b, 0xc0, 0xb6, 0xfb, 0x37,
            0x8c, 0x8e, 0xf1, 0x46, 0xbe, 0x00,
        ]
    );
}
#[test]
fn const_operations() {
    const DECODED: Option<[u8; 3]> = Radix::<16>::HEX.decode_array(b"DeAdBe");
    const ENCODED: ([u8; 6], Option<usize>) = {
        let mut output = [0; 6];
        let written = Radix::<16>::HEX_LOWER.encode_to_slice(b"\xde\xad\xbe", &mut output);
        (output, written)
    };
    assert_eq!(DECODED, Some([0xde, 0xad, 0xbe]));
    assert_eq!(ENCODED, (*b"deadbe", Some(6)));
}
