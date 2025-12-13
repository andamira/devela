// devela::sys::os::linux::file::fcntl
//
//! File-descriptor control commands.
//!
//! Defines `fcntl` command constants used to query and modify file-descriptor state.
//!
//! Defines [`LINUX_F_CMD`].
//

#![allow(non_camel_case_types)]

use crate::c_int;

/// [`Linux`][crate::Linux] File descriptor commands (for `fcntl`).
//
// - /usr/include/asm-generic/fcntl.h
// - /usr/include/bits/fcntl-linux.h
//
// Possible arch-specific variations (F_GETLK, F_SETLK, F_SETLKW may have different numeric values)
// - /usr/include/asm/fcntl.h
#[derive(Debug)]
pub struct LINUX_F_CMD;
impl LINUX_F_CMD {
    /// Duplicate file descriptor.
    pub const DUPFD: c_int = 0;
    /// Get file descriptor flags.
    pub const GETFD: c_int = 1;
    /// Set file descriptor flags.
    pub const SETFD: c_int = 2;

    /// Get file status flags.
    pub const GETFL: c_int = 3;
    /// Set file status flags.
    pub const SETFL: c_int = 4;

    /// Get record locking info.
    pub const GETLK: c_int = 5;
    /// Set record locking info.
    pub const SETLK: c_int = 6;
    /// Set record locking wait.
    pub const SETLKW: c_int = 7;

    /// Duplicate FD with CLOSE_ON_EXEC.
    pub const DUPFD_CLOEXEC: c_int = 1024;

    /// Get pipe buffer size.
    pub const GETPIPE_SZ: c_int = 1032;
    /// Set pipe buffer size.
    pub const SETPIPE_SZ: c_int = 1031;

    /// Add seals to file.
    pub const ADD_SEALS: c_int = 1033;
    /// Get seals from file.
    pub const GET_SEALS: c_int = 1034;

    /// Get owner's process ID.
    pub const GETOWN: c_int = 9;
    /// Set owner's process ID.
    pub const SETOWN: c_int = 8;

    /// Get SIGIO/SIGURG signals.
    pub const GETSIG: c_int = 11;
    /// Set SIGIO/SIGURG signals.
    pub const SETSIG: c_int = 10;
}
