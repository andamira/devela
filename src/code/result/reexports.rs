// devela::code::result::reexports
//
//! Reexported result-related items.
//

use crate::{TAG_ITERATOR, TAG_NICHE, TAG_RESULT, reexport};

/* `core::option` re-exports */

reexport! { rust: core::option,
    tag: TAG_NICHE!(),
    doc: "A type that represents an optional value.",
    Option
}
reexport! { rust: core::option,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over the value in [`Some`] variant of an [`Option`].",
    @IntoIter as OptionIntoIter
}
reexport! { rust: core::option,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over a reference to the [`Some`] variant of an [`Option`].",
    @Iter as OptionIter
}
reexport! { rust: core::option,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over a mutable reference to the [`Some`] variant of an [`Option`].",
    @IterMut as OptionIterMut
}

/* `core::result` re-exports */

reexport! { rust: core::result,
    tag: TAG_RESULT!(),
    doc: "A type that represents either success ([`Ok`]) or failure ([`Err`]).",
    Result
}
reexport! { rust: core::result,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over the value in [`Ok`] variant of a [`Result`].",
    @IntoIter as ResultIntoIter
}
reexport! { rust: core::result,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over a reference to the [`Ok`] variant of a [`Result`].",
    @Iter as ResultIter
}
reexport! { rust: core::result,
    tag: TAG_ITERATOR!(),
    doc: "An iterator over a mutable reference to the [`Ok`] variant of a [`Result`].",
    @IterMut as ResultIterMut
}
