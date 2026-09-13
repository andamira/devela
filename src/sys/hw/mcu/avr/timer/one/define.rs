// devela/sys/hw/mcu/avr/timer/one/define.rs
//
//! Defines [`AvrTimer1`].
//

use crate::AvrReg8;

#[doc = crate::_tags!(hw)]
/// A classic AVR 16-bit Timer/Counter1 peripheral.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/avr/timer", struct AvrTimer1),
    test_size_of(AvrTimer1 = 26|208; niche !Option),
}]
/// It is described by its control, counter, capture, compare, and interrupt registers.
///
/// Timer1's 16-bit counter, capture, and compare registers are exposed to the 8-bit
/// AVR CPU as pairs of byte registers. Timer1 coordinates paired accesses through a
/// shared temporary high-byte register, so accesses must follow the peripheral's
/// ordering rules and may require protection from concurrent Timer1 access.
///
/// Values can safely be copied and inspected. Operations that access the
/// described registers are unsafe because the addresses must correspond to
/// the active device and access must respect the peripheral's hardware state.
///
/// # Methods
///
/// The operational API is grouped by timer function:
///
/// - [Configuration](#configuration) — [`configure_normal`](#method.configure_normal) and
///   [`configure_ctc`](#method.configure_ctc) select a counting mode and clock.
/// - [Counter](#counter) — [`counter`](#method.counter) and [`set_counter`](#method.set_counter)
///   access the running 16-bit count.
/// - [Overflow](#overflow) — reports and clears counter-overflow events.
/// - [Output compare](#output-compare) — compare values and match events
///   are available independently for channels A and B.
/// - [Input capture](#input-capture) — selects the capture edge and filtering,
///   reports capture events, and reads the captured counter value.
/// - [PWM](#pwm) — configures fast PWM and connects the Timer1
///   compare channels to their hardware output pins.
/// - [Interrupt](#interrupt) — enables or disables each Timer1 interrupt source.
///   CPU-wide interrupt control remains separate.
///
/// [Semantic] and [datasheet] register accessors are also provided for lower-level use.
///
/// [Semantic]: #semantic-registers-api
/// [datasheet]: #datasheet-registers-api
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrTimer1 {
    pub(super) tccr1a: AvrReg8,
    pub(super) tccr1b: AvrReg8,
    pub(super) tccr1c: AvrReg8,

    pub(super) tcnt1l: AvrReg8,
    pub(super) tcnt1h: AvrReg8,

    pub(super) icr1l: AvrReg8,
    pub(super) icr1h: AvrReg8,

    pub(super) ocr1al: AvrReg8,
    pub(super) ocr1ah: AvrReg8,
    pub(super) ocr1bl: AvrReg8,
    pub(super) ocr1bh: AvrReg8,

    pub(super) timsk1: AvrReg8,
    pub(super) tifr1: AvrReg8,
}
#[rustfmt::skip]
impl AvrTimer1 {
    /// Creates an AVR Timer/Counter1 from its register data-space addresses.
    #[must_use]
    pub const fn new(
        tccr1a: u16,
        tccr1b: u16,
        tccr1c: u16,
        tcnt1l: u16,
        tcnt1h: u16,
        icr1l: u16,
        icr1h: u16,
        ocr1al: u16,
        ocr1ah: u16,
        ocr1bl: u16,
        ocr1bh: u16,
        timsk1: u16,
        tifr1: u16,
    ) -> Self {
        Self {
            tccr1a: AvrReg8::new(tccr1a),
            tccr1b: AvrReg8::new(tccr1b),
            tccr1c: AvrReg8::new(tccr1c),
            tcnt1l: AvrReg8::new(tcnt1l),
            tcnt1h: AvrReg8::new(tcnt1h),
            icr1l: AvrReg8::new(icr1l),
            icr1h: AvrReg8::new(icr1h),
            ocr1al: AvrReg8::new(ocr1al),
            ocr1ah: AvrReg8::new(ocr1ah),
            ocr1bl: AvrReg8::new(ocr1bl),
            ocr1bh: AvrReg8::new(ocr1bh),
            timsk1: AvrReg8::new(timsk1),
            tifr1: AvrReg8::new(tifr1),
        }
    }
    /// Returns all thirteen registers in constructor order.
    #[must_use]
    pub const fn into_parts(self) -> [AvrReg8; 13] {
        [
            self.tccr1a, self.tccr1b, self.tccr1c,
            self.tcnt1l, self.tcnt1h,
            self.icr1l, self.icr1h,
            self.ocr1al, self.ocr1ah,
            self.ocr1bl, self.ocr1bh,
            self.timsk1, self.tifr1,
        ]
    }
}

/* private helpers */

#[allow(dead_code)]
impl AvrTimer1 {
    // TCCR1A
    pub(super) const COM1A1: u8 = 1 << 7; // compare-output A mode bit 1
    pub(super) const COM1A0: u8 = 1 << 6; // compare-output A mode bit 0
    pub(super) const COM1B1: u8 = 1 << 5; // compare-output B mode bit 1
    pub(super) const COM1B0: u8 = 1 << 4; // compare-output B mode bit 0
    pub(super) const WGM11: u8 = 1 << 1; // waveform-generation mode bit 1

    // TCCR1B
    pub(super) const ICNC1: u8 = 1 << 7; // input-capture noise canceler
    pub(super) const ICES1: u8 = 1 << 6; // input-capture edge select
    pub(super) const WGM13: u8 = 1 << 4; // waveform-generation mode bit 3
    pub(super) const WGM12: u8 = 1 << 3; // waveform-generation mode bit 2

    // TIFR1
    pub(super) const ICF1: u8 = 1 << 5; // input-capture flag
    pub(super) const OCF1B: u8 = 1 << 2; // output-compare B match flag
    pub(super) const OCF1A: u8 = 1 << 1; // output-compare A match flag
    pub(super) const TOV1: u8 = 1 << 0; // timer overflow flag

    // All Timer1 event flags in `TIFR1`.
    pub(super) const EVENT_FLAGS: u8 = Self::ICF1 | Self::OCF1B | Self::OCF1A | Self::TOV1;

    // TIMSK1
    pub(super) const ICIE1: u8 = 1 << 5; // input-capture interrupt enable
    pub(super) const OCIE1B: u8 = 1 << 2; // output-compare B interrupt enable
    pub(super) const OCIE1A: u8 = 1 << 1; // output-compare A interrupt enable
    pub(super) const TOIE1: u8 = 1 << 0; // overflow interrupt enable

    pub(super) fn prescaler_bits(prescaler: u16) -> Option<u8> {
        match prescaler {
            1 => Some(0b001),
            8 => Some(0b010),
            64 => Some(0b011),
            256 => Some(0b100),
            1024 => Some(0b101),
            _ => None,
        }
    }
}

#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer1 {
    /// Reads a latched Timer1 16-bit register pair.
    ///
    /// Reading the low byte first copies the logical register's high byte into
    /// Timer1's shared temporary register; the following high-byte read retrieves
    /// that saved byte.
    ///
    /// # Safety
    /// `low` and `high` must be the corresponding halves of a readable Timer1
    /// 16-bit register that uses the temporary high-byte register. No concurrent
    /// Timer1 16-bit access may interfere with the shared temporary register.
    #[must_use]
    pub(super) unsafe fn read_16_latched(low: AvrReg8, high: AvrReg8) -> u16 {
        let [lo, hi] = [unsafe { low.read() }, unsafe { high.read() }];
        u16::from_le_bytes([lo, hi])
    }
    /// Reads a Timer1 16-bit register pair
    /// without using the shared temporary high-byte register.
    ///
    /// This is used for `OCR1A` and `OCR1B`, whose reads do not use TEMP.
    ///
    /// # Safety
    /// `low` and `high` must be the corresponding halves of a readable
    /// Timer1 16-bit register whose reads do not use TEMP.
    #[must_use]
    pub(super) unsafe fn read_16_unlatched(low: AvrReg8, high: AvrReg8) -> u16 {
        let lo = unsafe { low.read() };
        let hi = unsafe { high.read() };
        u16::from_le_bytes([lo, hi])
    }
    /// Writes a Timer1 16-bit register pair.
    ///
    /// Timer1 requires the high byte first; writing the low byte
    /// commits both bytes to the logical 16-bit register.
    ///
    /// # Safety
    /// `high` and `low` must be the corresponding halves of a writable
    /// Timer1 16-bit register. No concurrent Timer1 16-bit access may
    /// interfere with the peripheral's shared temporary register.
    pub(super) unsafe fn write_16(high: AvrReg8, low: AvrReg8, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        unsafe {
            high.write(hi);
            low.write(lo);
        }
    }
}
