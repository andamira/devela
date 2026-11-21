crate::mods_in! {
    #[cfg(all(test, feature = "angle"))]
    mod _test_int;

    mod core_traits;
    #[cfg(feature = "angle")]
    mod int;
    #[cfg(feature = "angle")]
    mod float;
}
