#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
name = "minimal-clib"
edition = "2024"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = "symbols"

[profile.dev]
panic = "abort"
---
//! Minimal hosted `no_std` executable using libc.

#![no_std]
#![no_main]

use core::{ffi::c_char, hint::spin_loop, panic::PanicInfo};

#[link(name = "c")]
unsafe extern "C" {
    fn puts(s: *const c_char) -> i32;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        spin_loop();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let message = cfg_select! {
        target_os = "linux" => b"is linux\0",
        _ => b"is NOT linux\0",
    };
    unsafe { puts(message.as_ptr().cast()) };
    0
}
