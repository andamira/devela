// devela::sys::env::env
//
//! Defines the [`Env`] struct namespace.
//
// TOC
// - Constants
// - Command line arguments
// - Environment variables
// - Paths

#[cfg(all(feature = "std", feature = "unsafe_thread"))]
use crate::_dep::_std::env::{remove_var, set_var};
#[allow(deprecated, reason = "WAIT for official undeprecation of home_dir")]
#[cfg(feature = "std")]
use crate::{
    _dep::_std::env::{
        args, args_os, current_dir, current_exe, home_dir, join_paths, set_current_dir,
        split_paths, temp_dir, var, var_os, vars, vars_os,
    },
    IoResult, JoinPathsError, OsStr, OsString, Path, PathBuf, VarError,
};
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(feature = "std", feature = "unsafe_ffi"))]
use crate::{IterArgsOsRef, args_os_ref_iter};
#[cfg(feature = "std")]
use ::std::env::{
    Args as IterArgs, ArgsOs as IterArgsOs, SplitPaths as IterSplitPaths, Vars as IterVars,
    VarsOs as IterVarsOs,
};

#[doc = crate::_tags!(namespace)]
/// A namespaced wrapper for `std::env` functions and constants.
#[doc = crate::_doc_meta!{location("sys/os")}]
#[derive(Debug)]
pub struct Env;

/// # Constants
// In sync with: /build/main/environment.rs
impl Env {
    /// The full Rust target triple.
    ///
    /// Mirrors Cargo's `TARGET` build-script environment variable.
    pub const TARGET: &str = env!("DEVELA_TARGET");

    /// The Rust `target_arch` value for the current compilation target.
    ///
    /// Mirrors `CARGO_CFG_TARGET_ARCH`.
    ///
    /// Example values include `"x86_64"`, `"aarch64"`, `"riscv64"`, and `"wasm32"`.
    ///
    /// See also [`std::env::consts::ARCH`].
    ///
    /// [`std::env::consts::ARCH`]: https://doc.rust-lang.org/std/env/consts/constant.ARCH.html
    pub const ARCH: &str = env!("DEVELA_TARGET_ARCH");

    /// The specific operating system in use.
    ///
    /// Mirrors `CARGO_CFG_TARGET_OS`.
    ///
    /// Example values include `"linux"`, `"windows"`, `"macos"`, and `"none"`.
    ///
    /// See also [`std::env::consts::OS`].
    ///
    /// [`std::env::consts::OS`]: https://doc.rust-lang.org/std/env/consts/constant.OS.html
    pub const OS: &str = env!("DEVELA_TARGET_OS");

    /// The target vendor.
    ///
    /// Mirrors `CARGO_CFG_TARGET_VENDOR`.
    ///
    /// Common values include `"pc"`, `"apple"`, and `"unknown"`.
    pub const VENDOR: &str = env!("DEVELA_TARGET_VENDOR");

    /// The primary target family.
    ///
    /// Usually `"unix"`, `"windows"`, `"wasm"`, `"itron"`, or `""`.
    ///
    /// For all target families, see [`Self::FAMILIES`].
    ///
    /// See also [`std::env::consts::FAMILY`].
    ///
    /// [`std::env::consts::FAMILY`]: https://doc.rust-lang.org/std/env/consts/constant.FAMILY.html
    pub const FAMILY: &str = env!("DEVELA_TARGET_FAMILY");

    /// All target families, comma-separated.
    ///
    /// Mirrors `CARGO_CFG_TARGET_FAMILY`.
    ///
    /// This can contain more than one value.
    pub const FAMILIES: &str = env!("DEVELA_TARGET_FAMILIES");

    /// The target environment/disambiguator.
    ///
    /// Mirrors `CARGO_CFG_TARGET_ENV`.
    ///
    /// Usually `""`, `"gnu"`, `"msvc"`, or `"musl"`.
    pub const ENV: &str = env!("DEVELA_TARGET_ENV");

    /// The target ABI/disambiguator.
    ///
    /// Mirrors `CARGO_CFG_TARGET_ABI`.
    ///
    /// Usually `""`; examples include `"eabihf"`, `"llvm"`, and `"abi64"`.
    pub const ABI: &str = env!("DEVELA_TARGET_ABI");

    /// The target endianness.
    ///
    /// Usually `"little"` or `"big"`.
    pub const ENDIAN: &str = env!("DEVELA_TARGET_ENDIAN");

    /// The target pointer width, in bits.
    ///
    /// Usually `"16"`, `"32"`, or `"64"`.
    pub const POINTER_WIDTH: &str = env!("DEVELA_TARGET_POINTER_WIDTH");

    /// The filename prefix used for shared libraries on this platform.
    ///
    /// Usually `"lib"` or `""`.
    ///
    /// See also [`std::env::consts::DLL_PREFIX`].
    ///
    /// [`std::env::consts::DLL_PREFIX`]: https://doc.rust-lang.org/std/env/consts/constant.DLL_PREFIX.html
    pub const DLL_PREFIX: &str = env!("DEVELA_DLL_PREFIX");

    /// The file extension used for shared libraries on this platform, without the dot.
    ///
    /// Usually `"so"`, `"dylib"`, `"dll"`, or `""`.
    ///
    /// See also [`std::env::consts::DLL_EXTENSION`].
    ///
    /// [`std::env::consts::DLL_EXTENSION`]: https://doc.rust-lang.org/std/env/consts/constant.DLL_EXTENSION.html
    pub const DLL_EXTENSION: &str = env!("DEVELA_DLL_EXTENSION");

    /// The filename suffix used for shared libraries on this platform.
    ///
    /// Usually `".so"`, `".dylib"`, `".dll"`, or `""`.
    ///
    /// See also [`std::env::consts::DLL_SUFFIX`].
    ///
    /// [`std::env::consts::DLL_SUFFIX`]: https://doc.rust-lang.org/std/env/consts/constant.DLL_SUFFIX.html
    pub const DLL_SUFFIX: &str = env!("DEVELA_DLL_SUFFIX");

    /// The file extension used for executable binaries on this platform, without the dot.
    ///
    /// Usually `"exe"` or `""`.
    ///
    /// Other target-specific values include `"bin"`, `"elf"`, `"js"`, and `"wasm"`.
    ///
    /// See also [`std::env::consts::EXE_EXTENSION`].
    ///
    /// [`std::env::consts::EXE_EXTENSION`]: https://doc.rust-lang.org/std/env/consts/constant.EXE_EXTENSION.html
    pub const EXE_EXTENSION: &str = env!("DEVELA_EXE_EXTENSION");

    /// The filename suffix used for executable binaries on this platform.
    ///
    /// Usually `".exe"` or `""`.
    ///
    /// This matches [`Self::EXE_EXTENSION`] with a leading dot when non-empty.
    ///
    /// See also [`std::env::consts::EXE_SUFFIX`].
    ///
    /// [`std::env::consts::EXE_SUFFIX`]: https://doc.rust-lang.org/std/env/consts/constant.EXE_SUFFIX.html
    pub const EXE_SUFFIX: &str = env!("DEVELA_EXE_SUFFIX");
}

/// # Command line arguments
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl Env {
    /// Returns the arguments that this program was started with.
    ///
    /// See [args].
    #[inline(always)]
    pub fn args() -> IterArgs {
        args()
    }

    /// See [args_os].
    #[inline(always)]
    pub fn args_os() -> IterArgsOs {
        args_os()
    }

    #[doc = crate::_doc_warn_miri!(tag)]
    #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
    /// Command line arguments by reference: `Iterator<Item = &'static OsStr>`.
    #[doc = crate::_doc_vendor!("argv")]
    #[cfg(all(feature = "std", feature = "unsafe_ffi"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_ffi"))))]
    pub fn args_os_ref() -> IterArgsOsRef {
        args_os_ref_iter()
    }
}

/// # Environment variables
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
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
    pub fn vars() -> IterVars {
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
    pub fn vars_os() -> IterVarsOs {
        vars_os()
    }

    /// Removes the environment variable `key` from the environment
    /// of the currently running process.
    ///
    /// # Safety
    /// See [remove_var].
    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_thread"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_thread")))]
    pub unsafe fn remove_var<K: AsRef<OsStr>>(key: K) {
        unsafe { remove_var(key) }
    }

    /// Sets the environment variable `key` to the value `value`
    /// for the currently running process.
    ///
    /// # Safety
    /// See [set_var].
    #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_thread"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_thread")))]
    pub unsafe fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
        unsafe { set_var(key, value) }
    }
}

/// # Paths
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
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

    /// Returns the path of the current user's home directory if known.
    ///
    /// See [home_dir].
    #[allow(deprecated, reason = "WAIT for official undeprecation")]
    pub fn home_dir() -> Option<PathBuf> {
        home_dir()
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
    pub fn split_paths<T: AsRef<OsStr> + ?Sized>(unparsed: &T) -> IterSplitPaths<'_> {
        split_paths(unparsed)
    }
}
