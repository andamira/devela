// DOCS/DEP_DISABLED.rs

/* disabled optional dependencies in: ../util/check.rs */

const DEP_NO_CROSS_COMPILE_EVER: &[&str] = &[
    // (windows, linux, macos) https://github.com/fltk-rs/fltk-rs/blob/master/FAQ.md
    "dep_fltk",
    // - https://docs.rs/nc/latest/nc/#supported-operating-systems-and-architectures
    "dep_nc",
];


/* disabled optional dependencies in: ../src/devela/_dep.rs */

reexport! { optional_crate (unsafe) "dep_fltk", "fltk", fltk,
    doc: "Rust bindings for the FLTK GUI library."
}
reexport! { optional_crate (unsafe) "dep_js_sys", "js-sys", js_sys,
    doc: "Bindings for all JS global objects and functions in all JS environments."
}
reexport! { optional_crate (unsafe) "dep_nc", "nc", nc,
    doc: "Access system calls directly in: linux, android, freebsd, netbsd, macos."
}
reexport! { optional_crate (safe) "dep_ring", "ring", ring,
    doc: "Safe, fast, small crypto using Rust."
}
reexport! { optional_crate (unsafe) "dep_rkyv", "rkyv", rkyv,
    doc: "A zero-copy deserialization framework for Rust."
}
reexport! { optional_crate (unsafe) "dep_rustix", "rustix", rustix,
    doc: "Safe Rust bindings to POSIX/Unix/Linux/Winsock-like syscalls."
}
reexport! { optional_crate (unsafe) "dep_sdl3", "sdl3", sdl3,
    doc: "SDL3 bindings for Rust."
}
reexport! { optional_crate (unsafe) "dep_tinyaudio", "tinyaudio", tinyaudio,
    doc: "A cross-platform, easy-to-use, low-level, audio output library.",
    features: "alloc"
}
reexport! { optional_crate (unsafe) "dep_tracing", "tracing", tracing,
    doc: "A scoped, structured logging and diagnostics system.",
    features: "alloc"
}
reexport! { optional_crate (unsafe) "dep_wasm_bindgen", "wasm-bindgen", wasm_bindgen,
    doc: "Easy support for interacting between JS and Rust."
}
reexport! { optional_crate (unsafe) "dep_web_sys", "web-sys", web_sys,
    doc: "Bindings for all Web APIs."
}
