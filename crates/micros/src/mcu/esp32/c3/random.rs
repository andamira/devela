//
//! ESP32-C3 hardware random source.
//

use crate::EspReg32;
#[cfg(feature = "unsafe_mmio")]
use crate::{Infallible, RandQualities, RandTry};

#[doc = crate::_tags!(hw rand)]
/// Access to the ESP32-C3 hardware random-number generator.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct Esp32C3Rng),
    test_size_of(Esp32C3Rng = 0),
}]
/// The hardware state may receive entropy from physical sources, but true-random
/// output is only guaranteed while a supported main entropy source is active.
///
/// Without RF or the internal SAR-ADC entropy source, output should be treated
/// as pseudo-random and is represented conservatively as weak entropy.
#[derive(Debug)]
pub struct Esp32C3Rng(());

impl Esp32C3Rng {
    /// RNG data register.
    ///
    /// `SYSCON_RND_DATA_REG = DR_REG_SYSCON_BASE + 0xB0`.
    pub const DATA: EspReg32 = EspReg32::new(0x6002_60B0);

    #[cfg(feature = "unsafe_mmio")]
    pub(crate) const unsafe fn new_unchecked() -> Self {
        Self(())
    }

    /// Reads one 32-bit value from the hardware RNG state.
    #[must_use]
    #[cfg(feature = "unsafe_mmio")]
    pub fn next_u32(&mut self) -> u32 {
        // SAFETY: construction establishes that this is the active ESP32-C3 RNG.
        unsafe { Self::DATA.read() }
    }
}

#[cfg(feature = "unsafe_mmio")]
impl RandTry for Esp32C3Rng {
    type Error = Infallible;

    const RAND_OUTPUT_BITS: u32 = 32;
    const RAND_STATE_BITS: u32 = 0;
    const RAND_QUALITIES: RandQualities = RandQualities::EXTERNAL.with_entropy_weak();

    fn rand_try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let lo = self.next_u32() as u64;
        let hi = self.next_u32() as u64;
        Ok(lo | hi << 32)
    }

    fn rand_try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }
}
