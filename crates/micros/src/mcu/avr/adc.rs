//
//! AVR ADC access and weak physical-noise harvesting.
//

use crate::AvrReg8;
#[cfg(feature = "unsafe_mmio")]
use crate::{AttemptLimitReached, RandQualities, RandTry, is};

#[cfg(target_pointer_width = "16")]
crate::test_size_of!(const AvrAdc = 12|96; niche !Option);

#[doc = crate::_tags!(hw)]
/// A classic AVR 10-bit successive-approximation ADC.
#[doc = crate::_doc_meta!{
    location("mcu/avr", struct AvrAdc),
    test_size_of(AvrAdc = 12|96; niche !Option),
}]
/// It is described by its data, control, multiplexer, and digital-input disable registers.
///
/// This models the classic ADC register layout used by devices such as the ATmega328P;
/// AVR families with different ADC peripherals may require different abstractions.
///
/// Values can safely be copied and inspected. Operations that access the
/// described registers are unsafe because the addresses must correspond to
/// the active device and access must respect the peripheral's hardware state.
///
/// # Methods
///
/// - [Conversion](#method.sample_blocking) — performs a blocking single conversion.
/// - [Noise harvesting](#method.prepare_noise) — prepares bounded weak-entropy extraction.
///
/// [Semantic] and [datasheet] register accessors are also provided for lower-level use.
///
/// [Semantic]: #semantic-registers-api
/// [datasheet]: #datasheet-registers-api
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrAdc {
    adcl: AvrReg8,
    adch: AvrReg8,
    adcsra: AvrReg8,
    adcsrb: AvrReg8,
    admux: AvrReg8,
    didr0: AvrReg8,
}

impl AvrAdc {
    /// Creates an ADC descriptor from its memory-mapped registers.
    #[must_use]
    pub const fn new(
        adcl: u16,
        adch: u16,
        adcsra: u16,
        adcsrb: u16,
        admux: u16,
        didr0: u16,
    ) -> Self {
        Self {
            adcl: AvrReg8::new(adcl),
            adch: AvrReg8::new(adch),
            adcsra: AvrReg8::new(adcsra),
            adcsrb: AvrReg8::new(adcsrb),
            admux: AvrReg8::new(admux),
            didr0: AvrReg8::new(didr0),
        }
    }

    /// Returns all six registers in constructor order.
    #[must_use]
    pub const fn into_parts(self) -> [AvrReg8; 6] {
        [self.adcl, self.adch, self.adcsra, self.adcsrb, self.admux, self.didr0]
    }

    /// Performs one blocking 10-bit ADC conversion.
    ///
    /// Returns the conversion result normalized to `0..=1023`,
    /// independently of the ADC result-adjustment setting.
    ///
    /// # Safety
    /// The ADC must belong to the active device, be enabled in single-conversion
    /// mode, and not be concurrently accessed or reconfigured.
    ///
    /// This may wait indefinitely if the conversion cannot complete.
    #[must_use]
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn sample_blocking(self) -> u16 {
        const ADSC: u8 = 1 << 6;
        const ADLAR: u8 = 1 << 5;
        const ADIF: u8 = 1 << 4;
        unsafe {
            let adcsra = self.adcsra.read() & !ADIF;
            self.adcsra.write(adcsra | ADSC);
            while self.adcsra.read() & ADSC != 0 {}
            // ADCL must be read before ADCH.
            let lo = self.adcl.read() as u16;
            let hi = self.adch.read() as u16;
            if self.admux.read() & ADLAR == 0 {
                lo | ((hi & 0b11) << 8)
            } else {
                (hi << 2) | (lo >> 6)
            }
        }
    }

    /// Creates a weak-entropy source from the low bit of ADC conversions.
    ///
    /// The source applies a Von Neumann extractor to pairs of ADC LSB samples.
    /// This removes first-order bit bias but cannot establish independence or
    /// a minimum entropy rate, so the resulting source deliberately carries
    /// [`RandQualities::ENTROPY_WEAK`].
    ///
    /// Configures single-ended conversions on `channel`
    /// using AVcc as the reference and the ADC clock `prescaler`.
    ///
    /// # Panics
    /// Panics if `channel` is not in `0..=7`, or if `prescaler`
    /// is not one of `2`, `4`, `8`, `16`, `32`, `64`, or `128`.
    ///
    /// # Safety
    /// The ADC and selected analog pin must not be concurrently configured
    /// or accessed elsewhere. This replaces their ADC configuration.
    #[must_use]
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn prepare_noise(self, channel: u8, prescaler: u16) -> AvrAdcNoise {
        assert!(channel <= 7, "AVR ADC channel must be in 0..=7");
        const REFS0: u8 = 1 << 6; // AVcc reference
        const ADEN: u8 = 1 << 7;
        const ADIF: u8 = 1 << 4;
        let Some(adps) = Self::prescaler_bits(prescaler) else {
            panic!("AVR ADC prescaler is not supported");
        };
        unsafe {
            // Take ownership of ADC operation: disabled, no auto-trigger, no interrupt.
            self.adcsra.write(0);
            // AVcc reference, right-adjusted result, selected single-ended channel.
            self.admux.write(REFS0 | channel);
            // ADC0..=ADC5 have digital input buffers; ADC6/7 are analog-only.
            is! { channel <= 5, self.didr0.write(self.didr0.read() | (1 << channel)) }
            // ADIF is write-one-to-clear.
            self.adcsra.write(ADEN | ADIF | adps);
            // Discard the extended first conversion used to initialize the ADC. (25 clocks vs 13)
            let _ = self.sample_blocking();
        }
        AvrAdcNoise(self)
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

/// # Semantic registers API
#[rustfmt::skip]
impl AvrAdc {
    /// Returns the low conversion-result register
    /// ([`ADCL`](#method.adcl_reg)).
    #[must_use]
    pub const fn result_low_reg(self) -> AvrReg8 { self.adcl_reg() }

    /// Returns the high conversion-result register
    /// ([`ADCH`](#method.adch_reg)).
    #[must_use]
    pub const fn result_high_reg(self) -> AvrReg8 { self.adch_reg() }

    /// Returns the main control and status register
    /// ([`ADCSRA`](#method.adcsra_reg)).
    #[must_use]
    pub const fn control_status_reg(self) -> AvrReg8 { self.adcsra_reg() }

    /// Returns the auxiliary control and auto-trigger register
    /// ([`ADCSRB`](#method.adcsrb_reg)).
    #[must_use]
    pub const fn auxiliary_control_reg(self) -> AvrReg8 { self.adcsrb_reg() }

    /// Returns the reference, result-adjustment, and input multiplexer register
    /// ([`ADMUX`](#method.admux_reg)).
    #[must_use]
    pub const fn mux_reg(self) -> AvrReg8 { self.admux_reg() }

    /// Returns the digital-input disable register
    /// ([`DIDR0`](#method.didr0_reg)).
    #[must_use]
    pub const fn digital_input_disable_reg(self) -> AvrReg8 { self.didr0_reg() }
}

/// # Datasheet registers API
#[rustfmt::skip]
impl AvrAdc {
    /// Returns the ADC data register low byte (`ADCL`).
    #[must_use]
    pub const fn adcl_reg(self) -> AvrReg8 { self.adcl }

    /// Returns the ADC data register high byte (`ADCH`).
    #[must_use]
    pub const fn adch_reg(self) -> AvrReg8 { self.adch }

    /// Returns the ADC control and status register A (`ADCSRA`).
    #[must_use]
    pub const fn adcsra_reg(self) -> AvrReg8 { self.adcsra }

    /// Returns the ADC control and status register B (`ADCSRB`).
    #[must_use]
    pub const fn adcsrb_reg(self) -> AvrReg8 { self.adcsrb }

    /// Returns the ADC multiplexer selection register (`ADMUX`).
    #[must_use]
    pub const fn admux_reg(self) -> AvrReg8 { self.admux }

    /// Returns the digital input disable register 0 (`DIDR0`).
    #[must_use]
    pub const fn didr0_reg(self) -> AvrReg8 { self.didr0 }
}

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
