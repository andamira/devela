// devela_base_core::code::util::unlikely
//
//! Defines [cold_path], [`likely`], [`unlikely`].
//

#[doc = crate::_tags!(code)]
/// Indicate that a given branch is **not** likely to be taken, relatively speaking.
#[doc = crate::_doc_location!("code/util")]
// WAIT: [cold_path](https://github.com/rust-lang/rust/issues/136873)
#[cold]
pub const fn cold_path() {}

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
