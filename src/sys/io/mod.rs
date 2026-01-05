// devela::sys::io
//
#![doc = crate::_DOC_SYS_IO!()]
//!
#![doc = crate::_doc!(extends: io)]
//!
//! Provides unified traits, buffers, transports, and no-std foundations for
//! reading, writing, and transforming data across abstract sources and sinks,
//! independent of filesystems, networks, or specific hardware.
//!
#![doc = crate::_doc!(vendor: "no_std_io")]
//
// safety
#![cfg_attr(feature = "safe_io", forbid(unsafe_code))]

mod duplex; // IoDuplex

#[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))))]
mod io; // Io

#[cfg(not(feature = "std"))]
mod no_std_define;
#[cfg(feature = "std")]
mod std_reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::duplex::*;

        #[cfg(any(feature = "std", all(not(feature = "std"), feature = "io")))]
        pub use super::io::*;

        crate::cfg_if! { if #[cfg(feature = "std")] {
            pub use super::std_reexports::*;
        } else {
            pub use super::no_std_define::*;
        }}
    }
}
