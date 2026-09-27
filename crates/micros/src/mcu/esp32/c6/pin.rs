//
//! Defines [`Esp32C6Pin`].
//

use crate::{EspReg32, McuEsp32C6, is};

#[doc = crate::_tags!(hw io)]
/// An ESP32-C6 GPIO pin identified by its GPIO number.
///
/// This provides low-level access to the GPIO output latch, output-enable
/// state, input level, and the basic pad routing needed for simple GPIO output.
///
/// Peripheral routing, pull resistors, drive strength, and input configuration
/// remain independent parts of ESP32-C6 pin configuration.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct Esp32C6Pin),
    test_size_of(Esp32C6Pin = 1|8; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Esp32C6Pin(u8);

#[rustfmt::skip]
impl Esp32C6Pin {
    /// Creates a pin from an ESP32-C6 GPIO number.
    ///
    /// # Panics
    /// Panics if `gpio` is greater than 30.
    #[must_use]
    pub const fn new(gpio: u8) -> Self {
        assert!(gpio < 31, "ESP32-C6 GPIO must be in 0..31");
        Self(gpio)
    }

    /// Returns its GPIO number.
    #[must_use]
    pub const fn gpio(self) -> u8 { self.0 }

    /// Returns its one-bit GPIO register mask.
    #[must_use]
    pub const fn mask(self) -> u32 { 1u32 << self.0 }
}

/* private pin-routing registers */

#[allow(dead_code, reason = "safe helpers used by unsafe-gated code")]
impl Esp32C6Pin {
    const IO_MUX_GPIO0: u32 = 0x6009_0004;
    const GPIO_PIN0: u32 = McuEsp32C6::GPIO_BASE + 0x74;
    const GPIO_FUNC0_OUT: u32 = McuEsp32C6::GPIO_BASE + 0x554;

    #[must_use]
    const fn io_mux_reg(self) -> EspReg32 {
        EspReg32::new(Self::IO_MUX_GPIO0 + self.0 as u32 * 4)
    }
    #[must_use]
    const fn pin_config_reg(self) -> EspReg32 {
        EspReg32::new(Self::GPIO_PIN0 + self.0 as u32 * 4)
    }
    #[must_use]
    const fn matrix_output_reg(self) -> EspReg32 {
        EspReg32::new(Self::GPIO_FUNC0_OUT + self.0 as u32 * 4)
    }
}

#[cfg(feature = "unsafe_mmio")]
impl Esp32C6Pin {
    /// Configures this pad as a simple push-pull GPIO output.
    ///
    /// Selects the GPIO IO-MUX function and routes this pin's `GPIO_OUT` latch
    /// through the GPIO matrix, with output enable controlled by
    /// `GPIO_ENABLE`. This does not change the output latch or enable the
    /// output driver.
    ///
    /// This step matters on ESP32-C6 pins whose reset IO-MUX function is a
    /// peripheral such as SDIO rather than GPIO.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device. The pin and its GPIO
    /// matrix output route must not be concurrently configured, and selecting
    /// simple GPIO output must be valid for the connected circuit.
    pub unsafe fn configure_output(self) {
        const PAD_DRIVER: u32 = 1 << 2;

        const FUN_SELECT_MASK: u32 = 0b111 << 12;
        const FUN_GPIO: u32 = 1 << 12;

        const OUT_SELECT_MASK: u32 = 0xff;
        const OUT_INVERT: u32 = 1 << 8;
        const OEN_SELECT: u32 = 1 << 9;
        const OEN_INVERT: u32 = 1 << 10;
        const GPIO_OUT_SIGNAL: u32 = 128;

        unsafe {
            // Simple digital output is push-pull rather than open-drain.
            let pin = self.pin_config_reg();
            pin.write(pin.read() & !PAD_DRIVER);

            // Select GPIO_OUT[n] and GPIO_ENABLE[n] as the output sources.
            let output = self.matrix_output_reg();
            output.write(
                (output.read() & !(OUT_SELECT_MASK | OUT_INVERT | OEN_SELECT | OEN_INVERT))
                    | GPIO_OUT_SIGNAL
                    | OEN_SELECT,
            );

            // ESP32-C6 uses IO-MUX function 1 for ordinary GPIO.
            let mux = self.io_mux_reg();
            mux.write((mux.read() & !FUN_SELECT_MASK) | FUN_GPIO);
        }
    }

    /// Configures this pad as a push-pull GPIO-matrix peripheral output.
    ///
    /// The peripheral signal drives the output value while `GPIO_ENABLE` keeps
    /// the pad output driver enabled continuously.
    ///
    /// # Safety
    /// This pin and matrix output route must not be concurrently configured,
    /// and `signal` must be a valid output signal for the active ESP32-C6.
    pub(crate) unsafe fn configure_peripheral_output(self, signal: u8) {
        const PAD_DRIVER: u32 = 1 << 2;
        const FUN_SELECT_MASK: u32 = 0b111 << 12;
        const FUN_GPIO: u32 = 1 << 12;
        const OUT_SELECT_MASK: u32 = 0xff;
        const OUT_INVERT: u32 = 1 << 8;
        const OEN_SELECT: u32 = 1 << 9;
        const OEN_INVERT: u32 = 1 << 10;

        unsafe {
            let pin = self.pin_config_reg();
            pin.write(pin.read() & !PAD_DRIVER);

            let output = self.matrix_output_reg();
            output.write(
                (output.read() & !(OUT_SELECT_MASK | OUT_INVERT | OEN_SELECT | OEN_INVERT))
                    | signal as u32
                    | OEN_SELECT,
            );

            let mux = self.io_mux_reg();
            mux.write((mux.read() & !FUN_SELECT_MASK) | FUN_GPIO);
            self.enable_output();
        }
    }

    /// Returns whether its output driver is enabled.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device.
    pub unsafe fn is_output_enabled(self) -> bool {
        unsafe { McuEsp32C6::GPIO_ENABLE.read() & self.mask() != 0 }
    }

    /// Enables its output driver, preserving the output latch.
    ///
    /// This does not alter IO-MUX or GPIO-matrix routing. For a complete
    /// simple GPIO setup use [`set_output_low`](#method.set_output_low) or
    /// [`set_output_high`](#method.set_output_high).
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device. The pin must not be
    /// concurrently configured, and enabling its driver must be valid for
    /// the current pin routing and connected circuit.
    pub unsafe fn enable_output(self) {
        unsafe { McuEsp32C6::GPIO_ENABLE_W1TS.write(self.mask()) };
    }

    /// Disables its output driver, leaving the pin undriven.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device,
    /// and the pin must not be concurrently configured.
    pub unsafe fn disable_output(self) {
        unsafe { McuEsp32C6::GPIO_ENABLE_W1TC.write(self.mask()) };
    }

    /// Returns whether its output latch is low.
    ///
    /// This reads the configured output level, not the physical pad input.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device.
    pub unsafe fn is_output_low(self) -> bool {
        unsafe { !self.is_output_high() }
    }

    /// Configures this pad as a simple push-pull GPIO output initially driven low.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device. The pin and its GPIO
    /// matrix output route must not be concurrently configured, and driving
    /// it low must be valid for the connected circuit.
    pub unsafe fn set_output_low(self) {
        unsafe {
            self.set_low();
            self.configure_output();
            self.enable_output();
        }
    }

    /// Returns whether its output latch is high.
    ///
    /// This reads the configured output level, not the physical pad input.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device.
    pub unsafe fn is_output_high(self) -> bool {
        unsafe { McuEsp32C6::GPIO_OUT.read() & self.mask() != 0 }
    }

    /// Configures this pad as a simple push-pull GPIO output initially driven high.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device. The pin and its GPIO
    /// matrix output route must not be concurrently configured, and driving
    /// it high must be valid for the connected circuit.
    pub unsafe fn set_output_high(self) {
        unsafe {
            self.set_high();
            self.configure_output();
            self.enable_output();
        }
    }

    /// Sets its output latch high.
    ///
    /// When its output driver is enabled, this drives the pin high.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device. If its output driver is enabled,
    /// driving the pin high must be valid for the current pin routing and connected circuit.
    pub unsafe fn set_high(self) {
        unsafe { McuEsp32C6::GPIO_OUT_W1TS.write(self.mask()) };
    }

    /// Sets its output latch low.
    ///
    /// When its output driver is enabled, this drives the pin low.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device. If its output driver is enabled,
    /// driving the pin low must be valid for the current pin routing and connected circuit.
    pub unsafe fn set_low(self) {
        unsafe { McuEsp32C6::GPIO_OUT_W1TC.write(self.mask()) };
    }

    /// Toggles this pin's output latch.
    ///
    /// This is a compound read-then-write operation rather than an atomic
    /// write-one-to-toggle register operation.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device. The pin's output latch
    /// must not be concurrently modified. If its output driver is enabled,
    /// driving the opposite level must be valid for the connected circuit.
    pub unsafe fn toggle(self) {
        unsafe {
            is! { self.is_output_high(), self.set_low(), self.set_high() }
        }
    }

    /// Reads the physical GPIO input level.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C6 device. The pad input path must
    /// be configured appropriately for meaningful results.
    pub unsafe fn is_high(self) -> bool {
        unsafe { McuEsp32C6::GPIO_IN.read() & self.mask() != 0 }
    }

    /// Reads whether the physical GPIO input level is low.
    ///
    /// # Safety
    /// This has the same requirements as [`is_high`](Self::is_high).
    pub unsafe fn is_low(self) -> bool {
        unsafe { !self.is_high() }
    }
}
