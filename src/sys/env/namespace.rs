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
    IoResult, IterArgs, IterArgsOs, IterSplitPaths, IterVars, IterVarsOs, JoinPathsError, OsStr,
    OsString, Path, PathBuf, VarError,
};

#[doc = crate::TAG_NAMESPACE!()]
/// A namespaced wrapper for `std::env` functions and constants.
pub struct Env;

/// # Constants
/*
NOTE: The lists of expected values for not(std) configuration flags can be copied from:
<https://github.com/rust-lang/rust/blob/master/tests/ui/check-cfg/well-known-values.stderr>.
Afterwards they can be procesed with vim commands similar to:
s/`//g
s/, /\r/g
s/\([a-z0-9_.]\+\)/#[cfg(target_arch = "\1")]\r{ "\1" }/
s/\([a-z0-9_.]\+\)/target_arch = "\1",/
*/
impl Env {
    #[doc = crate::TAG_MAYBE_STD!()]
    /// A string describing the architecture of the CPU that is currently in use.
    ///
    /// Expected values are:
    /// `aarch64`, `arm`, `arm64ec`, `avr`, `bpf`, `csky`, `hexagon`, `loongarch64`, `m68k`, `mips`, `mips32r6`, `mips64`, `mips64r6`, `msp430`, `nvptx64`, `powerpc`, `powerpc64`, `riscv32`, `riscv64`, `s390x`, `sparc`, `sparc64`, `unknown`, `wasm32`, `wasm64`, `x86`, `x86_64`, and `xtensa`.
    #[rustfmt::skip]
    pub const ARCH: &'static str = {
        #[cfg(feature = "std")]
        { ::std::env::consts::ARCH }
        #[cfg(not(feature = "std"))]
        { // 28 arches:
            #[cfg(target_arch = "aarch64")]
            { "aarch64" }
            #[cfg(target_arch = "amdgpu")]
            { "amdgpu" }
            #[cfg(target_arch = "arm")]
            { "arm" }
            #[cfg(target_arch = "arm64ec")]
            { "arm64ec" }
            #[cfg(target_arch = "avr")]
            { "avr" }
            #[cfg(target_arch = "bpf")]
            { "bpf" }
            #[cfg(target_arch = "csky")]
            { "csky" }
            #[cfg(target_arch = "hexagon")]
            { "hexagon" }
            #[cfg(target_arch = "loongarch64")]
            { "loongarch64" }
            #[cfg(target_arch = "m68k")]
            { "m68k" }
            #[cfg(target_arch = "mips")]
            { "mips" }
            #[cfg(target_arch = "mips32r6")]
            { "mips32r6" }
            #[cfg(target_arch = "mips64")]
            { "mips64" }
            #[cfg(target_arch = "mips64r6")]
            { "mips64r6" }
            #[cfg(target_arch = "msp430")]
            { "msp430" }
            #[cfg(target_arch = "nvptx64")]
            { "nvptx64" }
            #[cfg(target_arch = "powerpc")]
            { "powerpc" }
            #[cfg(target_arch = "powerpc64")]
            { "powerpc64" }
            #[cfg(target_arch = "riscv32")]
            { "riscv32" }
            #[cfg(target_arch = "riscv64")]
            { "riscv64" }
            #[cfg(target_arch = "s390x")]
            { "s390x" }
            #[cfg(target_arch = "sparc")]
            { "sparc" }
            #[cfg(target_arch = "sparc64")]
            { "sparc64" }
            #[cfg(target_arch = "wasm32")]
            { "wasm32" }
            #[cfg(target_arch = "wasm64")]
            { "wasm64" }
            #[cfg(target_arch = "x86")]
            { "x86" }
            #[cfg(target_arch = "x86_64")]
            { "x86_64" }
            #[cfg(target_arch = "xtensa")]
            { "xtensa" }
            #[cfg(not(any(
                target_arch = "aarch64",
                target_arch = "amdgpu",
                target_arch = "arm",
                target_arch = "arm64ec",
                target_arch = "avr",
                target_arch = "bpf",
                target_arch = "csky",
                target_arch = "hexagon",
                target_arch = "loongarch64",
                target_arch = "m68k",
                target_arch = "mips",
                target_arch = "mips32r6",
                target_arch = "mips64",
                target_arch = "mips64r6",
                target_arch = "msp430",
                target_arch = "nvptx64",
                target_arch = "powerpc",
                target_arch = "powerpc64",
                target_arch = "riscv32",
                target_arch = "riscv64",
                target_arch = "s390x",
                target_arch = "sparc",
                target_arch = "sparc64",
                target_arch = "wasm32",
                target_arch = "wasm64",
                target_arch = "x86",
                target_arch = "x86_64",
                target_arch = "xtensa",
            )))]
            { "unknown" }
        }
    };

    #[doc = crate::TAG_MAYBE_STD!()]
    /// The family of the operating system.
    ///
    /// The expected values are: `unix`, `unknown`, `wasm` and `windows`.
    #[rustfmt::skip]
    pub const FAMILY: &'static str = {
        #[cfg(feature = "std")]
        { ::std::env::consts::FAMILY }
        #[cfg(not(feature = "std"))]
        {
            #[cfg(target_family = "unix")]
            { "unix" }
            #[cfg(target_family = "wasm")]
            { "wasm" }
            #[cfg(target_family = "windows")]
            { "windows" }
            #[cfg(not(any(
                target_family = "unix",
                target_family = "wasm",
                target_family = "windows",
            )))]
            { "unknown" }
        }
    };

    /// A string describing the vendor of the CPU that is currently in use.
    ///
    /// Expected values are:
    ///  `amd`, `apple`, `espressif`, `fortanix`, `ibm`, `kmc`, `mti`, `nintendo`, `nvidia`, `openwrt`, `pc`, `risc0`, `sony`, `sun`, `unikraft`, `unknown`, `uwp`, `vex`, `win7`, and `wrs`.
    #[rustfmt::skip]
    pub const VENDOR: &'static str = { // 19 vendors + unknown
        #[cfg(target_vendor = "amd")]
        { "amd" }
        #[cfg(target_vendor = "apple")]
        { "apple" }
        #[cfg(target_vendor = "espressif")]
        { "espressif" }
        #[cfg(target_vendor = "fortanix")]
        { "fortanix" }
        #[cfg(target_vendor = "ibm")]
        { "ibm" }
        #[cfg(target_vendor = "kmc")]
        { "kmc" }
        #[cfg(target_vendor = "mti")]
        { "mti" }
        #[cfg(target_vendor = "nintendo")]
        { "nintendo" }
        #[cfg(target_vendor = "nvidia")]
        { "nvidia" }
        #[cfg(target_vendor = "openwrt")]
        { "openwrt" }
        #[cfg(target_vendor = "pc")]
        { "pc" }
        #[cfg(target_vendor = "risc0")]
        { "risc0" }
        #[cfg(target_vendor = "sony")]
        { "sony" }
        #[cfg(target_vendor = "sun")]
        { "sun" }
        #[cfg(target_vendor = "unikraft")]
        { "unikraft" }
        #[cfg(target_vendor = "uwp")]
        { "uwp" }
        #[cfg(target_vendor = "vex")]
        { "vex" } // armv7a-vex-v5
        #[cfg(target_vendor = "win7")]
        { "win7" }
        #[cfg(target_vendor = "wrs")]
        { "wrs" }
        #[cfg(not(any(
            target_vendor = "amd",
            target_vendor = "apple",
            target_vendor = "espressif",
            target_vendor = "fortanix",
            target_vendor = "ibm",
            target_vendor = "kmc",
            target_vendor = "mti",
            target_vendor = "nintendo",
            target_vendor = "nvidia",
            target_vendor = "openwrt",
            target_vendor = "pc",
            target_vendor = "risc0",
            target_vendor = "sony",
            target_vendor = "sun",
            target_vendor = "unikraft",
            target_vendor = "uwp",
            target_vendor = "vex",
            target_vendor = "win7",
            target_vendor = "wrs",
        )))]
        { "unknown" }
    };

    #[doc = crate::TAG_MAYBE_STD!()]
    /// A string describing the specific operating system in use.
    ///
    /// The expected values are:
    /// `aix`, `android`, `cuda`, `cygwin`, `dragonfly`, `emscripten`, `espidf`, `freebsd`, `fuchsia`, `haiku`, `hermit`, `horizon`, `hurd`, `illumos`, `ios`, `l4re`, `linux`, `macos`, `netbsd`, `none`, `nto`, `nuttx`, `openbsd`, `psp`, `psx`, `redox`, `rtems`, `solaris`, `solid_asp3`, `teeos`, `trusty`, `tvos`, `uefi`, `unknown`, `visionos`, `vita`, `vxworks`, `wasi`, `watchos`, `windows`, `xous`, and `zkvm`.
    #[rustfmt::skip]
    pub const OS: &'static str = {
        #[cfg(feature = "std")]
        { ::std::env::consts::OS }
        #[cfg(not(feature = "std"))]
        { // 42 oses:
            #[cfg(target_os = "aix")]
            { "aix" }
            #[cfg(target_os = "amdhsa")]
            { "amdhsa" }
            #[cfg(target_os = "android")]
            { "android" }
            #[cfg(target_os = "cuda")]
            { "cuda" }
            #[cfg(target_os = "cygwin")]
            { "cygwin" }
            #[cfg(target_os = "dragonfly")]
            { "dragonfly" }
            #[cfg(target_os = "emscripten")]
            { "emscripten" }
            #[cfg(target_os = "espidf")]
            { "espidf" }
            #[cfg(target_os = "freebsd")]
            { "freebsd" }
            #[cfg(target_os = "fuchsia")]
            { "fuchsia" }
            #[cfg(target_os = "haiku")]
            { "haiku" }
            #[cfg(target_os = "hermit")]
            { "hermit" }
            #[cfg(target_os = "horizon")]
            { "horizon" }
            #[cfg(target_os = "hurd")]
            { "hurd" }
            #[cfg(target_os = "illumos")]
            { "illumos" }
            #[cfg(target_os = "ios")]
            { "ios" }
            #[cfg(target_os = "l4re")]
            { "l4re" }
            #[cfg(target_os = "linux")]
            { "linux" }
            #[cfg(target_os = "macos")]
            { "macos" }
            #[cfg(target_os = "netbsd")]
            { "netbsd" }
            #[cfg(target_os = "none")]
            { "none" }
            #[cfg(target_os = "nto")]
            { "nto" }
            #[cfg(target_os = "nuttx")]
            { "nuttx" }
            #[cfg(target_os = "openbsd")]
            { "openbsd" }
            #[cfg(target_os = "psp")]
            { "psp" }
            #[cfg(target_os = "psx")]
            { "psx" }
            #[cfg(target_os = "redox")]
            { "redox" }
            #[cfg(target_os = "rtems")]
            { "rtems" }
            #[cfg(target_os = "solaris")]
            { "solaris" }
            #[cfg(target_os = "solid_asp3")]
            { "solid_asp3" }
            #[cfg(target_os = "teeos")]
            { "teeos" }
            #[cfg(target_os = "trusty")]
            { "trusty" }
            #[cfg(target_os = "tvos")]
            { "tvos" }
            #[cfg(target_os = "uefi")]
            { "uefi" }
            #[cfg(target_os = "visionos")]
            { "visionos" }
            #[cfg(target_os = "vita")]
            { "vita" }
            #[cfg(target_os = "vxworks")]
            { "vxworks" }
            #[cfg(target_os = "wasi")]
            { "wasi" }
            #[cfg(target_os = "watchos")]
            { "watchos" }
            #[cfg(target_os = "windows")]
            { "windows" }
            #[cfg(target_os = "xous")]
            { "xous" }
            #[cfg(target_os = "zkvm")]
            { "zkvm" }
            #[cfg(not(any(
                target_os = "aix",
                target_os = "amdhsa",
                target_os = "android",
                target_os = "cuda",
                target_os = "cygwin",
                target_os = "dragonfly",
                target_os = "emscripten",
                target_os = "espidf",
                target_os = "freebsd",
                target_os = "fuchsia",
                target_os = "haiku",
                target_os = "hermit",
                target_os = "horizon",
                target_os = "hurd",
                target_os = "illumos",
                target_os = "ios",
                target_os = "l4re",
                target_os = "linux",
                target_os = "macos",
                target_os = "netbsd",
                target_os = "none",
                target_os = "nto",
                target_os = "nuttx",
                target_os = "openbsd",
                target_os = "psp",
                target_os = "psx",
                target_os = "redox",
                target_os = "rtems",
                target_os = "solaris",
                target_os = "solid_asp3",
                target_os = "teeos",
                target_os = "trusty",
                target_os = "tvos",
                target_os = "uefi",
                target_os = "visionos",
                target_os = "vita",
                target_os = "vxworks",
                target_os = "wasi",
                target_os = "watchos",
                target_os = "windows",
                target_os = "xous",
                target_os = "zkvm",
            )))]
            { "unknown" }
        }
    };

    /// Specifies the file extension used for shared libraries on this platform
    /// that goes after the dot.
    ///
    /// Some possible values:
    /// - `so`
    /// - `dylib`
    /// - `dll`
    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    pub const DLL_EXTENSION: &'static str = ::std::env::consts::DLL_EXTENSION;

    /// Specifies the filename prefix used for shared libraries on this platform.
    ///
    /// Some possible values:
    /// - `lib`
    /// - `` (an empty string)
    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    pub const DLL_PREFIX: &'static str = ::std::env::consts::DLL_PREFIX;

    /// Specifies the filename suffix used for shared libraries on this platform.
    ///
    /// Some possible values:
    /// - `.so`
    /// - `.dylib`
    /// - `.dll`
    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    pub const DLL_SUFFIX: &'static str = ::std::env::consts::DLL_SUFFIX;

    /// Specifies the file extension, if any, used for executable binaries on this platform.
    ///
    /// Some possible values:
    /// - `exe`
    /// - `` (an empty string)
    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    pub const EXE_EXTENSION: &'static str = ::std::env::consts::EXE_EXTENSION;

    /// Specifies the filename suffix used for executable binaries on this platform.
    ///
    /// Some possible values:
    /// - `.exe`
    /// - `.nexe`
    /// - `.pexe`
    /// - `` (an empty string)
    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    pub const EXE_SUFFIX: &'static str = ::std::env::consts::EXE_SUFFIX;
}

/// # Command line arguments
#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
impl Env {
    /// Returns the arguments that this program was started with.
    ///
    /// See [args].
    pub fn args() -> IterArgs {
        args()
    }

    /// See [args_os].
    pub fn args_os() -> IterArgsOs {
        args_os()
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

    /// Returns the path of the current user’s home directory if known.
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
