// devela/src/data/topol/graph/adj/impls/_.rs

crate::mods_in! {
    mod array;
    mod vec;
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__graph_adj_impl_array,
            vec::__graph_adj_impl_vec,
        };
    }
}
