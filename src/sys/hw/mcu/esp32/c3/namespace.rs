// devela/src/sys/hw/mcu/esp32/c3/namespace.rs
//
//! Defines [`McuEsp32C3`].
//

use crate::{__cfg_item_unsafe_show, macro_apply};
use crate::{Esp32C3Pin, Esp32C3Uart, EspI2c, EspReg32, EspUsbSerialJtag};

#[doc = crate::_tags!(hw namespace)]
/// ESP32-C3 microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/esp32", struct McuEsp32C3),
    test_size_of(McuEsp32C3 = 0),
}]
/// The ESP32-C3 is a single-core 32-bit RISC-V microcontroller with a
/// maximum CPU frequency of 160 MHz, 384 KiB of ROM, and 400 KiB of SRAM.
///
/// It integrates 2.4 GHz Wi-Fi and Bluetooth LE, alongside peripherals
/// including two UARTs, I²C, SPI, timers, watchdogs, and USB Serial/JTAG.
/// The main ESP32-C3 variants provide GPIO0 through GPIO21.
///
/// devela currently provides direct-boot startup support together with
/// low-level access to GPIO, I²C0, UART0, and USB Serial/JTAG.
///
/// # GPIO
///
/// Simple digital output has two independent pieces of state:
///
/// - [`GPIO_OUT`] holds the output levels.
/// - [`GPIO_ENABLE`] selects which pins actively drive those levels.
///
/// Their `W1TS` and `W1TC` variants set or clear selected bits without
/// requiring a read-modify-write.
///
/// IO MUX and the GPIO matrix provide the wider pin-routing system.
/// Peripheral use therefore may require more than the basic GPIO registers.
///
/// See also:
///
/// - [ESP32-C3 datasheet]
/// - [ESP32-C3 Technical Reference Manual]
/// - [ESP32-C3 hardware reference]
///
/// [`GPIO_OUT`]: #associatedconstant.GPIO_OUT
/// [`GPIO_ENABLE`]: #associatedconstant.GPIO_ENABLE
/// [ESP32-C3 datasheet]: https://documentation.espressif.com/esp32-c3_datasheet_en.pdf
/// [ESP32-C3 Technical Reference Manual]: https://documentation.espressif.com/esp32-c3_technical_reference_manual_en.pdf
/// [ESP32-C3 hardware reference]: https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/index.html
#[derive(Debug)]
pub struct McuEsp32C3;

/// # Clock
impl McuEsp32C3 {
    /// External crystal frequency.
    pub const XTAL_HZ: u32 = 40_000_000;
}

/// # GPIO
impl McuEsp32C3 {
    /// GPIO peripheral base address.
    pub const GPIO_BASE: u32 = 0x6000_4000;

    /// GPIO output-value register.
    pub const GPIO_OUT: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0004);

    /// GPIO output set register.
    ///
    /// Writing a `1` sets the corresponding bit in [`GPIO_OUT`](#associatedconstant.GPIO_OUT).
    pub const GPIO_OUT_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0008);

    /// GPIO output clear register.
    ///
    /// Writing a `1` clears the corresponding bit in [`GPIO_OUT`](#associatedconstant.GPIO_OUT).
    pub const GPIO_OUT_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x000c);

    /// GPIO output-enable register.
    pub const GPIO_ENABLE: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0020);

    /// GPIO output-enable set register.
    pub const GPIO_ENABLE_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0024);

    /// GPIO output-enable clear register.
    pub const GPIO_ENABLE_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0028);

    /// GPIO input-value register.
    pub const GPIO_IN: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x003c);
}

/// # Peripherals
impl McuEsp32C3 {
    /// I²C0 peripheral base address.
    pub const I2C0_BASE: u32 = 0x6001_3000;
    /// I²C0 controller.
    pub const I2C0: EspI2c = EspI2c::new(Self::I2C0_BASE);

    /// UART0 peripheral base address.
    pub const UART0_BASE: u32 = 0x6000_0000;
    /// UART0 controller.
    pub const UART0: Esp32C3Uart = Esp32C3Uart::new(Self::UART0_BASE);

    /// USB Serial/JTAG peripheral base address.
    pub const USB_SERIAL_JTAG_BASE: u32 = 0x6004_3000;
    /// Native USB Serial/JTAG controller.
    pub const USB_SERIAL_JTAG: EspUsbSerialJtag = EspUsbSerialJtag::new(Self::USB_SERIAL_JTAG_BASE);
}

/* private registers */
#[allow(dead_code)]
impl McuEsp32C3 {
    const WDT_WKEY: u32 = 0x50D8_3AA1;

    const TIMG0_WDT_CONFIG0: EspReg32 = EspReg32::new(0x6001_F048);
    const TIMG0_WDT_WPROTECT: EspReg32 = EspReg32::new(0x6001_F064);

    const RTC_WDT_CONFIG0: EspReg32 = EspReg32::new(0x6000_8090);
    const RTC_WDT_WPROTECT: EspReg32 = EspReg32::new(0x6000_80A8);
}

/// # I²C
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl McuEsp32C3 {
    /// Enables I²C0, routes it through `sda` and `scl`,
    /// and configures it as a master at `bus_hz` from the 40 MHz XTAL.
    ///
    /// The routed pins are configured for open-drain operation with input
    /// enabled and their weak internal pull-ups enabled.
    ///
    /// # Safety
    /// I²C0 and both GPIOs must not be concurrently configured or accessed.
    pub unsafe fn prepare_i2c0(sda: Esp32C3Pin, scl: Esp32C3Pin, bus_hz: u32) {
        const SYSTEM_PERIP_CLK_EN0: EspReg32 = EspReg32::new(0x600C_0010);
        const SYSTEM_PERIP_RST_EN0: EspReg32 = EspReg32::new(0x600C_0018);
        const I2C0_CLOCK: u32 = 1 << 7;
        const I2C0_RESET: u32 = 1 << 7;

        const SCL_SIGNAL: u32 = 53;
        const SDA_SIGNAL: u32 = 54;

        unsafe {
            let clock = SYSTEM_PERIP_CLK_EN0;
            clock.write(clock.read() | I2C0_CLOCK);

            let reset = SYSTEM_PERIP_RST_EN0;
            reset.write(reset.read() | I2C0_RESET);
            reset.write(reset.read() & !I2C0_RESET);

            Self::prepare_i2c0_pin(sda, SDA_SIGNAL);
            Self::prepare_i2c0_pin(scl, SCL_SIGNAL);

            Self::I2C0.configure_master_xtal(Self::XTAL_HZ, bus_hz);
        }
    }

    unsafe fn prepare_i2c0_pin(pin: Esp32C3Pin, signal: u32) {
        const IO_MUX_GPIO0: u32 = 0x6000_9004;
        const GPIO_PIN0: u32 = McuEsp32C3::GPIO_BASE + 0x74;
        const GPIO_FUNC0_IN: u32 = McuEsp32C3::GPIO_BASE + 0x154;
        const GPIO_FUNC0_OUT: u32 = McuEsp32C3::GPIO_BASE + 0x554;

        const PAD_DRIVER: u32 = 1 << 2;

        const FUN_PULL_DOWN: u32 = 1 << 7;
        const FUN_PULL_UP: u32 = 1 << 8;
        const FUN_INPUT_ENABLE: u32 = 1 << 9;
        const FUN_SELECT_MASK: u32 = 0b111 << 12;
        const FUN_GPIO: u32 = 1 << 12;

        const INPUT_MATRIX_ENABLE: u32 = 1 << 6;

        let gpio = pin.gpio() as u32;

        let io_mux = EspReg32::new(IO_MUX_GPIO0 + gpio * 4);
        let pin_reg = EspReg32::new(GPIO_PIN0 + gpio * 4);
        let input = EspReg32::new(GPIO_FUNC0_IN + signal * 4);
        let output = EspReg32::new(GPIO_FUNC0_OUT + gpio * 4);

        unsafe {
            // I²C idles high.
            pin.set_high();

            // Open-drain pad.
            pin_reg.write(pin_reg.read() | PAD_DRIVER);

            // GPIO function, input enabled, weak internal pull-up,
            // and no internal pull-down.
            io_mux.write(
                (io_mux.read() & !(FUN_SELECT_MASK | FUN_PULL_DOWN))
                    | FUN_GPIO
                    | FUN_INPUT_ENABLE
                    | FUN_PULL_UP,
            );

            // Peripheral output → GPIO.
            //
            // Clearing bits 8..10 also selects:
            // - non-inverted output,
            // - output-enable controlled by the peripheral,
            // - non-inverted output-enable.
            output.write((output.read() & !0x7ff) | signal);

            // GPIO → peripheral input through the GPIO matrix.
            input.write((input.read() & !0x7f) | INPUT_MATRIX_ENABLE | gpio);
        }
    }
}

/// # UART
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl McuEsp32C3 {
    /// Enables UART0 and selects its direct IO-MUX pins:
    /// GPIO20 for RX and GPIO21 for TX.
    ///
    /// This does not configure baud rate or framing.
    ///
    /// # Safety
    /// UART0 and GPIO20/21 must not be concurrently configured.
    pub unsafe fn prepare_uart0_default() {
        const SYSTEM_PERIP_CLK_EN0: EspReg32 = EspReg32::new(0x600C_0010);
        const SYSTEM_PERIP_RST_EN0: EspReg32 = EspReg32::new(0x600C_0018);

        const UART0_CLOCK: u32 = 1 << 2;
        const UART0_RESET: u32 = 1 << 2;
        const UART_RST_CORE: u32 = 1 << 23;
        const UART_MEMORY_CLOCK: u32 = 1 << 24;

        const IO_MUX_RX: EspReg32 = EspReg32::new(0x6000_9054);
        const IO_MUX_TX: EspReg32 = EspReg32::new(0x6000_9058);

        const INPUT_ENABLE: u32 = 1 << 9;
        const FUNCTION_MASK: u32 = 0b111 << 12;

        unsafe {
            // Keep UART0 and its FIFO memory clocked.
            let clock = SYSTEM_PERIP_CLK_EN0;
            clock.write(clock.read() | UART0_CLOCK | UART_MEMORY_CLOCK);

            // Reset the UART peripheral and core together.
            //
            // The ESP32-C3 requires the core reset around the peripheral-reset
            // pulse to avoid transient UART output during initialization.
            let reset = SYSTEM_PERIP_RST_EN0;
            let uart_clock = Self::UART0.clock_config_reg();

            reset.write(reset.read() & !UART0_RESET);
            uart_clock.write(uart_clock.read() | UART_RST_CORE);
            reset.write(reset.read() | UART0_RESET);
            reset.write(reset.read() & !UART0_RESET);
            uart_clock.write(uart_clock.read() & !UART_RST_CORE);

            // Function 0 directly connects GPIO20/21 to U0RXD/U0TXD.
            let rx = IO_MUX_RX;
            rx.write((rx.read() & !FUNCTION_MASK) | INPUT_ENABLE);

            let tx = IO_MUX_TX;
            tx.write(tx.read() & !FUNCTION_MASK);
        }
    }
}

/// # Boot watchdogs
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl McuEsp32C3 {
    /// Disables the watchdog states left active by ROM flash boot.
    ///
    /// Direct boot bypasses the usual SDK startup that handles these
    /// boot-time watchdogs. Unless they are fed, reconfigured, or disabled,
    /// they can reset a long-running application shortly after startup.
    ///
    /// This disables the Timer Group 0 MWDT and the RTC watchdog,
    /// including their flash-boot modes.
    ///
    /// # Safety
    /// No other code may concurrently configure these watchdogs.
    pub unsafe fn disable_boot_watchdogs() {
        const WDT_WRITE_KEY: u32 = 0x50D8_3AA1;

        const WDT_ENABLE: u32 = 1 << 31;

        const MWDT_FLASHBOOT_ENABLE: u32 = 1 << 14;
        const MWDT_CONFIG_UPDATE: u32 = 1 << 22;

        const RWDT_FLASHBOOT_ENABLE: u32 = 1 << 12;

        unsafe {
            // TG0 MWDT: unlock its protected configuration,
            // disable normal and flash-boot operation, commit, and relock.
            Self::TIMG0_WDT_WPROTECT.write(WDT_WRITE_KEY);

            let config = Self::TIMG0_WDT_CONFIG0.read();
            Self::TIMG0_WDT_CONFIG0
                .write((config & !(WDT_ENABLE | MWDT_FLASHBOOT_ENABLE)) | MWDT_CONFIG_UPDATE);
            Self::TIMG0_WDT_WPROTECT.write(0);

            // RTC WDT: a separate watchdog with its own protected control block.
            Self::RTC_WDT_WPROTECT.write(WDT_WRITE_KEY);

            let config = Self::RTC_WDT_CONFIG0.read();
            Self::RTC_WDT_CONFIG0.write(config & !(WDT_ENABLE | RWDT_FLASHBOOT_ENABLE));
            Self::RTC_WDT_WPROTECT.write(0);
        }
    }
}
