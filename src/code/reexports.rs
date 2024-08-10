// devela::code::reexport
//
//! Reexported items.
//

#![allow(unused_imports)]

use super::reexport;

/// <span class="stab portability" title="re-exported from `devela_macros`
/// crate">`devela_macros`</span>
pub use devela_macros::{
    cif, coalesce, compile, compile_attr, compile_doc, ident_total, ident_total_unique,
    ident_unique,
};

pub use super::paste::paste;

/* `core` re-exports */

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
reexport! { rust: core,
doc: "Includes a UTF-8 encoded file as a string.", include_str }

/* `core::convert` re-exports */

// traits
reexport! { rust: core::convert,
doc: "Used to do a cheap mutable-to-mutable reference conversion.", AsMut }
reexport! { rust: core::convert,
doc: "Used to do a cheap reference-to-reference conversion.", AsRef }
reexport! { rust: core::convert,
doc: "Used to do value-to-value conversions while consuming the input value.", From }
reexport! { rust: core::convert,
doc: "A value-to-value conversion that consumes the input value.", Into }
reexport! { rust: core::convert,
doc: "Simple and safe type conversions that may fail in a controlled way under some circumstances",
TryFrom }
reexport! { rust: core::convert,
doc: "An attempted conversion that consumes self, which may or may not be expensive.", TryInto }

// functions
reexport! { rust: core::convert,
doc: "The identity function.", identity }

/* `core::default` re-exports */

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::default::Default;

/* `core::hint` re-exports */

reexport! { rust: core::hint,
doc: "Hints the compiler to be maximally pessimistic about what black_box could do.", black_box }
reexport! { rust: core::hint,
doc: "Signals the processor that it is running in a busy-wait spin-loop.", spin_loop }
reexport! { rust: core::hint,
doc: "Informs the compiler that the current calling site is not reachable.", unreachable_unchecked }

/* `core::clone` re-exports */

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::clone::Clone;

/* `core::marker` re-exports */

/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
// <https://doc.rust-lang.org/nomicon/phantom-data.html#table-of-phantomdata-patterns>
pub use core::marker::PhantomData;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::PhantomPinned;

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Copy;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Send;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Sized;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Sync;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Unpin;

/* `core::result` re-exports */

#[cfg(feature = "std")]
pub use std::*;
#[cfg(feature = "std")]
mod std {
    use super::reexport;

    // In sync with define_no_std_error::Error
    reexport! { rust: not(std)|std::error,
        doc: "A trait representing the basic expectations for error values.",
        Error
    }
}
