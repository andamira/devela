// devela::code::util::likely
//
//! Defines [`likely`], [`unlikely`].
//

use crate::cold_path;

#[doc = crate::_tags!(code)]
/// Indicate that a given `condition` is likely to be true.
#[doc = crate::_doc_location!("code/util")]
// WAIT: [likely_unlikely](https://github.com/rust-lang/rust/issues/151619)
pub const fn likely(condition: bool) -> bool {
    if !condition {
        cold_path();
    }
    condition
}

#[doc = crate::_tags!(code)]
/// Indicate that a given `condition` is likely to be false.
#[doc = crate::_doc_location!("code/util")]
// WAIT: [likely_unlikely](https://github.com/rust-lang/rust/issues/151619)
pub const fn unlikely(condition: bool) -> bool {
    if condition {
        cold_path();
    }
    condition
}
