// devela::code::util::reexports
//
//! Reexported macros and hints.
//

#![allow(unused_imports)]

use crate::reexport;

pub use devela_base::{CONST, is, items, sf};

/// <span class="stab portability" title="re-exported from the `devela_macros`
/// crate">`devela_macros`</span>
pub use devela_macros::{
    cif, coalesce, compile, compile_attr, compile_doc, enumint, field_of, ident_total,
    ident_total_unique, ident_unique,
};

/* `core::hint` functions re-exports */

reexport! { rust: core::hint,
doc: "Makes a *soundness* promise to the compiler that the `cond`ition holds.", assert_unchecked }
reexport! { rust: core::hint,
doc: "Hints the compiler to be maximally pessimistic about what black_box could do.", black_box }
reexport! { rust: core::hint,
doc: "Signals the processor that it is running in a busy-wait spin-loop.", spin_loop }
reexport! { rust: core::hint,
doc: "Informs the compiler that the current calling site is not reachable.", unreachable_unchecked }

/* `core` macros re-exports */

// FLAG_DISABLED:nightly_autodiff
// reexport! { rust: core::autodiff, extra_flags:(nightly_autodiff),
// doc: "Automatic Differentiation macro.", autodiff }

// source code
reexport! { rust: core,
doc: "Expands to the column number at which it was invoked.", @column as code_column }
reexport! { rust: core,
doc: "Expands to the line number at which it was invoked.", @line as code_line }
reexport! { rust: core,
doc: "Expands to the file name at which it was invoked.", @file as code_file }
reexport! { rust: core,
doc: "Expands to a string representing the current module path.", @module_path as code_module }

// assert
reexport! { rust: core,
doc: "Asserts that a boolean expression is true at runtime.", assert }
reexport! { rust: core,
doc: "Asserts that two expressions are equal to each other.", assert_eq }
reexport! { rust: core,
doc: "Asserts that two expressions are not equal to each other.", assert_ne }
reexport! { rust: core,
doc: "Asserts that a boolean expression is true at runtime.", debug_assert }
reexport! { rust: core,
doc: "Asserts that two expressions are equal to each other.", debug_assert_eq }
reexport! { rust: core,
doc: "Asserts that two expressions are not equal to each other.", debug_assert_ne }

// cfg
reexport! { rust: core,
doc: "Evaluates boolean combinations of configuration flags at compile-time.", cfg }

// include
reexport! { rust: core,
doc: "Parses a file as an expression or an item according to the context.", include }
reexport! { rust: core,
doc: "Includes a file as a reference to a byte array.", include_bytes }
reexport! { rust: core, tag: crate::TAG_TEXT!(),
doc: "Includes a UTF-8 encoded file as a string.", include_str }

// concatenating
reexport! { rust: core, doc: "Concatenates literals into a static string slice.", concat }
reexport! { rust: core, doc: "Stringifies its arguments.", stringify }
// WAIT: [concat_idents](https://github.com/rust-lang/rust/issues/29599)
// reexport! { rust: core, doc: "Concatenates identifiers into one identifier.", concat_idents }
// WAIT: [concat_bytes](https://github.com/rust-lang/rust/issues/87555)
// reexport! { rust: core, doc: "Concatenates literals into a byte slice.", concat_bytes }
