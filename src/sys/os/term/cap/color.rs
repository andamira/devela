// devela::sys::os::term::cap::color
//
//! Defines [`TermColorDepth`].
//

#[doc = crate::_tags!(term color)]
/// Terminal color depth.
#[doc = crate::_doc_location!("sys/os/term")]
///
/// Describes the maximum color vocabulary known to be available for terminal text output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TermColorDepth {
    /// Monochrome output.
    #[default]
    Mono = 0,
    /// 8 ANSI colors.
    Ansi3 = 1,
    /// 16 ANSI colors, including bright variants.
    Ansi4 = 2,
    /// 256-color indexed ANSI palette.
    Ansi8 = 3,
    /// 24-bit RGB color.
    Rgb24 = 4,
}
impl TermColorDepth {
    /// Returns `true` if this supports any color beyond monochrome.
    #[must_use]
    pub const fn has_color(self) -> bool {
        !matches!(self, Self::Mono)
    }

    /// Returns `true` if this supports 24-bit RGB colors.
    #[must_use]
    pub const fn has_rgb(self) -> bool {
        matches!(self, Self::Rgb24)
    }

    /// Returns whether this depth supports `other`.
    #[must_use]
    pub const fn supports(self, other: Self) -> bool {
        self as u8 >= other as u8
    }
}
