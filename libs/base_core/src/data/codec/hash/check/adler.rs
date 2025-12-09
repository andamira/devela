// devela_base_core::data::codec::hash::check::adler
//
//! Defines [`Adler32`].
//

use crate::{ConstInitCore, Hasher, Slice, define_lane, unwrap, whilst};

define_lane! {
    #[derive(Clone, Copy)]
    struct Lane4 lanes(4); unsigned(u32);
}

/// Adler-32 checksum.
///
/// A lightweight checksum consisting of two 16-bit accumulators (`a` and `b`)
/// updated modulo 65521, the largest prime below 2^16.
/// The final checksum packs `b` in the high 16 bits and `a` in the low 16 bits.
///
/// Adler-32 is **byte-order sensitive**: the checksum reflects the exact
/// sequence of input bytes. When used together with Rust’s `Hash` trait,
/// results may differ between little-endian and big-endian targets unless
/// the input is serialized in a fixed byte order.
///
/// This type supports incremental updates via [`write_bytes`][Self::write_bytes],
/// making it suitable for checksumming streaming data.
///
/// For details, see <https://en.wikipedia.org/wiki/Adler-32>.
#[derive(Debug, Copy, Clone)]
pub struct Adler32 {
    /// Running sum of bytes.
    a: u16,
    /// Running sum of all previous a values.
    b: u16,
}

#[rustfmt::skip]
impl ConstInitCore for Adler32 { const INIT: Self = Self::new(); }
#[rustfmt::skip]
impl Default for Adler32 { fn default() -> Self { Self::new() } }
#[rustfmt::skip]
impl Hasher for Adler32 {
    fn finish(&self) -> u64 { u64::from(self.checksum()) }
    fn write(&mut self, bytes: &[u8]) { self.write_bytes(bytes); }
}

#[rustfmt::skip]
impl Adler32 {
    /// Creates a new Adler-32 state with the initial value `1`.
    ///
    /// This corresponds to the standard Adler-32 initial state `(a=1, b=0)`.
    pub const fn new() -> Self { Self { a: 1, b: 0 } }

    /// Builds a checksum state from a previously computed Adler-32 value.
    pub const fn from_checksum(sum: u32) -> Self {
        Adler32 { a: sum as u16, b: (sum >> 16) as u16 }
    }

    /// Returns the current Adler-32 checksum as a packed `u32`.
    pub const fn checksum(&self) -> u32 {
        ((self.b as u32) << 16) | self.a as u32
    }

    /// Computes the Adler-32 checksum of a byte slice.
    ///
    /// This is a convenience wrapper that does not require
    /// constructing an [`Adler32`] value explicitly.
    pub const fn checksum_bytes(bytes: &[u8]) -> u32 {
        let mut new = Adler32::new();
        new.write_bytes(bytes);
        new.checksum()
    }

    /// Updates the checksum with the given bytes.
    ///
    /// This method may be called multiple times to checksum data streams.
    pub const fn write_bytes(&mut self, bytes: &[u8]) {
        self.update_chucked(bytes);
    }

    /// Internal Adler-32 update routine.
    ///
    /// Implements the standard Adler-32 update step:
    /// ```text
    /// a = (a + data[i]) mod 65521
    /// b = (b + a)        mod 65521
    /// ```
    /// using chunked reduction to avoid accumulator overflow.
    const fn update_chucked(&mut self, bytes: &[u8]) {
        const MOD: u32 = 65521;
        const OUTER_CHUNK: usize = 5552 * INNER_CHUNK;
        const INNER_CHUNK: usize = 4;
        let mut a = self.a as u32;
        let mut b = self.b as u32;
        let mut a_vec = Lane4::<u32>([0; INNER_CHUNK]);
        let mut b_vec = a_vec;

        let (bytes, final_remainder) = bytes.split_at(bytes.len() - bytes.len() % INNER_CHUNK);

        // vectorized outer processing (full OUTER_CHUNK blocks)
        let (outer_count, remainder_outer_chunk) = Slice::chunks_exact::<OUTER_CHUNK>(bytes);
        whilst! { i in 0..outer_count; {
            let outer_chunk = unwrap![some Slice::chunk::<OUTER_CHUNK>(bytes, i)];

            // vectorized inner accumulation over 4-byte groups
            let (inner_count, _) = Slice::chunks_exact::<INNER_CHUNK>(outer_chunk);
            whilst! { j in 0..inner_count; {
                let byte_vec = Slice::chunk::<INNER_CHUNK>(outer_chunk, j).unwrap();
                let val = Lane4::<u32>::from_bytes(byte_vec);
                a_vec.add_assign(val);
                b_vec.add_assign(a_vec);
            }}
            b += OUTER_CHUNK as u32 * a;
            a_vec.rem_scalar_assign(MOD);
            b_vec.rem_scalar_assign(MOD);
            b %= MOD;
        }}
        // vectorized remainder after the outer loop
        let (inner_count, _) = Slice::chunks_exact::<INNER_CHUNK>(remainder_outer_chunk);
        whilst! { i in 0..inner_count; {
            let byte_vec = unwrap![some Slice::chunk::<4>(remainder_outer_chunk, i)];
            let val = Lane4::<u32>::from_bytes(byte_vec);
            a_vec.add_assign(val);
            b_vec.add_assign(a_vec);
        }}

        // combine vector lanes into scalar a and b
        b += remainder_outer_chunk.len() as u32 * a;
        a_vec.rem_scalar_assign(MOD);
        b_vec.rem_scalar_assign(MOD);
        b %= MOD;
        b_vec.mul_scalar_assign(4);
        b_vec.0[1] += MOD - a_vec.0[1];
        b_vec.0[2] += (MOD - a_vec.0[2]) * 2;
        b_vec.0[3] += (MOD - a_vec.0[3]) * 3;
        whilst! { i in 0..a_vec.0.len(); { a += a_vec.0[i]; }}
        whilst! { i in 0..b_vec.0.len(); { b += b_vec.0[i]; }}

        // final scalar remainder (bytes < 4)
        whilst! { i in 0..final_remainder.len(); {
            a += final_remainder[i] as u32;
            b += a;
        }}
        // reduce to 16-bit result
        self.a = (a % MOD) as u16;
        self.b = (b % MOD) as u16;
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use crate::{Adler32, Hash};

    #[test]
    fn checksum_bytes() {
        assert_eq![0x00000001, Adler32::checksum_bytes(b"")];
        assert_eq![0x00620062, Adler32::checksum_bytes(b"a")];
        assert_eq![0x024D0127, Adler32::checksum_bytes(b"abc")];
        assert_eq![0x29750586, Adler32::checksum_bytes(b"message digest")];
        assert_eq![0x64A607E0, Adler32::checksum_bytes(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ")];
        assert_eq![0x11E60398, Adler32::checksum_bytes(b"Wikipedia")];
    }

    #[test]
    fn write_bytes() {
        let full = Adler32::checksum_bytes(b"HelloWorld");
        let mut st = Adler32::new();
        st.write_bytes(b"Hello");
        st.write_bytes(b"World");
        assert_eq!(full, st.checksum());
    }

    #[test]
    fn many_small_updates() {
        let input = b"abcdefghij1234567890";
        let mut a = Adler32::new();
        a.write_bytes(input);
        let full = a.checksum();
        let mut b = Adler32::new();
        for byte in input {
            b.write_bytes(&[*byte]);
        }
        assert_eq!(full, b.checksum());
    }

    #[test]
    fn hash_struct() {
        #[derive(Hash)]
        struct Packet { id: u16, flag: bool, value: u32, }
        let pkt = Packet { id: 0x1234, flag: true, value: 0xCAFEBABE };
        let mut ad = Adler32::new();
        pkt.hash(&mut ad);
        if cfg!(target_endian = "little") { assert_eq!(ad.checksum(), 0x09D00388); }
        if cfg!(target_endian = "big") { assert_eq!(ad.checksum(), 0x0A160388); }
    }
}
