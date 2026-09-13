// devela/sys/hw/mcu/avr/timer/two/define.rs
//
//! Defines [`AvrTimer2`].
//

use crate::AvrReg8;

#[doc = crate::_tags!(hw)]
/// A classic AVR 8-bit Timer/Counter2 peripheral.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/avr/timer", struct AvrTimer2),
    test_size_of(AvrTimer2 = 16|128; niche !Option),
}]
/// It provides an 8-bit counter, two output-compare channels,
/// interrupts, PWM facilities, and optional asynchronous clocking.
///
/// Unlike Timer0, Timer2 has its own prescaler choices
/// and can be clocked asynchronously through the Timer/Counter oscillator.
///
/// # Methods
///
/// The operational API is grouped by timer function:
///
/// - [Configuration](#configuration) — selects a counting mode and timer clock.
/// - [Counter](#counter) — accesses the running 8-bit counter.
/// - [Output compare](#output-compare) — accesses compare channels A and B
///   and their match events.
/// - [Overflow](#overflow) — reports and clears counter-overflow events.
/// - [Interrupt](#interrupt) — controls each Timer2 interrupt source.
///
/// [Semantic] and [datasheet] register accessors are also provided for lower-level use.
///
/// [Semantic]: #semantic-registers-api
/// [datasheet]: #datasheet-registers-api
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrTimer2 {
    pub(super) tccr2a: AvrReg8,
    pub(super) tccr2b: AvrReg8,
    pub(super) tcnt2: AvrReg8,
    pub(super) ocr2a: AvrReg8,
    pub(super) ocr2b: AvrReg8,
    pub(super) assr: AvrReg8,
    pub(super) timsk2: AvrReg8,
    pub(super) tifr2: AvrReg8,
}

#[rustfmt::skip]
impl AvrTimer2 {
    /// Creates an AVR Timer/Counter2 from its register data-space addresses.
    #[must_use]
    pub const fn new(
        tccr2a: u16,
        tccr2b: u16,
        tcnt2: u16,
        ocr2a: u16,
        ocr2b: u16,
        assr: u16,
        timsk2: u16,
        tifr2: u16,
    ) -> Self {
        Self {
            tccr2a: AvrReg8::new(tccr2a),
            tccr2b: AvrReg8::new(tccr2b),
            tcnt2: AvrReg8::new(tcnt2),
            ocr2a: AvrReg8::new(ocr2a),
            ocr2b: AvrReg8::new(ocr2b),
            assr: AvrReg8::new(assr),
            timsk2: AvrReg8::new(timsk2),
            tifr2: AvrReg8::new(tifr2),
        }
    }
    /// Returns all eight registers in constructor order.
    #[must_use]
    pub const fn into_parts(self) -> [AvrReg8; 8] {
        [
            self.tccr2a,
            self.tccr2b,
            self.tcnt2,
            self.ocr2a,
            self.ocr2b,
            self.assr,
            self.timsk2,
            self.tifr2,
        ]
    }
}

/* private helpers */

#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer2 {
    // TCCR2A
    pub(super) const WGM21: u8 = 1 << 1; // waveform-generation mode bit 1

    // TIFR2
    pub(super) const OCF2B: u8 = 1 << 2; // output-compare B match flag
    pub(super) const OCF2A: u8 = 1 << 1; // output-compare A match flag
    pub(super) const TOV2: u8 = 1 << 0; // timer overflow flag

    // All Timer2 event flags in `TIFR2`.
    pub(super) const EVENT_FLAGS: u8 = Self::OCF2B | Self::OCF2A | Self::TOV2;

    // TIMSK2
    pub(super) const OCIE2B: u8 = 1 << 2; // output-compare B interrupt enable
    pub(super) const OCIE2A: u8 = 1 << 1; // output-compare A interrupt enable
    pub(super) const TOIE2: u8 = 1 << 0; // overflow interrupt enable

    pub(super) fn prescaler_bits(prescaler: u16) -> Option<u8> {
        match prescaler {
            1 => Some(0b001),
            8 => Some(0b010),
            32 => Some(0b011), //
            64 => Some(0b100),
            128 => Some(0b101), //
            256 => Some(0b110),
            1024 => Some(0b111),
            _ => None,
        }
    }
}
