//
//! Defines [`EspI2c`].
//

use crate::EspReg32;
#[cfg(feature = "unsafe_mmio")]
use crate::{I2cAddr7, I2cControl, I2cError, is};

#[doc = crate::_tags!(hw io protocol)]
/// An Espressif I²C controller.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct EspI2c),
    test_size_of(EspI2c = 4|32; niche !Option),
}]
/// Provides low-level master configuration and blocking transfers
/// through the controller's command and FIFO engine.
///
/// Chip-specific clock enabling and GPIO-matrix routing are handled separately.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EspI2c(u32);

#[cfg(feature = "unsafe_mmio")]
unsafe impl I2cControl for EspI2c {
    type Error = I2cError;

    unsafe fn write_slices_unchecked(
        &mut self,
        address: I2cAddr7,
        slices: &[&[u8]],
    ) -> Result<(), Self::Error> {
        unsafe { (*self).write_slices_blocking(address, slices) }
    }

    unsafe fn read_unchecked(
        &mut self,
        address: I2cAddr7,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        unsafe { (*self).read_blocking(address, buffer) }
    }

    unsafe fn write_read_unchecked(
        &mut self,
        address: I2cAddr7,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        unsafe { (*self).write_read_blocking(address, write, read) }
    }
}

#[rustfmt::skip]
impl EspI2c {
    /// Hardware transmit/receive FIFO capacity in bytes.
    pub const FIFO_LEN: usize = 32;

    /// Creates a controller from its register base address.
    #[must_use]
    pub const fn new(base: u32) -> Self { Self(base) }

    /// Returns its register base address.
    #[must_use]
    pub const fn base_addr(self) -> u32 { self.0 }

    #[allow(clippy::identity_op)]
    #[must_use] /// Returns the SCL low-period register.
    pub const fn scl_low_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x00) }
    #[must_use] /// Returns the controller configuration register.
    pub const fn control_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x04) }
    #[must_use] /// Returns the controller status register.
    pub const fn status_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x08) }
    #[must_use] /// Returns the bus-timeout register.
    pub const fn timeout_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x0c) }
    #[must_use] /// Returns the FIFO configuration register.
    pub const fn fifo_conf_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x18) }
    #[must_use] /// Returns the FIFO data register.
    pub const fn data_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x1c) }
    #[must_use] /// Returns the raw interrupt-status register.
    pub const fn int_raw_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x20) }
    #[must_use] /// Returns the interrupt-clear register.
    pub const fn int_clr_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x24) }
    #[must_use] /// Returns the interrupt-enable register.
    pub const fn int_ena_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x28) }
    #[must_use] /// Returns the SDA hold-time register.
    pub const fn sda_hold_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x30) }
    #[must_use] /// Returns the SDA sample-time register.
    pub const fn sda_sample_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x34) }
    #[must_use] /// Returns the SCL high-period register.
    pub const fn scl_high_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x38) }
    #[must_use] /// Returns the START hold-time register.
    pub const fn start_hold_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x40) }
    #[must_use] /// Returns the repeated-START setup-time register.
    pub const fn rstart_setup_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x44) }
    #[must_use] /// Returns the STOP hold-time register.
    pub const fn stop_hold_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x48) }
    #[must_use] /// Returns the STOP setup-time register.
    pub const fn stop_setup_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x4c) }
    #[must_use] /// Returns the source-clock configuration register.
    pub const fn clock_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x54) }

    /// Returns hardware command register `index`.
    ///
    /// # Panics
    /// Panics unless `index` is in `0..8`.
    #[must_use]
    pub const fn command_reg(self, index: u8) -> EspReg32 {
        assert!(index < 8, "I2C command index must be in 0..8");
        EspReg32::new(self.0 + 0x58 + index as u32 * 4)
    }
}

#[cfg(feature = "unsafe_mmio")]
impl EspI2c {
    /// Configures master mode using an XTAL source clock.
    ///
    /// The timing calculation follows Espressif's low-level I²C timing model.
    ///
    /// # Panics
    /// Panics if either frequency is zero
    /// or the requested timing cannot be represented by this controller.
    ///
    /// # Safety
    /// The controller must be clocked and exclusively configured by this caller.
    pub unsafe fn configure_master_xtal(self, source_hz: u32, bus_hz: u32) {
        assert!(source_hz != 0);
        assert!(bus_hz != 0);

        let clkm_div = (source_hz as u64 / (bus_hz as u64 * 1024) + 1) as u32;
        let sclk_hz = source_hz / clkm_div;
        let half_cycle = sclk_hz / bus_hz / 2;
        assert!((4..=512).contains(&half_cycle), "I²C bus timing is not representable");
        let wait_high = if bus_hz >= 80_000 { half_cycle / 2 - 2 } else { half_cycle / 4 };
        let scl_high = half_cycle - wait_high;

        let sda_hold = half_cycle / 4;
        let sda_sample = half_cycle / 2;
        let setup = half_cycle;
        let hold = half_cycle;

        let timeout = 32 - (5 * half_cycle).leading_zeros() + 2;

        assert!(clkm_div <= 256);
        assert!(half_cycle <= 512);
        assert!(wait_high <= 0x7f);
        assert!(timeout <= 0x1f);

        unsafe {
            // Master, MSB first, open-drain peripheral outputs, no arbitration.
            let ctrl = self.control_reg();
            ctrl.write(
                (ctrl.read()
                    & !(Self::CTRL_ARBITRATION
                        | Self::CTRL_RX_LSB_FIRST
                        | Self::CTRL_TX_LSB_FIRST))
                    | Self::CTRL_MASTER
                    | Self::CTRL_SCL_OPEN_DRAIN
                    | Self::CTRL_SDA_OPEN_DRAIN,
            );

            self.reset_fifos();
            self.int_ena_reg().write(0);

            // XTAL source: sclk_sel = 0, fractional divider = 0.
            Self::modify(self.clock_reg(), 0x003f_ffff, (clkm_div - 1) | (1 << 21));

            Self::modify(self.scl_low_reg(), 0x1ff, half_cycle - 1);
            Self::modify(self.scl_high_reg(), 0xffff, scl_high | (wait_high << 9));

            Self::modify(self.sda_hold_reg(), 0x1ff, sda_hold - 1);
            Self::modify(self.sda_sample_reg(), 0x1ff, sda_sample - 1);

            Self::modify(self.rstart_setup_reg(), 0x1ff, setup - 1);
            Self::modify(self.stop_setup_reg(), 0x1ff, setup - 1);
            Self::modify(self.start_hold_reg(), 0x1ff, hold - 1);
            Self::modify(self.stop_hold_reg(), 0x1ff, hold - 1);

            Self::modify(self.timeout_reg(), 0x3f, timeout | (1 << 5));

            self.update();
        }
    }

    /// Tests whether a 7-bit address acknowledges.
    ///
    /// # Safety
    /// Has the same requirements as
    /// [`write_slices_blocking`](Self::write_slices_blocking).
    pub unsafe fn probe_blocking(self, address: I2cAddr7) -> Result<(), I2cError> {
        unsafe { self.write_slices_blocking(address, &[]) }
    }

    /// Performs one blocking read transaction.
    ///
    /// An empty buffer performs no bus transaction.
    ///
    /// Transfers larger than the hardware FIFO are continued transparently
    /// without releasing the bus.
    ///
    /// # Safety
    /// The controller and its routed pins must be configured for this bus
    /// and must not be concurrently accessed.
    pub unsafe fn read_blocking(
        self,
        address: I2cAddr7,
        buffer: &mut [u8],
    ) -> Result<(), I2cError> {
        is! { buffer.is_empty(), return Ok(()) }
        unsafe {
            self.prepare_transfer();
            self.read_phase(address, buffer)
        }
    }

    /// Performs a blocking write followed by a repeated-START read.
    ///
    /// No STOP is generated between the two phases.
    ///
    /// An empty write buffer degenerates to a normal read.
    /// An empty read buffer degenerates to a normal write.
    ///
    /// # Safety
    /// The controller and its routed pins must be configured for this bus
    /// and must not be concurrently accessed.
    pub unsafe fn write_read_blocking(
        self,
        address: I2cAddr7,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), I2cError> {
        is! { write.is_empty(), return unsafe { self.read_blocking(address, read) } }
        is! { read.is_empty(), return unsafe { self.write_blocking(address, write) } }
        unsafe {
            self.prepare_transfer();
            // Keep the bus after the write phase.
            self.write_slices_phase(address, &[write], false)?;
            // Issue the repeated START and begin the read phase.
            self.read_phase(address, read)
        }
    }

    /// Performs one blocking write transaction to a 7-bit I²C target.
    ///
    /// Transfers larger than the hardware FIFO are continued transparently
    /// without releasing the bus.
    ///
    /// # Safety
    /// The controller and its routed pins must be configured for this bus
    /// and must not be concurrently accessed.
    pub unsafe fn write_blocking(self, address: I2cAddr7, bytes: &[u8]) -> Result<(), I2cError> {
        unsafe { self.write_slices_blocking(address, &[bytes]) }
    }

    /// Performs one blocking write transaction from a sequence of byte slices.
    ///
    /// The slices are concatenated into a single I²C transfer without inserting
    /// STOP or repeated-START conditions between them. Empty slices are ignored.
    ///
    /// Transfers larger than the hardware FIFO are continued transparently
    /// without releasing the bus.
    ///
    /// Passing no data slices performs an address-only transaction, equivalent
    /// to [`probe_blocking`](Self::probe_blocking).
    ///
    /// # Safety
    /// The controller and its routed pins must be configured for this bus
    /// and must not be concurrently accessed.
    pub unsafe fn write_slices_blocking(
        self,
        address: I2cAddr7,
        slices: &[&[u8]],
    ) -> Result<(), I2cError> {
        unsafe {
            self.prepare_transfer();
            self.write_slices_phase(address, slices, true)
        }
    }
}

/* private helpers */

#[allow(dead_code, reason = "safe helpers used by unsafe-gated code")]
impl EspI2c {
    const CTRL_CONF_UPDATE: u32 = 1 << 11;
    const CTRL_FSM_RESET: u32 = 1 << 10;
    const CTRL_ARBITRATION: u32 = 1 << 9;
    const CTRL_RX_LSB_FIRST: u32 = 1 << 7;
    const CTRL_TX_LSB_FIRST: u32 = 1 << 6;
    const CTRL_TRANS_START: u32 = 1 << 5;
    const CTRL_MASTER: u32 = 1 << 4;
    const CTRL_SCL_OPEN_DRAIN: u32 = 1 << 1;
    const CTRL_SDA_OPEN_DRAIN: u32 = 1 << 0;

    const STATUS_BUS_BUSY: u32 = 1 << 4;

    const FIFO_TX_RESET: u32 = 1 << 13;
    const FIFO_RX_RESET: u32 = 1 << 12;
    const FIFO_NONFIFO: u32 = 1 << 10;

    const INT_NACK: u32 = 1 << 10;
    const INT_TIMEOUT: u32 = 1 << 8;
    const INT_ARBITRATION: u32 = 1 << 5;
    const INT_ALL: u32 = 0x3ffff;

    const CMD_WRITE: u32 = 1;
    const CMD_STOP: u32 = 2;
    const CMD_READ: u32 = 3;
    const CMD_END: u32 = 4;
    const CMD_RESTART: u32 = 6;
    const CMD_DONE: u32 = 1 << 31;

    const POLL_LIMIT: u32 = 1_000_000;

    /// Conservative payload size for one hardware read chunk.
    const READ_CHUNK_LEN: usize = Self::FIFO_LEN - 1;

    #[must_use]
    const fn read_command(bytes: u8, nack: bool) -> u32 {
        bytes as u32 | ((nack as u32) << 10) | (Self::CMD_READ << 11)
    }
    #[must_use]
    const fn command(op: u32, bytes: u8, check_ack: bool) -> u32 {
        bytes as u32 | ((check_ack as u32) << 8) | (op << 11) // expected ACK = 0
    }

    fn next_slice_byte(
        slices: &[&[u8]],
        slice_index: &mut usize,
        byte_index: &mut usize,
    ) -> Option<u8> {
        while *slice_index < slices.len() {
            let slice = slices[*slice_index];
            if *byte_index < slice.len() {
                let byte = slice[*byte_index];
                *byte_index += 1;
                return Some(byte);
            }
            *slice_index += 1;
            *byte_index = 0;
        }
        None
    }
}

#[cfg(feature = "unsafe_mmio")]
impl EspI2c {
    unsafe fn modify(reg: EspReg32, mask: u32, value: u32) {
        unsafe {
            reg.write((reg.read() & !mask) | (value & mask));
        }
    }
    unsafe fn reset_fifos(self) {
        let reg = self.fifo_conf_reg();
        unsafe {
            let value = reg.read() & !Self::FIFO_NONFIFO;
            reg.write(value | Self::FIFO_TX_RESET | Self::FIFO_RX_RESET);
            reg.write(value);
        }
    }
    unsafe fn update(self) {
        let reg = self.control_reg();
        unsafe { reg.write(reg.read() | Self::CTRL_CONF_UPDATE) };
    }
    unsafe fn reset_fsm(self) {
        let reg = self.control_reg();
        unsafe { reg.write(reg.read() | Self::CTRL_FSM_RESET) };
    }
    unsafe fn run_commands(self, last_command: u8) -> Result<(), I2cError> {
        unsafe {
            self.int_clr_reg().write(Self::INT_ALL);
            self.update();
            let control = self.control_reg();
            control.write(control.read() | Self::CTRL_TRANS_START);
            for _ in 0..Self::POLL_LIMIT {
                let status = self.int_raw_reg().read();
                let error = if status & Self::INT_NACK != 0 {
                    Some(I2cError::Nack)
                } else if status & Self::INT_TIMEOUT != 0 {
                    Some(I2cError::Timeout)
                } else if status & Self::INT_ARBITRATION != 0 {
                    Some(I2cError::ArbitrationLost)
                } else {
                    None
                };
                if let Some(error) = error {
                    self.int_clr_reg().write(Self::INT_ALL);
                    self.reset_fsm();
                    return Err(error);
                }
                if self.command_reg(last_command).read() & Self::CMD_DONE != 0 {
                    self.int_clr_reg().write(Self::INT_ALL);
                    return Ok(());
                }
            }
            self.reset_fsm();
            Err(I2cError::Timeout)
        }
    }

    unsafe fn prepare_transfer(self) {
        unsafe {
            is! { self.status_reg().read() & Self::STATUS_BUS_BUSY != 0, self.reset_fsm() }
            self.reset_fifos();
        }
    }
    unsafe fn read_phase(self, address: I2cAddr7, buffer: &mut [u8]) -> Result<(), I2cError> {
        debug_assert!(!buffer.is_empty());

        unsafe {
            // The read-address byte is transmitted before receiving data.
            self.data_reg().write(address.read_address_byte() as u32);

            let (mut offset, mut first) = (0usize, true);
            while offset < buffer.len() {
                let remaining = buffer.len() - offset;
                let count = remaining.min(Self::READ_CHUNK_LEN);
                let last_chunk = count == remaining;
                let mut command = 0u8;

                if first {
                    self.command_reg(command).write(Self::command(Self::CMD_RESTART, 0, false));
                    command += 1;

                    self.command_reg(command).write(Self::command(Self::CMD_WRITE, 1, true));
                    command += 1;
                }
                if last_chunk {
                    // ACK every byte except the final byte of the complete transfer.
                    if count > 1 {
                        self.command_reg(command)
                            .write(Self::read_command((count - 1) as u8, false));
                        command += 1;
                    }
                    self.command_reg(command).write(Self::read_command(1, true));
                    command += 1;
                    self.command_reg(command).write(Self::command(Self::CMD_STOP, 0, false));
                } else {
                    // More data follows, so ACK this complete chunk
                    // and retain ownership of the bus.
                    self.command_reg(command).write(Self::read_command(count as u8, false));
                    command += 1;
                    self.command_reg(command).write(Self::command(Self::CMD_END, 0, false));
                }
                self.run_commands(command)?;
                for byte in &mut buffer[offset..offset + count] {
                    *byte = self.data_reg().read() as u8;
                }
                offset += count;
                first = false;
            }
            Ok(())
        }
    }
    unsafe fn write_slices_phase(
        self,
        address: I2cAddr7,
        slices: &[&[u8]],
        stop: bool,
    ) -> Result<(), I2cError> {
        unsafe {
            self.data_reg().write(address.write_address_byte() as u32);
            self.command_reg(0).write(Self::command(Self::CMD_RESTART, 0, false));
            self.command_reg(1).write(Self::command(Self::CMD_WRITE, 1, true));

            let (mut slice_index, mut byte_index) = (0, 0);
            let mut next = Self::next_slice_byte(slices, &mut slice_index, &mut byte_index);

            if next.is_none() {
                let finish = if stop { Self::CMD_STOP } else { Self::CMD_END };
                self.command_reg(2).write(Self::command(finish, 0, false));
                return self.run_commands(2);
            }

            let mut first = true;
            while next.is_some() {
                // The first FIFO load already contains the address byte.
                let capacity = is! { first, Self::FIFO_LEN - 1, Self::FIFO_LEN };

                let mut count = 0usize;
                while count < capacity {
                    let Some(byte) = next else { break };
                    self.data_reg().write(byte as u32);
                    count += 1;
                    next = Self::next_slice_byte(slices, &mut slice_index, &mut byte_index);
                }

                let write_command = if first { 2 } else { 0 };
                self.command_reg(write_command).write(Self::command(
                    Self::CMD_WRITE,
                    count as u8,
                    true,
                ));
                let finish_command = write_command + 1;
                let finish = is! { next.is_none() && stop, Self::CMD_STOP, Self::CMD_END };
                self.command_reg(finish_command).write(Self::command(finish, 0, false));

                self.run_commands(finish_command)?;
                first = false;
            }

            Ok(())
        }
    }
}
