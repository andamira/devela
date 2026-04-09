// devela::code::_reexport_core
//
//! General reexported items, except macros and overloadable operators.
//

use crate::{_reexport, _tags};

/* `core::clone` */

// NOTE: the trait and the derive macro have the same name
_reexport! { rust: core::clone, location: "code", tag: _tags!(value),
doc: "A common trait that allows explicit creation of a duplicate value. (Derivable)", Clone }

/* `core::convert` */

// enums
_reexport! { rust: core::convert, location: "code", tag: _tags!(no error),
doc: "The error type for errors that can never happen.", Infallible }

// traits
_reexport! { rust: core::convert, location: "code", tag: _tags!(lifetime),
doc: "Used to do a cheap mutable-to-mutable reference conversion.", AsMut }
_reexport! { rust: core::convert, location: "code", tag: _tags!(lifetime),
doc: "Used to do a cheap reference-to-reference conversion.", AsRef }
_reexport! { rust: core::convert, location: "code", tag: _tags!(value),
doc: "Used to do value-to-value conversions while consuming the input value.", From }
_reexport! { rust: core::convert, location: "code", tag: _tags!(value),
doc: "A value-to-value conversion that consumes the input value.", Into }
_reexport! { rust: core::convert, location: "code", tag: _tags!(value),
doc: "Simple and safe type conversions that may fail in a controlled way.", TryFrom }
_reexport! { rust: core::convert, location: "code", tag: _tags!(value),
doc: "An attempted conversion that consumes self, which may be expensive.", TryInto }

// functions
_reexport! { rust: core::convert, location: "code", tag: _tags!(no),
doc: "The identity function. Just returns back its input.", identity }

/* `core::default` */

// NOTE: the trait and the derive macro have the same name
_reexport! { rust: core::default, location: "code", tag: _tags!(init),
doc: "A trait for giving a type a useful default value. (Derivable)", Default }
