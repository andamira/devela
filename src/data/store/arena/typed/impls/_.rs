// devela/src/data/store/arena/typed/impls/_.rs

crate::mods_in! {
    mod array; // __arena_impl_array!
    mod vec; // __arena_impl_vec!
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__arena_impl_array,
            vec::__arena_impl_vec,
        };
    }
}
