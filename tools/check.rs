#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [dependencies]
//! toml_edit = "0.20"
//! itertools = "0.13"
//! lexopt = "0.3"
//! devela = { version = "0.21.2", features = ["std", "sys"] }
//! ```
// This script needs [rusts-cript](https://crates.io/crates/rust-script) to run.

use devela::all::{crate_root, iif, sf};
use itertools::Itertools;
use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    process::{exit, Command, Stdio},
    thread,
};
use toml_edit::Document;

/* global configuration */

#[rustfmt::skip]
const ROOT_MODULES: [&str; 7] = [
    "code", "data", "exec", "mem", "num", "sys", "text",
];

const STD_ARCHES: &[&str] = &[
    // Linux 64-bit
    "x86_64-unknown-linux-gnu",
    // Windows Cygwin 64-bit
    "x86_64-pc-windows-msvc",
    // MacOs 64-bit
    "x86_64-apple-darwin",
    // Linux i686, 32-bit, std, little-endian, (kernel 3.2+, glibc 2.17+)
    // may need to install `libc6-dev-amd64-i386-cross` for testing
    "i686-unknown-linux-gnu",
];
const NO_STD_ARCHES: &[&str] = &[
    // Bare x86_64, softfloat, 64-bit, no_std
    // https://doc.rust-lang.org/nightly/rustc/platform-support/x86_64-unknown-none.html
    "x86_64-unknown-none",
    // Bare ARM64, hardfloat, 64-bit, no_std, little-endian, A64 set, (M1, M2 processors)
    "aarch64-unknown-none",
    // Bare ARMv7-M, 32-bit, no_std, little-endian, Thumb set, (Cortex-M processors)
    "thumbv7m-none-eabi",
];

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = get_args()?;
    let msrv = iif![args.no_msrv; "".into(); get_msrv().unwrap_or("".into())];

    let cmd = "check";

    /* tests */

    if args.tests {
        rust_setup_arches(&msrv)?;

        let cmd = "test";
        sf! { headline(0, &format!["`all` test checking [alloc|std] + [safe|unsafe]):"]); }

        // WAITING: https://github.com/rust-lang/cargo/issues/1983 (colored output)

        // std (un)safe
        run_cargo(&msrv, cmd, &["-F all,std,safe", "--", "--color=always"])?;
        run_cargo(&msrv, cmd, &["-F all,std,unsafe", "--", "--color=always"])?;

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
        rust_setup_nightly()?;
        headline(0, &format!["`all` docs compilation:"]);
        run_cargo("", "+nightly", &["doc", "--no-deps", "-F docsrs"])?;
    }

    /* arches */

    if args.arches {
        let cmd = "clippy";

        let arch_total: usize = STD_ARCHES.len() + NO_STD_ARCHES.len();
        let mut arch_count = 1_usize;

        sf! { headline(0, &format!["`all` checking in each architecture ({arch_total}):"]); }

        rust_setup_arches(&msrv)?;

        for arch in STD_ARCHES {
            sf! { headline(1, &format!("std,unsafe: arch {arch_count}/{arch_total}")); }
            run_cargo(&msrv, cmd, &["--target", arch, "-F all,std,unsafe"])?;
            arch_count += 1;
        }
        for arch in STD_ARCHES {
            sf! { headline(1, &format!("std,unsafe,dep_all: arch {arch_count}/{arch_total}")); }
            run_cargo(&msrv, cmd, &["--target", arch, "-F all,std,unsafe,dep_all"])?;
            arch_count += 1;
        }

        for arch in NO_STD_ARCHES {
            sf! { headline(1, &format!("no_std,unsafe: arch {arch_count}/{arch_total}")); }
            run_cargo(&msrv, cmd, &["--target", arch, "-F all,no_std,unsafe"])?;
            arch_count += 1;
        }
    }

    /* miri */

    if args.miri {
        let arch_total: usize = STD_ARCHES.len() + NO_STD_ARCHES.len();
        let mut arch_count = 1_usize;

        sf! { headline(0, &format!["miri testing in each architecture ({arch_total}):"]); }

        rust_setup_arches(&msrv)?;
        rust_setup_nightly()?;

        // std
        env::set_var("MIRIFLAGS", "-Zmiri-disable-isolation");
        for arch in STD_ARCHES {
            sf! { headline(1, &format!("std,unsafe: arch {arch_count}/{arch_total}")); }
            sf! { run_cargo("", "+nightly", &[ "miri", "test", "--target", arch,
            "-F all,std,unsafe,nightly"])?; }
            arch_count += 1;
        }

        // std + dep_all
        env::set_var("MIRIFLAGS", "-Zmiri-disable-isolation");
        for arch in STD_ARCHES {
            sf! { headline(1, &format!("std,unsafe,dep_all: arch {arch_count}/{arch_total}")); }
            sf! { run_cargo("", "+nightly", &[ "miri", "test", "--target", arch,
            "-F all,std,unsafe,nightly,dep_all"])?; }
            arch_count += 1;
        }

        // no_std
        env::remove_var("MIRIFLAGS");
        for arch in STD_ARCHES {
            sf! { headline(1, &format!("no_std,unsafe: arch {arch_count}/{arch_total}")); }
            sf! { run_cargo("", "+nightly", &[ "miri", "test", "--target", arch,
            "-F all,no_std,unsafe,nightly"])?; }
            arch_count += 1;
        }
        // WAITING for FIX: https://github.com/rust-lang/wg-cargo-std-aware/issues/69
        // for arch in NO_STD_ARCHES {}
    }

    /* modules */

    // check individual modules
    if args.single_modules {
        // let cmd = "clippy";

        let mod_total: usize = ROOT_MODULES.len() * 2;
        let mut mod_count = 1_usize;

        sf! { headline(0,
        &format!["Checking the presence and absence of individual modules ({mod_total}):"]); }

        for module in ROOT_MODULES {
            headline(1, &format!("module `{module}` {mod_count}/{mod_total}"));
            run_cargo(&msrv, cmd, &["-F", &module])?;
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
                let deref_combination: Vec<_> = combination.into_iter().map(|&s| s).collect();
                let combined = deref_combination.join(",");
                run_cargo(&msrv, cmd, &["-F", &combined])?;
                comb_count += 1;
            }
        }
    }

    Ok(())
}

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
    miri: bool,
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
    let mut miri = false;

    let mut parser = lexopt::Parser::from_env();

    // if there are no arguments, print help an exit
    iif![env::args_os().len() == 1; {print_help(&parser); exit(0); }];

    sf! { while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => { print_help(&parser); exit(0); }
            Short('t') | Long("tests") => { tests = true; }
            Short('d') | Long("docs") => { docs = true; }
            Short('a') | Long("arches") => { arches = true; }
            Short('m') | Long("single-modules") => { single_modules = true; }
            Long("all-modules-except") => {
                let exceptions: String = parser.value()?.parse()?;
                all_modules_except = Some(exceptions);
            }
            Long("all-modules-combinations") => { all_modules_combinations = true; }
            Long("miri") => { miri = true; }
            Long("no-msrv") => { no_msrv = true; }
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
        miri,
        ..Default::default()
    })
}

/// Prints the help.
fn print_help(parser: &lexopt::Parser) {
    let msrv = get_msrv().unwrap_or("".into());
    let mods: usize = ROOT_MODULES.len();
    let mods_combs: usize = 2_usize.pow(mods as u32) - 1;
    let arches: usize = STD_ARCHES.len() + NO_STD_ARCHES.len();

    println!(
        "Usage: {} [OPTIONS]

  -t, --tests                      run the tests
  -d, --docs                       compile the documentation
  -a, --arches                     check each architecture ({arches})
      --miri                       use `miri` to check each architecture
  -m, --single-modules             check root modules both individually and
                                   removed from the full module set ({mods} × 2).
      --all-modules-combinations   check all the modules combinations ({mods_combs})
      --all-modules-except m1,…    check all the modules together except the given list
      --no-msrv                    do not enforce using the configured MSRV ({msrv})
  -h, --help                       display this help and exit
",
        parser.bin_name().unwrap_or("check.rs")
    );
}

/// Returns the `rust-version` field from Cargo.toml
fn get_msrv() -> Result<String> {
    // read the Cargo.toml file
    let cargo = PathBuf::from(crate_root("Cargo.toml")?);
    let mut file = File::open(cargo)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the contents as TOML and return the `rust`-version field
    let cargo_toml: Document = contents.parse()?;
    cargo_toml
        .as_table()
        .get("package")
        .and_then(|package| package.as_table())
        .and_then(|package| package.get("rust-version"))
        .map(|msrv| msrv.as_str().unwrap().to_owned())
        .ok_or("error".into())
}

/// Runs the given cargo `command` with `arguments`, using the `msrv` rust version.
///
/// If `msrv` is empty then it will be ignored.
fn run_cargo(msrv: &str, command: &str, arguments: &[&str]) -> Result<()> {
    let mut child = if msrv.is_empty() {
        println!("$ cargo {command} {}", arguments.join(" "));

        Command::new("cargo")
            .arg(command)
            .args(["--color", "always"])
            .args(arguments)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    } else {
        sf! { println!("$ rustup run {msrv} cargo {command} {}", arguments.join(" ")); }
        Command::new("rustup")
            // .arg("--verbose")
            .arg("run")
            .arg(msrv)
            .arg("cargo")
            .args(["--color", "always"])
            .arg(command)
            .args(arguments)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    };

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

    if !status.success() {
        Err("ERROR".into())
    } else {
        Ok(())
    }
}

/// Makes sure to install all the architectures and components for the current MSRV.
fn rust_setup_arches(msrv: &str) -> Result<()> {
    if !msrv.is_empty() {
        println!("rustup override set {msrv}");
        sf! { let _ = Command::new("rustup").args(["override", "set", msrv]).status()?; }
        println!("rustup component add clippy");
        sf! { let _ = Command::new("rustup").args(["component", "add", "clippy"]).status()?; }
        println!("rustup component add rustfmt");
        sf! { let _ = Command::new("rustup").args(["component", "add", "rustfmt"]).status()?; }
    }

    for ref arch in STD_ARCHES.into_iter().chain(NO_STD_ARCHES.into_iter()) {
        println!("rustup target add {arch}");
        sf! { let _ = Command::new("rustup").args(["target", "add", arch]).status()?; }
    }
    Ok(())
}

/// Setups nightly, and adds the miri component.
fn rust_setup_nightly() -> Result<()> {
    println!("rustup toolchain install nightly");
    sf! { let _ = Command::new("rustup").args(["toolchain", "install", "nightly"]).status()?; }
    println!("rustup component add miri --toolchain nightly");
    let _ = Command::new("rustup")
        .args(["component", "add", "miri", "--toolchain", "nightly"])
        .status()?;
    Ok(())
}

/// Prints a headline
fn headline(level: usize, text: &str) {
    match level {
        0 => println!("\n{text}\n{}", "-".repeat(80)),
        1 => println!["\n> {text}"],
        _ => println!("\nUNKNOWN HEADLINE LEVEL\n{text}\n"),
    }
}
