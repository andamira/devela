// devela::code::reexports
//
//! General reexported items, except macros and overloadable operators.
//

#![allow(unused_imports)]

use crate::reexport;

/* `core::clone` re-exports */

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::clone::Clone;

/* `core::convert` re-exports */

// enums
reexport! { rust: core::convert,
doc: "The error type for errors that can never happen.", Infallible }

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
doc: "Simple and safe type conversions that may fail in a controlled way.", TryFrom }
reexport! { rust: core::convert,
doc: "An attempted conversion that consumes self, which may or may not be expensive.",
    TryInto }

// functions
reexport! { rust: core::convert,
doc: "The identity function. Just returns back its input.", identity }

/* `core::default` re-exports */

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::default::Default;
