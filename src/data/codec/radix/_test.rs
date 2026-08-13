// devela/src/data/codec/radix/_test.rs

use super::*;

const fn hex<const N: usize>(s: &str) -> [u8; N] {
    match Radix::<16>::HEX.decode_array(s.as_bytes()) {
        Some(bytes) => bytes,
        None => panic!("invalid hexadecimal"),
    }
}

mod base16 {
    use super::*;

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
        let input = b"\x00\x01\x0f\x10\x7f\x80\xfe\xff";
        for radix in [Radix::<16>::HEX, Radix::<16>::HEX_LOWER] {
            let mut encoded = [0; 16];
            let mut decoded = [0; 8];
            assert_eq!(radix.encode_to_slice(input, &mut encoded), Some(encoded.len()));
            assert_eq!(radix.decode_from_slice(&encoded, &mut decoded), Some(decoded.len()));
            assert_eq!(&decoded, input);
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
}

mod base32 {
    use super::*;
}

mod base64 {
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
        for radix in [Radix::<64>::STD, Radix::<64>::STD_UNPADDED] {
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
        for input in [b"A".as_slice(), b"Zg===".as_slice(), b"Z=g=".as_slice(), b"Zm9v=".as_slice()]
        {
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
        let input = b"\x00\x01\x7f\x80\xfe\xff hello!";
        for radix in [
            Radix::<64>::STD,
            Radix::<64>::STD_UNPADDED,
            Radix::<64>::URL,
            Radix::<64>::URL_UNPADDED,
        ] {
            let mut encoded = [0; 32];
            let mut decoded = [0; 14];
            let encoded_len = radix.encode_to_slice(input, &mut encoded).unwrap();
            assert_eq!(
                radix.decode_from_slice(&encoded[..encoded_len], &mut decoded,),
                Some(input.len())
            );
            assert_eq!(&decoded[..input.len()], input);
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
}
