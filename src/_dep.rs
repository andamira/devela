// devela::_dep
//
//!
//
// TOC
// - standard libraries
// - external dependencies

#![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]

use crate::code::reexport;

/* standard libraries */

/// <span class='stab portability' title='re-exported `alloc`'>`alloc`</span>
/// *Re-exported Rust `alloc` library environment.*
#[doc(inline)]
#[cfg(feature = "alloc")]
pub extern crate alloc as _liballoc;

/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library environment.*
#[doc(inline)]
pub use ::core as _libcore;

/// <span class='stab portability' title='re-exported `std`'>`std`</span>
/// *Re-exported Rust `std` library environment.*
#[cfg(feature = "std")]
#[doc(inline)]
pub use ::std as _libstd;

/* external dependencies */

reexport! { optional_crate "atomic" | atomic,
    doc: "A generic atomic wrapper type."
}
reexport! { optional_crate "bytemuck" | bytemuck,
    doc: "Small utilities for casting between plain data types."
}
reexport! { optional_crate "const-str" | const_str,
    doc: "Compile-time string operations."
}
reexport! { optional_crate "hashbrown" | hashbrown,
    doc: "A drop-in replacement for Rust’s standard `HashMap` and `HashSet`.",
    features: "alloc"
}
reexport! { optional_crate "libm" | libm,
    doc: "A port of [`MUSL`](https://musl.libc.org/)'s libm to Rust."
}
reexport! { optional_crate "log" | log,
    doc: "A lightweight logging facade."
}
reexport! { optional_crate "memchr" | memchr,
    doc: "Optimized routines for string search primitives."
}
reexport! { optional_crate "portable-atomic" | portable_atomic,
    doc: "Portable atomic types including 128-bit atomics, floats, etc."
}
reexport! { optional_crate "rand_core" | rand_core,
    doc: "Random number generation traits."
}
reexport! { optional_crate "unicode-segmentation" | unicode_segmentation,
    doc: "Split strings on Grapheme Clusters, Words or Sentences."
}
reexport! { optional_crate "unicode-width" | unicode_width,
    doc: "Determine displayed width of `char` and `str` types."
}
reexport! { optional_crate "wide" | wide,
    doc: "SIMD-compatible data types."
}
