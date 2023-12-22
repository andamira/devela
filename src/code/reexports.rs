// devela::code::reexport
//
//! Reexported items.
//

use super::reexport;

/// <span class="stab portability" title="re-exported from `devela_macros`
/// crate">`devela_macros`</span>
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
pub use devela_macros::{cif, coalesce, compile, compile_attr};

/// <span class="stab portability" title="re-exported from the `paste crate`">`paste`</span>
pub use super::paste::paste;

/* `core` reexports */

// source code
reexport! { rust: core, local_module: "code",
doc: "Expands to the column number at which it was invoked.", @column as code_column }
reexport! { rust: core, local_module: "code",
doc: "Expands to the line number at which it was invoked.", @line as code_line }
reexport! { rust: core, local_module: "code",
doc: "Expands to the file name at which it was invoked.", @file as code_file }
reexport! { rust: core, local_module: "code",
doc: "Expands to a string representing the current module path.", @module_path as code_module }

// assert
reexport! { rust: core, local_module: "code",
doc: "Asserts that a boolean expression is true at runtime.", assert }
reexport! { rust: core, local_module: "code",
doc: "Asserts that two expressions are equal to each other.", assert_eq }
reexport! { rust: core, local_module: "code",
doc: "Asserts that two expressions are not equal to each other.", assert_ne }
reexport! { rust: core, local_module: "code",
doc: "Asserts that a boolean expression is true at runtime.", debug_assert }
reexport! { rust: core, local_module: "code",
doc: "Asserts that two expressions are equal to each other.", debug_assert_eq }
reexport! { rust: core, local_module: "code",
doc: "Asserts that two expressions are not equal to each other.", debug_assert_ne }

// cfg
reexport! { rust: core, local_module: "code",
doc: "Evaluates boolean combinations of configuration flags at compile-time.", cfg }

// include
reexport! { rust: core, local_module: "code",
doc: "Parses a file as an expression or an item according to the context.", include }
reexport! { rust: core, local_module: "code",
doc: "Includes a file as a reference to a byte array.", include_bytes }
reexport! { rust: core, local_module: "code",
doc: "Includes a UTF-8 encoded file as a string.", include_str }

/* `core::hint` reexports */

reexport! { rust: core::hint, local_module: "code",
doc: "Hints the compiler to be maximally pessimistic about what black_box could do.", black_box }
reexport! { rust: core::hint, local_module: "code",
doc: "Signals the processor that it is running in a busy-wait spin-loop.", spin_loop }
reexport! { rust: core::hint, local_module: "code",
doc: "Informs the compiler that the current calling site is not reachable.", unreachable_unchecked }
