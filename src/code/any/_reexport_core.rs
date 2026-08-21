// devela/src/code/any/_reexport_core.rs

use crate::{_reexport, _tags};

_reexport! { rust: core::any,
    location: "code/any" => trait Any, tag: _tags!(code),
    doc: "A trait to emulate dynamic typing.",
    Any
}
_reexport! { rust: core::any,
    location: "code/any" => struct TypeId, tag: _tags!(code uid),
    doc: "Represents a globally unique identifier for a type.",
    TypeId
}
_reexport! { rust: core::any,
    location: "code/any" => fn any_type_name, tag: _tags!(code),
    doc: "Returns the name of a type as a string slice.",
    @type_name as any_type_name
}
