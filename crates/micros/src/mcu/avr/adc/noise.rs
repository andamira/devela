//
//! Defines [`AvrAdcNoise`].
//

use crate::AvrAdc;
#[cfg(feature = "unsafe_mmio")]
use crate::{AttemptLimitReached, AvrAdcInput, RandQualities, RandTry, is};

#[doc = crate::_tags!(hw rand)]
/// Weak entropy harvested from AVR ADC conversion noise.
#[doc = crate::_doc_meta!{
    location("mcu/avr", struct AvrAdcNoise),
    test_size_of(AvrAdcNoise = 12|96; niche !Option),
}]
/// This is intentionally *not* a cryptographic TRNG. ADC noise is board-,
/// channel-, environment-, and circuit-dependent and can be externally influenced.
///
/// Values can only be created by preparing an [`AvrAdc`] for exclusive
/// noise sampling. A Von Neumann extractor rejects equal bit pairs to reduce
/// simple bias, but does not establish independence or a minimum entropy rate.
#[derive(Debug)]
pub struct AvrAdcNoise(AvrAdc);

impl AvrAdcNoise {
    pub(crate) fn _new(adc: AvrAdc) -> Self {
        Self(adc)
    }

    /// Maximum raw sample pairs examined per requested output bit.
    ///
    /// This bounds extraction work; reaching the limit reports [`AttemptLimitReached`].
    /// The bound does not establish an entropy guarantee.
    pub const MAX_PAIRS_PER_BIT: usize = 64;

    /// Consumes this source and returns the underlying ADC descriptor.
    ///
    /// This does not disable the ADC or restore its previous configuration.
    #[must_use]
    pub const fn into_adc(self) -> AvrAdc {
        self.0
    }

    /* private helpers */

    #[cfg(feature = "unsafe_mmio")]
    fn sample_bit(&self) -> u8 {
        // SAFETY: AvrAdcNoise can only be created after exclusive ADC preparation.
        unsafe { self.0.sample_blocking() as u8 & 1 }
    }

    /// Attempts to extract `bits` using bounded Von Neumann pair rejection.
    #[cfg(feature = "unsafe_mmio")]
    fn try_bits(&mut self, bits: u32) -> Result<u64, AttemptLimitReached> {
        let max_pairs = bits as usize * Self::MAX_PAIRS_PER_BIT;
        let (mut out, mut produced, mut pairs) = (0u64, 0u32, 0usize);
        while produced < bits && pairs < max_pairs {
            let (a, b) = (self.sample_bit(), self.sample_bit());
            pairs += 1;
            if a != b {
                out |= (a as u64) << produced; // 01 → 0, 10 → 1
                produced += 1;
            }
        }
        is! { produced == bits, Ok(out), Err(AttemptLimitReached(Some(max_pairs))) }
    }
}

#[cfg(feature = "unsafe_mmio")]
impl RandTry for AvrAdcNoise {
    type Error = AttemptLimitReached;

    const RAND_OUTPUT_BITS: u32 = 1;
    const RAND_STATE_BITS: u32 = 0;
    const RAND_QUALITIES: RandQualities = RandQualities::EXTERNAL.with_entropy_weak();

    fn rand_try_next_bool(&mut self) -> Result<bool, Self::Error> {
        self.try_bits(1).map(|v| v != 0)
    }
    fn rand_try_next_u8(&mut self) -> Result<u8, Self::Error> {
        self.try_bits(8).map(|v| v as u8)
    }
    fn rand_try_next_u16(&mut self) -> Result<u16, Self::Error> {
        self.try_bits(16).map(|v| v as u16)
    }
    fn rand_try_next_u32(&mut self) -> Result<u32, Self::Error> {
        self.try_bits(32).map(|v| v as u32)
    }
    fn rand_try_next_u64(&mut self) -> Result<u64, Self::Error> {
        self.try_bits(64)
    }
    fn rand_try_fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        for byte in buffer {
            *byte = self.rand_try_next_u8()?;
        }
        Ok(())
    }
}

impl AvrAdc {
    /// Creates a weak-entropy source from the low bit of ADC conversions.
    ///
    /// The source applies a Von Neumann extractor to pairs of ADC LSB samples.
    /// This removes first-order bit bias but cannot establish independence or
    /// a minimum entropy rate, so the resulting source deliberately carries
    /// [`RandQualities::ENTROPY_WEAK`][crate::RandQualities::ENTROPY_WEAK].
    ///
    /// Configures single-ended conversions on `input`
    /// using AVcc as the reference and the ADC clock `prescaler`.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of `2`, `4`, `8`, `16`, `32`, `64`, or `128`.
    ///
    /// # Safety
    /// The ADC and selected analog pin must not be concurrently configured
    /// or accessed elsewhere. This replaces their ADC configuration.
    #[must_use]
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn prepare_noise(self, input: AvrAdcInput, prescaler: u16) -> AvrAdcNoise {
        const REFS0: u8 = 1 << 6;
        const ADEN: u8 = 1 << 7;
        const ADIF: u8 = 1 << 4;
        let Some(adps) = Self::prescaler_bits(prescaler) else {
            panic!("AVR ADC prescaler is not supported");
        };
        let selection = input.selection();
        unsafe {
            // Take ownership of ADC operation: disabled, no auto-trigger, no interrupt.
            self.adcsra.write(0);
            // AVcc reference, right-adjusted result, selected single-ended input.
            self.admux.write(REFS0 | selection);
            // ADC0..=ADC5 have digital input buffers;
            // ADC6/7 and internal ADC inputs do not use DIDR0.
            is! { selection <= 5, self.didr0.write(self.didr0.read() | (1 << selection)) }
            // ADIF is write-one-to-clear.
            self.adcsra.write(ADEN | ADIF | adps);
            // Discard the extended first conversion used to initialize the ADC. (25 clocks vs 13)
            let _ = self.sample_blocking();
        }
        AvrAdcNoise::_new(self)
    }
    #[cfg(feature = "unsafe_mmio")]
    const fn prescaler_bits(prescaler: u16) -> Option<u8> {
        match prescaler {
            2 => Some(0b001),
            4 => Some(0b010),
            8 => Some(0b011),
            16 => Some(0b100),
            32 => Some(0b101),
            64 => Some(0b110),
            128 => Some(0b111),
            _ => None,
        }
    }
}
