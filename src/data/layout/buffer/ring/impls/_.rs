// devela/src/data/layout/buffer/ring/impls/_.rs

crate::mods_in! {
    mod array;
    // mod uninit;
    mod option;
    //
    // mod r#mut;
    // mod slice;
    //
    // mod vec;
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__buffer_ring_impl_array,
            // uninit::__buffer_ring_impl_uninit,
            option::__buffer_ring_impl_option,
            //
            // r#mut::__buffer_ring_impl_slice_mut,
            // slice::__buffer_ring_impl_slice,
            //
            // vec::__buffer_ring_impl_vec,
        };
    }
}
