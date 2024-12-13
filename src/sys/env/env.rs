// devela::sys::env::env
//
//! A namespaced wrapper for std
//

use crate::IoResult;
use std::{
    env::*,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

/// A namespaced wrapper for `std::env` functions and constants.
pub struct Env;

impl Env {}

/// # Functions related to command line arguments
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
impl Env {
    /// Returns the arguments that this program was started with.
    ///
    /// See [args].
    pub fn args() -> Args {
        args()
    }

    /// See [args_os].
    pub fn args_os() -> ArgsOs {
        args_os()
    }
}

/// # Functions related to environment variables
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
impl Env {
    /// Fetches the environment variable key from the current process.
    ///
    /// See [var].
    pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String, VarError> {
        var(key)
    }

    /// Returns an iterator of (variable, value) pairs of strings,
    /// for all the environment variables of the current process.
    ///
    /// See [vars].
    pub fn vars() -> Vars {
        vars()
    }

    /// Fetches the environment variable key from the current process.
    ///
    /// See [var_os].
    pub fn var_os<K: AsRef<OsStr>>(key: K) -> Option<OsString> {
        var_os(key)
    }

    /// Returns an iterator of (variable, value) pairs of OS strings,
    /// for all the environment variables of the current process.
    ///
    /// See [vars_os].
    pub fn vars_os() -> VarsOs {
        vars_os()
    }

    /// Removes the environment variable `key` from the environment
    /// of the currently running process.
    ///
    /// # Safety
    /// See [remove_var].
    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_thread"))]
    pub unsafe fn remove_var<K: AsRef<OsStr>>(key: K) {
        unsafe { remove_var(key) }
    }

    /// Sets the environment variable `key` to the value `value`
    /// for the currently running process.
    ///
    /// # Safety
    /// See [set_var].
    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_thread"))]
    pub unsafe fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
        unsafe { set_var(key, value) }
    }
}

/// # Functions related to paths.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
impl Env {
    /// Returns the full filesystem path of the current running executable.
    ///
    /// See [current_exe].
    pub fn current_exe() -> IoResult<PathBuf> {
        current_exe()
    }

    /// Returns the current working directory.
    ///
    /// See [current_dir].
    pub fn current_dir() -> IoResult<PathBuf> {
        current_dir()
    }

    /// Changes the current working directory to the specified path.
    ///
    /// See [set_current_dir].
    pub fn set_current_dir<P: AsRef<Path>>(path: P) -> IoResult<()> {
        set_current_dir(path)
    }

    /// Returns the path of a temporary directory.
    ///
    /// See [temp_dir].
    pub fn temp_dir() -> PathBuf {
        temp_dir()
    }

    /// Joins a collection of [Path]s appropriately for the `PATH` environment variable.
    ///
    /// See [join_paths].
    pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<OsStr>,
    {
        join_paths(paths)
    }

    /// Parses input according to platform conventions for the `PATH` environment variable.
    ///
    /// See [split_paths].
    pub fn split_paths<T: AsRef<OsStr> + ?Sized>(unparsed: &T) -> SplitPaths<'_> {
        split_paths(unparsed)
    }
}

/// # Constants
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
impl Env {
    /// A string describing the architecture of the CPU that is currently in use.
    ///
    /// Some possible values:
    /// - x86
    /// - x86_64
    /// - arm
    /// - aarch64
    /// - loongarch64
    /// - m68k
    /// - csky
    /// - mips
    /// - mips64
    /// - powerpc
    /// - powerpc64
    /// - riscv64
    /// - s390x
    /// - sparc64
    pub const ARCH: &'static str = std::env::consts::ARCH;

    /// Specifies the file extension used for shared libraries on this platform
    /// that goes after the dot.
    ///
    /// Some possible values:
    /// - so
    /// - dylib
    /// - dll
    pub const DLL_EXTENSION: &'static str = std::env::consts::DLL_EXTENSION;

    /// Specifies the filename prefix used for shared libraries on this platform.
    ///
    /// Some possible values:
    /// - lib
    /// - "" (an empty string)
    pub const DLL_PREFIX: &'static str = std::env::consts::DLL_PREFIX;

    /// Specifies the filename suffix used for shared libraries on this platform.
    ///
    /// Some possible values:
    /// - .so
    /// - .dylib
    /// - .dll
    pub const DLL_SUFFIX: &'static str = std::env::consts::DLL_SUFFIX;

    /// Specifies the file extension, if any, used for executable binaries on this platform.
    ///
    /// Some possible values:
    /// - exe
    /// - "" (an empty string)
    pub const EXE_EXTENSION: &'static str = std::env::consts::EXE_EXTENSION;

    /// Specifies the filename suffix used for executable binaries on this platform.
    ///
    /// Some possible values:
    /// - .exe
    /// - .nexe
    /// - .pexe
    /// - "" (an empty string)
    pub const EXE_SUFFIX: &'static str = std::env::consts::EXE_SUFFIX;

    /// The family of the operating system.
    ///
    /// Some possible values:
    /// - unix
    /// - windows
    /// - wasm
    pub const FAMILY: &'static str = std::env::consts::FAMILY;

    /// A string describing the specific operating system in use.
    ///
    /// Some possible values:
    /// - linux
    /// - macos
    /// - ios
    /// - freebsd
    /// - dragonfly
    /// - netbsd
    /// - openbsd
    /// - solaris
    /// - android
    /// - windows
    pub const OS: &'static str = std::env::consts::OS;
}
