// devela_base_alloc::sys::mem::borrow::_reexport

use crate::{_TAG_LIFETIME, _TAG_MEM, _TAG_VALUE, _reexport};

_reexport! { rust: alloc::borrow, location: "sys/mem",
    tag: _TAG_MEM!() _TAG_LIFETIME!() _TAG_VALUE!(),
    doc: "A clone-on-write smart pointer.",
    Cow
}
_reexport! { rust: alloc::borrow, location: "sys/mem",
    tag: _TAG_MEM!() _TAG_VALUE!(),
    doc: "A generalization of Clone to borrowed data.",
    ToOwned
}
