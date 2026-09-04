// devela/src/sys/mem/size/expr/_.rs

crate::mods_in! {
    #[cfg(all(test, nightly_coro))]
    mod _test_coro;

    mod define;
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::define::size_of_expr;
    }
    _hidden {
        pub use super::define::__size_of_expr;
    }
}
