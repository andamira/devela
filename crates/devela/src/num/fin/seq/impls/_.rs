//
//! Macro emitters for individual sequence families.
//

crate::mods_in! {
    mod digit; // __seq_impl_digit!
    mod recurrence; // __seq_impl_recurrence!
}
crate::mods_out! { // _hidden
    _hidden {
        #[doc(hidden)]
        pub use super::{
            digit::__seq_impl_digit,
            recurrence::__seq_impl_recurrence,
        };
    }
}
