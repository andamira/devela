// devela::code::result::_reexport_core

use crate::{_reexport, _tags};

#[doc = crate::_tags!(result no error)]
/// A result type whose error case can never occur.
#[doc = crate::_doc_meta!{location("code/result")}]
pub type InfallibleResult<T> = Result<T, Infallible>;

/* `core::convert` */

_reexport! { rust: core::convert, location: "code/result", tag: _tags!(no error),
doc: "The error type for errors that can never occur.", Infallible }

/* `core::option` */

_reexport! { rust: core::option, location: "code/result", tag: _tags!(niche),
    doc: "A type that represents an optional value.",
    Option
}

/* `core::result` */

_reexport! { rust: core::result, location: "code/result", tag: _tags!(result),
    doc: "A type that represents either success ([`Ok`]) or failure ([`Err`]).",
    Result
}
