// devela::code::util::_reexport_core
//
//! Reexported macros and hints.
//

use crate::{_reexport, _tags};

/* `core::hint` functions */
// https://doc.rust-lang.org/stable/core/hint/

_reexport! { rust: core::hint, location: "code/util", tag: _tags!(assert),
doc: "Makes a *soundness* promise to the compiler that the `cond`ition holds.", assert_unchecked }
_reexport! { rust: core::hint, location: "code/util", tag: _tags!(code),
doc: "Hints the compiler to be maximally pessimistic about what black_box could do.", black_box }
_reexport! { rust: core::hint, location: "code/util", tag: _tags!(code),
doc: "Hints to the compiler that given path is cold, i.e., unlikely to be taken.", cold_path }
_reexport! { rust: core::hint, location: "code/util", tag: _tags!(code),
doc: "Hints the compiler that the `condition` is branch-unpredictable.", select_unpredictable }
_reexport! { rust: core::hint, location: "code/util", tag: _tags!(code),
doc: "Signals the processor that it is running in a busy-wait spin-loop.", spin_loop }
_reexport! { rust: core::hint, location: "code/util", tag: _tags!(assert),
doc: "Informs the compiler that the current calling site is not reachable.", unreachable_unchecked }

/* `core` macros */

// FLAG_DISABLED:nightly_autodiff
// _reexport! { rust: core::autodiff, location: "code/util", extra_flags:(nightly_autodiff),
// doc: "Automatic Differentiation macro.", autodiff }

// source code
_reexport! { rust: core, location: "code/util", tag: _tags!(code),
doc: "Expands to the column number at which it was invoked.", @column as code_column }
_reexport! { rust: core, location: "code/util", tag: _tags!(code),
doc: "Expands to the line number at which it was invoked.", @line as code_line }
_reexport! { rust: core, location: "code/util", tag: _tags!(code),
doc: "Expands to the file name at which it was invoked.", @file as code_file }
_reexport! { rust: core, location: "code/util", tag: _tags!(code),
doc: "Expands to a string representing the current module path.", @module_path as code_module }

// assert
_reexport! { rust: core, location: "code/util", tag: _tags!(assert),
doc: "Asserts that a boolean expression is true at runtime.", assert }
_reexport! { rust: core, location: "code/util", tag: _tags!(assert),
doc: "Asserts that two expressions are equal to each other.", assert_eq }
_reexport! { rust: core, location: "code/util", tag: _tags!(assert),
doc: "Asserts that two expressions are not equal to each other.", assert_ne }
_reexport! { rust: core, location: "code/util", tag: _tags!(assert),
doc: "Asserts that a boolean expression is true at runtime.", debug_assert }
_reexport! { rust: core, location: "code/util", tag: _tags!(assert),
doc: "Asserts that two expressions are equal to each other.", debug_assert_eq }
_reexport! { rust: core, location: "code/util", tag: _tags!(assert),
doc: "Asserts that two expressions are not equal to each other.", debug_assert_ne }

// cfg
_reexport! { rust: core, location: "code/util",
doc: "Evaluates boolean combinations of configuration flags at compile-time.", cfg }
_reexport! { rust: core, location: "code/util",
doc: "Selects code at compile-time based on cfg predicates.", cfg_select }

// include
_reexport! { rust: core, location: "code/util", tag: _tags!(code),
doc: "Parses a file as an expression or an item according to the context.", include }
_reexport! { rust: core, location: "code/util", tag: _tags!(code),
doc: "Includes a file as a reference to a byte array.", include_bytes }
_reexport! { rust: core, location: "code/util", tag: _tags!(code text),
doc: "Includes a UTF-8 encoded file as a string.", include_str }

// concatenating
_reexport! { rust: core, location: "code/util", tag: _tags!(code text),
doc: "Concatenates literals into a static string slice.", concat } // NOTE: needs direct tags
_reexport! { rust: core, location: "code/util", tag: _tags!(code text),
doc: "Stringifies its arguments.", stringify } // NOTE: needs direct tags

// WAIT: [concat_idents](https://github.com/rust-lang/rust/issues/29599)
// _reexport! { rust: core, doc: "Concatenates identifiers into one identifier.", concat_idents }
// WAIT: [concat_bytes](https://github.com/rust-lang/rust/issues/87555)
// _reexport! { rust: core, doc: "Concatenates literals into a byte slice.", concat_bytes }
