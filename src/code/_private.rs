// devela::code::_private
//
//! private meta helpers
//
// TOC
// - reexport!
// - doc_!
// - doc_availability!
// - doc_miri_warn!

/// Macro helper for documentation of re-exported items.
//
// FIX: unresolved link to `alloc`.
// IMPROVE: rendered paths with trailing `::`. E.g.:
// - Re-exported from std::backtrace:: .
// - Re-exported from [alloc]::boxed:: .
// - Re-exported from core::array:: IntoIter→ArrayIntoIter.
// MAYBE: new branch for: either a crate or core (for portable-atomic types).
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
        #[allow(rustdoc::broken_intra_doc_links)] // TEMP FIX unresolved link to alloc
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
    //
    // $dep_feat:    the dependency feature
    // $dep_name:    the dependency name
    // $dep_mod:     the dependency module
    // $description: the dependency decription
    // $f:           additional features needed
    (optional_crate $dep_feat:literal, $dep_name:literal, $dep_mod:ident,
     doc: $description:literal
     $(, features: $( $f:literal ),+ )?
    ) => { $crate::code::paste! {
        #[doc = "<span class='stab portability' title='re-exported `" $dep_name
            "`'>`" $dep_name "`</span>"]
        #[doc = $description "\n\n---" ]
        #[cfg_attr(
            feature = "nightly_doc",
            doc(cfg(all(
                feature = $dep_feat $(, $(feature = $f)+ )?
            )))
        )]
        #[cfg(all(feature = $dep_feat $(, $(feature = $f),+ )? ))]
        #[doc(inline)]
        pub use ::$dep_mod;
    }};

    // re-exports items from an external optional dependency, from any normal module.
    //
    // - Supports multiple re-exported items.
    // - Renamed items must be all at the end, and each one prefixed with @.
    //
    // $dep_feat:    the dependency feature
    // $dep_name:    the dependency name
    // $dep_mod:     the dependency module
    // $f:           additional features needed
    // $description: the dependency decription
    // $item:
    // $item_to_rename:
    // $item_renamed:
    ( $dep_feat:literal, $dep_name:literal, $dep_mod:ident $( :: $dep_path:path)?,
      $( features: $( $f:literal ),+ ,)?
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from `"
            $dep_name "`'>`" $dep_name "`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from the [`" $dep_name
            "`](https://docs.rs/" $dep_name " ) crate*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        // IMPROVE: can't use like this for portable-atomic | core::atomic,
        // should remove depend from here for the portable-atomic part,
        // but leave it in the following cfg attribute.
        // Also, the part of negated features and target_has_atomic
        // would require a different branch
        #[cfg_attr(
            feature = "nightly_doc",
            doc(cfg(all(
                feature = $dep_feat $(, $(feature = $f)+ )?
            )))
        )]

        #[cfg(all(feature = $dep_feat $(, $(feature = $f),+ )? ))]
        pub use crate::_dep::$dep_mod $( ::$dep_path )? :: {
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
}
pub(crate) use reexport;

// Generates a formatted meta-documentation string.
macro_rules! doc_ {
    (@meta_start) => { "<br/><i style='margin-left:0.618em;'></i><small style='color:#777'>" };
    (@meta_end) => { "</small>" };
    (newline) => { "<br/><br/>" };

    /* list of submodules */

    (modules: $path:path; $self:ident) => {
        concat!(
            crate::code::doc_!(@meta_start), "Modules: ",
            "[`", stringify!($self), "`][mod@", stringify!($path), "::", stringify!($self), "]",
            crate::code::doc_!(@meta_end),
        )
    };
    (modules: $path:path; $self:ident: $($mod:ident),+ $(,)?) => {
        concat!(
            crate::code::doc_!(@meta_start), "Modules: ",
            "[`", stringify!($self), "`][mod@", stringify!($path), "::", stringify!($self), "]`::{`",
            crate::code::doc_!(@modules: $path; $self: $($mod),+), "`}`",
            crate::code::doc_!(@meta_end),
        )
    };
    // Handles the list of modules ensuring commas are only added between elements.
    (@modules: $path:path; $self:ident: $first:ident $(, $rest:ident)*) => {
        concat!(
            "[`", stringify!($first), "`](",
                stringify!($path), "::", stringify!($self), "::",
                stringify!($first), ")",
            $(
                ", [`", stringify!($rest), "`](", stringify!($path), "::",
                stringify!($self), "::", stringify!($rest), ")"
            ),*
        )
    };

    /* list of std modules */

    (extends: $($mod:ident),+ $(,)?) => {
        concat!(
            crate::code::doc_!(@meta_start), "Extends: ",
            "`std::{`", crate::code::doc_!(@extends: $($mod),+), "`}`",
            crate::code::doc_!(@meta_end),
        )
    };
    // Handles the list of modules ensuring commas are only added between elements.
    (@extends: $first:ident $(, $rest:ident)*) => {
        concat!(
            "[`", stringify!($first), "`](mod@std::", stringify!($first), ")",
            $( ", [`", stringify!($rest), "`](mod@std::", stringify!($rest), ")" ),*
        )
    };
}
pub(crate) use doc_;

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

// Generates a formatted documentation string for a miri warning
#[allow(unused_macros)]
macro_rules! doc_miri_warn {
    (tag) => {
        concat!(
            "<span class='stab portability' ",
            "title='Fails to compile under Miri due to potential undefined behavior'>",
            "<code>⚠️</code></span>"
        )
    };
    (body $(, url: $url:literal)?) => {
        concat!(
            "<div class='warning'>",
            "Fails to compile under Miri due to potential undefined behavior.",
            $( "<p><em>See <a href = '", $url, "'>", $url, "</a>.</em></p>", )?
            "</div>"
        )
    };
}
#[allow(unused_imports)]
pub(crate) use doc_miri_warn;
