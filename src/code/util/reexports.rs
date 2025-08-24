// devela::code::util::reexports
//
//! Reexported macros and hints.
//

use crate::_reexport;

#[doc = crate::TAG_DEVELA_BASE!()]
#[doc(inline)] #[rustfmt::skip]
pub use devela_base::{
    CONST,
    cdbg,
    cfg_if,
    define_error,
    deprecate_feature,
    include_from, mod_from,
    is,
    items, sf,
    paste,
    // devela_code_macros:
    cif, compile, compile_attr, compile_doc,
    ident_total, ident_total_unique, ident_unique,
    coalesce, field_of,
};

#[cfg(feature = "devela_macros")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "devela_macros")))]
#[doc = crate::TAG_DEVELA_MACROS!()]
#[doc(inline)] #[rustfmt::skip]
pub use devela_macros::{
    enumint,
};

/* `core::hint` functions re-exports */

_reexport! { rust: core::hint,
doc: "Makes a *soundness* promise to the compiler that the `cond`ition holds.", assert_unchecked }
_reexport! { rust: core::hint,
doc: "Hints the compiler to be maximally pessimistic about what black_box could do.", black_box }
_reexport! { rust: core::hint,
doc: "Signals the processor that it is running in a busy-wait spin-loop.", spin_loop }
_reexport! { rust: core::hint,
doc: "Informs the compiler that the current calling site is not reachable.", unreachable_unchecked }

/* `core` macros re-exports */

// FLAG_DISABLED:nightly_autodiff
// _reexport! { rust: core::autodiff, extra_flags:(nightly_autodiff),
// doc: "Automatic Differentiation macro.", autodiff }

// source code
_reexport! { rust: core,
doc: "Expands to the column number at which it was invoked.", @column as code_column }
_reexport! { rust: core,
doc: "Expands to the line number at which it was invoked.", @line as code_line }
_reexport! { rust: core,
doc: "Expands to the file name at which it was invoked.", @file as code_file }
_reexport! { rust: core,
doc: "Expands to a string representing the current module path.", @module_path as code_module }

// assert
_reexport! { rust: core,
doc: "Asserts that a boolean expression is true at runtime.", assert }
_reexport! { rust: core,
doc: "Asserts that two expressions are equal to each other.", assert_eq }
_reexport! { rust: core,
doc: "Asserts that two expressions are not equal to each other.", assert_ne }
_reexport! { rust: core,
doc: "Asserts that a boolean expression is true at runtime.", debug_assert }
_reexport! { rust: core,
doc: "Asserts that two expressions are equal to each other.", debug_assert_eq }
_reexport! { rust: core,
doc: "Asserts that two expressions are not equal to each other.", debug_assert_ne }

// cfg
_reexport! { rust: core,
doc: "Evaluates boolean combinations of configuration flags at compile-time.", cfg }

// include
_reexport! { rust: core,
doc: "Parses a file as an expression or an item according to the context.", include }
_reexport! { rust: core,
doc: "Includes a file as a reference to a byte array.", include_bytes }
_reexport! { rust: core, tag: crate::TAG_TEXT!(),
doc: "Includes a UTF-8 encoded file as a string.", include_str }

// concatenating
_reexport! { rust: core, doc: "Concatenates literals into a static string slice.", concat }
_reexport! { rust: core, doc: "Stringifies its arguments.", stringify }
// WAIT: [concat_idents](https://github.com/rust-lang/rust/issues/29599)
// _reexport! { rust: core, doc: "Concatenates identifiers into one identifier.", concat_idents }
// WAIT: [concat_bytes](https://github.com/rust-lang/rust/issues/87555)
// _reexport! { rust: core, doc: "Concatenates literals into a byte slice.", concat_bytes }
