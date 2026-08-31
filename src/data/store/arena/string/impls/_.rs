// devela/src/data/store/arena/string/impls/_.rs

crate::mods_in! {
    mod array;
    mod vec;
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__arena_string_impl_array,
            vec::__arena_string_impl_vec,
        };
    }
}
