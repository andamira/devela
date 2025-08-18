// devela_base::code::util::_internal
//
//! Internal code-related utilities and macros.
//

mod tags; // EMOJI_*, TAG_*

crate::items! { // structural access: _internals
    #[allow(unused)]
    pub use _internals::*;

    pub(super) mod _internals {
        pub/*workspace*/ use super::tags::*;
    }
}
