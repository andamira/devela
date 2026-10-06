use crate::AvrUsart;

/// # Receiver
impl AvrUsart {
    /// Returns whether a received byte is ready to be read.
    ///
    /// # Safety
    /// The USART must belong to the active device.
    #[must_use]
    pub unsafe fn rx_ready(self) -> bool {
        unsafe { self.status_reg().read() & Self::RXC != 0 }
    }
    /// Enables the receiver, preserving the other control bits.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently used or configured.
    pub unsafe fn enable_rx(self) {
        let reg = self.control_reg();
        unsafe { reg.write(reg.read() | Self::RXEN) };
    }
    /// Disables the receiver, preserving the other control bits.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently used or configured.
    pub unsafe fn disable_rx(self) {
        let reg = self.control_reg();
        unsafe { reg.write(reg.read() & !Self::RXEN) };
    }

    /// Attempts to read one received byte without waiting.
    ///
    /// Returns `None` if no received byte is available.
    ///
    /// The receiver must already be enabled.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently read.
    #[must_use]
    pub unsafe fn try_read_byte(self) -> Option<u8> {
        if unsafe { self.rx_ready() } {
            Some(unsafe { self.data_reg().read() })
        } else {
            None
        }
    }
    /// Reads one received byte, waiting until one is available.
    ///
    /// The receiver must already be enabled.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently read.
    #[must_use]
    pub unsafe fn read_byte_blocking(self) -> u8 {
        while !unsafe { self.rx_ready() } {}
        unsafe { self.data_reg().read() }
    }
}

/// # Transmitter
impl AvrUsart {
    /// Returns whether the transmit data register can accept another byte.
    ///
    /// # Safety
    /// The USART must belong to the active device.
    #[must_use]
    pub unsafe fn tx_ready(self) -> bool {
        unsafe { self.status_reg().read() & Self::UDRE != 0 }
    }
    /// Enables the transmitter, preserving the other control bits.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently used or configured.
    pub unsafe fn enable_tx(self) {
        let reg = self.control_reg();
        unsafe { reg.write(reg.read() | Self::TXEN) };
    }
    /// Disables the transmitter, preserving the other control bits.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently used or configured.
    pub unsafe fn disable_tx(self) {
        let reg = self.control_reg();
        unsafe { reg.write(reg.read() & !Self::TXEN) };
    }

    /// Attempts to queue one byte for transmission without waiting.
    ///
    /// Returns `true` if the byte was accepted,
    /// or `false` if the transmit data register was still busy.
    ///
    /// The transmitter must already be enabled.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently written.
    #[must_use]
    pub unsafe fn try_write_byte(self, byte: u8) -> bool {
        if unsafe { self.tx_ready() } {
            unsafe { self.data_reg().write(byte) };
            true
        } else {
            false
        }
    }
    /// Queues one byte for transmission, waiting until the USART can accept it.
    ///
    /// Waits for space in the transmit data register;
    /// it does not wait for the byte to finish shifting onto the wire.
    ///
    /// The transmitter must already be enabled.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently written.
    pub unsafe fn write_byte_blocking(self, byte: u8) {
        while !unsafe { self.tx_ready() } {}
        unsafe { self.data_reg().write(byte) };
    }
    /// Queues all bytes for transmission, waiting for space as needed.
    ///
    /// Returns after the final byte has been accepted by the USART,
    /// which may be before that byte has finished transmitting on the wire.
    ///
    /// The transmitter must already be enabled.
    ///
    /// # Safety
    /// The USART must belong to the active device and not be concurrently written.
    pub unsafe fn write_bytes_blocking(self, bytes: &[u8]) {
        for &byte in bytes {
            unsafe { self.write_byte_blocking(byte) };
        }
    }
}
