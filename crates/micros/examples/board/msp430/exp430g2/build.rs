use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=MSP430_CHIP");

    let chip = env::var("MSP430_CHIP").unwrap_or_else(|_| "g2553".into());
    let file = match chip.as_str() {
        "g2231" => "memory/g2231.x",
        "g2452" => "memory/g2452.x",
        "g2553" => "memory/g2553.x",
        _ => panic!("unsupported MSP430_CHIP={chip:?} (expected g2231, g2452 or g2553)"),
    };

    println!("cargo:rerun-if-changed={file}");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(file, out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
