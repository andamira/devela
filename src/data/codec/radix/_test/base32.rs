// devela/src/data/codec/radix/_test/base32.rs

use super::*;

const VECTORS_RFC4648: &[(&[u8], &[u8])] = &[
    (b"", b""),
    (b"f", b"MY======"),
    (b"fo", b"MZXQ===="),
    (b"foo", b"MZXW6==="),
    (b"foob", b"MZXW6YQ="),
    (b"fooba", b"MZXW6YTB"),
    (b"foobar", b"MZXW6YTBOI======"),
];
const VECTORS_HEX: &[(&[u8], &[u8])] = &[
    (b"", b""),
    (b"f", b"CO======"),
    (b"fo", b"CPNG===="),
    (b"foo", b"CPNMU==="),
    (b"foob", b"CPNMUOG="),
    (b"fooba", b"CPNMUOJ1"),
    (b"foobar", b"CPNMUOJ1E8======"),
];
const VECTORS_CROCKFORD: &[(&[u8], &[u8])] = &[
    (b"f", b"CR"),
    (b"fo", b"CSQG"),
    (b"foo", b"CSQPY"),
    (b"foob", b"CSQPYRG"),
    (b"fooba", b"CSQPYRK1"),
    (b"foobar", b"CSQPYRK1E8"),
];

fn without_padding(input: &[u8]) -> &[u8] {
    let mut len = input.len();
    while len > 0 && input[len - 1] == b'=' {
        len -= 1;
    }
    &input[..len]
}

#[test]
fn base() {
    assert_eq!(Radix::<32>::BASE, 32);
}
#[test]
fn unpadded_vectors() {
    for (radix, vectors) in
        [(Radix::<32>::STD_UNPADDED, VECTORS_RFC4648), (Radix::<32>::HEX_UNPADDED, VECTORS_HEX)]
    {
        for &(input, padded) in vectors {
            let expected = without_padding(padded);

            let mut encoded = [0; 16];
            let written = radix.encode_to_slice(input, &mut encoded).unwrap();
            assert_eq!(&encoded[..written], expected);

            let mut decoded = [0; 16];
            let written = radix.decode_from_slice(expected, &mut decoded).unwrap();
            assert_eq!(&decoded[..written], input);
        }
    }
}
#[test]
fn strict_and_relaxed_padding() {
    assert_eq!(Radix::<32>::STD.decode_array::<1>(b"MY======"), Some(*b"f"));
    assert_eq!(Radix::<32>::STD.decode_array::<1>(b"MY"), None);
    assert_eq!(Radix::<32>::STD_UNPADDED.decode_array::<1>(b"MY"), Some(*b"f"));
    assert_eq!(Radix::<32>::STD_UNPADDED.decode_array::<1>(b"MY======"), None);
    for radix in [Radix::<32>::STD, Radix::<32>::STD_UNPADDED]
    // WORKS
    // FAILS:
    // [Radix::<64>::STD, Radix::<64>::STD_UNPADDED, Radix::<64>::URL, Radix::<64>::URL_UNPADDED]
    {
        for input in [
            b"MY".as_slice(),
            b"MY=".as_slice(),
            b"MY==".as_slice(),
            b"MY===".as_slice(),
            b"MY====".as_slice(),
            b"MY=====".as_slice(),
            b"MY======".as_slice(),
        ] {
            assert_eq!(radix.decode_array_relaxed::<1>(input), Some(*b"f"));
        }
    }
}
#[test]
fn encode_rfc4648_vectors() {
    for &(input, expected) in VECTORS_RFC4648 {
        let mut output = [0; 16];
        let written = Radix::<32>::STD.encode_to_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn decode_rfc4648_vectors() {
    for &(expected, input) in VECTORS_RFC4648 {
        let mut output = [0; 16];
        let written = Radix::<32>::STD.decode_from_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn encode_hex_vectors() {
    for &(input, expected) in VECTORS_HEX {
        let mut output = [0; 16];
        let written = Radix::<32>::HEX.encode_to_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn decode_hex_vectors() {
    for &(expected, input) in VECTORS_HEX {
        let mut output = [0; 16];
        let written = Radix::<32>::HEX.decode_from_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn encode_crockford_vectors() {
    for &(input, expected) in VECTORS_CROCKFORD {
        let mut output = [0; 16];
        let written = Radix::<32>::CROCKFORD.encode_to_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn decode_crockford_vectors() {
    for &(expected, input) in VECTORS_CROCKFORD {
        let mut output = [0; 16];
        let written = Radix::<32>::CROCKFORD.decode_from_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn relaxed() {
    assert_eq!(Radix::<32>::STD.decode_array_relaxed::<6>(b"mzxw6ytboi"), Some(*b"foobar"));
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<6>(b"CSQPY-RK1E8"), Some(*b"foobar"),);
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<6>(b"csqpyrkle8"), Some(*b"foobar"),);
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<6>(b"csqpyrkie8"), Some(*b"foobar"),);
}
#[test]
fn relaxed_case() {
    assert_eq!(Radix::<32>::STD.decode_array::<6>(b"mzxw6ytboi======"), None);
    assert_eq!(Radix::<32>::STD.decode_array_relaxed::<6>(b"mzxw6ytboi"), Some(*b"foobar"));
    assert_eq!(Radix::<32>::HEX.decode_array::<6>(b"cpnmuoj1e8======"), None);
    assert_eq!(Radix::<32>::HEX.decode_array_relaxed::<6>(b"cpnmuoj1e8"), Some(*b"foobar"));
}
#[test]
fn crockford_relaxed_forms() {
    assert_eq!(Radix::<32>::CROCKFORD.decode_array::<6>(b"CSQPYRK1E8"), Some(*b"foobar"));
    // Strict decoding accepts only canonical Crockford symbols.
    assert_eq!(Radix::<32>::CROCKFORD.decode_array::<6>(b"csqpyrk1e8"), None);
    assert_eq!(Radix::<32>::CROCKFORD.decode_array::<6>(b"CSQPY-RK1E8"), None);
    assert_eq!(Radix::<32>::CROCKFORD.decode_array::<1>(b"O0"), None);
    assert_eq!(Radix::<32>::CROCKFORD.decode_array::<1>(b"I0"), None);
    assert_eq!(Radix::<32>::CROCKFORD.decode_array::<1>(b"L0"), None);
    // Relaxed Crockford accepts case, separators and ambiguity aliases.
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<6>(b"csqpy-rk1e8"), Some(*b"foobar"));
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<1>(b"OO"), Some([0]));
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<1>(b"I0"), Some([8]));
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<1>(b"L0"), Some([8]));
    // U is not part of the Crockford alphabet and has no alias.
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<1>(b"U0"), None);
    assert_eq!(Radix::<32>::CROCKFORD.decode_array_relaxed::<1>(b"u0"), None);
}
#[test]
fn rejects_noncanonical_trailing_bits() {
    let mut output = [0; 8];
    // Canonical forms are MY, MZXQ, MZXW6, MZXW6YQ.
    // These change only unused trailing bits.
    for input in [b"MZ".as_slice(), b"MZXR".as_slice(), b"MZXW7".as_slice(), b"MZXW6YR".as_slice()]
    {
        assert_eq!(Radix::<32>::STD_UNPADDED.decode_from_slice(input, &mut output), None);
        assert_eq!(Radix::<32>::STD.decode_from_slice_relaxed(input, &mut output), None);
    }
    // Same canonical-bit requirement applies to Crockford.
    // "CR" encodes `f`; "CS" differs only in unused bits.
    assert_eq!(Radix::<32>::CROCKFORD.decode_from_slice(b"CS", &mut output), None);
}
#[test]
fn rejects_malformed_input() {
    let mut output = [0; 16];
    // Invalid unpadded symbol counts: 1, 3 and 6 modulo 8.
    for input in [b"M".as_slice(), b"MZX".as_slice(), b"MZXW6Y".as_slice()] {
        assert_eq!(Radix::<32>::STD.decode_from_slice_relaxed(input, &mut output), None);
    }
    // Excess, interior and inappropriate padding.
    for input in [
        b"MY=======".as_slice(),
        b"M=Y======".as_slice(),
        b"MY=====A".as_slice(),
        b"MZXW6YTB=".as_slice(),
    ] {
        assert_eq!(Radix::<32>::STD.decode_from_slice_relaxed(input, &mut output), None);
    }
    // Relaxed does not mean whitespace-tolerant.
    assert_eq!(Radix::<32>::STD.decode_from_slice_relaxed(b"M Y======", &mut output), None);
}
#[test]
fn decode_array_exact() {
    assert_eq!(Radix::<32>::STD.decode_array::<6>(b"MZXW6YTBOI======"), Some(*b"foobar"));
    assert_eq!(Radix::<32>::STD.decode_array::<5>(b"MZXW6YTBOI======"), None);
    assert_eq!(Radix::<32>::STD.decode_array::<7>(b"MZXW6YTBOI======"), None);
}
#[test]
fn buffer_bounds() {
    let mut encoded = [0; 1];
    assert_eq!(Radix::<32>::STD_UNPADDED.encode_to_slice(b"f", &mut encoded), None);
    let mut decoded = [];
    assert_eq!(Radix::<32>::STD_UNPADDED.decode_from_slice(b"MY", &mut decoded), None);
    let mut encoded = [0xaa; 8];
    let written = Radix::<32>::STD_UNPADDED.encode_to_slice(b"f", &mut encoded).unwrap();
    assert_eq!(written, 2);
    assert_eq!(&encoded[..2], b"MY");
    assert_eq!(&encoded[2..], &[0xaa; 6]);
    let mut decoded = [0xaa; 4];
    let written = Radix::<32>::STD_UNPADDED.decode_from_slice(b"MY", &mut decoded).unwrap();
    assert_eq!(written, 1);
    assert_eq!(decoded[0], b'f');
    assert_eq!(&decoded[1..], &[0xaa; 3]);
}
#[test]
fn roundtrip() {
    for radix in [
        Radix::<32>::STD,
        Radix::<32>::STD_UNPADDED,
        Radix::<32>::HEX,
        Radix::<32>::HEX_UNPADDED,
        Radix::<32>::CROCKFORD,
    ] {
        let mut encoded = [0; 512];
        let mut decoded = [0; 256];
        let encoded_len = radix.encode_to_slice(&ALL_BYTES, &mut encoded).unwrap();
        assert_eq!(
            radix.decode_from_slice(&encoded[..encoded_len], &mut decoded),
            Some(ALL_BYTES.len())
        );
        assert_eq!(decoded, ALL_BYTES);
    }
}
#[test]
fn const_operations() {
    const DECODED: Option<[u8; 6]> = Radix::<32>::STD.decode_array(b"MZXW6YTBOI======");
    const RELAXED: Option<[u8; 6]> = Radix::<32>::CROCKFORD.decode_array_relaxed(b"csqpy-rk1e8");
    const ENCODED: ([u8; 10], Option<usize>) = {
        let mut output = [0; 10];
        let written = Radix::<32>::CROCKFORD.encode_to_slice(b"foobar", &mut output);
        (output, written)
    };
    assert_eq!(DECODED, Some(*b"foobar"));
    assert_eq!(RELAXED, Some(*b"foobar"));
    assert_eq!(ENCODED, (*b"CSQPYRK1E8", Some(10)));
}
