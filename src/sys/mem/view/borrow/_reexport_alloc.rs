// devela::sys::mem::view::borrow::_reexport_alloc

use crate::{_reexport, _tags};

_reexport! { rust: alloc::borrow, location: "sys/mem", tag: _tags!(mem lifetime value),
    doc: "A clone-on-write smart pointer.",
    Cow
}
_reexport! { rust: alloc::borrow, location: "sys/mem", tag: _tags!(mem value),
    doc: "A generalization of Clone to borrowed data.",
    ToOwned
}
