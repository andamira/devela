// devela/src/data/store/pool/seq/impls/_.rs

crate::mods_in! {
    mod array;
    // mod vec; // TODO
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__pool_seq_impl_array,
            // vec::__pool_seq_impl_vec,
        };
    }
}
