// devela::data::codec::crypto::tests_sha

use crate::{_hex, Digest, Otp, Sha1, whilst};

macro_rules! _doc {
    () => {
        "This implementation is allocation-free, compile-time friendly,
and supports incremental updates.

Use [`update`][Self::update] to feed bytes and [`finalize`][Self::finalize]
to consume the state and return the digest.

Generated with [`digest!`][crate::digest]."
    };
}

// crate::digest![pub struct Md5: Md5]; // TODO
// crate::digest![pub struct Sha1: Sha1]; // TODO
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-256 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc!()]
    pub struct Sha256: Sha256
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-512 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc!()]
    pub struct Sha512: Sha512
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-224 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc!()]
    pub struct Sha224: Sha224
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-384 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc!()]
    pub struct Sha384: Sha384
}
/*
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-512/224 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc!()]
    pub struct Sha512_224: Sha512_224
}
crate::digest! {
    #[doc = crate::_tags!(example crypto hash)] #[doc = "Incremental SHA-512/256 state."]
    #[doc = crate::_doc_location!("data/codec/crypto")] #[doc = _doc!()]
    pub struct Sha512_256: Sha512_256
}
*/

mod sha1 {
    use super::*;

    fn digest_from_hex(hex: &str) -> Digest<{ Sha1::DIGEST_LEN }> {
        Digest(self::_hex(hex))
    }
    fn assert_digest(input: &[u8], expected: &str) {
        assert_eq!(Sha1::digest_bytes(input).unwrap(), digest_from_hex(expected));
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#section-8.5
    fn known_vectors() {
        assert_digest(b"", "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_digest(b"abc", "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert_digest(
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "84983e441c3bd26ebaae4aa1f95129e5e54670f1",
        );
    }
    #[test]
    #[cfg(not(miri))] // too slow for miri
    fn million_a() {
        let mut sha = Sha1::new();
        for _ in 0..1_000_000 {
            sha.update(b"a").unwrap();
        }
        assert_eq!(sha.finalize(), digest_from_hex("34aa973cd4c4daa4f61eeb2bdbad27316534016f"),);
    }
    #[test]
    // https://www.rfc-editor.org/rfc/rfc2202#section-2
    // https://datatracker.ietf.org/doc/html/rfc6234#page-100
    fn hmac_vectors() {
        // 1
        let (key, data) = ([0x0b; 20], b"Hi There");
        let mac = Sha1::hmac(&key, data).unwrap();
        assert_eq!(mac.as_array(), &_hex("b617318655057264e28bc0b6fb378c8ef146be00"));
        // 2
        let (key, data) = (b"Jefe", b"what do ya want for nothing?");
        let mac = Sha1::hmac(key, data).unwrap();
        assert_eq!(mac.as_array(), &_hex("effcdf6ae5eb2fa2d27416d5f184df9c259a7c79"));
        // 3
        let (key, data) = ([0xaa; 20], [0xdd; 50]);
        let mac = Sha1::hmac(&key, &data).unwrap();
        assert_eq!(mac.as_array(), &_hex("125d7342b9ac11cd91a39af48aa17b4f63f175d3"));
        // 4
        let key = _hex::<25>("0102030405060708090a0b0c0d0e0f10111213141516171819");
        let data = [0xcd; 50];
        let mac = Sha1::hmac(&key, &data).unwrap();
        assert_eq!(mac.as_array(), &_hex("4c9007f4026250c6bc8414f9bf50c86c2d7235da"));
        // …
        // 7
        let key = [0xaa; 80];
        let data = b"Test Using Larger Than Block-Size Key and Larger Than One Block-Size Data";
        let mac = Sha1::hmac(&key, data).unwrap();
        assert_eq!(mac.as_array(), &_hex("e8e99d0f45237d786d6bbaa7965c7808bbff1a91"));
    }
    #[test]
    // https://datatracker.ietf.org/doc/html/rfc4226#page-32
    fn otp_sha1_rfc4226() {
        let key = _hex::<20>("3132333435363738393031323334353637383930");
        let hotp = Sha1::hotp(&key, 0, Otp::DEFAULT_DIGITS).unwrap();
        assert_eq![hotp.digits(), 6];
        assert_eq![hotp.code(), 755224];
        assert_eq![Sha1::hotp(&key, 1, Otp::DEFAULT_DIGITS).unwrap().code(), 287082];
        assert_eq![Sha1::hotp(&key, 2, Otp::DEFAULT_DIGITS).unwrap().code(), 359152];
        assert_eq![Sha1::hotp(&key, 3, Otp::DEFAULT_DIGITS).unwrap().code(), 969429];
        assert_eq![Sha1::hotp(&key, 4, Otp::DEFAULT_DIGITS).unwrap().code(), 338314];
        assert_eq![Sha1::hotp(&key, 5, Otp::DEFAULT_DIGITS).unwrap().code(), 254676];
        assert_eq![Sha1::hotp(&key, 6, Otp::DEFAULT_DIGITS).unwrap().code(), 287922];
        assert_eq![Sha1::hotp(&key, 7, Otp::DEFAULT_DIGITS).unwrap().code(), 162583];
        assert_eq![Sha1::hotp(&key, 8, Otp::DEFAULT_DIGITS).unwrap().code(), 399871];
        assert_eq![Sha1::hotp(&key, 9, Otp::DEFAULT_DIGITS).unwrap().code(), 520489];
    }
    /* https://www.rfc-editor.org/rfc/rfc6238#appendix-B */
    #[test]
    fn otp_sha1_rfc6238() {
        let key = b"12345678901234567890"; // 20
        assert_eq![Sha1::totp_with(key, 59, 0, 30, 8).unwrap().code(), 94287082];
        assert_eq![Sha1::totp_with(key, 1_111_111_109, 0, 30, 8).unwrap().code(), 07081804];
        assert_eq![Sha1::totp_with(key, 1_111_111_111, 0, 30, 8).unwrap().code(), 14050471];
        assert_eq![Sha1::totp_with(key, 1_234_567_890, 0, 30, 8).unwrap().code(), 89005924];
        assert_eq![Sha1::totp_with(key, 2_000_000_000, 0, 30, 8).unwrap().code(), 69279037];
        assert_eq![Sha1::totp_with(key, 20_000_000_000, 0, 30, 8).unwrap().code(), 65353130];
    }
    #[test]
    fn chunked_updates_match_one_shot() {
        let input = b"the quick brown fox jumps over the lazy dog";
        let full = Sha1::digest_bytes(input).unwrap();
        let mut chunked = Sha1::new();
        chunked.update(b"the quick ").unwrap();
        chunked.update(b"brown fox ").unwrap();
        chunked.update(b"jumps over ").unwrap();
        chunked.update(b"the lazy dog").unwrap();
        assert_eq!(full, chunked.finalize());
    }
    #[test]
    fn boundary_lengths() {
        for len in [0, 1, 55, 56, 57, 63, 64, 65, 119, 120] {
            let mut bytes = [0u8; 120];
            whilst! { i in 0..len; { bytes[i] = i as u8; }}
            let full = Sha1::digest_bytes(&bytes[..len]).unwrap();
            let mut chunked = Sha1::new();
            for chunk in bytes[..len].chunks(3) {
                chunked.update(chunk).unwrap();
            }
            assert_eq!(full, chunked.finalize());
        }
    }
    #[test]
    fn reset() {
        let mut sha = Sha1::new();
        sha.update(b"abc").unwrap();
        sha.reset();
        sha.update(b"abc").unwrap();
        assert_eq!(sha.finalize(), digest_from_hex("a9993e364706816aba3e25717850c26c9cd0d89d"),);
    }
}

mod sha2_256 {
    use super::*;

    fn digest_from_hex(hex: &str) -> Digest<{ Sha256::DIGEST_LEN }> {
        Digest(self::_hex(hex))
    }
    fn assert_digest(input: &[u8], expected: &str) {
        assert_eq!(Sha256::digest_bytes(input).unwrap(), digest_from_hex(expected));
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#section-8.5
    fn known_vectors() {
        assert_digest(b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_digest(b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_digest(
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        );
    }
    #[test]
    #[cfg(not(miri))] // too slow for miri
    fn million_a() {
        let mut sha = Sha256::new();
        for _ in 0..1_000_000 {
            sha.update(b"a").unwrap();
        }
        assert_eq!(
            sha.finalize(),
            digest_from_hex("cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"),
        );
    }
    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#page-100
    fn hmac_vectors() {
        // 1
        let key = [0x0b; 20];
        assert_eq!(
            Sha256::hmac(&key, b"Hi There").unwrap(),
            digest_from_hex("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"),
        );
        // 2
        assert_eq!(
            Sha256::hmac(b"Jefe", b"what do ya want for nothing?").unwrap(),
            digest_from_hex("5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"),
        );
    }
    #[test]
    fn otp_sha256_rfc6238() {
        let key = b"12345678901234567890123456789012"; // 32
        assert_eq![Sha256::totp_with(key, 59, 0, 30, 8).unwrap().code(), 46119246];
        assert_eq![Sha256::totp_with(key, 1_111_111_109, 0, 30, 8).unwrap().code(), 68084774];
        assert_eq![Sha256::totp_with(key, 1_111_111_111, 0, 30, 8).unwrap().code(), 67062674];
        assert_eq![Sha256::totp_with(key, 1_234_567_890, 0, 30, 8).unwrap().code(), 91819424];
        assert_eq![Sha256::totp_with(key, 2_000_000_000, 0, 30, 8).unwrap().code(), 90698825];
        assert_eq![Sha256::totp_with(key, 20_000_000_000, 0, 30, 8).unwrap().code(), 77737706];
    }
    #[test]
    fn chunked_updates_match_one_shot() {
        let input = b"the quick brown fox jumps over the lazy dog";
        let full = Sha256::digest_bytes(input).unwrap();
        let mut chunked = Sha256::new();
        chunked.update(b"the quick ").unwrap();
        chunked.update(b"brown fox ").unwrap();
        chunked.update(b"jumps over ").unwrap();
        chunked.update(b"the lazy dog").unwrap();
        assert_eq!(full, chunked.finalize());
    }

    #[test]
    fn boundary_lengths() {
        for len in [0, 1, 55, 56, 57, 63, 64, 65, 119, 120] {
            let mut bytes = [0u8; 120];
            whilst! { i in 0..len; { bytes[i] = i as u8; }}
            let full = Sha256::digest_bytes(&bytes[..len]).unwrap();
            let mut chunked = Sha256::new();
            for chunk in bytes[..len].chunks(3) {
                chunked.update(chunk).unwrap();
            }
            assert_eq!(full, chunked.finalize());
        }
    }
    #[test]
    fn reset() {
        let mut sha = Sha256::new();
        sha.update(b"abc").unwrap();
        sha.reset();
        sha.update(b"abc").unwrap();
        assert_eq!(
            sha.finalize(),
            digest_from_hex("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        );
    }
}

mod sha2_512 {
    use super::*;

    fn digest_from_hex(hex: &str) -> Digest<{ Sha512::DIGEST_LEN }> {
        Digest(self::_hex(hex))
    }
    fn assert_digest(input: &[u8], expected: &str) {
        assert_eq!(Sha512::digest_bytes(input).unwrap(), digest_from_hex(expected));
    }

    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#section-8.5
    fn known_vectors() {
        assert_digest(
            b"",
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
             47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
        );
        assert_digest(
            b"abc",
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        );
        assert_digest(
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "204a8fc6dda82f0a0ced7beb8e08a41657c16ef468b228a8279be331a703c335\
            96fd15c13b1b07f9aa1d3bea57789ca031ad85c7a71dd70354ec631238ca3445",
        );
    }
    #[test]
    #[cfg(not(miri))] // too slow for miri
    fn million_a() {
        let mut sha = Sha512::new();
        for _ in 0..1_000_000 {
            sha.update(b"a").unwrap();
        }
        assert_eq!(
            sha.finalize(),
            digest_from_hex(
                "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973eb\
                 de0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b",
            ),
        );
    }
    #[test]
    // https://datatracker.ietf.org/doc/html/rfc6234#page-100
    fn hmac_vectors() {
        // 1
        let key = [0x0b; 20];
        assert_eq!(
            Sha512::hmac(&key, b"Hi There").unwrap(),
            digest_from_hex(
                "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde\
                 daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854",
            ),
        );
        // 2
        assert_eq!(
            Sha512::hmac(b"Jefe", b"what do ya want for nothing?").unwrap(),
            digest_from_hex(
                "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea250554\
                 9758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737",
            ),
        );
    }
    #[test]
    fn otp_sha512_rfc6238() {
        let key = b"1234567890123456789012345678901234567890123456789012345678901234"; // 64
        assert_eq![Sha512::totp_with(key, 59, 0, 30, 8).unwrap().code(), 90693936];
        assert_eq![Sha512::totp_with(key, 1_111_111_109, 0, 30, 8).unwrap().code(), 25091201];
        assert_eq![Sha512::totp_with(key, 1_111_111_111, 0, 30, 8).unwrap().code(), 99943326];
        assert_eq![Sha512::totp_with(key, 1_234_567_890, 0, 30, 8).unwrap().code(), 93441116];
        assert_eq![Sha512::totp_with(key, 2_000_000_000, 0, 30, 8).unwrap().code(), 38618901];
        assert_eq![Sha512::totp_with(key, 20_000_000_000, 0, 30, 8).unwrap().code(), 47863826];
    }
    #[test]
    fn chunked_updates_match_one_shot() {
        let input = b"the quick brown fox jumps over the lazy dog";
        let full = Sha512::digest_bytes(input).unwrap();
        let mut chunked = Sha512::new();
        chunked.update(b"the quick ").unwrap();
        chunked.update(b"brown fox ").unwrap();
        chunked.update(b"jumps over ").unwrap();
        chunked.update(b"the lazy dog").unwrap();
        assert_eq!(full, chunked.finalize());
    }
    #[test]
    fn boundary_lengths() {
        for len in [0, 1, 111, 112, 113, 127, 128, 129, 239, 240] {
            let mut bytes = [0u8; 240];
            whilst! { i in 0..len; { bytes[i] = i as u8; }}
            let full = Sha512::digest_bytes(&bytes[..len]).unwrap();
            let mut chunked = Sha512::new();
            for chunk in bytes[..len].chunks(3) {
                chunked.update(chunk).unwrap();
            }
            assert_eq!(full, chunked.finalize());
        }
    }
    #[test]
    fn reset() {
        let mut sha = Sha512::new();
        sha.update(b"abc").unwrap();
        sha.reset();
        sha.update(b"abc").unwrap();
        assert_eq!(
            sha.finalize(),
            digest_from_hex(
                "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
                 2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
            ),
        );
    }
}
