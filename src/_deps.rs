// devela::_deps
//
//!
//! # Dependencies
//!
//! Several minimal optional dependencies are included, providing useful
//! functionality missing from the standard library.
//

#![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]

/* environment */

/// <span class='stab portability' title='re-exported `alloc`'>`alloc`</span>
/// *Re-exported Rust `alloc` library environment.*
#[doc(inline)]
#[cfg(feature = "alloc")]
pub extern crate alloc;

/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library environment.*
#[doc(inline)]
pub use ::core;

/// <span class='stab portability' title='re-exported `std`'>`std`</span>
/// *Re-exported Rust `std` library environment.*
#[cfg(feature = "std")]
#[doc(inline)]
pub use ::std;

/* always compiled */

pub use ::bytemuck;

/* feature-gated */

#[doc(inline)]
#[cfg(feature = "atomic")]
pub use ::atomic;

#[doc(inline)]
#[cfg(feature = "const-str")]
pub use ::const_str;

#[doc(inline)]
#[cfg(feature = "crossterm")]
pub use ::crossterm;

#[doc(inline)]
#[cfg(all(feature = "hashbrown", feature = "alloc"))]
pub use ::hashbrown;

#[doc(inline)]
#[cfg(feature = "libm")]
pub use ::libm;

#[doc(inline)]
#[cfg(feature = "memchr")]
pub use ::memchr;

#[doc(inline)]
#[cfg(feature = "miniquad")]
pub use ::miniquad;

#[doc(inline)]
#[cfg(feature = "portable-atomic")]
pub use ::portable_atomic;

#[doc(inline)]
#[cfg(feature = "rand_core")]
pub use ::rand_core;

#[doc(inline)]
#[cfg(feature = "unicode-segmentation")]
pub use ::unicode_segmentation;

#[doc(inline)]
#[cfg(feature = "unicode-width")]
pub use ::unicode_width;

#[doc(inline)]
#[cfg(feature = "wide")]
pub use ::wide;
