---cargo
[package]
name = "minimal-linux"
edition = "2024"

[dependencies.devela]
path = "../../../.."
default-features = false
features = ["linux"]

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = "symbols"

[profile.dev]
panic = "abort"
---
//! Minimal `no_std` executable using devela's Linux syscall layer.

#![no_std]
#![no_main]

use devela::{Env, Linux, LinuxResult, cfg_select, linux_entry, set_panic_handler};

linux_entry! { linux_result }
set_panic_handler! { loop }

fn main() -> LinuxResult<()> {
    Linux::println(cfg_select! {
        target_os = "linux" => "cfg: linux",
        _ => "cfg: NOT linux",
    })?;
    Linux::print("os:  ")?;
    Linux::println(Env::OS)?;
    Ok(())
}
