// devela/src/data/store/arena/bytes/impls/_.rs

crate::mods_in! {
    mod array; // __arena_bytes_impl_array!
    mod vec; // __arena_bytes_impl_vec!
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__arena_bytes_impl_array,
            vec::__arena_bytes_impl_vec,
        };
    }
}
