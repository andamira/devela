// devela::code::result::reexports
//
//! Reexported result-related items.
//

use crate::{TAG_ITERATOR, TAG_NICHE, TAG_RESULT, _reexport};

/* `core::option` re-exports */

_reexport! { rust: core::option,
    tag: TAG_NICHE!(),
    doc: "A type that represents an optional value.",
    Option
}
_reexport! { rust: core::option,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over the value in [`Some`] variant of an [`Option`].",
    @IntoIter as OptionIntoIter
}
_reexport! { rust: core::option,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over a reference to the [`Some`] variant of an [`Option`].",
    @Iter as OptionIter
}
_reexport! { rust: core::option,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over a mutable reference to the [`Some`] variant of an [`Option`].",
    @IterMut as OptionIterMut
}

/* `core::result` re-exports */

_reexport! { rust: core::result,
    tag: TAG_RESULT!(),
    doc: "A type that represents either success ([`Ok`]) or failure ([`Err`]).",
    Result
}
_reexport! { rust: core::result,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over the value in [`Ok`] variant of a [`Result`].",
    @IntoIter as ResultIntoIter
}
_reexport! { rust: core::result,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over a reference to the [`Ok`] variant of a [`Result`].",
    @Iter as ResultIter
}
_reexport! { rust: core::result,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over a mutable reference to the [`Ok`] variant of a [`Result`].",
    @IterMut as ResultIterMut
}
