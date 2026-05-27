// devela::yard::_doc::availability
//
//! Defines [`_doc_availability!`].
//

#[doc = crate::_tags!(internal)]
/// Generates a formatted documentation string for conditional availability.
#[doc = crate::_doc_meta!{location("yard")}]
///
/// It's intended to be used like this:
/// ```txt
/// #[doc = crate::doc_availability!(feature = "one")]
/// #[doc = crate::doc_availability!(all(feature = "one", feature = "two")]
/// #[doc = crate::doc_availability!(any(feature = "one", feature = "two")]
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_availability· {
    (feature = $feat:literal) => {
        $crate::_doc_availability!{@wrap
            "Available on <strong>crate feature ",
            $crate::_doc_availability!{@code $feat},
            "</strong> only."
        }
    };

    // Handle one or more required features
    ( all( $(feature = $feat:literal),+ ) ) => {
        $crate::_doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::_doc_availability!{@join_features_and $($feat),+},
            "</strong> only."
        }
    };
    // Handle one or more alternative features
    ( any( $(feature = $feat:literal),+ ) ) => {
        $crate::_doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::_doc_availability!{@join_features_or $($feat),+},
            "</strong> only."
        }
    };

    /* helper arms */

    // Outer wrap
    (@wrap $($strings:tt)+) => {
        concat!(
            "<div class='item-info' style='margin-left:0;'>",
            "<div class='stab portability'>",
            $($strings)+,
            "</div></div>"
        )
    };

    // code-formatted literal
    (@code $string:literal) => {
        concat!("<code style='background:none'>", $string, "</code>")
    };

    // single arm for joining features with "and"
    (@join_features_and $first:literal $(, $rest:literal)*) => {
        concat!(
            $crate::_doc_availability!{@code $first}
            $(
                , " and ", $crate::_doc_availability!{@code $rest}
            )*
        )
    };
    // single arm for joining features with "or"
    (@join_features_or $first:literal $(, $rest:literal)*) => {
        concat!(
            $crate::_doc_availability!{@code $first}
            $(
                , " or ", $crate::_doc_availability!{@code $rest}
            )*
        )
    };
}
#[doc(inline)]
pub use _doc_availability· as _doc_availability;
