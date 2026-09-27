//
//! Defines [`Tm1638Frame`] and [`Tm1638LedKey8`].
//

#[doc = crate::_tags!(hw display)]
/// A complete 16-byte TM1638 display-RAM image.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tm1638Frame {
    bytes: [u8; 16],
}

impl Tm1638Frame {
    /// Creates a cleared frame.
    #[must_use]
    pub const fn new() -> Self {
        Self { bytes: [0; 16] }
    }

    /// Returns the native display-RAM bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    /// Clears the frame.
    pub const fn clear(&mut self) {
        self.bytes = [0; 16];
    }

    /// Sets one raw display-RAM byte.
    pub const fn set_byte(&mut self, address: u8, value: u8) {
        assert!(address < 16);
        self.bytes[address as usize] = value;
    }

    /// Returns one raw display-RAM byte.
    #[must_use]
    pub const fn byte(&self, address: u8) -> u8 {
        assert!(address < 16);
        self.bytes[address as usize]
    }

    /// Sets the ten segment bits belonging to one grid.
    pub const fn set_grid(&mut self, grid: u8, segments: u16) {
        assert!(grid < 8);
        assert!(segments <= 0x03FF);

        let address = grid as usize * 2;
        self.bytes[address] = segments as u8;
        self.bytes[address + 1] = (segments >> 8) as u8;
    }
}

impl Default for Tm1638Frame {
    fn default() -> Self {
        Self::new()
    }
}

#[doc = crate::_tags!(hw display interaction)]
/// Wiring profile for the TM1638 "LED & KEY" 8-digit module.
///
/// This profile has:
/// - 8 seven-segment digits with decimal points,
/// - 8 individual LEDs,
/// - 8 push buttons.
#[derive(Clone, Copy, Debug, Default)]
pub struct Tm1638LedKey8;

impl Tm1638LedKey8 {
    /// Writes one raw 8-bit seven-segment pattern.
    pub const fn set_digit(frame: &mut Tm1638Frame, position: u8, segments: u8) {
        assert!(position < 8);
        frame.set_byte(position * 2, segments);
    }

    /// Sets one of the eight auxiliary LEDs.
    pub const fn set_led(frame: &mut Tm1638Frame, position: u8, enabled: bool) {
        assert!(position < 8);

        let address = position * 2 + 1;
        let old = frame.byte(address);
        frame.set_byte(address, (old & !1) | enabled as u8);
    }

    /// Decodes the module's eight push buttons into one bit mask.
    #[must_use]
    pub const fn decode_keys(raw: [u8; 4]) -> u8 {
        (raw[0] & 0x01)
            | ((raw[1] & 0x01) << 1)
            | ((raw[2] & 0x01) << 2)
            | ((raw[3] & 0x01) << 3)
            | (raw[0] & 0x10)
            | ((raw[1] & 0x10) << 1)
            | ((raw[2] & 0x10) << 2)
            | ((raw[3] & 0x10) << 3)
    }
}
