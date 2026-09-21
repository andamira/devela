//
//! Defines [`Esp32C3Pin`].
//

use crate::{EspReg32, McuEsp32C3};

#[doc = crate::_tags!(hw io)]
/// An ESP32-C3 GPIO pin identified by its GPIO number.
///
/// This provides low-level access to the simple
/// GPIO output latch and output-enable state.
///
/// It does not configure the IO MUX, GPIO matrix, pull resistors, or input path.
/// Those are independent parts of the ESP32-C3 pin configuration.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct Esp32C3Pin),
    test_size_of(Esp32C3Pin = 1|8; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Esp32C3Pin(u8);

#[rustfmt::skip]
impl Esp32C3Pin {
    /// Creates a pin from an ESP32-C3 GPIO number.
    ///
    /// # Panics
    /// Panics if `gpio` is greater than 21.
    #[must_use]
    pub const fn new(gpio: u8) -> Self {
        assert!(gpio < 22, "ESP32-C3 GPIO must be in 0..22");
        Self(gpio)
    }

    /// Returns its GPIO number.
    #[must_use]
    pub const fn gpio(self) -> u8 { self.0 }

    /// Returns its one-bit GPIO register mask.
    #[must_use]
    pub const fn mask(self) -> u32 { 1u32 << self.0 }
}

#[cfg(feature = "unsafe_mmio")]
impl Esp32C3Pin {
    /// Returns whether its output driver is enabled.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device.
    pub unsafe fn is_output_enabled(self) -> bool {
        unsafe { McuEsp32C3::GPIO_ENABLE.read() & self.mask() != 0 }
    }
    /// Enables its output driver, preserving the output latch.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device. The pin must not be
    /// concurrently configured, and enabling its driver must be valid for
    /// the current pin routing and connected circuit.
    pub unsafe fn enable_output(self) {
        unsafe { McuEsp32C3::GPIO_ENABLE_W1TS.write(self.mask()) };
    }
    /// Disables its output driver, leaving the pin undriven.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device,
    /// and the pin must not be concurrently configured.
    pub unsafe fn disable_output(self) {
        unsafe { McuEsp32C3::GPIO_ENABLE_W1TC.write(self.mask()) };
    }

    /// Returns whether its output latch is low.
    ///
    /// This reads the configured output level, not the physical pad input.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device.
    pub unsafe fn is_output_low(self) -> bool {
        unsafe { !self.is_output_high() }
    }
    /// Enables its output driver after setting its latch low.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device. The pin must not be
    /// concurrently configured, and driving it low must be valid
    /// for the current pin routing and connected circuit.
    pub unsafe fn set_output_low(self) {
        unsafe {
            self.set_low();
            self.enable_output();
        }
    }

    /// Returns whether its output latch is high.
    ///
    /// This reads the configured output level, not the physical pad input.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device.
    pub unsafe fn is_output_high(self) -> bool {
        unsafe { McuEsp32C3::GPIO_OUT.read() & self.mask() != 0 }
    }
    /// Enables its output driver after setting its latch high.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device. The pin must not be
    /// concurrently configured, and driving it high must be valid
    /// for the current pin routing and connected circuit.
    pub unsafe fn set_output_high(self) {
        unsafe {
            self.set_high();
            self.enable_output();
        }
    }

    /// Sets its output latch high.
    ///
    /// When its output driver is enabled, this drives the pin high.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device. If its output driver is enabled,
    /// driving the pin high must be valid for the current pin routing and connected circuit.
    pub unsafe fn set_high(self) {
        unsafe { McuEsp32C3::GPIO_OUT_W1TS.write(self.mask()) };
    }
    /// Sets its output latch low.
    ///
    /// When its output driver is enabled, this drives the pin low.
    ///
    /// # Safety
    /// This must execute on the active ESP32-C3 device. If its output driver is enabled,
    /// driving the pin low must be valid for the current pin routing and connected circuit.
    pub unsafe fn set_low(self) {
        unsafe { McuEsp32C3::GPIO_OUT_W1TC.write(self.mask()) };
    }
}

/* private pin-routing registers */

#[allow(dead_code, reason = "safe helpers used by unsafe-gated code")]
impl Esp32C3Pin {
    const IO_MUX_GPIO0: u32 = 0x6000_9004;
    const GPIO_PIN0: u32 = McuEsp32C3::GPIO_BASE + 0x74;
    const GPIO_FUNC0_IN: u32 = McuEsp32C3::GPIO_BASE + 0x154;
    const GPIO_FUNC0_OUT: u32 = McuEsp32C3::GPIO_BASE + 0x554;

    #[must_use]
    const fn io_mux_reg(self) -> EspReg32 {
        EspReg32::new(Self::IO_MUX_GPIO0 + self.0 as u32 * 4)
    }
    #[must_use]
    const fn pin_config_reg(self) -> EspReg32 {
        EspReg32::new(Self::GPIO_PIN0 + self.0 as u32 * 4)
    }
    #[must_use]
    const fn matrix_input_reg(signal: u8) -> EspReg32 {
        EspReg32::new(Self::GPIO_FUNC0_IN + signal as u32 * 4)
    }
    #[must_use]
    const fn matrix_output_reg(self) -> EspReg32 {
        EspReg32::new(Self::GPIO_FUNC0_OUT + self.0 as u32 * 4)
    }
}

/* private helpers */

#[cfg(feature = "unsafe_mmio")]
impl Esp32C3Pin {
    /// Selects the GPIO function and configures this pad for an
    /// input-enabled, open-drain signal with a weak internal pull-up.
    ///
    /// This is intended for peripheral signals such as I²C that use
    /// the GPIO matrix.
    ///
    /// # Safety
    /// This pin must not be concurrently configured or accessed.
    pub(crate) unsafe fn configure_open_drain_pullup(self) {
        const PAD_DRIVER: u32 = 1 << 2;

        const FUN_PULL_DOWN: u32 = 1 << 7;
        const FUN_PULL_UP: u32 = 1 << 8;
        const FUN_INPUT_ENABLE: u32 = 1 << 9;
        const FUN_SELECT_MASK: u32 = 0b111 << 12;
        const FUN_GPIO: u32 = 1 << 12;

        unsafe {
            // Open-drain buses idle high.
            self.set_high();

            let pin = self.pin_config_reg();
            pin.write(pin.read() | PAD_DRIVER);

            let mux = self.io_mux_reg();
            mux.write(
                (mux.read() & !(FUN_SELECT_MASK | FUN_PULL_DOWN))
                    | FUN_GPIO
                    | FUN_INPUT_ENABLE
                    | FUN_PULL_UP,
            );
        }
    }

    /// Routes a peripheral output signal through the GPIO matrix to this pin.
    ///
    /// Peripheral control of output-enable is preserved.
    ///
    /// # Safety
    /// This pin and matrix output route must not be concurrently configured.
    pub(crate) unsafe fn matrix_route_output(self, signal: u8) {
        unsafe {
            let output = self.matrix_output_reg();

            // Bits 0..7: output signal.
            //
            // Clearing bits 8..10 also selects:
            // - non-inverted output,
            // - output-enable controlled by the peripheral,
            // - non-inverted output-enable.
            output.write((output.read() & !0x7ff) | signal as u32);
        }
    }

    /// Routes this pin through the GPIO matrix to a peripheral input signal.
    ///
    /// # Safety
    /// This pin and matrix input route must not be concurrently configured.
    pub(crate) unsafe fn matrix_route_input(self, signal: u8) {
        const INPUT_MATRIX_ENABLE: u32 = 1 << 6;
        unsafe {
            let input = Self::matrix_input_reg(signal);
            input.write((input.read() & !0x7f) | INPUT_MATRIX_ENABLE | self.gpio() as u32);
        }
    }
}
