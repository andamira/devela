// devela/src/data/topol/graph/csr/impls/_.rs

crate::mods_in! {
    mod array;
    mod vec;
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__graph_csr_impl_array,
            vec::__graph_csr_impl_vec,
        };
    }
}
