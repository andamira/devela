// devela::_dep
//
//! Re-exported optional dependencies.
//
// TOC
// - standard libraries
// - external dependencies

#![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]

use crate::code::reexport;

/* standard libraries */

#[doc(hidden)]
pub use super::_core; // for completion

/// <span class='stab portability' title='re-exported `alloc`'>`alloc`</span>
/// *Re-exported Rust `alloc` library.*
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
#[doc(inline)]
pub extern crate alloc as _alloc;

/// <span class='stab portability' title='re-exported `std`'>`std`</span>
/// *Re-exported Rust `std` library.*
/// <br/><hr>
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
#[doc(inline)]
pub use ::std as _std;

/* 39 optional dependencies */
// In sync with ../Cargo.toml::dep_all & ../config/dep_all.rs

reexport! { optional_crate (unsafe) "dep_allocator_api2", "allocator-api2", allocator_api2,
    doc: "Mirror of Rust's allocator api for use on stable rust"
}
reexport! { optional_crate (unsafe) "dep_atomic", "atomic", atomic,
    doc: "A generic atomic wrapper type."
}
reexport! { optional_crate (unsafe) "dep_bumpalo", "bumpalo", bumpalo,
    doc: "A fast bump allocation arena for Rust."
}
reexport! { optional_crate (unsafe) "dep_bytemuck", "bytemuck", bytemuck,
    doc: "Small utilities for casting between plain data types."
}
reexport! { optional_crate (unsafe) "dep_const_str", "const-str", const_str,
    doc: "Compile-time string operations."
}
reexport! { optional_crate (safe) "dep_crossterm", "crossterm", crossterm,
    doc: "Cross-platform Terminal Manipulation Library.",
    features: "std"
}
reexport! { optional_crate (unsafe) "dep_hashbrown", "hashbrown", hashbrown,
    doc: "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`.",
    features: "alloc"
}
reexport! { optional_crate (safe) "dep_jiff", "jiff", jiff,
    doc: "A high level datetime library that is secure and performant.",
    features: "alloc"
}
reexport! { optional_crate (unsafe) "dep_js_sys", "js-sys", js_sys,
    doc: "Bindings for all JS global objects and functions in all JS environments."
}
reexport! { optional_crate (safe) "dep_kira", "kira", kira,
    doc: "Expressive audio library for games."
}
reexport! { optional_crate (unsafe) "dep_libm", "libm", libm,
    doc: "A port of [`MUSL`](https://musl.libc.org/)'s libm to Rust."
}
reexport! { optional_crate (safe) "dep_log", "log", log,
    doc: "A lightweight logging facade."
}
reexport! { optional_crate (unsafe) "dep_memchr", "memchr", memchr,
    doc: "Optimized routines for string search primitives."
}
reexport! { optional_crate (unsafe) "dep_midir", "midir", midir,
    doc: "A cross-platform, realtime MIDI processing library."
}
reexport! { optional_crate (unsafe) "dep_miniquad", "miniquad", miniquad,
    doc: "Cross-platform window context and rendering library."
}
reexport! { optional_crate (unsafe) "dep_nc", "nc", nc,
    doc: "Access system calls directly in: linux, android, freebsd, netbsd, macos."
}
reexport! { optional_crate (unsafe) "dep_portable_atomic", "portable-atomic", portable_atomic,
    doc: "Portable atomic types including 128-bit atomics, floats, etc."
}
reexport! { optional_crate (unsafe) "dep_pyo3", "pyo3", pyo3,
    doc: " Bindings to Python interpreter.",
    features: "std"
}
reexport! { optional_crate (safe) "dep_rand_core", "rand_core", rand_core,
    doc: "Random number generation traits."
}
reexport! { optional_crate (safe) "dep_rayon", "rayon", rayon,
    doc: "Simple work-stealing parallelism for Rust."
}
reexport! { optional_crate (unsafe) "dep_raw_cpuid", "raw-cpuid", raw_cpuid,
    doc: "A library to parse the x86 CPUID instruction."
}
reexport! { optional_crate (safe) "dep_regex_lite", "regex-lite", regex_lite,
    doc: " A lightweight regex engine optimized for binary size and compilation time."
}
reexport! { optional_crate (unsafe) "dep_rkyv", "rkyv", rkyv,
    doc: "A zero-copy deserialization framework for Rust."
}
reexport! { optional_crate (unsafe) "dep_rodio", "rodio", rodio,
    doc: "Audio playback library."
}
reexport! { optional_crate (unsafe) "dep_rustix", "rustix", rustix,
    doc: "Safe Rust bindings to POSIX/Unix/Linux/Winsock-like syscalls."
}
reexport! { optional_crate (unsafe) "dep_safe_arch", "safe_arch", safe_arch,
    doc: "Exposes arch-specific intrinsics as safe functions."
}
reexport! { optional_crate (unsafe) "dep_serde", "serde", serde,
    doc: " A generic serialization/deserialization framework."
}
reexport! { optional_crate (unsafe) "dep_stringzilla", "stringzilla", stringzilla,
    doc: "SIMD-accelerated string search, sorting, fingerprints, and edit distances."
}
reexport! { optional_crate (safe) "dep_symphonia", "symphonia", symphonia,
    doc: " Pure Rust media container and audio decoding library."
}
reexport! { optional_crate (unsafe) "dep_sysinfo", "sysinfo", sysinfo,
    doc: "Get system information such as processes, CPUs, disks, and networks."
}
reexport! { optional_crate (unsafe) "dep_tinyaudio", "tinyaudio", tinyaudio,
    doc: "A cross-platform, easy-to-use, low-level, audio output library.",
    features: "alloc"
}
reexport! { optional_crate (unsafe) "dep_tokio", "tokio", tokio,
    doc: "A runtime for writing reliable network applications without compromising speed.",
    features: "std"
}
reexport! { optional_crate (unsafe) "dep_tracing", "tracing", tracing,
    doc: "A scoped, structured logging and diagnostics system.",
    features: "alloc"
}
reexport! { optional_crate (safe)
    "dep_unicode_segmentation", "unicode-segmentation", unicode_segmentation,
    doc: "Split strings on Grapheme Clusters, Words or Sentences."
}
reexport! { optional_crate (safe) "dep_unicode_width", "unicode-width", unicode_width,
    doc: "Determine displayed width of `char` and `str` types."
}
reexport! { optional_crate (unsafe) "dep_wasm_bindgen", "wasm-bindgen", wasm_bindgen,
    doc: "Easy support for interacting between JS and Rust."
}
reexport! { optional_crate (unsafe) "dep_web_sys", "web-sys", web_sys,
    doc: "Bindings for all Web APIs."
}
reexport! { optional_crate (unsafe) "dep_wide", "wide", wide,
    doc: "SIMD-compatible data types."
}
reexport! { optional_crate (safe) /*±*/ "dep_winnow", "winnow", winnow,
    doc: "A byte-oriented, zero-copy, parser combinators library."
}
