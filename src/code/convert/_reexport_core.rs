// devela/src/code/convert/_reexport_core.rs
//
//! General reexported items, except macros and overloadable operators.
//

use crate::{_reexport, _tags};

/* `core::convert` */

// NOTE: Infallible is re-exported from code::result

// traits
_reexport! { rust: core::convert,
    location: "code" => trait AsMut, tag: _tags!(lifetime),
    doc: "Used to do a cheap mutable-to-mutable reference conversion.", AsMut
}
_reexport! { rust: core::convert,
    location: "code" => trait AsRef, tag: _tags!(lifetime),
    doc: "Used to do a cheap reference-to-reference conversion.", AsRef
}
_reexport! { rust: core::convert,
    location: "code" => trait From, tag: _tags!(value),
    doc: "Used to do value-to-value conversions while consuming the input value.", From
}
_reexport! { rust: core::convert,
    location: "code" => trait Into, tag: _tags!(value),
    doc: "A value-to-value conversion that consumes the input value.", Into
}
_reexport! { rust: core::convert,
    location: "code" => trait TryFrom, tag: _tags!(value),
    doc: "Simple and safe type conversions that may fail in a controlled way.", TryFrom
}
_reexport! { rust: core::convert,
    location: "code" => trait TryInto, tag: _tags!(value),
    doc: "An attempted conversion that consumes self, which may be expensive.", TryInto
}

// functions
_reexport! { rust: core::convert,
    location: "code" => fn identity, tag: _tags!(no),
    doc: "The identity function. Just returns back its input.", identity
}
