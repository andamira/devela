// devela/src/yard/_doc/warn.rs
//
//! Defines [`_doc_warn_miri`].
//

#[doc = crate::_tags!(internal)]
/// Generates a formatted documentation string for a miri warning.
#[doc = crate::_doc_meta!{location("yard")}]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_warn_miri· {
    (tag) => {
        concat!(
            "<span class='stab portability' ",
            "title='Fails to compile with Miri.'>",
            "<code>⚠️</code></span>"
        )
    };
    (body $(, url: $url:literal)?) => {
        concat!(
            "<div class='warning'>",
            "Fails to compile with Miri.",
            $( "<p><em>See <a href = '", $url, "'>", $url, "</a>.</em></p>", )?
            "</div>"
        )
    };
}
#[doc(inline)]
pub use _doc_warn_miri· as _doc_warn_miri;
