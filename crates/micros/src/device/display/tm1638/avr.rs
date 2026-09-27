//
//! Defines [`Tm1638AvrBus`].
//

use crate::AvrPin;
#[cfg(all(feature = "unsafe_mmio", feature = "unsafe_hint"))]
use crate::{Infallible, Tm1638Bus, asm};

#[doc = crate::_tags!(hw io protocol)]
/// Bit-banged TM1638 serial interface over AVR GPIO.
///
/// The implementation uses conservative ~1 µs timing steps and is intended
/// for AVR clocks up to 20 MHz.
pub struct Tm1638AvrBus {
    stb: AvrPin,
    clk: AvrPin,
    dio: AvrPin,
}
impl Tm1638AvrBus {
    /// Releases the pins.
    #[must_use]
    pub fn into_pins(self) -> (AvrPin, AvrPin, AvrPin) {
        (self.stb, self.clk, self.dio)
    }
}
#[cfg(all(feature = "unsafe_mmio", feature = "unsafe_hint"))]
impl Tm1638AvrBus {
    /// Acquires three AVR pins for exclusive TM1638 use.
    ///
    /// # Safety
    /// The pins must belong to the active MCU, must be distinct, and their
    /// GPIO registers must not be concurrently modified while this value lives.
    #[must_use]
    pub unsafe fn new_unchecked(stb: AvrPin, clk: AvrPin, dio: AvrPin) -> Self {
        assert!(stb != clk && stb != dio && clk != dio);
        unsafe {
            stb.set_output_high();
            clk.set_output_low();
            dio.set_output_low();
        }
        Self { stb, clk, dio }
    }

    #[inline(always)]
    fn wait_1us() {
        // At 20 MHz, twenty NOPs are one microsecond.
        // Loop overhead only makes this more conservative.
        for _ in 0..20 {
            unsafe {
                asm!("nop", options(nomem, nostack, preserves_flags));
            }
        }
    }

    fn write_byte(&mut self, mut byte: u8) {
        for _ in 0..8 {
            unsafe {
                self.clk.set_low();

                if byte & 1 != 0 {
                    self.dio.set_high();
                } else {
                    self.dio.set_low();
                }
            }

            Self::wait_1us();

            unsafe { self.clk.set_high() };
            Self::wait_1us();

            byte >>= 1;
        }

        unsafe { self.clk.set_low() };
    }

    fn read_byte(&mut self) -> u8 {
        let mut byte = 0;

        for bit in 0..8 {
            unsafe { self.clk.set_high() };
            Self::wait_1us();

            if unsafe { self.dio.is_high() } {
                byte |= 1 << bit;
            }

            unsafe { self.clk.set_low() };
            Self::wait_1us();
        }

        byte
    }

    fn begin_write(&mut self) {
        unsafe {
            self.clk.set_low();
            self.dio.set_output_low();
            self.stb.set_low();
        }
        Self::wait_1us();
    }

    fn end(&mut self) {
        Self::wait_1us();
        unsafe { self.stb.set_high() };
        Self::wait_1us();
    }
}

#[cfg(all(feature = "unsafe_mmio", feature = "unsafe_hint"))]
impl Tm1638Bus for Tm1638AvrBus {
    type Error = Infallible;

    fn write_slices(&mut self, slices: &[&[u8]]) -> Result<(), Self::Error> {
        self.begin_write();

        for slice in slices {
            for &byte in *slice {
                self.write_byte(byte);
            }
        }

        self.end();
        Ok(())
    }

    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.begin_write();

        for &byte in write {
            self.write_byte(byte);
        }

        // The command's final high pulse has already lasted >= 1 µs.
        // Release the open-drain DIO line before receiving.
        unsafe { self.dio.set_input_floating() };
        Self::wait_1us();

        for byte in read {
            *byte = self.read_byte();
        }

        // End the transaction before taking ownership of DIO again,
        // avoiding contention with the TM1638's open-drain output.
        self.end();
        unsafe { self.dio.set_output_low() };

        Ok(())
    }
}
