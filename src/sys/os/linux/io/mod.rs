// devela::sys::os::linux::io
//
//! Linux-specific extensions to [`std::io`].
//

mod file; // LinuxStat, LINUX_[F_CMD|FILENO|IOCTL|O_FLAGS|S_IFMT|SEEK]
#[cfg(feature = "term")]
mod term; // LinuxTermios, LINUX_TERMIOS

crate::structural_mods! { // _mods
    _mods {
        pub use super::file::*;
        #[cfg(feature = "term")]
        pub use super::term::*;
    }
}
