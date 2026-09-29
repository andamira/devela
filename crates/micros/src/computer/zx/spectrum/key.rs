//
//! Defines [`SpectrumKey`], [`SpectrumKeys`].
//

#[rustfmt::skip]
#[allow(missing_docs)]
#[doc = crate::_tags!(hw interaction)]
/// A key in the classic ZX Spectrum 40-key keyboard matrix.
#[doc = crate::_doc_meta!{
    location("computer/zx", enum SpectrumKey),
    test_size_of(SpectrumKey = 1|8; niche Option),
}]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpectrumKey {
    CapsShift = 0x00,
    Z         = 0x01,
    X         = 0x02,
    C         = 0x03,
    V         = 0x04,

    A         = 0x08,
    S         = 0x09,
    D         = 0x0A,
    F         = 0x0B,
    G         = 0x0C,

    Q         = 0x10,
    W         = 0x11,
    E         = 0x12,
    R         = 0x13,
    T         = 0x14,

    Num1      = 0x18,
    Num2      = 0x19,
    Num3      = 0x1A,
    Num4      = 0x1B,
    Num5      = 0x1C,

    Num0      = 0x20,
    Num9      = 0x21,
    Num8      = 0x22,
    Num7      = 0x23,
    Num6      = 0x24,

    P         = 0x28,
    O         = 0x29,
    I         = 0x2A,
    U         = 0x2B,
    Y         = 0x2C,

    Enter     = 0x30,
    L         = 0x31,
    K         = 0x32,
    J         = 0x33,
    H         = 0x34,

    Space     = 0x38,
    SymbolShift = 0x39,
    M         = 0x3A,
    N         = 0x3B,
    B         = 0x3C,
}

impl SpectrumKey {
    /// Returns this key's keyboard half-row index (`0..=7`).
    #[must_use]
    pub const fn half_row(self) -> u8 {
        (self as u8) >> 3
    }
    /// Returns this key's bit index within its half-row (`0..=4`).
    #[must_use]
    pub const fn bit(self) -> u8 {
        (self as u8) & 0x07
    }
    /// Returns this key's bit mask within its half-row.
    #[must_use]
    pub const fn mask(self) -> u8 {
        1 << self.bit()
    }
    /// Returns the complete 16-bit Z80 I/O address selecting this key's half-row.
    // FEFE  Caps Z X C V
    // FDFE  A S D F G
    // FBFE  Q W E R T
    // F7FE  1 2 3 4 5
    // EFFE  0 9 8 7 6
    // DFFE  P O I U Y
    // BFFE  Enter L K J H
    // 7FFE  Space Sym M N B
    #[must_use]
    pub const fn port(self) -> u16 {
        let high = !(1_u8 << self.half_row());
        ((high as u16) << 8) | 0xFE
    }
}

#[doc = crate::_tags!(hw interaction state)]
/// A snapshot of the ZX Spectrum keyboard matrix.
#[doc = crate::_doc_meta!{
    location("computer/zx", struct SpectrumKeys),
    test_size_of(SpectrumKeys = 8|64; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SpectrumKeys([u8; 8]);

impl SpectrumKeys {
    /// No keys pressed.
    pub const NONE: Self = Self([0; 8]);

    pub(crate) const fn new(rows: [u8; 8]) -> Self {
        Self(rows)
    }

    /// Returns whether `key` is pressed.
    #[must_use]
    pub const fn is_pressed(self, key: SpectrumKey) -> bool {
        self.0[key.half_row() as usize] & key.mask() != 0
    }
    /// Returns whether `key` became pressed since `previous`.
    #[must_use]
    pub const fn just_pressed(self, previous: Self, key: SpectrumKey) -> bool {
        self.is_pressed(key) && !previous.is_pressed(key)
    }
    /// Returns whether `key` became released since `previous`.
    #[must_use]
    pub const fn just_released(self, previous: Self, key: SpectrumKey) -> bool {
        !self.is_pressed(key) && previous.is_pressed(key)
    }
}
