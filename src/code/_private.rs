// devela::code::_private
//
//! private meta helpers
//
// TOC
// - reexport!
// - doc_extends!

/// Macro helper for documentation of re-exported items.
macro_rules! reexport {
    /* re-exports from normal modules */

    // the following 4 arms allows reexporting items from:
    // `core`, `alloc`, `std` or `no_std`|`std`.
    //
    // - Supports multiple re-exported items.
    // - Renamed items must be all at the end, and each one prefixed with @.
    //
    // # Examples
    // ```
    // reexport! { rust: core::any, local_module: "any",
    //     "Represents a globally unique identifier for a type.", TypeId }
    // ```
    //
    // when the item is available in `core`
    ( rust : core $( :: $( $core_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      $( extra_features: $($extraf:literal),+ $(,)? )?
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from rust&#39;s "
        "`core`'>`core`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from [`core" $( "`]::[`" $( $core_path "::" )+ )?
            "`](https://doc.rust-lang.org/core/" $($( $core_path "/" )+)? ")*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(feature = "nightly_doc", doc(cfg(all(
            $( feature = $module_feature, )?
            $( all($(feature = $extraf),+) )?
        ))))]
        #[cfg(all(
            $( $(feature = $extraf),+ )?
        ))]
        pub use core :: $($( $core_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};
    // when the item is available in `alloc`
    ( rust : alloc $( :: $( $alloc_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[allow(rustdoc::broken_intra_doc_links)] // TEMP FIXME unresolved link to alloc
        #[doc = "<span class='stab portability' title='re-exported from rust&#39;s "
        "`alloc`'>`alloc`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from [`alloc" $( "`]::[`" $( $alloc_path "::" )+ )?
            "`](https://doc.rust-lang.org/alloc/" $($( $alloc_path "/" )+)? ")*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(
            feature = "nightly_doc",
            doc(cfg(all(
                $( feature = $module_feature, )?
                feature = "alloc"
            )))
        )]

        #[cfg(feature = "alloc")]
        pub use crate::_dep::_alloc :: $($( $alloc_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};
    // when the item is available in `std`
    ( rust : std $( :: $( $std_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from rust&#39;s "
        "`std`'>`std`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from [`std" $( "`]::[`" $( $std_path "::" )+ )?
            "`](https://doc.rust-lang.org/std/" $($( $std_path "/" )+)? ")*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(
            feature = "nightly_doc",
            doc(cfg(all(
                $( feature = $module_feature, )?
                feature = "std"
            )))
        )]

        #[cfg(feature = "std")]
        pub use std :: $($( $std_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};
    // when the item is available in either `no_std` or `std`
    ( rust : no_std|std $( :: $( $std_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        /// <span class='stab portability' title='re-exported from rust&#39;s `std`
        /// or recreated for `no_std`'>`[no_]std`</span>
        #[doc = $description]
        #[doc = "\n\n*Re-exported from [`std" $( "`]::[`" $( $std_path "::" )+ )?
            "`](https://doc.rust-lang.org/std/" $($( $std_path "/" )+)? ")*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(
            feature = "nightly_doc",
            doc(cfg(all(
                $( feature = $module_feature, )?
                any(feature = "std", feature = "no_std")
            )))
        )]

        #[cfg(feature = "std")]
        pub use std :: $($( $std_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // when the item is available in either `not(std)` or `std` (always, more transparent)
    ( rust : not(std)|std $( :: $( $std_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        /// <span class='stab portability' title='re-exported from rust&#39;s `std`
        /// or recreated if `not(std)`'>`?std`</span>
        #[doc = $description]
        #[doc = "\n\n*Re-exported from [`std" $( "`]::[`" $( $std_path "::" )+ )?
            "`](https://doc.rust-lang.org/std/" $($( $std_path "/" )+)? ")*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(
            feature = "nightly_doc",
            doc(cfg(all(
                $( feature = $module_feature, )?
            )))
        )]

        #[cfg(feature = "std")]
        pub use std :: $($( $std_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};


    /* external dependencies */

    // Re-exports a non-optional crate
    (crate $dep_str:literal | $dep:ident,
     doc: $description:literal
     $(, features: $( $f:literal ),+ )?
    ) => { $crate::code::paste! {
        #[doc = "<span class='stab portability' title='re-exported `" $dep_str
            "`'>`" $dep_str "`</span>"]
        #[doc = $description "\n\n---" ]
        #[doc(inline)]
        pub use ::$dep;
    }};

    // Re-exports an optional crate
    (optional_crate $dep_str:literal | $dep:ident,
     doc: $description:literal
     $(, features: $( $f:literal ),+ )?
    ) => { $crate::code::paste! {
        #[doc = "<span class='stab portability' title='re-exported `" $dep_str
            "`'>`" $dep_str "`</span>"]
        #[doc = $description "\n\n---" ]
        #[cfg_attr(
            feature = "nightly_doc",
            doc(cfg(all(
                feature = $dep_str $(, $(feature = $f)+ )?
            )))
        )]
        #[cfg(any(
            all(feature = $dep_str $(, $(feature = $f),+ )? )
        ))]
        #[doc(inline)]
        pub use ::$dep;
    }};

    // re-exports items from an external optional dependency, from any normal module.
    //
    // - Supports multiple re-exported items.
    // - Renamed items must be all at the end, and each one prefixed with @.
    ( $dep_str:literal | $dep:ident $( :: $dep_path:path)?,
      $( features: $( $f:literal ),+ ,)?
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from `"
            $dep_str "`'>`" $dep_str "`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from the [`" $dep_str
            "`](https://docs.rs/" $dep_str " ) crate*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        // IMPROVE: can't use like this for portable-atomic | core::atomic,
        // should remove depend from here for the portable-atomic part,
        // but leave it in the following cfg attribute.
        // Also, the part of negated features and target_has_atomic
        // would require a different branch
        #[cfg_attr(
            feature = "nightly_doc",
            doc(cfg(all(
                feature = $dep_str $(, $(feature = $f)+ )?
            )))
        )]

        #[cfg(any(
            all(feature = $dep_str $(, $(feature = $f),+ )? )
        ))]
        pub use crate::_dep::$dep $( ::$dep_path )? :: {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // re-exports items from an external non-optional dependency, from any normal module.
    //
    // - Supports multiple re-exported items.
    // - Renamed items must be all at the end, and each one prefixed with @.
    //
    // in the past it was used for: result::Either
    (non-optional $dep_str:literal | $dep:ident $( :: $dep_path:path)?,
      $( features: $( $f:literal ),+ ,)?
      $( local_module: $module_feature:literal ,)?
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from `" $dep_str
            "`'>`" $dep_str "`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from the [`" $dep_str
            "`](https://docs.rs/" $dep_str " ) crate*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(feature = "nightly_doc", doc(cfg(all(
            $( feature = $module_feature ,)?
            $( $( feature = $f ),+ )?
        ))))]
        #[cfg(all( $($( feature = $f )+)? ))]

        pub use ::$dep $( ::$dep_path )? :: {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // TODO: new branch for: either a crate or core (for portable-atomic types)
}
pub(crate) use reexport;

// Generates a formatted documentation string for Rust modules,
// including a list of standard library modules it extends.
macro_rules! doc_extends {
    ($($mod:ident),+ $(,)?) => {
        concat!(
            // "<small>Extends ",
            "<br/><i style='margin-left:0.618em;'></i><small style='color:#999'>Extends ",
            "`std::{`", crate::code::doc_extends!(@inner $($mod),+), "`}`",
            "</small>"
        )
    };
    // Handles the list of modules ensuring commas are only added between elements.
    (@inner $first:ident $(, $rest:ident)*) => {
        concat!(
            // "[`", stringify!($first), "`]",
            // $( ", [`", stringify!($rest), "`]" ),*
            "[`", stringify!($first), "`](mod@std::", stringify!($first), ")",
            $( ", [`", stringify!($rest), "`](mod@std::", stringify!($rest), ")" ),*

        )
    };
}
pub(crate) use doc_extends;

// Generates a formatted documentation string for conditional availability.
//
// It's intended to be used like this:
// ```
// #[doc = crate::code::doc_availability!(feature = "one"]
// #[doc = crate::code::doc_availability!(all(feature = "one", feature = "two")]
// #[doc = crate::code::doc_availability!(any(feature = "one", feature = "two")]
// ```
#[allow(unused_macros)]
macro_rules! doc_availability {
    (feature = $feat:literal) => {
        $crate::code::doc_availability!{@wrap
            "Available on <strong>crate feature ",
            $crate::code::doc_availability!{@code $feat},
            "</strong> only."
        }
    };

    // Handle one or more required features
    ( all( $(feature = $feat:literal),+ ) ) => {
        $crate::code::doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::code::doc_availability!{@join_features_and $($feat),+},
            "</strong> only."
        }
    };
    // Handle one or more alternative features
    ( any( $(feature = $feat:literal),+ ) ) => {
        $crate::code::doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::code::doc_availability!{@join_features_or $($feat),+},
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
            $crate::code::doc_availability!{@code $first}
            $(
                , " and ", $crate::code::doc_availability!{@code $rest}
            )*
        )
    };
    // single arm for joining features with "or"
    (@join_features_or $first:literal $(, $rest:literal)*) => {
        concat!(
            $crate::code::doc_availability!{@code $first}
            $(
                , " or ", $crate::code::doc_availability!{@code $rest}
            )*
        )
    };
}
#[allow(unused_imports)]
pub(crate) use doc_availability;
