// devela_base_core::sys::mem::view::borrow::_reexport

use crate::{_reexport, _tags};

_reexport! { rust: core::borrow, location: "sys/mem/view", tag: _tags!(lifetime),
    doc: "A trait for borrowing data.",
    Borrow
}
_reexport! { rust: core::borrow, location: "sys/mem/view", tag: _tags!(lifetime),
    doc: "A trait for mutably borrowing data.",
    BorrowMut
}
