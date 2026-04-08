// devela::sys::os::fd::_reexport_std

#[cfg(all(not(miri), any(unix, target_os = "wasi")))]
pub use fd_reexports::*;
#[cfg(all(not(miri), any(unix, target_os = "wasi")))]
mod fd_reexports {
    use crate::{_reexport, _tags};

    _reexport! { rust: std::os::fd, location: "sys/os/fd", tag: _tags!(fs guard),
        doc: "An owned file descriptor.",
        @OwnedFd as FdOwned
    }
    _reexport! { rust: std::os::fd, location: "sys/os/fd", tag: _tags!(fs lifetime),
        doc: "A borrowed file descriptor.",
        @BorrowedFd as FdBorrowed
    }
    _reexport! { rust: std::os::fd, location: "sys/os/fd", tag: _tags!(fs),
        doc: "A trait to borrow the file descriptor from an underlying object.",
        AsFd
    }

    _reexport! { rust: std::os::fd, location: "sys/os/fd", tag: _tags!(fs uid),
        doc: "A trait to extract the raw file descriptor from an underlying object.",
        @AsRawFd as AsFdRaw
    }
    _reexport! { rust: std::os::fd, location: "sys/os/fd", tag: _tags!(fs uid value),
        doc: "Expresses the ability to construct an object from a raw file descriptor.",
        @FromRawFd as FromFdRaw
    }
    _reexport! { rust: std::os::fd, location: "sys/os/fd", tag: _tags!(fs uid value),
        doc: "Expresses the ability to consume an object and own its raw file descriptor.",
        @IntoRawFd as IntoFdRaw
    }
}
