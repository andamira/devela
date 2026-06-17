// devela/src/sys/os/linux/io/file/fd/seek.rs
//
//! Defines [`LinuxSeekFrom`].
//

use crate::{LINUX_SEEK, c_int, c_off_t};

#[doc = crate::_tags!(linux fs)]
/// A Linux file-positioning directive.
#[doc = crate::_doc_meta!{
    location("sys/os/linux/io"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(LinuxSeekFrom = 12|96),
    #[cfg(target_pointer_width = "64")]
    test_size_of(LinuxSeekFrom = 16|128),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxSeekFrom {
    /// Seek from the start of the file.
    Start(c_off_t),

    /// Seek from the current file position.
    Current(c_off_t),

    /// Seek from the end of the file.
    End(c_off_t),

    /// Seek to the next data region at or after the offset.
    Data(c_off_t),

    /// Seek to the next hole at or after the offset.
    Hole(c_off_t),
}

impl LinuxSeekFrom {
    /// Returns the raw Linux `offset` and `whence`.
    pub(crate) const fn raw(self) -> (c_off_t, c_int) {
        match self {
            Self::Start(offset) => (offset, LINUX_SEEK::SET),
            Self::Current(offset) => (offset, LINUX_SEEK::CUR),
            Self::End(offset) => (offset, LINUX_SEEK::END),
            Self::Data(offset) => (offset, LINUX_SEEK::DATA),
            Self::Hole(offset) => (offset, LINUX_SEEK::HOLE),
        }
    }
}
