// devela::num::prob::rand::std
//
//! Defines [`RandStd`].
//

use crate::{Hasher, HasherBuild, Infallible, RandQualities, RandTry, RandomState};

#[cfg(feature = "std")]
#[doc = crate::_tags!(rand)]
/// Standard-library hash-random source.
#[doc = crate::_doc_meta!{location("num/prob/rand")}]
#[derive(Clone, Copy, Debug, Default)]
pub struct StdRand;

#[cfg(feature = "std")]
impl StdRand {
    /// Domain separator for the hash-derived output stream.
    const DOMAIN: &[u8] = b"devela::StdRand/v1";

    /// Returns hash-randomized bits from `RandomState`.
    ///
    /// This is ordinary std-provided randomized seed material,
    /// not a dedicated cryptographic entropy API.
    #[inline(never)]
    pub fn random_u64() -> u64 {
        let h = RandomState::new();
        let mut hasher = h.build_hasher();
        hasher.write(Self::DOMAIN);
        hasher.finish()
    }
    /// Fills `buf` using repeated `RandomState` hash outputs.
    #[inline(never)]
    pub fn random_bytes(buf: &mut [u8]) {
        let h = RandomState::new();
        let mut counter = 0u64;
        let mut i = 0;
        while i < buf.len() {
            let mut hasher = h.build_hasher();
            hasher.write(Self::DOMAIN);
            hasher.write_u64(counter);
            let bytes = hasher.finish().to_ne_bytes();
            let n = usize::min(8, buf.len() - i);
            buf[i..i + n].copy_from_slice(&bytes[..n]);
            counter = counter.wrapping_add(1);
            i += n;
        }
    }
}

#[cfg(feature = "std")]
impl RandTry for StdRand {
    type Error = Infallible;

    const RAND_OUTPUT_BITS: u32 = 64;
    const RAND_STATE_BITS: u32 = 0;
    const RAND_QUALITIES: RandQualities = RandQualities::EXTERNAL;

    #[inline(always)]
    fn rand_try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(StdRand::random_u64())
    }
    #[inline(always)]
    fn rand_try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        StdRand::random_bytes(buf);
        Ok(())
    }
}
