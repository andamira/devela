use crate::{_impl_init, _reexport, _tags, ConstInit};

/* impls */

_impl_init![<B: ConstInit, C: ConstInit> Self::Continue(C::INIT) => ControlFlow<B, C> ];

/* enums */

_reexport! { rust: core::ops,
    location: "code/ops" => enum ControlFlow, tag: _tags!(code),
    doc: "Used to tell an operation whether it should exit early or go on as usual.", ControlFlow
}

/* `core::clone` */

// NOTE: the following trait and the corresponding derive macro have the same name:
_reexport! { rust: core::clone,
    location: "code/ops" => trait Clone, tag: _tags!(value),
    doc: "A common trait that allows explicit creation of a duplicate value. (Derivable)", Clone
}
