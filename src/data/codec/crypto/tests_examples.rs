// devela::data::codec::crypto::tests_sha
//
// TOC
// - define test example hashes
// - define test helper macros
// - tests:
//   - md5
//   - sha1
//   - sha2_256
//   - sha2_512

#![cfg_attr(not(test), allow(unused))]

macro_rules! _doc_example {
    () => {
        "This implementation is allocation-free, compile-time friendly,
and supports incremental updates.

Use [`update`][Self::update] to feed bytes and [`finalize`][Self::finalize]
to consume the state and return the digest.

Generated with [`digest!`][crate::digest]."
    };
}

crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental Md5 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc_example!()]
    pub struct Md5: Md5
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-1 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc_example!()]
    pub struct Sha1: Sha1
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-256 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc_example!()]
    pub struct Sha256: Sha256
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-512 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc_example!()]
    pub struct Sha512: Sha512
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-224 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc_example!()]
    pub struct Sha224: Sha224
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-384 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc_example!()]
    pub struct Sha384: Sha384
}
/*
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-512/224 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc_example!()]
    pub struct Sha512_224: Sha512_224
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-512/256 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc_example!()]
    pub struct Sha512_256: Sha512_256
}
*/

/* helper macros */

macro_rules! _assert_digest_vectors {
    ($Alg:ident; $($input:expr => $hex:literal),+ $(,)?) => {
        $( assert_eq!($Alg::digest_bytes($input).unwrap(), $crate::Digest($crate::_hex($hex))); )+
    };
}
macro_rules! _assert_hmac_vectors {
    ($Alg:ident; $($key:expr, $data:expr => $hex:literal),+ $(,)?) => {
        $( assert_eq!($Alg::hmac($key, $data).unwrap(), $crate::Digest($crate::_hex($hex))); )+
    };
}
macro_rules! _assert_hotp_codes {
    ($Alg:ident, $key:expr, $digits:expr; $($counter:expr => $code:expr),+ $(,)?) => {
        $( assert_eq!($Alg::hotp($key, $counter, $digits).unwrap().code(), $code); )+
    };
}
macro_rules! _assert_totp_codes {
    ($Alg:ident, $key:expr; $($time:expr => $code:expr),+ $(,)?) => {
        $( assert_eq!($Alg::totp_with($key, $time, 0, 30, 8).unwrap().code(), $code); )+
    };
}
macro_rules! _digest_common_tests {
    ($Alg:ident;
        abc: $abc:literal,
        block: $block:literal,
        pad_at: $pad_at:literal,
        million_a: $million:literal $(,)?
    ) => {
        #[test]
        #[cfg(not(miri))] // too slow for miri
        fn million_a() {
            let mut digest = $Alg::new();
            for _ in 0..1_000_000 {
                digest.update(b"a").unwrap();
            }
            assert_eq!(digest.finalize(), $crate::Digest($crate::_hex($million)));
        }
        #[test]
        fn chunked_updates_match_one_shot() {
            let input = b"the quick brown fox jumps over the lazy dog";
            let full = $Alg::digest_bytes(input).unwrap();
            let mut chunked = $Alg::new();
            chunked.update(b"the quick ").unwrap();
            chunked.update(b"brown fox ").unwrap();
            chunked.update(b"jumps over ").unwrap();
            chunked.update(b"the lazy dog").unwrap();
            assert_eq!(full, chunked.finalize());
        }
        #[test]
        fn boundary_lengths() {
            let lengths = [
                0,
                1,
                $pad_at - 1,
                $pad_at,
                $pad_at + 1,
                $block - 1,
                $block,
                $block + 1,
                $block * 2 - 9,
                $block * 2 - 8,
            ];
            let mut bytes = [0u8; 256];
            for len in lengths {
                crate::whilst! { i in 0..len; { bytes[i] = i as u8; }}
                let full = $Alg::digest_bytes(&bytes[..len]).unwrap();
                let mut chunked = $Alg::new();
                for chunk in bytes[..len].chunks(3) {
                    chunked.update(chunk).unwrap();
                }
                assert_eq!(full, chunked.finalize());
            }
        }
        #[test]
        fn reset() {
            let mut digest = $Alg::new();
            digest.update(b"abc").unwrap();
            digest.reset();
            digest.update(b"abc").unwrap();
            assert_eq!(digest.finalize(), $crate::Digest($crate::_hex($abc)));
        }
    };
}

mod md5 {
    use super::Md5;

    _digest_common_tests! { Md5;
        abc: "900150983cd24fb0d6963f7d28e17f72",
        block: 64,
        pad_at: 56,
        million_a: "7707d6ae4e027c70eea2a935c2296f21",
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc1321#appendix-A.5
    fn known_vectors() {
        _assert_digest_vectors! { Md5;
            b"" => "d41d8cd98f00b204e9800998ecf8427e",
            b"a" => "0cc175b9c0f1b6a831c399e269772661",
            b"abc" => "900150983cd24fb0d6963f7d28e17f72",
            b"message digest" => "f96b697d7cb7938d525a2f31aaf161d0",
            b"abcdefghijklmnopqrstuvwxyz" => "c3fcd3d76192e4007dfb496cca67e13b",
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                => "d174ab98d277d9f5a5611c2c9f419d9f",
            b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                => "57edf4a22be3c955ac49da2e2107b67a",
        }
    }
    #[test]
    // https://www.rfc-editor.org/rfc/rfc2202.html#section-2
    fn hmac_vectors() {
        let key = [0x0b; 16];
        _assert_hmac_vectors! { Md5;
            &key, b"Hi There" => "9294727a3638bb1c13f48ef8158bfc9d",
            b"Jefe", b"what do ya want for nothing?" => "750c783e6ab0b503eaa86e310a5db738",
        }
    }
}
mod sha1 {
    use super::Sha1;

    _digest_common_tests! { Sha1;
        abc: "a9993e364706816aba3e25717850c26c9cd0d89d",
        block: 64,
        pad_at: 56,
        million_a: "34aa973cd4c4daa4f61eeb2bdbad27316534016f",
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#section-8.5
    fn known_vectors() {
        _assert_digest_vectors! { Sha1;
            b"" => "da39a3ee5e6b4b0d3255bfef95601890afd80709",
            b"abc" => "a9993e364706816aba3e25717850c26c9cd0d89d",
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
                => "84983e441c3bd26ebaae4aa1f95129e5e54670f1",
        }
    }
    #[test]
    // https://www.rfc-editor.org/rfc/rfc2202#section-2
    // https://datatracker.ietf.org/doc/html/rfc6234#page-100
    fn hmac_vectors() {
        _assert_hmac_vectors! { Sha1;
            // 1
            &[0x0b; 20], b"Hi There" => "b617318655057264e28bc0b6fb378c8ef146be00",
            // 2
            b"Jefe", b"what do ya want for nothing?"
                => "effcdf6ae5eb2fa2d27416d5f184df9c259a7c79",
            // 3
            &[0xaa; 20], &[0xdd; 50] => "125d7342b9ac11cd91a39af48aa17b4f63f175d3",
            // 4
            &crate::_hex::<25>("0102030405060708090a0b0c0d0e0f10111213141516171819"), &[0xcd; 50]
                => "4c9007f4026250c6bc8414f9bf50c86c2d7235da",
            // 7
            &[0xaa; 80],
            b"Test Using Larger Than Block-Size Key and Larger Than One Block-Size Data"
                => "e8e99d0f45237d786d6bbaa7965c7808bbff1a91",
        }
    }
    #[test]
    // https://datatracker.ietf.org/doc/html/rfc4226#page-32
    fn otp_sha1_rfc4226() {
        let key = crate::_hex::<20>("3132333435363738393031323334353637383930");
        _assert_hotp_codes! { Sha1, &key, 6;
            0 => 755224, 1 => 287082, 2 => 359152, 3 => 969429, 4 => 338314,
            5 => 254676, 6 => 287922, 7 => 162583, 8 => 399871, 9 => 520489,
        }
    }
    /* https://www.rfc-editor.org/rfc/rfc6238#appendix-B */
    #[test]
    fn otp_sha1_rfc6238() {
        _assert_totp_codes! { Sha1, b"12345678901234567890"; // 20
            59 => 94287082,
            1_111_111_109 => 07081804,
            1_111_111_111 => 14050471,
            1_234_567_890 => 89005924,
            2_000_000_000 => 69279037,
            20_000_000_000 => 65353130,
        }
    }
}
mod sha2_256 {
    use super::Sha256;

    _digest_common_tests! { Sha256;
        abc: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        block: 64,
        pad_at: 56,
        million_a: "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0",
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#section-8.5
    fn known_vectors() {
        _assert_digest_vectors! { Sha256;
            b"" => "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            b"abc" => "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq" =>
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        }
    }
    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#page-100
    fn hmac_vectors() {
        _assert_hmac_vectors! { Sha256;
            // 1
            &[0x0b; 20], b"Hi There"
                => "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7",
            // 2
            b"Jefe", b"what do ya want for nothing?"
                => "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843",
        }
    }
    #[test]
    fn otp_sha256_rfc6238() {
        _assert_totp_codes! { Sha256, b"12345678901234567890123456789012"; // 32
            59 => 46119246,
            1_111_111_109 => 68084774,
            1_111_111_111 => 67062674,
            1_234_567_890 => 91819424,
            2_000_000_000 => 90698825,
            20_000_000_000 => 77737706,
        }
    }
}
mod sha2_512 {
    use super::Sha512;

    _digest_common_tests! { Sha512;
        abc: "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
              2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        block: 128,
        pad_at: 112,
        million_a: "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973eb\
                    de0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b",
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#section-8.5
    fn known_vectors() {
        _assert_digest_vectors! { Sha512;
            b"" =>
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
             47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
            b"abc" =>
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq" =>
            "204a8fc6dda82f0a0ced7beb8e08a41657c16ef468b228a8279be331a703c335\
            96fd15c13b1b07f9aa1d3bea57789ca031ad85c7a71dd70354ec631238ca3445",
        }
    }
    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#page-100
    fn hmac_vectors() {
        _assert_hmac_vectors! { Sha512;
           // 1
           &[0x0b; 20], b"Hi There" =>
               "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde\
                daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854",
           // 2
           b"Jefe", b"what do ya want for nothing?" =>
               "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea250554\
                9758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737",
        }
    }
    #[test]
    fn otp_sha512_rfc6238() {
        _assert_totp_codes! { Sha512,
            b"1234567890123456789012345678901234567890123456789012345678901234"; // 64
            59 => 90693936,
            1_111_111_109 => 25091201,
            1_111_111_111 => 99943326,
            1_234_567_890 => 93441116,
            2_000_000_000 => 38618901,
            20_000_000_000 => 47863826,
        }
    }
}
