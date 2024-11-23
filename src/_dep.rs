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

/* external dependencies */

reexport! { optional_crate (unsafe) "dep_atomic", "atomic", atomic,
    doc: "A generic atomic wrapper type."
}
reexport! { optional_crate (unsafe) "dep_bytemuck", "bytemuck", bytemuck,
    doc: "Small utilities for casting between plain data types."
}
reexport! { optional_crate (unsafe) "dep_const_str", "const-str", const_str,
    doc: "Compile-time string operations."
}
reexport! { optional_crate (unsafe) "dep_hashbrown", "hashbrown", hashbrown,
    doc: "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`.",
    features: "alloc"
}
reexport! { optional_crate (safe) "dep_jiff", "jiff", jiff,
    doc: "A high level datetime library that is secure and performant."
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
reexport! { optional_crate (unsafe) "dep_portable_atomic", "portable-atomic", portable_atomic,
    doc: "Portable atomic types including 128-bit atomics, floats, etc."
}
reexport! { optional_crate (safe) "dep_rand_core", "rand_core", rand_core,
    doc: "Random number generation traits."
}
reexport! { optional_crate (safe) "dep_rayon", "rayon", rayon,
    doc: "Simple work-stealing parallelism for Rust."
}
reexport! { optional_crate (unsafe) "dep_rodio", "rodio", rodio,
    doc: "Audio playback library."
}
reexport! { optional_crate (unsafe) "dep_tinyaudio", "tinyaudio", tinyaudio,
    doc: "A cross-platform, easy-to-use, low-level, audio output library.",
    features: "alloc"
}
reexport! { optional_crate (safe) "dep_unicode_segmentation", "unicode-segmentation", unicode_segmentation,
    doc: "Split strings on Grapheme Clusters, Words or Sentences."
}
reexport! { optional_crate (safe) "dep_unicode_width", "unicode-width", unicode_width,
    doc: "Determine displayed width of `char` and `str` types."
}
reexport! { optional_crate (unsafe) "dep_wide", "wide", wide,
    doc: "SIMD-compatible data types."
}
