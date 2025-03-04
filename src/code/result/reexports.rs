// devela::code::result::reexports
//
//! Reexported result-related items.
//

use crate::reexport;

/* `core` re-exports */

reexport! { rust: core::result,
    tag: crate::TAG_RESULT!(),
    doc: "A type that represents either success ([`Ok`]) or failure ([`Err`]).",
    Result
}
