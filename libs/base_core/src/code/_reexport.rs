// devela_base_core::code::_reexport
//
//! General reexported items, except macros and overloadable operators.
//

use crate::{_TAG_ERROR, _TAG_INIT, _TAG_LIFETIME, _TAG_NO, _TAG_VALUE, _reexport};

/* `core::clone` re-exports */

// NOTE: the trait and the derive macro have the same name
_reexport! { rust: core::clone, location: "code", tag: _TAG_VALUE!(),
doc: "A common trait that allows explicit creation of a duplicate value. (Derivable)", Clone }

/* `core::convert` re-exports */

// enums
_reexport! { rust: core::convert, location: "code", tag: _TAG_NO!() _TAG_ERROR!(),
doc: "The error type for errors that can never happen.", Infallible }

// traits
_reexport! { rust: core::convert, location: "code", tag: _TAG_LIFETIME!(),
doc: "Used to do a cheap mutable-to-mutable reference conversion.", AsMut }
_reexport! { rust: core::convert, location: "code", tag: _TAG_LIFETIME!(),
doc: "Used to do a cheap reference-to-reference conversion.", AsRef }
_reexport! { rust: core::convert, location: "code", tag: _TAG_VALUE!(),
doc: "Used to do value-to-value conversions while consuming the input value.", From }
_reexport! { rust: core::convert, location: "code", tag: _TAG_VALUE!(),
doc: "A value-to-value conversion that consumes the input value.", Into }
_reexport! { rust: core::convert, location: "code", tag: _TAG_VALUE!(),
doc: "Simple and safe type conversions that may fail in a controlled way.", TryFrom }
_reexport! { rust: core::convert, location: "code", tag: _TAG_VALUE!(),
doc: "An attempted conversion that consumes self, which may or may not be expensive.",
TryInto }

// functions
_reexport! { rust: core::convert, location: "code", tag: crate::_TAG_NO!(),
doc: "The identity function. Just returns back its input.", identity }

/* `core::default` re-exports */

// NOTE: the trait and the derive macro have the same name
_reexport! { rust: core::default, location: "code", tag: _TAG_INIT!(),
doc: "A trait for giving a type a useful default value. (Derivable)", Default }
