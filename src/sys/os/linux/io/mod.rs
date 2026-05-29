// devela::sys::os::linux::io
//
//! Linux-specific extensions to [`std::io`].
//

mod file; // LinuxStat, LINUX_[F_CMD|FILENO|IOCTL|O_FLAGS|S_IFMT|SEEK]
#[cfg(feature = "term")]
mod term; // LinuxTermios, LINUX_TERMIOS

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::file::_all::*;
        #[cfg(feature = "term")]
        pub use super::term::_all::*;
    }
    _crate_internals {
        #[cfg(feature = "term")]
        pub use super::term::_crate_internals::*;
    }
}
