// devela/sys/hw/mcu/avr/timer/one.rs
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrTimer1 {
    tccr1a: AvrReg8,
    tccr1b: AvrReg8,
    tccr1c: AvrReg8,

    tcnt1l: AvrReg8,
    tcnt1h: AvrReg8,

    icr1l: AvrReg8,
    icr1h: AvrReg8,

    ocr1al: AvrReg8,
    ocr1ah: AvrReg8,
    ocr1bl: AvrReg8,
    ocr1bh: AvrReg8,

    timsk1: AvrReg8,
    tifr1: AvrReg8,
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

/// # Semantic registers API
#[rustfmt::skip]
impl AvrTimer1 {
    /// Returns the interrupt mask register
    /// ([`TIMSK1`][Self::timsk1_reg]).
    #[must_use]
    pub const fn interrupt_mask_reg(self) -> AvrReg8 {
        self.timsk1_reg()
    }

    /// Returns the interrupt flag register
    /// ([`TIFR1`][Self::tifr1_reg]).
    #[must_use]
    pub const fn interrupt_flag_reg(self) -> AvrReg8 {
        self.tifr1_reg()
    }
}

/// # Datasheet registers API
#[rustfmt::skip]
impl AvrTimer1 {
    /// Returns the Timer/Counter1 control register A (`TCCR1A`).
    #[must_use]
    pub const fn tccr1a_reg(self) -> AvrReg8 { self.tccr1a }

    /// Returns the Timer/Counter1 control register B (`TCCR1B`).
    #[must_use]
    pub const fn tccr1b_reg(self) -> AvrReg8 { self.tccr1b }

    /// Returns the Timer/Counter1 control register C (`TCCR1C`).
    #[must_use]
    pub const fn tccr1c_reg(self) -> AvrReg8 { self.tccr1c }

    /// Returns the low Timer/Counter1 counter register (`TCNT1L`).
    #[must_use]
    pub const fn tcnt1l_reg(self) -> AvrReg8 { self.tcnt1l }

    /// Returns the high Timer/Counter1 counter register (`TCNT1H`).
    #[must_use]
    pub const fn tcnt1h_reg(self) -> AvrReg8 { self.tcnt1h }

    /// Returns the low input capture register (`ICR1L`).
    #[must_use]
    pub const fn icr1l_reg(self) -> AvrReg8 { self.icr1l }

    /// Returns the high input capture register (`ICR1H`).
    #[must_use]
    pub const fn icr1h_reg(self) -> AvrReg8 { self.icr1h }

    /// Returns the low output compare register A (`OCR1AL`).
    #[must_use]
    pub const fn ocr1al_reg(self) -> AvrReg8 { self.ocr1al }

    /// Returns the high output compare register A (`OCR1AH`).
    #[must_use]
    pub const fn ocr1ah_reg(self) -> AvrReg8 { self.ocr1ah }

    /// Returns the low output compare register B (`OCR1BL`).
    #[must_use]
    pub const fn ocr1bl_reg(self) -> AvrReg8 { self.ocr1bl }

    /// Returns the high output compare register B (`OCR1BH`).
    #[must_use]
    pub const fn ocr1bh_reg(self) -> AvrReg8 { self.ocr1bh }

    /// Returns the interrupt mask register (`TIMSK1`).
    #[must_use]
    pub const fn timsk1_reg(self) -> AvrReg8 { self.timsk1 }

    /// Returns the interrupt flag register (`TIFR1`).
    #[must_use]
    pub const fn tifr1_reg(self) -> AvrReg8 { self.tifr1 }
}

/// # Operational API
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer1 {
    /* private helpers */

    // TCCR1B
    const ICNC1: u8 = 1 << 7; // input-capture noise canceler
    const ICES1: u8 = 1 << 6; // input-capture edge select
    const WGM12: u8 = 1 << 3; // waveform-generation mode bit 2

    // TIFR1
    const ICF1: u8 = 1 << 5; // input-capture flag
    const OCF1B: u8 = 1 << 2; // output-compare B match flag
    const OCF1A: u8 = 1 << 1; // output-compare A match flag
    const TOV1: u8 = 1 << 0; // timer overflow flag

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
    unsafe fn read_16_latched(low: AvrReg8, high: AvrReg8) -> u16 {
        let [lo, hi] = [unsafe { low.read() }, unsafe { high.read() }];
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
    unsafe fn write_16(high: AvrReg8, low: AvrReg8, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        unsafe {
            high.write(hi);
            low.write(lo);
        }
    }

    /* configuration */

    /// Configures normal mode and starts the timer.
    ///
    /// The counter runs from `0x0000` through `0xFFFF` and then wraps to zero.
    /// Timer1 interrupts are initially disabled and output-compare pins remain
    /// disconnected.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of `1`, `8`, `64`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently configured.
    /// No interrupt may access Timer1's 16-bit registers during the paired counter write.
    pub unsafe fn configure_normal(self, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer1 prescaler is not supported");
        };
        unsafe {
            // Stop before taking ownership of its configuration.
            self.tccr1b_reg().write(0);
            // WGM13:0 = 0b0000: normal mode; OC1A/B disconnected.
            self.tccr1a_reg().write(0);
            self.tccr1c_reg().write(0);
            self.interrupt_mask_reg().write(0);
            self.set_counter(0);
            // Clear all Timer1 event flags.
            self.interrupt_flag_reg().write(Self::ICF1 | Self::OCF1B | Self::OCF1A | Self::TOV1);
            self.tccr1b_reg().write(clock);
        }
    }
    /// Configures CTC mode with `OCR1A` as TOP and starts the timer.
    ///
    /// The counter advances from zero through `top`, inclusive,
    /// so one compare interval contains `top + 1` timer ticks:
    ///
    /// `period = prescaler × (top + 1) / source_clock`
    ///
    /// Timer1 interrupts are initially disabled
    /// and output-compare pins remain disconnected.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of `1`, `8`, `64`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently
    /// configured. No interrupt may access Timer1's 16-bit registers
    /// during the paired writes performed by this operation.
    pub unsafe fn configure_ctc(self, top: u16, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer1 prescaler is not supported");
        };
        unsafe {
            // Stop Timer1 before taking ownership of its configuration.
            self.tccr1b_reg().write(0);
            // Mode 4: CTC with OCR1A as TOP. OC1A/B remain disconnected.
            self.tccr1a_reg().write(0);
            self.tccr1c_reg().write(0);
            // Start with Timer1 interrupts disabled.
            self.interrupt_mask_reg().write(0);
            Self::write_16(self.tcnt1h_reg(), self.tcnt1l_reg(), 0);
            Self::write_16(self.ocr1ah_reg(), self.ocr1al_reg(), top);
            // OCF1A is write-one-to-clear.
            self.interrupt_flag_reg().write(Self::OCF1A);
            // WGM13:0 = 0b0100; CS12:0 selects the timer clock.
            self.tccr1b_reg().write(Self::WGM12 | clock);
        }
    }

    /* counter */

    /// Returns the current 16-bit counter value.
    ///
    /// The low-byte access snapshots the corresponding high byte
    /// so both bytes represent one logical counter value.
    ///
    /// # Safety
    /// The timer must belong to the active device. No concurrent Timer1 16-bit
    /// access may interfere with its shared temporary register.
    #[must_use]
    pub unsafe fn counter(self) -> u16 {
        unsafe { Self::read_16_latched(self.tcnt1l_reg(), self.tcnt1h_reg()) }
    }
    /// Sets the 16-bit counter value.
    ///
    /// # Safety
    /// The timer must belong to the active device. No concurrent Timer1 16-bit
    /// access may interfere with its shared temporary register.
    pub unsafe fn set_counter(self, value: u16) {
        unsafe { Self::write_16(self.tcnt1h_reg(), self.tcnt1l_reg(), value) }
    }

    /* output compare */

    /// Returns whether an output-compare A match is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_a_match_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::OCF1A != 0 }
    }
    /// Clears the output-compare A match flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_compare_a_match(self) {
        // OCF1A is write-one-to-clear: do not read-modify-write TIFR1.
        unsafe { self.interrupt_flag_reg().write(Self::OCF1A) };
    }

    /* input capture */

    /// Selects rising edges for input capture.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn set_capture_rising_edge(self) {
        let reg = self.tccr1b_reg();
        unsafe { reg.write(reg.read() | Self::ICES1) };
    }
    /// Selects falling edges for input capture.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn set_capture_falling_edge(self) {
        let reg = self.tccr1b_reg();
        unsafe { reg.write(reg.read() & !Self::ICES1) };
    }

    /// Enables the input-capture noise canceler.
    ///
    /// The input must remain stable for four consecutive system-clock samples,
    /// introducing four system-clock cycles of capture delay.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn enable_capture_noise_cancel(self) {
        let reg = self.tccr1b_reg();
        unsafe { reg.write(reg.read() | Self::ICNC1) };
    }
    /// Disables the input-capture noise canceler.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn disable_capture_noise_cancel(self) {
        let reg = self.tccr1b_reg();
        unsafe { reg.write(reg.read() & !Self::ICNC1) };
    }

    /// Returns whether an input-capture event is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn input_capture_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::ICF1 != 0 }
    }
    /// Returns the most recently captured 16-bit counter value.
    ///
    /// Reading `ICR1L` snapshots the corresponding high byte
    /// so both bytes represent the same capture event.
    ///
    /// # Safety
    /// The timer must belong to the active device. No concurrent Timer1 16-bit
    /// access may interfere with its shared temporary register.
    #[must_use]
    pub unsafe fn capture_value(self) -> u16 {
        unsafe { Self::read_16_latched(self.icr1l_reg(), self.icr1h_reg()) }
    }
    /// Clears the input-capture flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_input_capture(self) {
        // ICF1 is write-one-to-clear: do not read-modify-write TIFR1.
        unsafe { self.interrupt_flag_reg().write(Self::ICF1) };
    }
}
