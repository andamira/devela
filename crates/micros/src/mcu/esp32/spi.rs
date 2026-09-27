//
//! Defines [`EspSpi`].
//

use crate::{EspReg32, Timeout, is};

#[doc = crate::_tags!(hw io protocol)]
/// An Espressif general-purpose SPI controller.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct EspSpi),
    test_size_of(EspSpi = 4|32; niche !Option),
}]
/// Provides CPU-buffered, blocking master writes through the controller's
/// 64-byte data window.
///
/// Chip-specific clock enabling and GPIO-matrix routing are handled separately.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EspSpi(u32);

#[rustfmt::skip]
impl EspSpi {
    /// Hardware CPU-controlled data-buffer capacity in bytes.
    pub const CPU_BUFFER_LEN: usize = 64;

    /// Creates a controller from its register base address.
    #[must_use]
    pub const fn new(base: u32) -> Self { Self(base) }

    /// Returns its register base address.
    #[must_use]
    pub const fn base_addr(self) -> u32 { self.0 }

    #[allow(clippy::identity_op)]
    #[must_use] /// Returns the command-control register.
    pub const fn command_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x00) }
    #[must_use] /// Returns the clock-control register.
    pub const fn clock_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x0c) }
    #[must_use] /// Returns the USER control register.
    pub const fn user_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x10) }
    #[must_use] /// Returns the USER1 timing/phase register.
    pub const fn user1_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x18) }
    #[must_use] /// Returns the master data-bit-length register.
    pub const fn data_len_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x1c) }
    #[must_use] /// Returns the miscellaneous-control register.
    pub const fn misc_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x20) }
    #[must_use] /// Returns the DMA-control register.
    pub const fn dma_conf_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x30) }
    #[must_use] /// Returns the slave-control register.
    pub const fn slave_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0xe0) }

    /// Returns CPU data-buffer register `index`.
    ///
    /// # Panics
    /// Panics unless `index` is in `0..16`.
    #[must_use]
    pub const fn data_reg(self, index: u8) -> EspReg32 {
        assert!(index < 16, "SPI data register index must be in 0..16");
        EspReg32::new(self.0 + 0x98 + index as u32 * 4)
    }
}

#[allow(dead_code, reason = "safe helpers used by unsafe-gated code")]
impl EspSpi {
    const CMD_UPDATE: u32 = 1 << 23;
    const CMD_USR: u32 = 1 << 24;

    const USER_USR_MOSI: u32 = 1 << 27;

    const MISC_CS_DISABLE_MASK: u32 = 0x3f;
    const MISC_CK_IDLE_EDGE: u32 = 1 << 29;

    const POLL_LIMIT: u32 = 1_000_000;
}

#[cfg(feature = "unsafe_mmio")]
impl EspSpi {
    unsafe fn wait_clear(self, reg: EspReg32, mask: u32) -> Result<(), Timeout> {
        unsafe {
            for _ in 0..Self::POLL_LIMIT {
                is! { reg.read() & mask == 0, return Ok(()) }
            }
            Err(Timeout)
        }
    }

    unsafe fn apply_config(self) -> Result<(), Timeout> {
        unsafe {
            let command = self.command_reg();
            command.write(command.read() | Self::CMD_UPDATE);
            self.wait_clear(command, Self::CMD_UPDATE)
        }
    }

    /// Configures a CPU-buffered, single-line SPI master in mode 0.
    ///
    /// `source_hz` is the clock source already selected by the chip-specific
    /// peripheral setup. The closest representable frequency not exceeding
    /// `bus_hz` is selected and returned.
    ///
    /// # Panics
    /// Panics if either frequency is zero or no divider can represent a
    /// frequency at or below `bus_hz`.
    ///
    /// # Safety
    /// The controller must be clocked, released from reset, and exclusively
    /// configured by this caller.
    pub unsafe fn configure_master_mode0(
        self,
        source_hz: u32,
        bus_hz: u32,
    ) -> Result<u32, Timeout> {
        assert!(source_hz != 0);
        assert!(bus_hz != 0);

        let (clock, actual_hz) = if bus_hz >= source_hz {
            (1 << 31, source_hz)
        } else {
            let mut best_hz = 0;
            let mut best_pre = 0;
            let mut best_n = 0;

            let mut pre = 1u32;
            while pre <= 16 {
                let mut n = 2u32;
                while n <= 64 {
                    let hz = source_hz / pre / n;
                    if hz <= bus_hz && hz > best_hz {
                        best_hz = hz;
                        best_pre = pre;
                        best_n = n;
                    }
                    n += 1;
                }
                pre += 1;
            }
            assert!(best_hz != 0, "SPI bus timing is not representable");

            let high = best_n.div_ceil(2);
            let value =
                (best_n - 1) | ((high - 1) << 6) | ((best_n - 1) << 12) | ((best_pre - 1) << 18);
            (value, best_hz)
        };

        unsafe {
            // CPU-buffered, one-line, output-only user transactions in mode 0.
            self.slave_reg().write(0);
            self.user_reg().write(Self::USER_USR_MOSI);
            self.user1_reg().write(0);

            // No hardware CS line: board/device adapters drive CS as GPIO.
            let misc = self.misc_reg();
            misc.write((misc.read() & !Self::MISC_CK_IDLE_EDGE) | Self::MISC_CS_DISABLE_MASK);

            // Match Espressif's master initialization for segmented-transfer
            // FIFO clearing while otherwise leaving DMA disabled.
            self.dma_conf_reg().write((1 << 19) | (1 << 20));
            self.clock_reg().write(clock);

            self.apply_config()?;
        }
        Ok(actual_hz)
    }

    /// Performs a blocking single-line master write.
    ///
    /// Payloads larger than the 64-byte hardware buffer are split into
    /// consecutive controller transactions. Chip select is not managed here.
    ///
    /// # Safety
    /// The controller and its routed output pins must be configured for this
    /// bus and must not be concurrently accessed.
    pub unsafe fn write_blocking(self, bytes: &[u8]) -> Result<(), Timeout> {
        unsafe {
            for chunk in bytes.chunks(Self::CPU_BUFFER_LEN) {
                let mut offset = 0usize;
                let mut word_index = 0u8;
                while offset < chunk.len() {
                    let mut word = 0u32;
                    let mut byte = 0usize;
                    while byte < 4 && offset + byte < chunk.len() {
                        word |= (chunk[offset + byte] as u32) << (byte * 8);
                        byte += 1;
                    }
                    self.data_reg(word_index).write(word);
                    offset += 4;
                    word_index += 1;
                }

                self.data_len_reg().write((chunk.len() as u32 * 8) - 1);
                self.apply_config()?;

                let command = self.command_reg();
                command.write(command.read() | Self::CMD_USR);
                self.wait_clear(command, Self::CMD_USR)?;
            }
            Ok(())
        }
    }
}
