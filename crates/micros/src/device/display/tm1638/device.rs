//
//! Defines [`Tm1638`] and [`Tm1638Brightness`].
//

use crate::{BittenU8, Tm1638Bus, Tm1638Frame};

const CMD_WRITE_AUTO: u8 = 0x40;
const CMD_READ_KEYS: u8 = 0x42;
const CMD_WRITE_FIXED: u8 = 0x44;

const CMD_ADDRESS: u8 = 0xC0;
const CMD_DISPLAY: u8 = 0x80;
const DISPLAY_ON: u8 = 0x08;

#[doc = crate::_tags!(hw io)]
/// TM1638 display brightness level.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tm1638Brightness(BittenU8<5>);

impl Tm1638Brightness {
    /// Minimum brightness.
    pub const MIN: Self = Self(BittenU8::<5>::MIN);

    /// Maximum brightness.
    pub const MAX: Self = Self(BittenU8::<5>::MAX);

    /// Creates a brightness level from `0..=7`.
    #[must_use]
    pub const fn new(level: u8) -> Option<Self> {
        match BittenU8::<5>::new(level) {
            Some(level) => Some(Self(level)),
            None => None,
        }
    }

    /// Creates a brightness level, clamped to `7`.
    #[must_use]
    pub const fn new_lossy(level: u8) -> Self {
        Self(BittenU8::<5>::new_lossy(level))
    }

    /// Returns the encoded brightness level.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0.get()
    }
}

#[doc = crate::_tags!(hw io display interaction)]
/// A TM1638 LED display and key-scan controller.
#[doc = crate::_doc_meta!{
    location("device/display", struct Tm1638),
    test_size_of(Tm1638 = 0),
}]
#[derive(Clone, Copy, Debug, Default)]
pub struct Tm1638;

impl Tm1638 {
    /// Number of display-RAM bytes.
    pub const RAM_BYTES: usize = 16;

    /// Number of bytes returned by one key scan.
    pub const KEY_BYTES: usize = 4;

    /// Creates a TM1638 controller value.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Clears display RAM and enables the display.
    pub fn init<B: Tm1638Bus>(
        self,
        bus: &mut B,
        brightness: Tm1638Brightness,
    ) -> Result<(), B::Error> {
        self.clear(bus)?;
        self.set_display(bus, true, brightness)
    }

    /// Sets display enable and brightness.
    pub fn set_display<B: Tm1638Bus>(
        self,
        bus: &mut B,
        enabled: bool,
        brightness: Tm1638Brightness,
    ) -> Result<(), B::Error> {
        let command = [CMD_DISPLAY | if enabled { DISPLAY_ON } else { 0 } | brightness.get()];
        bus.write(&command)
    }

    /// Clears all 16 display-RAM bytes.
    pub fn clear<B: Tm1638Bus>(self, bus: &mut B) -> Result<(), B::Error> {
        self.write_ram(bus, 0, &[0; Self::RAM_BYTES])
    }

    /// Writes a complete display frame.
    pub fn write_frame<B: Tm1638Bus>(
        self,
        bus: &mut B,
        frame: &Tm1638Frame,
    ) -> Result<(), B::Error> {
        self.write_ram(bus, 0, frame.as_bytes())
    }

    /// Writes consecutive display RAM beginning at `address`.
    pub fn write_ram<B: Tm1638Bus>(
        self,
        bus: &mut B,
        address: u8,
        data: &[u8],
    ) -> Result<(), B::Error> {
        assert!(address < Self::RAM_BYTES as u8);
        assert!(data.len() <= Self::RAM_BYTES - address as usize);

        bus.write(&[CMD_WRITE_AUTO])?;

        let address = [CMD_ADDRESS | address];
        bus.write_slices(&[&address, data])
    }

    /// Writes one display-RAM byte using fixed-address mode.
    pub fn write_byte<B: Tm1638Bus>(
        self,
        bus: &mut B,
        address: u8,
        value: u8,
    ) -> Result<(), B::Error> {
        assert!(address < Self::RAM_BYTES as u8);

        bus.write(&[CMD_WRITE_FIXED])?;

        let address = [CMD_ADDRESS | address];
        let value = [value];
        bus.write_slices(&[&address, &value])
    }

    /// Reads the four raw key-scan bytes.
    pub fn read_keys<B: Tm1638Bus>(self, bus: &mut B) -> Result<[u8; Self::KEY_BYTES], B::Error> {
        let mut keys = [0; Self::KEY_BYTES];
        bus.write_read(&[CMD_READ_KEYS], &mut keys)?;
        Ok(keys)
    }
}
