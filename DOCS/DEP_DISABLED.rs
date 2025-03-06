// devela::_dep
// disabled dependencies

reexport! { optional_crate (unsafe) "dep_js_sys", "js-sys", js_sys,
    doc: "Bindings for all JS global objects and functions in all JS environments."
}
reexport! { optional_crate (safe) "dep_ring", "ring", ring,
    doc: "Safe, fast, small crypto using Rust."
}
reexport! { optional_crate (unsafe) "dep_rkyv", "rkyv", rkyv,
    doc: "A zero-copy deserialization framework for Rust."
}
reexport! { optional_crate (unsafe) "dep_wasm_bindgen", "wasm-bindgen", wasm_bindgen,
    doc: "Easy support for interacting between JS and Rust."
}
reexport! { optional_crate (unsafe) "dep_web_sys", "web-sys", web_sys,
    doc: "Bindings for all Web APIs."
}
