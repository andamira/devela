// devela/src/data/codec/radix/_test/base64.rs

use super::*;

#[test]
fn base() {
    assert_eq!(Radix::<64>::BASE, 64);
}
#[test]
fn encode_rfc4648_vectors() {
    let vectors: &[(&[u8], &[u8])] = &[
        (b"", b""),
        (b"f", b"Zg=="),
        (b"fo", b"Zm8="),
        (b"foo", b"Zm9v"),
        (b"foob", b"Zm9vYg=="),
        (b"fooba", b"Zm9vYmE="),
        (b"foobar", b"Zm9vYmFy"),
    ];
    for &(input, expected) in vectors {
        let mut output = [0; 16];
        let written = Radix::<64>::STD.encode_to_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn encode_unpadded() {
    let vectors: &[(&[u8], &[u8])] = &[
        (b"", b""),
        (b"f", b"Zg"),
        (b"fo", b"Zm8"),
        (b"foo", b"Zm9v"),
        (b"foob", b"Zm9vYg"),
        (b"fooba", b"Zm9vYmE"),
        (b"foobar", b"Zm9vYmFy"),
    ];
    for &(input, expected) in vectors {
        let mut output = [0; 16];
        let written = Radix::<64>::STD_UNPADDED.encode_to_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn url_alphabet() {
    let input = [0xfb, 0xff];
    let mut std = [0; 4];
    let mut url = [0; 4];
    assert_eq!(Radix::<64>::STD.encode_to_slice(&input, &mut std), Some(4));
    assert_eq!(Radix::<64>::URL.encode_to_slice(&input, &mut url), Some(4));
    assert_eq!(&std, b"+/8=");
    assert_eq!(&url, b"-_8=");
    let mut url_unpadded = [0; 3];
    assert_eq!(Radix::<64>::URL_UNPADDED.encode_to_slice(&input, &mut url_unpadded), Some(3));
    assert_eq!(&url_unpadded, b"-_8");
    assert_eq!(Radix::<64>::URL_UNPADDED.decode_array::<2>(b"-_8"), Some([0xfb, 0xff]));
}
#[test]
fn decode_rfc4648_vectors() {
    let vectors: &[(&[u8], &[u8])] = &[
        (b"", b""),
        (b"Zg==", b"f"),
        (b"Zm8=", b"fo"),
        (b"Zm9v", b"foo"),
        (b"Zm9vYg==", b"foob"),
        (b"Zm9vYmE=", b"fooba"),
        (b"Zm9vYmFy", b"foobar"),
    ];
    for &(input, expected) in vectors {
        let mut output = [0; 16];
        let written = Radix::<64>::STD.decode_from_slice(input, &mut output).unwrap();
        assert_eq!(&output[..written], expected);
    }
}
#[test]
fn strict_padding() {
    let mut output = [0; 1];
    assert_eq!(Radix::<64>::STD.decode_from_slice(b"Zg==", &mut output), Some(1));
    assert_eq!(Radix::<64>::STD.decode_from_slice(b"Zg", &mut output), None);
    assert_eq!(Radix::<64>::STD_UNPADDED.decode_from_slice(b"Zg", &mut output), Some(1));
    assert_eq!(Radix::<64>::STD_UNPADDED.decode_from_slice(b"Zg==", &mut output), None);
}
#[test]
fn relaxed_padding() {
    for radix in
        [Radix::<64>::STD, Radix::<64>::STD_UNPADDED, Radix::<64>::URL, Radix::<64>::URL_UNPADDED]
    {
        let mut output = [0; 1];
        assert_eq!(radix.decode_from_slice_relaxed(b"Zg", &mut output), Some(1));
        assert_eq!(radix.decode_from_slice_relaxed(b"Zg=", &mut output), Some(1));
        assert_eq!(radix.decode_from_slice_relaxed(b"Zg==", &mut output), Some(1));
        assert_eq!(output, *b"f");
    }
}
#[test]
fn rejects_invalid_padding() {
    let mut output = [0; 8];
    for input in [b"A".as_slice(), b"Zg===".as_slice(), b"Z=g=".as_slice(), b"Zm9v=".as_slice()] {
        assert_eq!(Radix::<64>::STD.decode_from_slice_relaxed(input, &mut output), None);
    }
}
#[test]
fn rejects_wrong_alphabet() {
    let mut output = [0; 2];
    assert_eq!(Radix::<64>::STD.decode_from_slice(b"-_8=", &mut output), None);
    assert_eq!(Radix::<64>::URL.decode_from_slice(b"+/8=", &mut output), None);
    assert_eq!(Radix::<64>::URL.decode_from_slice(b"-_8=", &mut output), Some(2));
    assert_eq!(output, [0xfb, 0xff]);
}
#[test]
fn rejects_noncanonical_trailing_bits() {
    let mut output = [0; 2];
    // Both would decode to the same bytes if unused bits were ignored.
    assert_eq!(Radix::<64>::STD.decode_from_slice(b"Zh==", &mut output), None);
    assert_eq!(Radix::<64>::STD.decode_from_slice(b"Zm9=", &mut output), None);
    // Relaxing padding does not relax trailing-bit validation.
    assert_eq!(Radix::<64>::STD.decode_from_slice_relaxed(b"Zh", &mut output), None);
    assert_eq!(Radix::<64>::STD.decode_from_slice_relaxed(b"Zm9", &mut output), None);
}
#[test]
fn decode_array_exact() {
    assert_eq!(Radix::<64>::STD.decode_array::<3>(b"Zm9v"), Some(*b"foo"));
    assert_eq!(Radix::<64>::STD.decode_array::<2>(b"Zm9v"), None);
    assert_eq!(Radix::<64>::STD.decode_array::<4>(b"Zm9v"), None);
    assert_eq!(Radix::<64>::STD.decode_array_relaxed::<1>(b"Zg"), Some(*b"f"));
}
#[test]
fn roundtrip() {
    for radix in
        [Radix::<64>::STD, Radix::<64>::STD_UNPADDED, Radix::<64>::URL, Radix::<64>::URL_UNPADDED]
    {
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
    const DECODED: Option<[u8; 3]> = Radix::<64>::STD.decode_array(b"Zm9v");
    const ENCODED: ([u8; 4], Option<usize>) = {
        let mut output = [0; 4];
        let written = Radix::<64>::URL.encode_to_slice(&[0xfb, 0xff], &mut output);
        (output, written)
    };
    assert_eq!(DECODED, Some(*b"foo"));
    assert_eq!(ENCODED, (*b"-_8=", Some(4)));
}
#[test]
fn case_sensitive_and_rejects_non_alphabet() {
    assert_eq!(Radix::<64>::STD.decode_array::<1>(b"QQ=="), Some([0x41]));
    assert_eq!(Radix::<64>::STD.decode_array::<1>(b"qQ=="), Some([0xa9]));
    let mut output = [0; 8];
    for input in [b"Z g==".as_slice(), b"Zg==\n".as_slice(), b"*g==".as_slice()] {
        assert_eq!(Radix::<64>::STD.decode_from_slice_relaxed(input, &mut output), None);
    }
}
