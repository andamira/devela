// devela/src/yard/_doc/vendor.rs
//
//! Defines [`_doc_vendor!`].
//

#[doc = crate::_tags!(internal)]
/// Shows the `Vendored` doc section and links to the info line.
#[doc = crate::_doc_meta!{location("yard")}]
///
/// See the documentation for [vendored work].
#[doc = crate::doclink!(custom devela "[vendored work]" "_doc/vendored" @mod)]
///
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_vendor· {
    (
    // Shows the `Vendored` doc section and links to the info line.
    //
    // $crate_id: the crate's name and html id anchor on the docs.
    $crate_id:literal) => {
        concat!("\n\n# Vendored\n\nThis is adapted work from [", $crate_id, "](",
            $crate::doclink![custom devela "_doc/vendored" @mod],
            "#", $crate_id, ").\n\n"
        )
    };
    // MAYBE
    // (
    // // Shows the `Vendored` doc section and links to the *modifications* module.
    // $crate_id:literal, module:$mod_id:ident) => { concat!(
    //     "\n\n# Vendored\n\nThis is adapted work from [",
    //     $crate_id, "][crate::_doc::vendored::", $mod_id, "].\n\n"
    // )};

    (
    // Assumes the path is in current directory. Used in `_doc/vendored`.
    //
    // $crate_id:  the crate's name and html id anchor on the docs.
    // $text_path: the path to the text file to include, explaining the modifications.
    //
    // MAYBE: link to crate
    // MAYBE: add more information
    mod: $crate_id:literal, $mod_id:ident) => {
        #[doc = concat!(
            "# `", $crate_id,
            "` modifications\n\n[*(↑)*][crate::_doc::vendored#", $crate_id, "] ",
            include_str!(concat!("./", $crate_id, ".md"))
        )]
        pub mod $mod_id {}
    };
    // (
    // // Does not assume the path. TEMP: unused
    //
    // // $crate_id:  the crate's name and html id anchor on the docs.
    // // $text_path: the path to the text file to include, explaining the modifications.
    // mod: $crate_id:literal, $text_path:literal) => { concat!(
    //     "# ", $crate_id, "\n\n[*(↑)*][crate::_doc::vendored#", $crate_id, "] ",
    //     include_str!($text_path),
    // )};
}
#[doc(inline)]
pub use _doc_vendor· as _doc_vendor;
