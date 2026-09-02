// devela/src/text/unicode/scalar/scalar/_.rs
//
//!
//

crate::mods_in! {
    mod define; // char7, char8, char16, charu, charu_niche, ch

    mod shared; // shared methods
    mod traits; // common traits

    // specific implementations
    mod c16;
    mod c7;
    mod c8;
    mod utf8;
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            define::{char7, char8, char16, charu, charu_niche, ch},
        };
    }
    _crate_internals { // IMPROVE: _self_internals
        pub(crate) use super::define::{
            NonSurrogateU16,
        };
    }
}

mod impls {
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_layout"))]
    unsafe impl crate::MemPod for crate::char8 {}
}
