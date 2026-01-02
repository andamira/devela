// devela_base_core::code::any::_reexport

use crate::{_TAG_CODE, _TAG_UID, _reexport};

_reexport! { rust: core::any,
    location: "code/any",
    tag: _TAG_CODE!(),
    doc: "A trait to emulate dynamic typing.",
    Any
}
_reexport! { rust: core::any,
    location: "code/any",
    tag: _TAG_CODE!() _TAG_UID!(),
    doc: "Represents a globally unique identifier for a type.",
    TypeId
}
_reexport! { rust: core::any,
    location: "code/any",
    tag: _TAG_CODE!(),
    doc: "Returns the name of a type as a string slice.",
    @type_name as any_type_name
}
