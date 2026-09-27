//
//! Defines [`AvrAdc`].
//

use crate::AvrReg8;

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
    pub(super) adcl: AvrReg8,
    pub(super) adch: AvrReg8,
    pub(super) adcsra: AvrReg8,
    pub(super) adcsrb: AvrReg8,
    pub(super) admux: AvrReg8,
    pub(super) didr0: AvrReg8,
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
}
