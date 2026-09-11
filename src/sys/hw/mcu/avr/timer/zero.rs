// devela/sys/hw/mcu/avr/timer/zero.rs
//
//! Defines [`AvrTimer0`].
//

use crate::AvrReg8;

#[doc = crate::_tags!(hw)]
/// A classic AVR 8-bit Timer/Counter0 peripheral.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/avr", struct AvrTimer0),
    test_size_of(AvrTimer0 = 14|112; niche !Option),
}]
/// It is described by its control, counter, compare, and interrupt registers.
///
/// This models the numbered Timer/Counter0 design used by devices such as the ATmega328P.
///
/// It is not a universal AVR timer layout; other AVR families
/// use different timer peripherals and register interfaces.
///
/// Values can safely be copied and inspected. Operations that access the
/// described registers are unsafe because the addresses must correspond to
/// the active device and access must respect the peripheral's hardware state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrTimer0 {
    tccr0a: AvrReg8,
    tccr0b: AvrReg8,
    tcnt0: AvrReg8,
    ocr0a: AvrReg8,
    ocr0b: AvrReg8,
    timsk0: AvrReg8,
    tifr0: AvrReg8,
}

#[rustfmt::skip]
impl AvrTimer0 {
    /// Creates an AVR Timer/Counter0 from its register data-space addresses.
    #[must_use]
    pub const fn new(
        tccr0a: u16,
        tccr0b: u16,
        tcnt0: u16,
        ocr0a: u16,
        ocr0b: u16,
        timsk0: u16,
        tifr0: u16,
    ) -> Self {
        Self {
            tccr0a: AvrReg8::new(tccr0a),
            tccr0b: AvrReg8::new(tccr0b),
            tcnt0: AvrReg8::new(tcnt0),
            ocr0a: AvrReg8::new(ocr0a),
            ocr0b: AvrReg8::new(ocr0b),
            timsk0: AvrReg8::new(timsk0),
            tifr0: AvrReg8::new(tifr0),
        }
    }

    /// Returns all seven registers in constructor order.
    #[must_use]
    pub const fn into_parts(self) -> [AvrReg8; 7] {
        [
            self.tccr0a,
            self.tccr0b,
            self.tcnt0,
            self.ocr0a,
            self.ocr0b,
            self.timsk0,
            self.tifr0,
        ]
    }
}

/// # Semantic registers API
#[rustfmt::skip]
impl AvrTimer0 {
    /// Returns the counter value register
    /// ([`TCNT0`](#method.tcnt0_reg)).
    #[must_use]
    pub const fn counter_reg(self) -> AvrReg8 { self.tcnt0_reg() }

    /// Returns the output compare register A
    /// ([`OCR0A`](#method.ocr0a_reg)).
    #[must_use]
    pub const fn compare_a_reg(self) -> AvrReg8 { self.ocr0a_reg() }

    /// Returns the output compare register B
    /// ([`OCR0B`](#method.ocr0b_reg)).
    #[must_use]
    pub const fn compare_b_reg(self) -> AvrReg8 { self.ocr0b_reg() }

    /// Returns the interrupt mask register
    /// ([`TIMSK0`](#method.timsk0_reg)).
    #[must_use]
    pub const fn interrupt_mask_reg(self) -> AvrReg8 { self.timsk0_reg() }

    /// Returns the interrupt flag register
    /// ([`TIFR0`](#method.tifr0_reg)).
    #[must_use]
    pub const fn interrupt_flag_reg(self) -> AvrReg8 { self.tifr0_reg() }
}
/// # Datasheet registers API
#[rustfmt::skip]
impl AvrTimer0 {
    /// Returns the Timer/Counter control register A (`TCCR0A`).
    #[must_use]
    pub const fn tccr0a_reg(self) -> AvrReg8 { self.tccr0a }

    /// Returns the Timer/Counter control register B (`TCCR0B`).
    #[must_use]
    pub const fn tccr0b_reg(self) -> AvrReg8 { self.tccr0b }

    /// Returns the Timer/Counter register (`TCNT0`).
    #[must_use]
    pub const fn tcnt0_reg(self) -> AvrReg8 { self.tcnt0 }

    /// Returns the output compare register A (`OCR0A`).
    #[must_use]
    pub const fn ocr0a_reg(self) -> AvrReg8 { self.ocr0a }

    /// Returns the output compare register B (`OCR0B`).
    #[must_use]
    pub const fn ocr0b_reg(self) -> AvrReg8 { self.ocr0b }

    /// Returns the interrupt mask register (`TIMSK0`).
    #[must_use]
    pub const fn timsk0_reg(self) -> AvrReg8 { self.timsk0 }

    /// Returns the interrupt flag register (`TIFR0`).
    #[must_use]
    pub const fn tifr0_reg(self) -> AvrReg8 { self.tifr0 }
}

/// # Operational API
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer0 {
    const WGM01: u8 = 1 << 1; // TCCR0A
    const OCF0A: u8 = 1 << 1; // TIFR0

    fn prescaler_bits(prescaler: u16) -> Option<u8> {
        match prescaler {
            1 => Some(0b001),
            8 => Some(0b010),
            64 => Some(0b011),
            256 => Some(0b100),
            1024 => Some(0b101),
            _ => None,
        }
    }

    /// Configures CTC mode with `OCR0A` as TOP and starts the timer.
    ///
    /// The counter advances from zero through `top`, inclusive, so one compare
    /// interval contains `top + 1` timer ticks. Its duration is therefore:
    ///
    /// `prescaler × (top + 1) / source_clock`
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of `1`, `8`, `64`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently configured.
    pub unsafe fn configure_ctc(self, top: u8, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer0 prescaler is not supported");
        };
        unsafe {
            self.tccr0b_reg().write(0); // Stop before taking ownership of its configuration
            self.tccr0a_reg().write(Self::WGM01); // WGM02:0 = 0b010: CTC, with OC0A/B disconnected
            self.counter_reg().write(0);
            self.compare_a_reg().write(top);
            self.interrupt_mask_reg().write(0); // Polling configuration: no Timer0 interrupts
            self.interrupt_flag_reg().write(Self::OCF0A); // TIFR0 flags are write-one-to-clear
            // WGM02 remains zero; CS02:0 selects the prescaled clock.
            self.tccr0b_reg().write(clock);
        }
    }

    /// Returns whether an output-compare A match is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_a_match_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::OCF0A != 0 }
    }
    /// Clears the output-compare A match flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_compare_a_match(self) {
        // OCF0A is write-one-to-clear: do not read-modify-write TIFR0.
        unsafe { self.interrupt_flag_reg().write(Self::OCF0A) };
    }
}
