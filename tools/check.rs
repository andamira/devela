#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [dependencies]
//! devela = { version = "0.23.0", features = ["std"] }
//! lexopt = "0.3"
//! itertools = "0.14"
//! toml_edit = "0.23"
//! ```
// devela::tools::check
//
//!
//
// NOTE: needs [rust-script](https://crates.io/crates/rust-script) to run.
// NOTE: run clippy as follow:
// cargo clippy --manifest-path (rust-script --package tools/check.rs)/Cargo.toml
//
// TOC
// - config:
//   - const root_modules
//   - const dep_all
//   - const dep_no_cross_compile_std
//   - const dep_no_cross_compile_ever
//   - const std_arches_no_cross_compile
//   - const std_arches
//   - const no_std_arches
//   - const linux_arches
//
// - fn main:
//   - tests
//   - docs
//   - arches
//   - miri
//   - modules
//
// - helpers:
//   - struct Args
//   - fn get_args
//   - fn print_help
//   - fn get_msrv
//   - fn run_cargo
//   - fn rust_install_arches
//   - fn rust_install_nightly
//   - fn headline
//   - fn is_current_host_compatible
//   - fn filter_deps
//
// IMPROVE:
// - filter-out cross-compiling only for no_std platforms.
// - allow to associate a platform with features to be enabled.
// - avoid downloading everything at the start make a separate command. use it in CI.

#![allow(clippy::useless_format)]

use devela::all::{CONST, FsPath, is, sf};
use itertools::Itertools;
use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader, Read},
    process::{Command, Stdio, exit},
    thread,
};
use toml_edit::Document;

/* config */

CONST![NIGHTLY = "nightly-2025-08-14"]; // NOTE: 2025-08-15 fails
const NIGHTLY_VERSION: &str = NIGHTLY![];
const NIGHTLY_TOOLCHAIN: &str = concat!["+", NIGHTLY![]];

#[rustfmt::skip]
const ROOT_MODULES: [&str; 11 + 1] = [
    "code", "data", "game", "lang", "media", "num", "phys", "sys", "text", "ui", "work",

    // sys::os submodules (platforms)
    "linux", // "windows",
];
#[rustfmt::skip]
const SUB_MODULES: &[&str] = &[
    // code
    "error",
    // data
    "hash",
    // game
    // lang
    "glsl", "js",
    // media
    "audio", "color", "draw", "font", "image", "midi", "video",
    // num
    "geom",
        "linear", "metric", "shape",
    "prim",
        "cast", "join", "split",
    "rand",
    "unit",
    // phys
    "time",
    "wave",
    // sys
    "io",
    "mem",
        "bit",
    // text
    "ascii", "fmt", "str",
    // ui
    "layout",
    /* front */
        "desk", "term", "web",
    // work
    "process", "sync", "thread",
];

//* dependencies *//

#[rustfmt::skip]
/// All the optional external dependencies.
const DEP_ALL: &[&str] = &include!["../config/dep_all.rs"];

/// All the optional library-modular dependencies.
#[expect(unused, reason = "TODO")]
const LIB_ALL: &[&str] = &include!["../config/lib_all.rs"];

#[rustfmt::skip]
/// Dependencies to not cross compile in arches in STD_ARCHES_NO_CROSS_COMPILE.
const DEP_NO_CROSS_COMPILE_STD: &[&str] = &[
    "dep_midir", "dep_rodio", "dep_tinyaudio", // REASON: alsa-sys
    "dep_kira", // REASON: alsa-sys (feature `cpal`)
];

/// Dependencies to not cross compile, ever.
const DEP_NO_CROSS_COMPILE_EVER: &[&str] = &[
    // IMPROVE: allow activating `windows` feature
    "dep_crossterm",
    // (fails on miri on windows)
    // https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building#building-on-windows
    // https://github.com/shssoichiro/ffmpeg-the-third/blob/master/.github/workflows/build.yml#L132
    "dep_ffmpeg",
    // because of pkg-config & libudev-sys
    // SEE: https://gitlab.com/gilrs-project/gilrs/-/issues/86
    "dep_gilrs",
    // IMPROVE: https://pyo3.rs/v0.23.2/building-and-distribution.html#cross-compiling
    "dep_pyo3",
    // - https://docs.rs/safe_arch/latest/safe_arch/#current-support
    "dep_safe_arch",
    // (experimental)
    "dep_sdl3",
    // WAIT: [x86_64-pc-windows-msvc](https://github.com/ashvardanian/StringZilla/pull/169)
    "dep_stringzilla",
    //
    // DEP_DISABLED:
    // // (cross-compile to many platforms fails)
    // // - [to darwin from linux](https://github.com/briansmith/ring/issues/1442)
    // // - [to windows from linux](https://github.com/briansmith/ring/issues/894)
    // "dep_ring",
];

#[rustfmt::skip]
/// Dependencies to not include in minimal-versions test.
// - https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#minimal-versions
// - https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#direct-minimal-versions
// - [minimal-versions](https://github.com/rust-lang/cargo/issues/5657)
const DEP_NO_MINIMAL_VERSIONS: &[&str] = &[
    // Using libudev-sys which uses pkc-config 3.2 from (2015-05-25)
    "dep_gilrs",
    // WAIT: [minimal-versions](https://github.com/diwic/alsa-sys/issues/14)
    // WAIT: [minimal-versions](https://github.com/rust-num/num-rational/issues/133)
    "dep_midir", "dep_rodio", "dep_tinyaudio", "dep_kira", // REASON: alsa-sys
    // WAIT: [minimal-versions](https://github.com/PyO3/pyo3/issues/4877)
    "dep_pyo3",
];

//* cross-compilation targets *//

const STD_ARCHES: &[&str] = &[
    // Linux 64-bit
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools
    "x86_64-unknown-linux-gnu",
    //
    // Windows Cygwin 64-bit
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools
    "x86_64-pc-windows-gnu",
];
const STD_ARCHES_NO_CROSS_COMPILE: &[&str] = &[
    // Linux i686, 32-bit, std, little-endian, (kernel 3.2+, glibc 2.17+)
    // may need to install `libc6-dev-amd64-i386-cross` for testing
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools
    "i686-unknown-linux-gnu",
    //
    // MacOs 64-bit
    // https://doc.rust-lang.org/rustc/platform-support/x86_64h-apple-darwin.html
    // NOTE: Build fails cross-compiling the coreaudio-sys transitive dependency.
    "x86_64-apple-darwin",
    //
    // https://doc.rust-lang.org/nightly/rustc/platform-support/riscv64gc-unknown-linux-gnu.html
    "riscv64gc-unknown-linux-gnu",
];
const NO_STD_ARCHES: &[&str] = &[
    // Bare x86_64, softfloat, 64-bit, no_std
    // https://doc.rust-lang.org/nightly/rustc/platform-support/x86_64-unknown-none.html
    "x86_64-unknown-none",
    //
    // Bare ARM64, hardfloat, 64-bit, no_std, little-endian, A64 set, (M1, M2 processors)
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-2-without-host-tools
    "aarch64-unknown-none",
    //
    // Bare ARMv7-M, 32-bit, no_std, little-endian, Thumb set, (Cortex-M processors)
    // https://doc.rust-lang.org/rustc/platform-support/thumbv7m-none-eabi.html
    "thumbv7m-none-eabi",
    //
    // https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html
    "wasm32-wasip1",
    //
    // https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1-threads.html
    // "wasm32-wasip1-threads",
    //
    // https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip2.html
    // "wasm32-wasip2",
    //
    // https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
    "wasm32-unknown-unknown",
];
// For testing sys::linux
const LINUX_ARCHES: &[&str] = &[
    //* Tier 1 (+ tier 2 for musl) *
    //------------------------------
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools
    //
    // Linux 64-bit
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools
    "x86_64-unknown-linux-gnu",
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-2-with-host-tools
    "x86_64-unknown-linux-musl",
    //
    // Linux i686, 32-bit, std, little-endian, (kernel 3.2+, glibc 2.17+)
    // may need to install `libc6-dev-amd64-i386-cross` for testing
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools
    "i686-unknown-linux-gnu",
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-2-without-host-tools
    "i686-unknown-linux-musl",
    //
    // ARM64 Linux (kernel 4.1, glibc 2.17+)
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools
    "aarch64-unknown-linux-gnu",
    // https://doc.rust-lang.org/rustc/platform-support.html#tier-2-with-host-tools
    "aarch64-unknown-linux-musl",
    //
    //* Tier 2 with host tools *
    //--------------------------
    // https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2-with-host-tools
    //
    // https://doc.rust-lang.org/nightly/rustc/platform-support/riscv64gc-unknown-linux-gnu.html
    "riscv64gc-unknown-linux-gnu",
    //
    "arm-unknown-linux-gnueabihf",
    //
    "armv7-unknown-linux-gnueabihf",
    //
    //* Tier 3 *
    //--------------------------
    // https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-3
    //
    // WAIT?: https://github.com/rust-lang/rust/issues/88995
    // "riscv32gc-unknown-linux-gnu", // not available
];

type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

/// Main logic.
fn main() -> Result<()> {
    let args = get_args()?;
    let msrv = is![args.no_msrv; "".into(); get_msrv().unwrap_or("".into())];

    let cmd = "check";

    if args.install_arches {
        rust_install_arches(&msrv)?;
    }
    if args.install_nightly {
        rust_install_nightly()?;
    }

    /* tests */

    if args.tests {
        let cmd = "test";
        sf! { headline(0, &format!["`all` test checking [alloc|std] + [safe|unsafe]):"]); }

        // WAIT: https://github.com/rust-lang/cargo/issues/1983 (colored output)

        // nightly unsafe
        sf! { run_cargo_with_env("", NIGHTLY_TOOLCHAIN, &[cmd, "-F _docsrs", "--", "--color=always"],
        &[("RUSTFLAGS", "--cfg nightly")])?; }

        // std (un)safe (max capabilities)
        run_cargo(&msrv, cmd, &["-F all,std,safe,_docs,_max", "--", "--color=always"])?;
        run_cargo(&msrv, cmd, &["-F all,std,unsafe,_docs,_max", "--", "--color=always"])?;

        // std (un)safe + dep_all
        sf! { run_cargo(&msrv, cmd, &["-F all,std,safe,dep_all", "--", "--color=always"])?; }
        sf! { run_cargo(&msrv, cmd, &["-F all,std,unsafe,dep_all", "--", "--color=always"])?; }

        // alloc (un)safe
        sf! { run_cargo(&msrv, cmd, &["-F all,alloc,safe", "--", "--color=always"])?; }
        sf! { run_cargo(&msrv, cmd, &["-F all,alloc,unsafe", "--", "--color=always"])?; }

        // alloc (un)safe + dep_all
        sf! { run_cargo(&msrv, cmd, &["-F all,alloc,safe,dep_all", "--", "--color=always"])?; }
        sf! { run_cargo(&msrv, cmd, &["-F all,alloc,unsafe,dep_all", "--", "--color=always"])?; }

        // no_std (un)safe
        // run_cargo(&msrv, cmd, &["-F all,no_std,safe"])?;
        // run_cargo(&msrv, cmd, &["-F all,no_std,unsafe"])?;

        // no_std (un)safe + dep_all
        // run_cargo(&msrv, cmd, &["-F all,no_std,safe,dep_all"])?;
        // run_cargo(&msrv, cmd, &["-F all,no_std,unsafe,dep_all"])?;
    }

    /* docs */

    if args.docs {
        headline(0, &format!["`all` docs compilation:"]);
        sf! { run_cargo_with_env("", NIGHTLY_TOOLCHAIN, &["doc", "--no-deps", "-F _docsrs"],
        &[("RUSTFLAGS", "--cfg nightly")])?; }
    }

    /* arches */

    if args.arches {
        let cmd = "clippy";

        let atotal: usize = LINUX_ARCHES.len()
            + NO_STD_ARCHES.len()
            + STD_ARCHES.len() * 2
            + STD_ARCHES_NO_CROSS_COMPILE.len() * 2;
        let mut a = 1_usize;

        sf! { headline(0, &format!["`all` checking in each architecture ({atotal}):"]); }

        // linux
        for arch in LINUX_ARCHES {
            sf! { headline(1, &format!("all,linux,unsafe_syscall: arch {a}/{atotal}")); }
            run_cargo(&msrv, cmd, &["--target", arch, "-F all,linux,unsafe_syscall"])?;
            a += 1;
        }

        // no-std
        for arch in NO_STD_ARCHES {
            sf! { headline(1, &format!("no_std,unsafe: arch {a}/{atotal}")); }
            run_cargo(&msrv, cmd, &["--target", arch, "-F all,no_std,unsafe"])?;
            a += 1;
        }

        // std, no dependencies
        for arch in STD_ARCHES {
            sf! { headline(1, &format!("std,unsafe: arch {a}/{atotal}")); }
            run_cargo(&msrv, cmd, &["--target", arch, "-F all,std,unsafe"])?;
            a += 1;
        }
        for arch in STD_ARCHES_NO_CROSS_COMPILE {
            sf! { headline(1, &format!("std,unsafe: arch {a}/{atotal}")); }
            run_cargo(&msrv, cmd, &["--target", arch, "-F all,std,unsafe"])?;
            a += 1;
        }

        // std, all dependencies (except DEP_NO_CROSS_COMPILE_EVER)
        for arch in STD_ARCHES {
            let deps = filter_deps(DEP_ALL, &[DEP_NO_CROSS_COMPILE_EVER]);
            let feature_flags = format!("all,std,unsafe,{}", deps.join(","));
            sf! { headline(1, &format!("std,unsafe,dep_all(filtered:_ever): arch {a}/{atotal}")); }
            run_cargo(&msrv, cmd, &["--target", arch, "-F", &feature_flags])?;

            a += 1;
        }

        // std, all dependencies (except DEP_NO_CROSS_COMPILE_[EVER|STD])
        for arch in STD_ARCHES_NO_CROSS_COMPILE {
            if is_current_host_compatible(arch) {
                sf! { headline(1, &format!("std,unsafe,dep_all: arch {a}/{atotal}")); }
                run_cargo(&msrv, cmd, &["--target", arch, "-F all,std,unsafe,dep_all"])?;
                a += 1;
            } else {
                let deps =
                    filter_deps(DEP_ALL, &[DEP_NO_CROSS_COMPILE_EVER, DEP_NO_CROSS_COMPILE_STD]);
                let feature_flags = format!("all,std,unsafe,{}", deps.join(","));
                sf! { headline(1, &format!("std,unsafe,dep_all(filtered:_ever,_std): arch {a}/{atotal}")); }
                run_cargo(&msrv, cmd, &["--target", arch, "-F", &feature_flags])?;
                a += 1;
            }
        }
    }

    /* miri */

    if args.miri {
        let atotal: usize = STD_ARCHES.len() + NO_STD_ARCHES.len();
        let mut a = 1_usize;

        sf! { headline(0, &format!["miri testing in each architecture ({atotal}):"]); }

        // std
        env::set_var("MIRIFLAGS", "-Zmiri-disable-isolation");
        for arch in STD_ARCHES {
            sf! { headline(1, &format!("std,unsafe: arch {a}/{atotal}")); }
            sf! { run_cargo_with_env("", NIGHTLY_TOOLCHAIN,
            &["miri", "test", "--target", arch, "-F", "all,std,unsafe"],
            &[("RUSTFLAGS", "--cfg nightly")],)?; }
            a += 1;
        }

        // std + dep_all (except DEP_*_EVER)
        env::set_var("MIRIFLAGS", "-Zmiri-disable-isolation");
        for arch in STD_ARCHES {
            let deps = filter_deps(DEP_ALL, &[DEP_NO_CROSS_COMPILE_EVER]);
            let feature_flags = format!("all,std,unsafe,{}", deps.join(","));

            sf! { headline(1, &format!("std,unsafe,dep_all(filtered:_ever) arch {a}/{atotal}")); }
            sf! { run_cargo_with_env("", NIGHTLY_TOOLCHAIN,
            &["miri", "test", "--target", arch, "-F", &feature_flags],
            &[("RUSTFLAGS", "--cfg nightly")])?; }

            a += 1;
        }

        // no_std
        env::remove_var("MIRIFLAGS");
        for arch in STD_ARCHES {
            sf! { headline(1, &format!("no_std,unsafe: arch {a}/{atotal}")); }
            sf! { run_cargo_with_env("", NIGHTLY_TOOLCHAIN,
            &["miri", "test", "--target", arch, "-F", "all,no_std,unsafe"],
            &[("RUSTFLAGS", "--cfg nightly")])?; }
            a += 1;
        }
        // WAITING for FIX: https://github.com/rust-lang/wg-cargo-std-aware/issues/69
        // for arch in NO_STD_ARCHES {}
    }

    /* minimal-versions */

    if args.minimal_versions {
        headline(0, &format!["minimal versions:"]);

        run_cargo("", NIGHTLY_TOOLCHAIN, &["update", "-Z", "minimal-versions"])?; // set min versions

        let deps = filter_deps(DEP_ALL, &[DEP_NO_MINIMAL_VERSIONS]);
        let feature_flags = format!("_docsrs_nodep,{}", deps.join(","));
        sf! { run_cargo_with_env( "", NIGHTLY_TOOLCHAIN, &["build", "-F", &feature_flags],
        &[("RUSTFLAGS", "--cfg nightly")])?; }

        run_cargo("", NIGHTLY_TOOLCHAIN, &["update"])?; // set default max versions
    }

    /* modules */

    // check individual modules
    if args.single_modules {
        let mod_total: usize = (ROOT_MODULES.len() + SUB_MODULES.len()) * 2;
        let mut mod_count = 1_usize;

        sf! { headline(0,
        &format!["Checking the presence and absence of individual modules ({mod_total}):"]); }

        for module in ROOT_MODULES {
            headline(1, &format!("root-module `{module}` {mod_count}/{mod_total}"));
            run_cargo(&msrv, cmd, &["-F", module])?;
            mod_count += 1;
        }

        for module in SUB_MODULES {
            headline(1, &format!("sub-module `{module}` {mod_count}/{mod_total}"));
            run_cargo(&msrv, cmd, &["-F", module])?;
            mod_count += 1;
        }

        for module_to_filter in ROOT_MODULES {
            sf! { headline(1,
            &format!("all modules except `{module_to_filter}` {mod_count}/{mod_total}")); }

            let modules = ROOT_MODULES
                .iter()
                .filter(|&m| m != &module_to_filter)
                .copied()
                .collect::<Vec<_>>()
                .join(",");
            run_cargo(&msrv, cmd, &["-F", &modules])?;
            mod_count += 1;
        }
        for module_to_filter in SUB_MODULES {
            sf! { headline(1,
            &format!("all modules except `{module_to_filter}` {mod_count}/{mod_total}")); }

            let modules = SUB_MODULES
                .iter()
                .filter(|&m| m != module_to_filter)
                .copied()
                .collect::<Vec<_>>()
                .join(",");
            run_cargo(&msrv, cmd, &["-F", &modules])?;
            mod_count += 1;
        }
    }

    // check all modules except the given list of exceptions
    if args.all_modules_except.is_some() {
        let exceptions = args.all_modules_except.unwrap();

        sf! { headline(0,
        &format!["Checking all the modules except `{exceptions}`:"]); }

        let exceptions: HashSet<&str> = exceptions.split(',').collect();

        let modules = ROOT_MODULES
            .iter()
            .filter(|&m| !exceptions.contains(m))
            .copied()
            .collect::<Vec<_>>()
            .join(",");
        run_cargo(&msrv, cmd, &["-F", &modules])?;
    }

    // check all the combinations of individual modules
    if args.all_modules_combinations {
        // (exclude 0 modules from the count)
        let mod_comb_total: usize = 2_usize.pow(ROOT_MODULES.len() as u32) - 1;
        let mut comb_count = 1_usize;

        sf! { headline(0, "Checking all the modules combinations ({mod_comb_total}):"); }

        for i in 1..=ROOT_MODULES.len() {
            for combination in ROOT_MODULES.iter().combinations(i) {
                sf! { headline(1,
                &format!("modules combination {comb_count}/{mod_comb_total}")); }
                let deref_combination: Vec<_> = combination.into_iter().copied().collect();
                let combined = deref_combination.join(",");
                run_cargo(&msrv, cmd, &["-F", &combined])?;
                comb_count += 1;
            }
        }
    }

    Ok(())
}

/* helpers */

/// CLI arguments state.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Args {
    no_msrv: bool,
    tests: bool,
    docs: bool,
    arches: bool,
    single_modules: bool,
    all_modules_combinations: bool,
    all_modules_except: Option<String>,
    minimal_versions: bool,
    miri: bool,
    install_arches: bool,
    install_nightly: bool,
}

/// CLI arguments parser.
fn get_args() -> Result<Args> {
    use lexopt::prelude::*;

    let mut no_msrv = false;
    let mut tests = false;
    let mut docs = false;
    let mut arches = false;
    let mut single_modules = false;
    let mut all_modules_combinations = false;
    let mut all_modules_except = None;
    let mut minimal_versions = false;
    let mut miri = false;
    let mut install_arches = false;
    let mut install_nightly = false;

    let mut parser = lexopt::Parser::from_env();

    // if there are no arguments, print help an exit
    is![env::args_os().len() == 1; {print_help(&parser); exit(0); }];

    sf! { while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => { print_help(&parser); exit(0); }
            Short('t') | Long("tests") => { tests = true; }
            Short('d') | Long("docs") => { docs = true; }
            Short('a') | Long("arches") => { arches = true; }
            // Short('s') | Long("show-arches") => { show_arches = true; }
            Short('m') | Long("single-modules") => { single_modules = true; }
            Long("all-modules-except") => {
                let exceptions: String = parser.value()?.parse()?;
                all_modules_except = Some(exceptions);
            }
            Long("all-modules-combinations") => { all_modules_combinations = true; }
            Long("miri") => { miri = true; }
            Long("minimal-versions") => { minimal_versions = true; }
            Long("no-msrv") => { no_msrv = true; }
            Short('A') | Long("install-arches") => { install_arches = true; }
            Short('N') | Long("install-nightly") => { install_nightly = true; }
            _ => { let err = arg.unexpected(); print_help(&parser); return Err(Box::new(err)); }
        }
    }}

    Ok(Args {
        no_msrv,
        tests,
        docs,
        arches,
        single_modules,
        all_modules_combinations,
        all_modules_except,
        minimal_versions,
        miri,
        install_arches,
        install_nightly,
        // ..Default::default()
    })
}

/// Prints the help.
fn print_help(parser: &lexopt::Parser) {
    let msrv = get_msrv().unwrap_or("".into());
    let mods: usize = ROOT_MODULES.len();
    let mods_combs: usize = 2_usize.pow(mods as u32) - 1;
    let arches: usize = STD_ARCHES.len()
        + NO_STD_ARCHES.len()
        + STD_ARCHES_NO_CROSS_COMPILE.len()
        + LINUX_ARCHES.len();

    println!(
        "Usage: {name} [OPTIONS]

  -t, --tests                      run the tests
  -d, --docs                       compile the documentation
  -a, --arches                     check all of the {arches} architectures
      --miri                       use `miri` to check each architecture
  -m, --single-modules             check root modules both individually and
                                   removed from the full module set ({mods} × 2).
      --all-modules-combinations   check all the modules combinations ({mods_combs})
      --all-modules-except m1,…    check all the modules together except the given list
      --no-msrv                    do not enforce using the configured MSRV ({msrv})
  -A, --install-arches             install all of the {arches} architectures
  -N, --install-nightly            install the nightly toolchain
  -h, --help                       display this help and exit

Architectures:
  std: {STD_ARCHES:?}
  no_std: {NO_STD_ARCHES:?}
  no cross-compile: {STD_ARCHES_NO_CROSS_COMPILE:?}
  linux: {LINUX_ARCHES:?}
",
        name = parser.bin_name().unwrap_or("check.rs")
    );
}

/// Returns the `rust-version` field from Cargo.toml
fn get_msrv() -> Result<String> {
    // read the Cargo.toml file
    let cargo = FsPath::from_crate_root("Cargo.toml")?.into_inner();
    let mut file = File::open(cargo)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the contents as TOML and return the `rust`-version field
    let cargo_toml: Document<_> = contents.parse()?;
    cargo_toml
        .as_table()
        .get("workspace")
        .and_then(|ws| ws.as_table())
        .and_then(|ws| ws.get("package"))
        .and_then(|pkg| pkg.as_table())
        .and_then(|pkg| pkg.get("rust-version"))
        .map(|msrv| msrv.as_str().unwrap().to_owned())
        .ok_or("error".into())
}

/// Runs the given cargo `command` with `arguments`, using the `msrv` rust version.
///
/// If `msrv` is empty then it will be ignored.
fn run_cargo(msrv: &str, command: &str, arguments: &[&str]) -> Result<()> {
    run_cargo_inner(msrv, command, arguments, &[])
}

/// Runs the given cargo `command` with `arguments`, using the `msrv` rust version,
/// and applies the provided environment variables.
fn run_cargo_with_env(
    msrv: &str,
    command: &str,
    arguments: &[&str],
    envs: &[(&str, &str)],
) -> Result<()> {
    run_cargo_inner(msrv, command, arguments, envs)
}

/// Shared internal function to execute cargo with optional environment variables.
fn run_cargo_inner(
    msrv: &str,
    command: &str,
    arguments: &[&str],
    envs: &[(&str, &str)],
) -> Result<()> {
    let mut cmd = if msrv.is_empty() {
        println!("$ cargo {command} {}", arguments.join(" "));

        let mut c = Command::new("cargo");
        c.arg(command).args(["--color", "always"]).args(arguments);
        c
    } else {
        println!("$ rustup run {msrv} cargo {command} {}", arguments.join(" "));

        let mut c = Command::new("rustup");
        c.arg("run")
            .arg(msrv)
            .arg("cargo")
            .args(["--color", "always"])
            .arg(command)
            .args(arguments);
        c
    };

    // Set environment variables if provided
    for &(key, value) in envs {
        cmd.env(key, value);
    }

    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let stdout_handle = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    });

    let stderr_handle = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            eprintln!("{}", line.unwrap());
        }
    });

    stdout_handle.join().unwrap();
    stderr_handle.join().unwrap();

    let status = child.wait()?;

    if !status.success() { Err("ERROR".into()) } else { Ok(()) }
}

/// Makes sure to install all the architectures and components for the current MSRV and nightly.
fn rust_install_arches(msrv: &str) -> Result<()> {
    println!("> rust_install_arches() // for MSRV and nightly");

    if !msrv.is_empty() {
        println!("rustup override set {msrv}");
        sf! { let _ = Command::new("rustup").args(["override", "set", msrv]).status()?; }
        println!("rustup component add clippy");
        sf! { let _ = Command::new("rustup").args(["component", "add", "clippy"]).status()?; }
        println!("rustup component add rustfmt");
        sf! { let _ = Command::new("rustup").args(["component", "add", "rustfmt"]).status()?; }
    }
    for ref arch in STD_ARCHES
        .iter()
        .chain(LINUX_ARCHES.iter())
        .chain(NO_STD_ARCHES.iter())
        .chain(STD_ARCHES_NO_CROSS_COMPILE.iter())
    {
        println!("rustup target add {arch}");
        sf! { let _ = Command::new("rustup").args(["target", "add", arch]).status()?; }
        sf! { let _ = Command::new("rustup").args([NIGHTLY_TOOLCHAIN, "target", "add", arch]).status()?; }
    }
    Ok(())
}

/// Setups nightly, and adds the miri component.
fn rust_install_nightly() -> Result<()> {
    println!("rustup toolchain install {NIGHTLY_VERSION}");
    sf! { let _ = Command::new("rustup").args(["toolchain", "install", NIGHTLY_VERSION]).status()?; }
    println!("rustup component add miri --toolchain {NIGHTLY_VERSION}");
    let _ = Command::new("rustup")
        .args(["component", "add", "miri", "--toolchain", NIGHTLY_VERSION])
        .status()?;
    Ok(())
}

/// Prints a headline
fn headline(level: usize, text: &str) {
    match level {
        0 => println!("\n{text}\n{}", "=".repeat(80)),
        1 => println!("\n> {text}\n{}", "-".repeat(60)),
        _ => println!("\nUNKNOWN HEADLINE LEVEL\n{text}\n"),
    }
}

/// Checks if the current host is compatible with the provided target architecture.
fn is_current_host_compatible(target_arch: &str) -> bool {
    let is_arch_compatible = {
        #[cfg(target_arch = "x86_64")]
        {
            target_arch.contains("x86_64")
        }
        #[cfg(target_arch = "aarch64")]
        {
            target_arch.contains("aarch64")
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            false
        }
    };

    let is_os_compatible = {
        #[cfg(target_os = "macos")]
        {
            target_arch.contains("apple-darwin")
        }
        #[cfg(target_os = "linux")]
        {
            target_arch.contains("linux")
        }
        #[cfg(target_os = "windows")]
        {
            target_arch.contains("windows")
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            false
        }
    };

    is_arch_compatible && is_os_compatible
}

/// Filters a list of dependencies by excluding those found in any of the provided exclusion lists.
fn filter_deps<'a>(all_deps: &'a [&'a str], exclude_lists: &[&[&'a str]]) -> Vec<&'a str> {
    all_deps
        .iter()
        .filter(|&&dep| exclude_lists.iter().all(|exclude| !exclude.contains(&dep)))
        .copied()
        .collect()
}
