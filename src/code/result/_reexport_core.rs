// devela::code::result::_reexport_core

use crate::{_reexport, _tags};

/* `core::option` */

_reexport! { rust: core::option, location: "code/result", tag: _tags!(niche),
    doc: "A type that represents an optional value.",
    Option
}
_reexport! { rust: core::option, location: "code/result", tag: _tags!(iterator),
    doc: "An iterator over the value in [`Some`] variant of an [`Option`].",
    @IntoIter as OptionIntoIter
}
_reexport! { rust: core::option, location: "code/result", tag: _tags!(iterator),
    doc: "An iterator over a reference to the [`Some`] variant of an [`Option`].",
    @Iter as OptionIter
}
_reexport! { rust: core::option, location: "code/result", tag: _tags!(iterator),
    doc: "An iterator over a mutable reference to the [`Some`] variant of an [`Option`].",
    @IterMut as OptionIterMut
}

/* `core::result` */

_reexport! { rust: core::result, location: "code/result", tag: _tags!(result),
    doc: "A type that represents either success ([`Ok`]) or failure ([`Err`]).",
    Result
}
_reexport! { rust: core::result, location: "code/result", tag: _tags!(iterator),
    doc: "An iterator over the value in [`Ok`] variant of a [`Result`].",
    @IntoIter as ResultIntoIter
}
_reexport! { rust: core::result, location: "code/result", tag: _tags!(iterator),
    doc: "An iterator over a reference to the [`Ok`] variant of a [`Result`].",
    @Iter as ResultIter
}
_reexport! { rust: core::result, location: "code/result", tag: _tags!(iterator),
    doc: "An iterator over a mutable reference to the [`Ok`] variant of a [`Result`].",
    @IterMut as ResultIterMut
}
