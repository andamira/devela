// devela::_info
//
//! Extra information about the library.
//

#![cfg(any(doc, test))]
#![cfg_attr(feature = "nightly_doc", doc(cfg(doc)))]

#[cfg(feature = "std")]
#[path = "../../build/mod.rs"]
mod build;

/// Documented examples.
pub mod examples;

/// Library features.
#[cfg(doc)]
pub mod features {
    #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./features.md")]
}

/// Vendored work.
///
// In sync with:
#[doc = include_str!("../../DOCS/VENDORED_rustdoc.md")] // has to come before
#[doc = include_str!("../../DOCS/VENDORED.md")]
pub mod vendored {
    /// Detailed list of modifications in adapted work.
    //
    // Apache-2.0 or MIT:
    #[doc = crate::doc_!(vendor_mod: "etcetera", "../sys/env/app/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "fxhash", "../data/codec/hash/fx/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "no_std_io", "../sys/io/define_no_std_io/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "numtoa", "../text/fmt/num_to_str/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "pollster", "../work/future/block/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "quickdiv", "../num/int/divisor/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "stack_dst", "../data/dst/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "stated-scope-guard", "../code/guard/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "static_assertions", "../code/util/asserts/static/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "tailcall-chunk", "../data/list/array/vec/chunk/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "tupl", "../data/list/tuple/MODIFICATIONS.md")]
    // MIT:
    #[doc = crate::doc_!(vendor_mod: "crunchy", "../code/util/unroll/MODIFICATIONS.md")]
    #[doc = crate::doc_!(vendor_mod: "object-id", "../data/uid/pin/MODIFICATIONS.md")]
    // other:
    // #[doc = crate::doc_!(vendor_mod: "blit-fonts", "../media/font/bitmap/MODS_BLIT.md")] // WIP
    #[doc = crate::doc_!(vendor_mod: "pengyhash", "../data/codec/hash/pengy/MODIFICATIONS.md")]
    pub mod modifications {}
}
