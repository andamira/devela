// devela::build::namespace
//
//! Defines the [`Build`] namespace.
//
// TOC
// - directories
// - print
// - other

use std::{
    env,
    fmt::{self, Display},
    path::PathBuf,
    process::{Command, Stdio},
};

#[doc = crate::_TAG_NAMESPACE!()] // NOTE: use tag directly to work from /build
/// Build-related operations.
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
#[derive(Debug)]
pub struct Build;

/// # Build environment
impl Build {
    /// Returns the current crate's name.
    pub fn crate_name() -> String {
        env::var("CARGO_CRATE_NAME").expect("CARGO_CRATE_NAME not set")
    }

    /// Retuns the path of `OUT_DIR`.
    pub fn out_dir() -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"))
    }
    /// Retuns the path of `CARGO_MANIFEST_DIR`.
    pub fn manifest_dir() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
    }
    /// Retuns the path of `CARGO_MANIFEST_PATH`.
    pub fn manifest_path() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_PATH").expect("CARGO_MANIFEST_PATH not set"))
    }
    /// Marks an environment variable as affecting build-script reruns.
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed
    pub fn rerun_if_env_changed(var: &str) {
        println!("cargo:rerun-if-env-changed={var}");
    }
}

/// # Print
#[rustfmt::skip]
impl Build {
    /// Prints a message to *stdout* from the build script.
    pub fn println(msg: impl Display) { println!("cargo:warning={}", msg); }

    /// Prints a heading message to *stdout*, underlined.
    pub fn println_heading(msg: &str) {
        Self::println("");
        Self::println(msg);
        Self::println("-".repeat(msg.len()));
    }
    /// Prints the value of an environment variable.
    pub fn println_var(var: &str) {
        if let Ok(v) = env::var(var) {
            Self::println(fmt::from_fn(|f| write!(f, "· {var}: {v}")));
        } else {
            Self::println(fmt::from_fn(|f| write!(f, "x {var}:")));
        }
    }
    /// Prints the value of an encoded environment variable,
    /// replacing unit separator characters `'0x1f'` with spaces.
    ///
    /// It accepts a new name to show for the decoded variable.
    pub fn println_var_encoded(var: &str, new_var_name: &str) {
        if let Ok(ev) = env::var(var) {
            let v = ev.replace('\x1f', " ");
            Self::println(format!["· {new_var_name}(*): {v}"]);
        } else {
            Self::println(format!["x {new_var_name}:"]);
        }
    }
    /// Prints the build script `start` or end message to *stdout*.
    pub fn println_start_end(name: &str, start: bool) {
        let msg = if start { format!("~ Start of {name} ~") }
            else { Self::println(""); format!("~ End of {name} ~") };
        let line = "~".repeat(msg.len());
        Self::println(&line); Self::println(&msg); Self::println(&line);
    }
}

/// # Emit `cfg` flags
#[rustfmt::skip]
impl Build {
    /// Emits a custom `cfg` flag for the current crate.
    pub fn emit_flag(flag: &str) { println!("cargo:rustc-cfg={flag}"); }
    /// Registers a custom `cfg` flag with rustc's checked-cfg lint.
    pub fn emit_check_cfg(flag: &str) { println!("cargo:rustc-check-cfg=cfg({flag})"); }
    /// Emits a custom `cfg` flag and registers it for checked cfg validation.
    pub fn emit_checked_flag(flag: &str) { Self::emit_check_cfg(flag); Self::emit_flag(flag); }
    /// Emits a custom `cfg` flag if `condition` is true. Returns whether the flag was emitted.
    pub fn emit_flag_if(flag: &str, condition: bool) -> bool {
        Self::emit_check_cfg(flag);
        if condition { Self::emit_flag(flag); }
        condition
    }
    /// Emits a compile-time environment variable for the current crate.
    pub fn emit_env(key: &str, value: impl Display) { println!("cargo:rustc-env={key}={value}"); }
    /// Emits a compile-time environment marker with an empty value.
    pub fn emit_env_marker(key: &str) { println!("cargo:rustc-env={key}="); }
}

/// # Native libraries
#[rustfmt::skip]
impl Build {
    /// Returns whether a native library is discoverable.
    ///
    /// Uses `pkg-config`/`pkgconf` first, then falls back to direct file search
    /// in common platform library paths.
    #[must_use]
    pub fn has_lib(lib: &str) -> bool { Self::lib_found_source(lib).is_some() }

    /// Emits `flag` if a native library is discoverable.
    ///
    /// The flag is emitted as-is. It may be a normal cfg flag or a devela
    /// reflective flag such as `ffi_xcb_shm··`.
    ///
    /// Returns whether the flag was emitted.
    #[must_use]
    pub fn emit_flag_if_lib(flag: &str, lib: &str) -> bool {
        Self::emit_check_cfg(flag);
        Self::rerun_if_lib_env_changed();
        let found = Self::lib_found_source(lib);
        if found.is_some() { Self::emit_flag(flag); }
        #[cfg(feature = "__dbg")]
        match &found { Some(source) =>
            Self::println(format!("· lib {lib}: found via {source}; emitted {flag}")),
            None => Self::println(format!("x lib {lib}: not found; skipped {flag}")),
        }
        found.is_some()
    }

    /* private helpers*/

    fn rerun_if_lib_env_changed() {
        for var in [ "PKG_CONFIG", "PKG_CONFIG_PATH", "PKG_CONFIG_LIBDIR",
            "PKG_CONFIG_SYSROOT_DIR", "LIBRARY_PATH", "LD_LIBRARY_PATH",
            "DYLD_LIBRARY_PATH", "LIB", "PATH", "CC", ] {
            Self::rerun_if_env_changed(var);
        }
    }
    fn lib_found_source(lib: &str) -> Option<String> {
        if Self::lib_found_pkg_config(lib) { return Some("pkg-config".to_string()); }
        Self::find_lib_file(lib).map(|path| path.display().to_string())
    }
    fn lib_found_pkg_config(lib: &str) -> bool {
        if let Some(cmd) = env::var_os("PKG_CONFIG") { return Self::pkg_config_exists(cmd, lib); }
        Self::pkg_config_exists("pkg-config", lib) || Self::pkg_config_exists("pkgconf", lib)
    }
    fn pkg_config_exists(cmd: impl AsRef<std::ffi::OsStr>, lib: &str) -> bool {
        Command::new(cmd).arg("--exists").arg(lib)
            .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null())
            .status().is_ok_and(|status| status.success())
    }
    fn find_lib_file(lib: &str) -> Option<PathBuf> {
        let filenames = Self::lib_filenames(lib);
        Self::lib_search_dirs().into_iter()
            .find_map(|dir| filenames.iter().map(|file| dir.join(file)).find(|path| path.is_file()))
    }
    fn lib_filenames(lib: &str) -> Vec<String> {
        let lib = lib.strip_prefix("lib").unwrap_or(lib);
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        match target_os.as_str() {
            "windows" => vec![format!("{lib}.lib"), format!("{lib}.dll"), format!("lib{lib}.a")],
            "macos" | "ios" => vec![format!("lib{lib}.dylib"), format!("lib{lib}.a")],
            _ => vec![format!("lib{lib}.so"), format!("lib{lib}.a")],
        }
    }
    fn lib_search_dirs() -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        for var in ["LIBRARY_PATH", "LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH", "LIB"] {
            if let Some(value) = env::var_os(var) { dirs.extend(env::split_paths(&value)); }
        }
        Self::push_cc_library_dirs(&mut dirs);
        Self::push_common_library_dirs(&mut dirs);
        dirs.sort(); dirs.dedup();
        dirs
    }
    fn push_cc_library_dirs(dirs: &mut Vec<PathBuf>) {
        let cc = env::var_os("CC").unwrap_or_else(|| "cc".into());
        let Ok(output) = Command::new(cc).arg("-print-search-dirs").output() else { return; };
        if !output.status.success() { return; }
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(paths) = line.strip_prefix("libraries: =") {
                dirs.extend(env::split_paths(paths));
            }
        }
    }
    fn push_common_library_dirs(dirs: &mut Vec<PathBuf>) {
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        match target_os.as_str() {
            "linux" | "freebsd" | "netbsd" | "openbsd" => {
                if let Some(multiarch) = Self::linux_multiarch_dir() {
                    dirs.push(format!("/usr/lib/{multiarch}").into());
                    dirs.push(format!("/lib/{multiarch}").into());
                }
                dirs.extend([
                    "/usr/local/lib".into(), "/usr/lib".into(), "/lib".into(),
                    "/usr/lib64".into(), "/lib64".into(), ]);
            }
            "macos" | "ios" => {
                dirs.extend([
                    "/usr/local/lib".into(), "/opt/homebrew/lib".into(), "/usr/lib".into(), ]);
            }
            "windows" => {
                if let Some(value) = env::var_os("PATH") { dirs.extend(env::split_paths(&value)); }
            }
            _ => {}
        }
    }
    fn linux_multiarch_dir() -> Option<&'static str> {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
        match arch.as_str() {
            "x86_64" => Some("x86_64-linux-gnu"),
            "x86" => Some("i386-linux-gnu"),
            "aarch64" => Some("aarch64-linux-gnu"),
            "arm" => Some("arm-linux-gnueabihf"),
            "riscv64" => Some("riscv64-linux-gnu"),
            "powerpc64" => Some("powerpc64-linux-gnu"),
            "s390x" => Some("s390x-linux-gnu"),
            _ => None,
        }
    }
}

// RETHINK IMPROVE
#[rustfmt::skip]
impl Build {
    /// `n` tabs, `n`*4 spaces.
    pub const fn tab(n: usize) -> &'static str {
        match n {
            0 => "",
            1 => "    ",
            2 => "        ",
            3 => "            ",
            4 => "                ",
            5 => "                    ",
            6 => "                        ",
            7 => "                            ",
            8 => "                                ",
            _ => "                                    ",
        }
    }
    #[doc = "0 tabs, 0 spaces."]  pub const TAB0: &str = "";
    #[doc = "1 tabs, 4 spaces."]  pub const TAB1: &str = "    ";
    #[doc = "2 tabs, 8 spaces."]  pub const TAB2: &str = "        ";
    #[doc = "3 tabs, 12 spaces."] pub const TAB3: &str = "            ";
    #[doc = "4 tabs, 16 spaces."] pub const TAB4: &str = "                ";
    #[doc = "5 tabs, 20 spaces."] pub const TAB5: &str = "                    ";
    #[doc = "6 tabs, 24 spaces."] pub const TAB6: &str = "                        ";
    #[doc = "7 tabs, 28 spaces."] pub const TAB7: &str = "                            ";
    #[doc = "8 tabs, 32 spaces."] pub const TAB8: &str = "                                ";
    #[doc = "9 tabs, 36 spaces."] pub const TAB9: &str = "                                    ";
}
