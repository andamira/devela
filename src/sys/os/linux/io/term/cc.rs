// devela::sys::os::linux::io::term::cc
//
//! Typed Linux termios special-character indices.
//

use crate::LINUX_TERMIOS_CC as CC;

#[doc = crate::_tags!(linux term)]
/// A symbolic index into [`LinuxTermios::c_cc`][crate::LinuxTermios::c_cc].
#[doc = crate::_doc_meta!{
    location("sys/os/linux/io/term"),
    test_size_of(LinuxTermiosCc = 1|8),
}]
///
/// These constants identify slots in the terminal control-character array.
/// They are indices, not character values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LinuxTermiosCc(u8);

impl LinuxTermiosCc {
    /// The number of Linux termios control-character slots.
    pub const COUNT: usize = 19; // would be CC::NCCS

    /// Interrupt character.
    pub const VINTR: Self = Self(CC::VINTR as u8);
    /// Quit character.
    pub const VQUIT: Self = Self(CC::VQUIT as u8);
    /// Erase character.
    pub const VERASE: Self = Self(CC::VERASE as u8);
    /// Kill character.
    pub const VKILL: Self = Self(CC::VKILL as u8);
    /// End-of-file character.
    pub const VEOF: Self = Self(CC::VEOF as u8);
    /// Timeout value for noncanonical reads.
    pub const VTIME: Self = Self(CC::VTIME as u8);
    /// Minimum byte count for noncanonical reads.
    pub const VMIN: Self = Self(CC::VMIN as u8);
    /// Switch character.
    pub const VSWTC: Self = Self(CC::VSWTC as u8);
    /// Start character.
    pub const VSTART: Self = Self(CC::VSTART as u8);
    /// Stop character.
    pub const VSTOP: Self = Self(CC::VSTOP as u8);
    /// Suspend character.
    pub const VSUSP: Self = Self(CC::VSUSP as u8);
    /// Additional end-of-line character.
    pub const VEOL: Self = Self(CC::VEOL as u8);
    /// Reprint unread characters.
    pub const VREPRINT: Self = Self(CC::VREPRINT as u8);
    /// Discard pending output.
    pub const VDISCARD: Self = Self(CC::VDISCARD as u8);
    /// Word erase character.
    pub const VWERASE: Self = Self(CC::VWERASE as u8);
    /// Literal next character.
    pub const VLNEXT: Self = Self(CC::VLNEXT as u8);
    /// Second additional end-of-line character.
    pub const VEOL2: Self = Self(CC::VEOL2 as u8);

    /// Returns a checked control-character index.
    #[must_use]
    pub const fn new(index: u8) -> Option<Self> {
        if (index as usize) < Self::COUNT { Some(Self(index)) } else { None }
    }
    /// Returns the raw index as `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
    /// Returns the raw index as `usize`.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    const _ASSERT_RAW_VALUES: () = {
        assert!(Self::VINTR.index() == CC::VINTR as usize);
        assert!(Self::VQUIT.index() == CC::VQUIT as usize);
        assert!(Self::VERASE.index() == CC::VERASE as usize);
        assert!(Self::VKILL.index() == CC::VKILL as usize);
        assert!(Self::VEOF.index() == CC::VEOF as usize);
        assert!(Self::VTIME.index() == CC::VTIME as usize);
        assert!(Self::VMIN.index() == CC::VMIN as usize);
        assert!(Self::VSWTC.index() == CC::VSWTC as usize);
        assert!(Self::VSTART.index() == CC::VSTART as usize);
        assert!(Self::VSTOP.index() == CC::VSTOP as usize);
        assert!(Self::VSUSP.index() == CC::VSUSP as usize);
        assert!(Self::VEOL.index() == CC::VEOL as usize);
        assert!(Self::VREPRINT.index() == CC::VREPRINT as usize);
        assert!(Self::VDISCARD.index() == CC::VDISCARD as usize);
        assert!(Self::VWERASE.index() == CC::VWERASE as usize);
        assert!(Self::VLNEXT.index() == CC::VLNEXT as usize);
        assert!(Self::VEOL2.index() == CC::VEOL2 as usize);
    };
}
