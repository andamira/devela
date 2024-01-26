// devela::code::_private
//
//! private meta helpers
//
// TOC
// - reexport!

#![allow(unused)]

/// Macro helper for documentation of re-exported items.
macro_rules! reexport {
    /* reexports from the `depend` module */

    // reexports an optional dependency from the `dep` group,
    // which depends on any feature `$f` and any feature `$f2`, e.g. "alloc":
    ( depend
      $( any_features: $($anyf:literal),+ $(,)? )?
      $( all_features: $($allf:literal),+ $(,)? )?
      dep: $dep_name:literal, $dep_module:ident, $dep_description:literal
      $(,)?
    ) => {
        // with the crate manually enabled
        #[doc(inline)]
        #[doc = concat!("<span class='stab portability' title='re-exported `",
            $dep_name, "` crate (independently or via the `depend` feature)'>`",
            $dep_name, "`</span>")]
        #[doc = $dep_description]
        #[doc = concat!("\n\n*Re-exported [`", $dep_name,
            "`](https://docs.rs/", $dep_name, ")* crate.\n\n---")]
        #[cfg_attr(feature = "nightly", doc(cfg(all(
            any( $( $(feature = $anyf),+ )? )
            $(, $(feature = $anyf),+ )?
        ))))]
        #[cfg(all(
            feature = $dep_name
            $(, not(any($(feature = $anyf),+)) )?
            $(, not(all($(feature = $allf),+)) )?
        ))]
        pub use ::$dep_module;

        // with the "dep" feature enabled
        #[doc(inline)]
        #[doc = concat!("<span class='stab portability' title='re-exported `",
            $dep_name, "` crate (independently or via the `depend` feature)'>`",
            $dep_name, "`</span>")]
        #[doc = $dep_description]
        #[doc = concat!("\n\n*Re-exported [`", $dep_name,
            "`](https://docs.rs/", $dep_name, ")* crate.\n\n---")]
        #[cfg_attr(feature = "nightly", doc(cfg(all(
            any( $($(feature = $anyf),+)? )
            $(, all($(feature = $allf),+) )?
        ))))]
        #[cfg(all(
            any( $( $(feature = $anyf),+ )? )
            $(, all($(feature = $allf),+) )?
        ))]
        pub use dep::$dep_module;
    };

    /* reexports from normal modules */

    // the following 4 arms allows reexporting items from:
    // `core`, `alloc`, `std` or `no_std`|`std`.
    //
    // - Supports multiple reexported items.
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
            "`](https://doc.rust-lang.org/core/" $($( $core_path "/" )+)? ")*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(feature = "nightly", doc(cfg(feature = $module_feature)))]

        pub use core :: $($( $core_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};
    // when the item is available in `alloc`
    ( rust : alloc $( :: $( $alloc_path:ident )::+)?,
      local_module: $module_feature:literal,
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        #[doc = "<span class='stab portability' title='re-exported from rust&#39;s "
        "`alloc`'>`alloc`</span>"]
        #[doc = $description]
        #[doc = "\n\n*Re-exported from [`alloc" $( "`]::[`" $( $alloc_path "::" )+ )?
            "`](https://doc.rust-lang.org/alloc/" $($( $alloc_path "/" )+)? ")*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(
            feature = "nightly",
            doc(cfg(all(feature = $module_feature, feature = "alloc")))
        )]

        #[cfg(feature = "alloc")]
        pub use crate::_deps::alloc :: $($( $alloc_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};
    // when the item is available in `std`
    ( rust : std $( :: $( $std_path:ident )::+)?,
      local_module: $module_feature:literal,
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
            feature = "nightly",
            doc(cfg(all(feature = $module_feature, feature = "std")))
        )]

        #[cfg(feature = "std")]
        pub use std :: $($( $std_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};
    // when the item is available in either `no_std` or `std`
    ( rust : no_std|std $( :: $( $std_path:ident )::+)?,
      local_module: $module_feature:literal,
      doc: $description:literal,
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::code::paste! {
        #[doc(inline)]
        /// <span class='stab portability' title='re-exported from rust&#39;s `std`
        /// or re-created for `no_std`'>`[no_]std`</span>
        #[doc = $description]
        #[doc = "\n\n*Re-exported from [`std" $( "`]::[`" $( $std_path "::" )+ )?
            "`](https://doc.rust-lang.org/std/" $($( $std_path "/" )+)? ")*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

        #[cfg_attr(
            feature = "nightly",
            doc(cfg(all(feature = $module_feature, any(feature = "std", feature = "no_std"))))
        )]

        #[cfg(feature = "std")]
        pub use std :: $($( $std_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // reexports items from an external dependency, from any normal module.
    //
    // - Supports multiple reexported items.
    // - Renamed items must be all at the end, and each one prefixed with @.
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
        #[doc = "\n\n*Re-exported from the [`" $dep_str
            "`](https://docs.rs/" $dep_str " ) crate*"]

        #[doc = $("`" $item_to_rename "`→[`" $item_renamed "`]")* ".\n\n---"]

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
        pub use crate::_deps::$dep $( ::$dep_path )? :: {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    // TODO: new branch for: either a crate or core (for portable-atomic types)
}
pub(crate) use reexport;
