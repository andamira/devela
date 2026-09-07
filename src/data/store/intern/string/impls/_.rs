// devela/src/data/store/intern/string/impls/_.rs

crate::mods_in! {
    mod array;
    mod vec;
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__intern_string_impl_array,
            vec::__intern_string_impl_vec,
        };
    }
}
