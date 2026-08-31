// devela/src/data/layout/buffer/linear/impls/_.rs

crate::mods_in! {
    mod array;
    mod uninit;
    mod option;
    //
    mod r#mut;
    mod slice;
    //
    mod vec;
}
crate::mods_out! { // _hidden
    _hidden {
        pub use super::{
            array::__buffer_linear_impl_array,
            uninit::__buffer_linear_impl_uninit,
            option::__buffer_linear_impl_option,
            r#mut::__buffer_linear_impl_slice_mut,
            slice::__buffer_linear_impl_slice,
            vec::__buffer_linear_impl_vec,
        };
    }
}
