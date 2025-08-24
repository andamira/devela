// devela_base::code::reexports
//
//! General reexported items, except macros and overloadable operators.
//

use crate::_reexport;

/* `core::clone` re-exports */

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::clone::Clone;

/* `core::convert` re-exports */

// enums
_reexport! { rust: core::convert,
tag: crate::TAG_ERROR!(),
doc: "The error type for errors that can never happen.", Infallible }

// traits
_reexport! { rust: core::convert,
doc: "Used to do a cheap mutable-to-mutable reference conversion.", AsMut }
_reexport! { rust: core::convert,
doc: "Used to do a cheap reference-to-reference conversion.", AsRef }
_reexport! { rust: core::convert,
doc: "Used to do value-to-value conversions while consuming the input value.", From }
_reexport! { rust: core::convert,
doc: "A value-to-value conversion that consumes the input value.", Into }
_reexport! { rust: core::convert,
doc: "Simple and safe type conversions that may fail in a controlled way.", TryFrom }
_reexport! { rust: core::convert,
doc: "An attempted conversion that consumes self, which may or may not be expensive.",
    TryInto }

// functions
_reexport! { rust: core::convert, tag: crate::TAG_NO!(),
doc: "The identity function. Just returns back its input.", identity }

/* `core::default` re-exports */

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use ::core::default::Default;
