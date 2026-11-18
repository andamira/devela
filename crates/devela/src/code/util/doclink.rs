//
//! Public facade for the bootstrapped `doclink!` macro.
//!
//! Normal builds re-export the implementation directly. Rustdoc uses a
//! forwarding macro to retain the public documentation here.
//

crate::CONST! { hidden macro_export,
    /// Documentation for the `doclink!` macro.
    _DOC_DOCLINK = "\n\nConstructs a URL to Rust documentation.

Module links use `@mod`. Exact item links use `@item kind Item`, allowing
`doclink!` to construct the rustdoc filename without requiring callers to
encode names such as `struct.Item.html` manually.

Supported item kinds are:
`struct`, `enum`, `union`, `trait`, `type`, `fn`, `const`, `static`,
`macro`, `attr`, and `derive`.

The `custom` forms use two crate-root macros to construct published
documentation URLs:

- `__DOCLINK_CUSTOM_DOMAIN!` defines the documentation root and must expand
  to a string literal ending in `/`.
- `__DOCLINK_CUSTOM_CHANNEL!` defines the publication channel or version and
  must expand to a string literal surrounded by `/`, such as `\"/latest/\"` or `\"/wip/\"`.

For example:
```
#[macro_export] #[doc(hidden)]
macro_rules! __DOCLINK_CUSTOM_DOMAIN {
    () => { \"https://docs.rs/\" };
}

#[macro_export] #[doc(hidden)]
macro_rules! __DOCLINK_CUSTOM_CHANNEL {
    () => { \"latest\" };
}
```

Together these produce URLs shaped like:
`{domain}{crate}{channel}{crate}/…`.

Because these helpers are resolved from the current crate, crates
using the custom forms must provide both macros at their crate root.

# Features
With the `__publish` feature enabled, links target published documentation.
Otherwise they target the locally generated rustdoc tree.

The local forms require `CARGO_TARGET_DIR` to be defined.
In devela this is provided by `/build/main/environment.rs`.

# Examples
```
use devela::doclink;

/// Links to [`AnotherExample`] and [some module].
#[doc = doclink!(crate_name \"[`AnotherExample`]\" \"path/to/module\" @item struct AnotherExample)]
#[doc = doclink!(crate_name \"[some module]\" \"path/to/some_module\" @mod)]
///
/// [`Item`] is linked in the crate currently being compiled.
#[doc = doclink!(crate \"[`Item`]\" \"path/to/module\" @item struct Item)]
pub struct Example;
```
    ";
}

#[doc(hidden)]
pub use crate::boot::{__DOCLINK_CUSTOM_CHANNEL, __DOCLINK_CUSTOM_DOMAIN};

#[cfg(not(doc))]
pub use crate::boot::doclink;

#[cfg(doc)]
#[doc(hidden)]
#[doc = crate::_tags!(code)]
/// Helps doc-linking items in downstream crates.
#[doc = crate::_doc_meta!{
    location("code/util", macro doclink),
}]
#[doc = _DOC_DOCLINK!()]
#[macro_export]
macro_rules! __devela_doclink_docs {
    ($($tokens:tt)*) => { $crate::doclink·! { $($tokens)* } };
}
#[cfg(doc)]
#[doc(inline)]
pub use __devela_doclink_docs as doclink;
