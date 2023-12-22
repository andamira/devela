// devela::code::_private
//
//! private meta helpers
//

#![allow(unused)]

// Macro helper for documentation of re-exported items.
macro_rules! reexport {
    /* reexports from the `depend` module */

    // reexports an optional dependency from the `depend` features.
    ( depend
      dep: $dep_name:literal, $dep_module:ident, $dep_description:literal
      $(,)?
    ) => {
        #[doc(inline)]
        #[doc = concat!("<span class='stab portability' title='re-exported `",
            $dep_name, "` crate (independently or via the `depend` feature)'>`",
            $dep_name, "`</span>")]
        #[doc = $dep_description]
        #[doc = concat!("\n\n*Re-exported [`", $dep_name,
            "`](https://docs.rs/", $dep_name, ")* crate.\n\n---")]
        pub use dep::$dep_module;
    };


    // reexports an optional dependency from the `depend` features,
    // which depends on feature `$f`.
    ( depend
      feature: $f:literal,
      dep: $dep_name:literal, $dep_module:ident, $dep_description:literal
      $(,)?
    ) => {
        #[cfg(all(
            feature = $dep_name,
            not(feature = $f)
        ))]
        pub use ::$dep_module;

        // ---

        #[doc(inline)]
        #[doc = concat!("<span class='stab portability' title='re-exported `",
            $dep_name, "` crate (independently or via the `depend` feature)'>`",
            $dep_name, "`</span>")]
        #[doc = $dep_description]
        #[doc = concat!("\n\n*Re-exported [`", $dep_name,
            "`](https://docs.rs/", $dep_name, ")* crate.\n\n---")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = $f)))]

        #[cfg(feature = $f)]
        pub use dep::$dep_module;
    };

    // reexports an optional dependency from the `depend` features,
    // which depends on feature `$f` and also on `$another_feature`, e.g. "alloc":
    ( depend
      feature: $f:literal,
      also: $another_feature:literal,
      dep: $dep_name:literal, $dep_module:ident, $dep_description:literal
      $(,)?
    ) => {
        #[cfg(all(
            feature = $dep_name,
            feature = $another_feature,
            not(feature = $f)
        ))]
        pub use ::$dep_module;

        // ---

        #[doc(inline)]
        #[doc = concat!("<span class='stab portability' title='re-exported `",
            $dep_name, "` crate (independently or via the `depend` feature)'>`",
            $dep_name, "`</span>")]
        #[doc = $dep_description]
        #[doc = concat!("\n\n*Re-exported [`", $dep_name,
            "`](https://docs.rs/", $dep_name, ")* crate.\n\n---")]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(
                feature = $f,
                feature = $another_feature))
        ))]

        #[cfg(all(
            feature = $f,
            feature = $another_feature)
        )]
        pub use dep::$dep_module;
    };

    /* reexports from normal modules */

    // reexports items from `core`, from any normal module.
    //
    // - Supports multiple reexported items.
    // - Renamed items must be all at the end, and each one prefixed with @.
    //
    // # Examples
    // ```
    // reexport! { rust: core::any, local_module: "any",
    //     "Represents a globally unique identifier for a type.", TypeId }
    // ```
    ( rust : core $( :: $( $core_path:ident )::+)?,
      local_module: $module_feature:literal,
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
            "`](https://doc.rust-lang.org/core/" $($( $core_path "/" )+)? ")*.\n\n---"]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = $module_feature)))]

        pub use core :: $($( $core_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // reexports items from `alloc` or `std`, from any normal module.
    //
    // - Supports multiple reexported items.
    // - Renamed items must be all at the end, and each one prefixed with @.
    //
    // # Examples
    // ```
    // reexport! { rust:"alloc"|alloc::collections, local_module: "data",
    //     "A double-ended queue implemented with a growable ring buffer.",
    //     VecDeque }
    // ```
    ( rust : $env_str:literal | $env_module:ident $( :: $env_path:ident )::*,
      local_module: $module_feature:literal,
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from rust&#39;s `"
            $env_str "`'>`" $env_str "`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from rust's `" $env_str "::`[`" $( $env_path "::" )*
            "`](https://doc.rust-lang.org/" $env_str "/" $( $env_path "/" )* ")*.\n\n---"]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = $module_feature, feature = $env_str))))]

        #[cfg(feature = $env_str)]
        pub use $env_module $( :: $env_path )::* :: { // CHECK `::` are ok
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // reexports items from an external dependency, from any normal module.
    //
    // - Supports multiple reexported items.
    // - Renamed items must be all at the end, and each one prefixed with @.
    //
    ( $dep_str:literal | $dep:ident $( :: $dep_path:path)?,
      features: $( $f:literal ),+ ,
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from `" $dep_str
        "` (which can be enabled instead of `depend`)'>`" $dep_str "`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Reexported from the [`" $dep_str
            "`](https://docs.rs/" $dep_str " ) crate*.\n\n---"]

        // IMPROVE: can't use like this for portable-atomic | core::atomic,
        // should remove depend from here for the portable-atomic part,
        // but leave it in the following cfg attribute.
        // Also, the part of negated features and target_has_atomic
        // would require a different branch
        #[cfg_attr(feature = "nightly", doc(cfg(
            all(feature = "dep" $( , feature = $f )+ ),
        )))]

        #[cfg(any(
            all(feature = "dep" $( , feature = $f )+ ),
            feature = $dep_str
        ))]
        pub use crate::_dep::$dep $( ::$dep_path )? :: {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // TODO: new branch for: either a crate or core (for portable-atomic types)
}
pub(crate) use reexport;
