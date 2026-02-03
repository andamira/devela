// devela_base_core::sys::mem::view::borrow::_reexport

use crate::{_TAG_LIFETIME, _reexport};

_reexport! { rust: core::borrow, location: "sys/mem", tag: _TAG_LIFETIME!(),
    doc: "A trait for borrowing data.",
    Borrow
}
_reexport! { rust: core::borrow, location: "sys/mem", tag: _TAG_LIFETIME!(),
    doc: "A trait for mutably borrowing data.",
    BorrowMut
}
