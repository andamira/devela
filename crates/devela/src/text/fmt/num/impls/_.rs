// Only macro definitions; no primitive type implementations are emitted here.
crate::mods_in! {
    mod int;
    mod float;
}
crate::mods_out! { // _hidden
    _hidden {
        #[doc(hidden)]
        pub use super::{
            int::__fmt_num_impl_int,
            float::__fmt_num_impl_float,
        };
    }
}
