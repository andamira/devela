// devela/src/data/codec/integrity/crc/_.rs
//
//! Cyclic redundancy checks.
//

use crate::{ConstInit, Hasher, is, whilst};

#[doc = crate::_tags!(hash)]
/// A cyclic redundancy check, implemented for
/// [`u8`](#impl-Crc<u8>),
/// [`u16`](#impl-Crc<u16>),
/// [`u32`](#impl-Crc<u32>) and
/// [`u64`](#impl-Crc<u64>).
#[doc = crate::_doc_meta!{
    location("data/codec", struct Crc),
    test_size_of(__: Crc<u64> = 8|64; niche !Option),
}]
/// Each integer width uses its conventional bare CRC model:
/// - `Crc<u8>`: CRC-8/SMBUS.
/// - `Crc<u16>`: CRC-16/ARC.
/// - `Crc<u32>`: CRC-32/ISO-HDLC.
/// - `Crc<u64>`: CRC-64/ECMA-182.
///
/// The value stores only the running CRC register.
/// Algorithm parameters are compile-time constants of each implementation.
///
/// CRCs are **byte-order sensitive**: the checksum reflects the exact sequence
/// of input bytes. When used through Rust's [`Hash`][core::hash::Hash] trait,
/// results may therefore differ between targets unless values are serialized
/// to a fixed byte order first.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crc<T> {
    state: T,
}

macro_rules! impl_crc {
    (
        $t:ty;
        name: $name:literal;
        poly: $poly:expr;
        init: $init:expr;
        refin: $refin:literal;
        refout: $refout:literal;
        xorout: $xorout:expr;
        check: $check:expr;
        residue: $residue:expr;
    ) => {
        crate::sf! {
            impl ConstInit for Crc<$t> { const INIT: Self = Self::new(); }
            impl Default for Crc<$t> { fn default() -> Self { Self::new() } }
            impl Hasher for Crc<$t> {
                fn finish(&self) -> u64 { self.checksum() as u64 }
                fn write(&mut self, bytes: &[u8]) { self.write_bytes(bytes); }
            }
        }

        impl Crc<$t> {
            /* model */

            /// Canonical model name.
            pub const NAME: &str = $name;

            /// Width of the CRC register in bits.
            pub const WIDTH: u32 = <$t>::BITS;

            /// Generator polynomial, excluding the implicit highest term.
            pub const POLYNOMIAL: $t = $poly;

            /// Initial register value.
            pub const INITIAL: $t = $init;

            /// Whether input bits are reflected.
            pub const REFLECT_INPUT: bool = $refin;

            /// Whether the final register is reflected.
            pub const REFLECT_OUTPUT: bool = $refout;

            /// Value XORed with the finalized register.
            pub const XOR_OUTPUT: $t = $xorout;

            /// Check value for `b"123456789"`.
            pub const CHECK: $t = $check;

            /// Residue of a valid codeword.
            pub const RESIDUE: $t = $residue;

            /* constructors */

            /// Creates a new CRC state.
            pub const fn new() -> Self {
                // LSB-first processing uses the reflected register representation.
                let state = is![Self::REFLECT_INPUT, Self::INITIAL.reverse_bits(), Self::INITIAL];
                Self { state }
            }
            /// Reconstructs a running state from a finalized checksum.
            pub const fn from_checksum(checksum: $t) -> Self {
                let mut state = checksum ^ Self::XOR_OUTPUT;
                is! { Self::REFLECT_INPUT != Self::REFLECT_OUTPUT, state = state.reverse_bits() }
                Self { state }
            }

            /* checksum */

            /// Returns the finalized checksum at the current position.
            pub const fn checksum(&self) -> $t {
                let mut state = self.state;
                is! { Self::REFLECT_INPUT != Self::REFLECT_OUTPUT, state = state.reverse_bits() }
                state ^ Self::XOR_OUTPUT
            }
            /// Computes the checksum of a complete byte slice.
            pub const fn checksum_bytes(bytes: &[u8]) -> $t {
                let mut crc = Self::new();
                crc.write_bytes(bytes);
                crc.checksum()
            }

            /* update */

            /// Updates the CRC with one byte.
            pub const fn write_byte(&mut self, byte: u8) {
                let mut state = self.state;
                if Self::REFLECT_INPUT {
                    let polynomial = Self::POLYNOMIAL.reverse_bits();
                    state ^= byte as $t;
                    whilst! { i in 0..8; {
                        state = is! { state & 1 != 0, (state >> 1) ^ polynomial, state >> 1 };
                    }}
                } else {
                    let high_bit = (1 as $t) << (<$t>::BITS - 1);
                    state ^= (byte as $t) << (<$t>::BITS - 8);
                    whilst! { i in 0..8; {
                        state = if state & high_bit != 0 {
                            (state << 1) ^ Self::POLYNOMIAL
                        } else {
                            state << 1
                        };
                    }}
                }
                self.state = state;
            }
            /// Updates the CRC with a byte slice.
            pub const fn write_bytes(&mut self, bytes: &[u8]) {
                whilst! { i in 0..bytes.len(); {
                    self.write_byte(bytes[i]);
                }}
            }
        }
    };
}

impl_crc! {
    u8;
    name: "CRC-8/SMBUS";
    poly: 0x07;
    init: 0x00;
    refin: false;
    refout: false;
    xorout: 0x00;
    check: 0xF4;
    residue: 0x00;
}
impl_crc! {
    u16;
    name: "CRC-16/ARC";
    poly: 0x8005;
    init: 0x0000;
    refin: true;
    refout: true;
    xorout: 0x0000;
    check: 0xBB3D;
    residue: 0x0000;
}
impl_crc! {
    u32;
    name: "CRC-32/ISO-HDLC";
    poly: 0x04C1_1DB7;
    init: 0xFFFF_FFFF;
    refin: true;
    refout: true;
    xorout: 0xFFFF_FFFF;
    check: 0xCBF4_3926;
    residue: 0xDEBB_20E3;
}
impl_crc! {
    u64;
    name: "CRC-64/ECMA-182";
    poly: 0x42F0_E1EB_A9EA_3693;
    init: 0x0000_0000_0000_0000;
    refin: false;
    refout: false;
    xorout: 0x0000_0000_0000_0000;
    check: 0x6C40_DF5F_0B49_7347;
    residue: 0x0000_0000_0000_0000;
}

#[cfg(test)]
#[rustfmt::skip]
mod _test{
    use super::Crc;

    const INPUT: &[u8] = b"123456789";
    #[test]
    fn standard_check_values() {
        assert_eq![Crc::<u8>::CHECK, Crc::<u8>::checksum_bytes(INPUT)];
        assert_eq![Crc::<u16>::CHECK, Crc::<u16>::checksum_bytes(INPUT)];
        assert_eq![Crc::<u32>::CHECK, Crc::<u32>::checksum_bytes(INPUT)];
        assert_eq![Crc::<u64>::CHECK, Crc::<u64>::checksum_bytes(INPUT)];
    }
    #[test]
    fn incremental() {
        let full = Crc::<u32>::checksum_bytes(b"HelloWorld");
        let mut crc = Crc::<u32>::new();
        crc.write_bytes(b"Hello");
        crc.write_bytes(b"World");
        assert_eq![full, crc.checksum()];
    }
    #[test]
    fn byte_by_byte() {
        let mut crc = Crc::<u32>::new();
        for byte in INPUT {
            crc.write_byte(*byte);
        }
        assert_eq![Crc::<u32>::CHECK, crc.checksum()];
    }
    #[test]
    fn resume_from_checksum() {
        let full = Crc::<u32>::checksum_bytes(b"HelloWorld");
        let first = Crc::<u32>::checksum_bytes(b"Hello");
        let mut crc = Crc::<u32>::from_checksum(first);
        crc.write_bytes(b"World");
        assert_eq![full, crc.checksum()];
    }
    #[test]
    fn const_checksum() {
        const CHECK: u32 = Crc::<u32>::checksum_bytes(INPUT);
        assert_eq![Crc::<u32>::CHECK, CHECK];
    }
}
