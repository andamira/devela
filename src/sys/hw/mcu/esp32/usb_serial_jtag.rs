// devela/sys/hw/mcu/esp32/usb_serial_jtag.rs
//
//! Defines [`EspUsbSerialJtag`].
//

use crate::{__cfg_item_unsafe_show, EspReg32, macro_apply};

#[doc = crate::_tags!(hw io)]
/// An Espressif USB Serial/JTAG controller.
///
/// Provides byte-oriented polling access to the CDC-ACM serial endpoint.
///
/// Unlike a UART, this transport has no baud rate or framing configuration:
/// bytes are exchanged as USB packets through the controller's endpoint FIFO.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/esp32", struct EspUsbSerialJtag),
    test_size_of(EspUsbSerialJtag = 4|32; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EspUsbSerialJtag(u32);

#[rustfmt::skip]
impl EspUsbSerialJtag {
    /// Creates a USB Serial/JTAG controller from its register base address.
    #[must_use]
    pub const fn new(base: u32) -> Self { Self(base) }

    /// Returns its register base address.
    #[must_use]
    pub const fn base_addr(self) -> u32 { self.0 }

    /// Returns the CDC data FIFO register.
    #[must_use]
    pub const fn data_reg(self) -> EspReg32 {
        EspReg32::new(self.0)
    }
    /// Returns the CDC endpoint configuration and status register.
    #[must_use]
    pub const fn ep1_conf_reg(self) -> EspReg32 {
        EspReg32::new(self.0 + 0x04)
    }
}

/* private helpers */

#[allow(dead_code)]
impl EspUsbSerialJtag {
    const WR_DONE: u32 = 1 << 0;
    const TX_FREE: u32 = 1 << 1;
    const RX_AVAIL: u32 = 1 << 2;
}

/// # Receiver
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl EspUsbSerialJtag {
    /// Returns whether a received byte is ready.
    ///
    /// # Safety
    /// This must describe the active USB Serial/JTAG peripheral.
    #[must_use]
    pub unsafe fn rx_ready(self) -> bool {
        unsafe { self.ep1_conf_reg().read() & Self::RX_AVAIL != 0 }
    }
    /// Attempts to read one received byte without waiting.
    ///
    /// # Safety
    /// This must describe the active USB Serial/JTAG peripheral
    /// and not be concurrently read.
    #[must_use]
    pub unsafe fn try_read_byte(self) -> Option<u8> {
        if unsafe { self.rx_ready() } {
            Some(unsafe { self.data_reg().read() as u8 })
        } else {
            None
        }
    }
    /// Reads one received byte, waiting until one is available.
    ///
    /// # Safety
    /// This must describe the active USB Serial/JTAG peripheral
    /// and not be concurrently read.
    #[must_use]
    pub unsafe fn read_byte_blocking(self) -> u8 {
        while !unsafe { self.rx_ready() } {}
        unsafe { self.data_reg().read() as u8 }
    }
}

/// # Transmitter
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl EspUsbSerialJtag {
    /// Returns whether the transmit FIFO can accept data.
    ///
    /// # Safety
    /// This must describe the active USB Serial/JTAG peripheral.
    #[must_use]
    pub unsafe fn tx_ready(self) -> bool {
        unsafe { self.ep1_conf_reg().read() & Self::TX_FREE != 0 }
    }
    /// Marks the current transmit data as ready for the USB host.
    ///
    /// # Safety
    /// This must describe the active USB Serial/JTAG peripheral
    /// and not be concurrently written.
    pub unsafe fn flush_tx(self) {
        unsafe { self.ep1_conf_reg().write(Self::WR_DONE) };
    }
    /// Attempts to send one byte without waiting.
    ///
    /// Returns `false` if the transmit FIFO is currently unavailable.
    ///
    /// # Safety
    /// This must describe the active USB Serial/JTAG peripheral
    /// and not be concurrently written.
    #[must_use]
    pub unsafe fn try_write_byte(self, byte: u8) -> bool {
        if unsafe { self.tx_ready() } {
            unsafe {
                self.data_reg().write(byte as u32);
                self.flush_tx();
            }
            true
        } else {
            false
        }
    }
    /// Sends one byte, waiting until the transmit FIFO is available.
    ///
    /// This may wait indefinitely if the USB host stops consuming the CDC-ACM endpoint.
    ///
    /// # Safety
    /// This must describe the active USB Serial/JTAG peripheral
    /// and not be concurrently written.
    pub unsafe fn write_byte_blocking(self, byte: u8) {
        while !unsafe { self.tx_ready() } {}
        unsafe {
            self.data_reg().write(byte as u32);
            self.flush_tx();
        }
    }
    /// Sends all bytes, waiting for the host as needed.
    ///
    /// This may wait indefinitely if the USB host stops consuming the CDC-ACM endpoint.
    ///
    /// # Safety
    /// This must describe the active USB Serial/JTAG peripheral
    /// and not be concurrently written.
    pub unsafe fn write_bytes_blocking(self, bytes: &[u8]) {
        for &byte in bytes {
            unsafe { self.write_byte_blocking(byte) };
        }
    }
}
