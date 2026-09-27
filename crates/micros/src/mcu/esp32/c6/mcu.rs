//
//! Defines [`McuEsp32C6`].
//
// TOC
// - struct McuEsp32C6
// - impl GPIO
// - impl Peripherals
// - impl Clock
// - impl Boot watchdogs
// - impl Private registers

use crate::EspReg32;

#[doc = crate::_tags!(hw namespace)]
/// ESP32-C6 microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct McuEsp32C6),
    test_size_of(McuEsp32C6 = 0),
}]
/// The ESP32-C6 combines a high-performance RV32IMAC core running at up to
/// 160 MHz with a low-power RISC-V core, 320 KiB of ROM, 512 KiB of HP SRAM,
/// and 16 KiB of LP SRAM.
///
/// It integrates 2.4 GHz Wi-Fi 6 and Bluetooth LE, alongside peripherals
/// including UART, I²C, SPI, timers, watchdogs, and USB Serial/JTAG.
/// GPIO0 through GPIO30 are implemented by the silicon, subject to package,
/// strapping, and peripheral-routing constraints.
///
/// devela currently provides ROM direct-boot startup and low-level digital
/// GPIO access for the ESP32-C6. Additional peripheral APIs are added only
/// after their C6-specific clocking and routing contracts are represented.
///
/// See also:
///
/// - [ESP32-C6 datasheet]
/// - [ESP32-C6 Technical Reference Manual]
/// - [ESP32-C6 hardware reference]
///
/// [ESP32-C6 datasheet]: https://www.espressif.com/sites/default/files/documentation/esp32-c6_datasheet_en.pdf
/// [ESP32-C6 Technical Reference Manual]: https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf
/// [ESP32-C6 hardware reference]: https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/hw-reference/index.html
#[derive(Debug)]
pub struct McuEsp32C6;

/// # GPIO
impl McuEsp32C6 {
    /// GPIO peripheral base address.
    pub const GPIO_BASE: u32 = 0x6009_1000;

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
impl McuEsp32C6 {
    /// UART0 peripheral base address.
    pub const UART0_BASE: u32 = 0x6000_0000;

    /// I²C0 peripheral base address.
    pub const I2C0_BASE: u32 = 0x6000_4000;

    /// Timer Group 0 peripheral base address.
    pub const TIMG0_BASE: u32 = 0x6000_8000;

    /// Timer Group 1 peripheral base address.
    pub const TIMG1_BASE: u32 = 0x6000_9000;

    /// System Timer peripheral base address.
    pub const SYSTIMER_BASE: u32 = 0x6000_a000;

    /// USB Serial/JTAG peripheral base address.
    pub const USB_SERIAL_JTAG_BASE: u32 = 0x6000_f000;

    /// General-purpose SPI2 peripheral base address.
    pub const SPI2_BASE: u32 = 0x6008_1000;
}

/// # Clock
impl McuEsp32C6 {
    /// External main-crystal (`XTAL_CLK`) frequency, in hertz.
    pub const XTAL_HZ: u32 = 40_000_000;
}

/// # Boot watchdogs
#[cfg(feature = "unsafe_mmio")]
impl McuEsp32C6 {
    /// Completes the watchdog handoff from ROM flash boot to direct-boot code.
    ///
    /// Direct boot bypasses the usual SDK startup that takes ownership of
    /// watchdog state established during boot. This disables the Timer Group
    /// watchdogs and LP watchdogs so a long-running bare-metal application is
    /// not reset by boot-time watchdog state.
    ///
    /// # Safety
    /// No other code may concurrently configure these watchdogs.
    pub unsafe fn handoff_boot_watchdogs() {
        const WDT_WRITE_KEY: u32 = 0x50D8_3AA1;
        const WDT_INTERRUPT: u32 = 1 << 1;
        const LP_WDT_INTERRUPTS: u32 = (1 << 31) | (1 << 30);
        const LP_SWD_DISABLE: u32 = 1 << 30;

        unsafe {
            // Timer Group 0 MWDT.
            Self::TIMG0_WDT_WPROTECT.write(WDT_WRITE_KEY);
            Self::TIMG0_WDT_CONFIG0.write(0);
            Self::TIMG0_INT_CLR.write(WDT_INTERRUPT);
            Self::TIMG0_WDT_WPROTECT.write(0);

            // Timer Group 1 MWDT.
            Self::TIMG1_WDT_WPROTECT.write(WDT_WRITE_KEY);
            Self::TIMG1_WDT_CONFIG0.write(0);
            Self::TIMG1_INT_CLR.write(WDT_INTERRUPT);
            Self::TIMG1_WDT_WPROTECT.write(0);

            // LP watchdog.
            Self::LP_WDT_WPROTECT.write(WDT_WRITE_KEY);
            Self::LP_WDT_CONFIG0.write(0);
            Self::LP_WDT_WPROTECT.write(0);

            // Super watchdog.
            Self::LP_SWD_WPROTECT.write(WDT_WRITE_KEY);
            Self::LP_SWD_CONFIG.write(LP_SWD_DISABLE);
            Self::LP_SWD_WPROTECT.write(0);

            // Drop any boot-time watchdog interrupt state.
            Self::LP_WDT_INT_CLR.write(LP_WDT_INTERRUPTS);
        }
    }
}

// # Private registers
#[allow(dead_code, reason = "safe helpers used by unsafe-gated code")]
impl McuEsp32C6 {
    const TIMG0_WDT_CONFIG0: EspReg32 = EspReg32::new(Self::TIMG0_BASE + 0x48);
    const TIMG0_WDT_WPROTECT: EspReg32 = EspReg32::new(Self::TIMG0_BASE + 0x64);
    const TIMG0_INT_CLR: EspReg32 = EspReg32::new(Self::TIMG0_BASE + 0x7c);

    const TIMG1_WDT_CONFIG0: EspReg32 = EspReg32::new(Self::TIMG1_BASE + 0x48);
    const TIMG1_WDT_WPROTECT: EspReg32 = EspReg32::new(Self::TIMG1_BASE + 0x64);
    const TIMG1_INT_CLR: EspReg32 = EspReg32::new(Self::TIMG1_BASE + 0x7c);

    const LP_WDT_BASE: u32 = 0x600b_1c00;
    const LP_WDT_CONFIG0: EspReg32 = EspReg32::new(Self::LP_WDT_BASE);
    const LP_WDT_WPROTECT: EspReg32 = EspReg32::new(Self::LP_WDT_BASE + 0x18);
    const LP_SWD_CONFIG: EspReg32 = EspReg32::new(Self::LP_WDT_BASE + 0x1c);
    const LP_SWD_WPROTECT: EspReg32 = EspReg32::new(Self::LP_WDT_BASE + 0x20);
    const LP_WDT_INT_CLR: EspReg32 = EspReg32::new(Self::LP_WDT_BASE + 0x30);
}
