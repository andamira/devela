//
//! Defines [`McuEsp32S3`].
//

#[cfg(feature = "unsafe_mmio")]
use crate::{Esp32S3Pin, I2cController};
use crate::{EspI2c, EspReg32, EspUsbSerialJtag};

#[doc = crate::_tags!(hw namespace)]
/// ESP32-S3 microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct McuEsp32S3),
    test_size_of(McuEsp32S3 = 0),
}]
/// The ESP32-S3 is a dual-core 32-bit Xtensa LX7 microcontroller running
/// at up to 240 MHz, with 384 KiB of ROM and 512 KiB of SRAM.
///
/// It integrates 2.4 GHz Wi-Fi and Bluetooth LE, alongside peripherals
/// including UART, I²C, SPI, LCD/camera, USB OTG, USB Serial/JTAG,
/// timers, watchdogs, and GDMA.
///
/// The silicon provides GPIO0 through GPIO21 and GPIO26 through GPIO48.
/// Availability and intended use remain subject to package, flash/PSRAM,
/// strapping, and board-level constraints.
///
/// devela currently provides the linker foundations, USB Serial/JTAG access,
/// and low-level digital GPIO and I²C0 access for the ESP32-S3.
/// Startup remains provided by `xtensa-lx-rt` for now.
///
/// See also:
///
/// - [ESP32-S3 datasheet]
/// - [ESP32-S3 Technical Reference Manual]
/// - [ESP32-S3 hardware reference]
///
/// [ESP32-S3 datasheet]: https://documentation.espressif.com/esp32_s3_datasheet_en.pdf
/// [ESP32-S3 Technical Reference Manual]: https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf
/// [ESP32-S3 hardware reference]: https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/hw-reference/index.html
#[derive(Debug)]
pub struct McuEsp32S3;

/// # GPIO
impl McuEsp32S3 {
    /// GPIO peripheral base address.
    pub const GPIO_BASE: u32 = 0x6000_4000;

    /// GPIO0..31 output-value register.
    pub const GPIO_OUT: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0004);

    /// GPIO0..31 output set register.
    ///
    /// Writing a `1` sets the corresponding bit in [`GPIO_OUT`](Self::GPIO_OUT).
    pub const GPIO_OUT_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0008);

    /// GPIO0..31 output clear register.
    ///
    /// Writing a `1` clears the corresponding bit in [`GPIO_OUT`](Self::GPIO_OUT).
    pub const GPIO_OUT_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x000c);

    /// Upper GPIO output-value register.
    ///
    /// Bit zero corresponds to GPIO32.
    pub const GPIO_OUT1: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0010);

    /// Upper GPIO output set register.
    ///
    /// Bit zero corresponds to GPIO32.
    pub const GPIO_OUT1_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0014);

    /// Upper GPIO output clear register.
    ///
    /// Bit zero corresponds to GPIO32.
    pub const GPIO_OUT1_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0018);

    /// GPIO0..31 output-enable register.
    pub const GPIO_ENABLE: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0020);

    /// GPIO0..31 output-enable set register.
    pub const GPIO_ENABLE_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0024);

    /// GPIO0..31 output-enable clear register.
    pub const GPIO_ENABLE_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0028);

    /// Upper GPIO output-enable register.
    ///
    /// Bit zero corresponds to GPIO32.
    pub const GPIO_ENABLE1: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x002c);

    /// Upper GPIO output-enable set register.
    ///
    /// Bit zero corresponds to GPIO32.
    pub const GPIO_ENABLE1_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0030);

    /// Upper GPIO output-enable clear register.
    ///
    /// Bit zero corresponds to GPIO32.
    pub const GPIO_ENABLE1_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0034);

    /// GPIO0..31 input-value register.
    pub const GPIO_IN: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x003c);

    /// Upper GPIO input-value register.
    ///
    /// Bit zero corresponds to GPIO32.
    pub const GPIO_IN1: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0040);
}

/// # Peripherals
impl McuEsp32S3 {
    /// I²C0 peripheral base address.
    pub const I2C0_BASE: u32 = 0x6001_3000;

    /// I²C0 controller.
    pub const I2C0: EspI2c = EspI2c::new(Self::I2C0_BASE);

    /// USB Serial/JTAG peripheral base address.
    pub const USB_SERIAL_JTAG_BASE: u32 = 0x6003_8000;

    /// Native USB Serial/JTAG controller.
    pub const USB_SERIAL_JTAG: EspUsbSerialJtag = EspUsbSerialJtag::new(Self::USB_SERIAL_JTAG_BASE);
}

/// # I²C
#[cfg(feature = "unsafe_mmio")]
impl McuEsp32S3 {
    /// Enables I²C0, routes it through `sda` and `scl`,
    /// and configures it as a master at `bus_hz` from the 40 MHz XTAL.
    ///
    /// The routed pins are configured for open-drain operation with input
    /// enabled and their weak internal pull-ups enabled.
    ///
    /// # Safety
    /// I²C0 and both GPIOs must not be concurrently configured or accessed.
    ///
    /// While the returned controller is alive, I²C0 and its routed pins
    /// must not be accessed through another raw hardware handle.
    pub unsafe fn prepare_i2c0(
        sda: Esp32S3Pin,
        scl: Esp32S3Pin,
        bus_hz: u32,
    ) -> I2cController<EspI2c> {
        const SYSTEM_PERIP_CLK_EN0: EspReg32 = EspReg32::new(0x600C_0018);
        const SYSTEM_PERIP_RST_EN0: EspReg32 = EspReg32::new(0x600C_0020);
        const I2C0_CLOCK: u32 = 1 << 7;
        const I2C0_RESET: u32 = 1 << 7;
        const SCL_SIGNAL: u8 = 89;
        const SDA_SIGNAL: u8 = 90;

        unsafe {
            let clock = SYSTEM_PERIP_CLK_EN0;
            clock.write(clock.read() | I2C0_CLOCK);

            let reset = SYSTEM_PERIP_RST_EN0;
            reset.write(reset.read() | I2C0_RESET);
            reset.write(reset.read() & !I2C0_RESET);

            Self::prepare_i2c0_pin(sda, SDA_SIGNAL);
            Self::prepare_i2c0_pin(scl, SCL_SIGNAL);

            Self::I2C0.configure_master_xtal(Self::XTAL_HZ, bus_hz);
            I2cController::new_unchecked(Self::I2C0)
        }
    }
    unsafe fn prepare_i2c0_pin(pin: Esp32S3Pin, signal: u8) {
        unsafe {
            pin.configure_open_drain_pullup();
            pin.matrix_route_output(signal);
            pin.matrix_route_input(signal);
        }
    }
}

/// # Clock
impl McuEsp32S3 {
    /// External main-crystal (`XTAL_CLK`) frequency, in hertz.
    pub const XTAL_HZ: u32 = 40_000_000;
}
