// devela/src/data/store/pool/impls/_.rs

crate::mods_in! {
    mod array;
    mod vec;
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__pool_impl_array,
            vec::__pool_impl_vec,
        };
    }
}
