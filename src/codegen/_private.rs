// devela::codegen::_private
//
//! private codegen helpers
//

#![allow(unused)]

// Macro helper for documentation of re-exported items.
macro_rules! reexport {
    /* reexports from the `depend` module */

    // reexports an optional dependency from the `depend` features:
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
            $dep_name, "` crate'>`", $dep_name, "`</span>")]
        #[doc = $dep_description]
        #[doc = concat!("\n\n*Re-exported [`", $dep_name,
            "`](https://docs.rs/", $dep_name, ")* crate.\n\n---")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = $f)))]

        #[cfg(feature = $f)]
        pub use depend::$dep_module;
    };

    // reexports an optional dependency from the `depend` features,
    // which also depends on another feature, e.g. "alloc":
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
            $dep_name, "` crate'>`", $dep_name, "`</span>")]
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
        pub use depend::$dep_module;
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
    ( rust : core :: $( $core_path:ident )::+,
      local_module: $module_feature:literal,
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::codegen::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from
            `core`'>`core`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from`core::`[`" $( $core_path "::" )+
            "`](https://doc.rust-lang.org/core/" $( $core_path "/" )+ ")*.\n\n---"]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = $module_feature)))]

        pub use core :: $( $core_path :: )+ {
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
    // reexport! { rust:"alloc"|alloc::collections, local_module: "collections",
    //     "A double-ended queue implemented with a growable ring buffer.",
    //     VecDeque }
    // ```
    ( rust : $env_str:literal | $env:ident $( :: $env_path:ident )::*,
      local_module: $module_feature:literal,
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::codegen::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from `"
            $env "`'>`" $env "`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from`" $env "::`[`" $( $env_path "::" )*
            "`](https://doc.rust-lang.org/" $env "/" $( $env_path "/" )* ")*.\n\n---"]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = $module_feature, feature = $env_str))))]

        #[cfg(feature = $env_str)]
        pub use $env $( :: $env_path )::* :: { // CHECK `::` are ok
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
    ) => { $crate::codegen::paste! {
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
            all(feature = "depend" $( , feature = $f )+ ),
        )))]

        #[cfg(any(
            all(feature = "depend" $( , feature = $f )+ ),
            feature = $dep_str
        ))]
        pub use crate::depend::$dep $( ::$dep_path )? :: {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // TODO: new branch for: either a crate or core (for portable-atomic types)
}
pub(crate) use reexport;
