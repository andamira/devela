// devela/src/yard/_reexport_macro.rs
//
//! private reexport meta helper
//

#[doc = crate::_tags!(internal)]
/// Macro helper for documentation of re-exported items.
#[doc = crate::_doc_meta!{location("yard")}]
//
// WAIT: [missing cross-crate docs](https://github.com/rust-lang/rust/issues/120927)
//       the solution is to re-export from core/alloc/std items on each crate.
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
#[allow(clippy::crate_in_macro_def, reason = "_dep relative to macro call")]
macro_rules! _reexport· {
    // -------------------------------------------------------------------------
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
    ( // when the item is available in `core`
      rust : core $( :: $( $core_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      $( extra_features: $($extra_feat:literal),+ $(,)? )?
      $( extra_flags:($($extra_flag:ident),+) $(,)? )?
      $( location: $($location:literal)+ ,)?
      $( tag: $($tag:expr)+ ,)?
      doc: $doc_line:literal,
      $( +doc: $($doc_more:literal),+, )?
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::paste! {
        #[doc(inline)]
        $( $(#[doc = $tag])+ )? // tags
        #[doc = "<span class='stab portability' title='re-exported from rust&#39;s "
        "`core`'>`core`</span>"]
        #[doc = $doc_line] // first doc line
        #[doc = $crate::_doc_meta! {
            $( location(re-exported $($location)?), )?
            origin(rust core $(:: $( $core_path )::+)? $(
                ; renamed($item_to_rename as $item_renamed)
            )?),
        }]
        #[doc = $crate::_reexport!(@doc_local_tail $($($doc_more)+)?)]

        #[cfg_attr(nightly_doc, doc(cfg(all(
            $( feature = $module_feature, )?
            $( all($(feature = $extra_feat),+) )?
            $( all($($extra_flag),+) )?
        ))))]
        #[cfg(all(
            $( $(feature = $extra_feat),+ )?
            $( $($extra_flag),+ )?
        ))]
        pub use core :: $($( $core_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};
    // when the item is available in `alloc`
    ( rust : alloc $( :: $( $alloc_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      $( location: $($location:literal)+ ,)?
      $( tag: $($tag:expr)+ ,)?
      doc: $doc_line:literal,
      $( +doc: $($doc_more:literal),+, )?
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::paste! {
        #[doc(inline)]
        $( $(#[doc = $tag])+ )? // tags
        #[doc = "<span class='stab portability' title='re-exported from rust&#39;s "
        "`alloc`'>`alloc`</span>"]
        #[doc = $doc_line] // first doc line
        #[doc = $crate::_doc_meta! {
            $( location(re-exported $($location)?), )?
            origin(rust alloc $(:: $( $alloc_path )::+)? $(
                ; renamed($item_to_rename as $item_renamed)
            )?),
        }]
        #[doc = $crate::_reexport!(@doc_local_tail $($($doc_more)+)?)]

        #[cfg_attr(
            nightly_doc,
            doc(cfg(all(
                $( feature = $module_feature, )?
                feature = "alloc"
            )))
        )]
        #[cfg(feature = "alloc")]
        pub use alloc :: $($( $alloc_path :: )+)? {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};
    ( // when the item is available in `std`
      rust : std $( :: $( $std_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      $( location: $($location:literal)+ ,)?
      $( tag: $($tag:expr)+ ,)?
      doc: $doc_line:literal,
      $( +doc: $($doc_more:literal),+, )?
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::paste! {
        #[doc(inline)]
        $( $(#[doc = $tag])+ )? // tags
        #[doc = "<span class='stab portability' title='re-exported from rust&#39;s "
        "`std`'>`std`</span>"]
        #[doc = $doc_line] // first doc line
        #[doc = $crate::_doc_meta! {
            $( location(re-exported $($location)?), )?
            origin(rust std $(:: $( $std_path )::+)? $(
                ; renamed($item_to_rename as $item_renamed)
            )?),
        }]
        #[doc = $crate::_reexport!(@doc_local_tail $($($doc_more)+)?)]

        #[cfg_attr(
            nightly_doc,
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
    ( // when the item is available in either `not(std)` or `std` (always, more transparent)
      rust : not(std)|std $( :: $( $std_path:ident )::+)?,
      $( local_module: $module_feature:literal, )?
      $( location: $($location:literal)+ ,)?
      $( tag: $($tag:expr)+ ,)?
      doc: $doc_line:literal,
      $( +doc: $($doc_more:literal),+, )?
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::paste! {
        #[doc(inline)]
        // NOTE: In sync with _doc/_tags.rs::_TAG_MAYBE_STD
        $( $(#[doc = $tag])+ )? // tags
        /// <span class='stab portability' title='re-exported from rust&#39;s `std`
        /// or recreated if `not(std)`'>`?std`</span>
        #[doc = $doc_line] // first doc line
        #[doc = $crate::_doc_meta! {
            $( location(re-exported $($location)?), )?
            origin(rust std $(:: $( $std_path )::+)? $(
                ; renamed($item_to_rename as $item_renamed)
            )?),
        }]
        #[doc = $crate::_reexport!(@doc_local_tail $($($doc_more)+)?)]

        #[cfg_attr(
            nightly_doc,
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
    // -------------------------------------------------------------------------
    (
      /* external dependencies */

      // Re-exports a non-optional crate
      crate $dep_str:literal | $dep:ident,
      doc: $doc_line:literal
      $(, features: $( $f:literal ),+ )?
    ) => { $crate::paste! {
        #[doc = "<span class='stab portability' title='re-exported `" $dep_str
            "`'>`" $dep_str "`</span>"]
        #[doc = $doc_line "\n\n---" ]
        #[doc(inline)]
        pub use ::$dep;
    }};
    (
      // Re-exports an optional crate (used in */src/yard/dep.rs)
      //
      // $dep_safe:    [safe|unsafe] (affects compilation with "safest" feature)
      // $dep_feat:    the dependency feature that enables it
      // $dep_name:    the dependency real name
      // $dep_mod:     the dependency module name (using _ instead of -)
      // $doc_line:    the dependency description
      // $f:           additional features needed to enable this dependency
      optional_crate ($dep_safe:tt)
      $dep_feat:literal, $dep_name:literal, $dep_mod:ident,
      doc: $doc_line:literal
      $(, features: $( $f:literal ),+ )?
    ) => { $crate::paste! {
        #[cfg(all(feature = $dep_feat $(, $(feature = $f),+ )? ))]
        $crate::items! {
            // safety-guard: panics if `safest` is enabled and dependency is not safe
            #[cfg(feature = "safest")]
            #[$crate::compile(diff($dep_safe, safe))]
            const [<SAFEST_ $dep_name:upper>]: () = { panic![concat!["The `", $dep_name,
            "` dependency is not compatible with the `safest` feature"]] };

            #[doc = "<span class='stab portability' title='re-exported `" $dep_name
                "`'>`" $dep_name "`</span>"]
            #[doc = $doc_line "\n\n---" ]
            #[cfg_attr(
                nightly_doc,
                doc(cfg(all(
                    feature = $dep_feat $(, $(feature = $f)+ )?
                )))
            )]
            #[doc(no_inline)] // MAYBE IMPROVE: allow customization
            pub use ::$dep_mod;
        }
    }};
    (
      // re-exports items from an external optional dependency, from any normal module.
      //
      // - Supports multiple re-exported items.
      // - Renamed items must be all at the end, and each one prefixed with @.
      //
      // $dep_feat:    the dependency feature
      // $dep_name:    the dependency name
      // $dep_mod:     the dependency module
      // $f:           additional features needed
      // $doc_line:    the dependency description
      // $item:
      // $item_to_rename:
      // $item_renamed:
      $dep_feat:literal, $dep_name:literal, $dep_mod:ident $( :: $dep_path:path)?,
      $( features: $( $f:literal ),+ ,)?
      $( location: $($location:literal)+ ,)?
      $( tag: $($tag:expr)+ ,)?
      doc: $doc_line:literal,
      $( +doc: $($doc_more:literal),+, )?
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::paste! {
        #[doc(inline)]
        $( $(#[doc = $tag])+ )? // tags
        #[doc = "<span class='stab portability' title='re-exported from the `"
            $dep_name "` crate'>`" $dep_name "`</span>"]
        #[doc = $doc_line] // first doc line
        #[doc = $crate::_doc_meta! {
            $( location(re-exported $($location)?), )?
            origin(crate $dep_name $(
                ; renamed($item_to_rename as $item_renamed)
            )?),
        }]
        #[doc = $crate::_reexport!(@doc_local_tail $($($doc_more)+)?)]

        // IMPROVE: can't use like this for portable-atomic | core::atomic,
        // should remove depend from here for the portable-atomic part,
        // but leave it in the following cfg attribute.
        // Also, the part of negated features and target_has_atomic
        // would require a different branch
        #[cfg_attr(
            nightly_doc,
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
    (
      // re-exports items from an external non-optional dependency, from any normal module.
      //
      // - Supports multiple re-exported items.
      // - Renamed items must be all at the end, and each one prefixed with @.
      //
      // in the past it was used for: result::Either
      non-optional $dep_str:literal | $dep:ident $( :: $dep_path:path)?,
      $( features: $( $f:literal ),+ ,)?
      $( local_module: $module_feature:literal ,)?
      $( location: $($location:literal)+ ,)?
      $( tag: $($tag:expr)+ ,)?
      doc: $doc_line:literal,
      $( +doc: $($doc_more:literal),+, )?
      $( $item:ident ),*
      $(@ $item_to_rename:ident as $item_renamed:ident),*
      $(,)?
    ) => { $crate::paste! {
        #[doc(inline)]
        $( $(#[doc = $tag])+ )? // tags
        #[doc = "<span class='stab portability' title='re-exported from the `"
            $dep_str "` crate'>`" $dep_str "`</span>"]
        #[doc = $crate::_doc_meta! {
            $( location(re-exported $($location)?), )?
            origin(crate $dep_str => $dep_name $(
            // origin(crate $dep_name $( // simpler form
                ; renamed($item_to_rename as $item_renamed)
            )?),
        }]
        #[doc = $crate::_reexport!(@doc_local_tail $($($doc_more)+)?)]

        #[cfg_attr(nightly_doc, doc(cfg(all(
            $( feature = $module_feature ,)?
            $( $( feature = $f ),+ )?
        ))))]
        #[cfg(all( $($( feature = $f )+)? ))]

        pub use ::$dep $( ::$dep_path )? :: {
            $( $item ),*
            $( $item_to_rename as $item_renamed ),*
        };
    }};

    /* local docs tail */

    // Finishes the local documentation emitted by `_reexport!`.
    (@doc_local_tail) => {
        concat!(
            "\n\n---\n\n",
            "<span class='stab portability' title='Upstream/original documentation'>📜</span><br>"
        )
    };
    (@doc_local_tail $($doc_more:expr)+) => {
        concat!(
            "\n\n", $($doc_more, "\n\n",)+
            "\n\n---\n\n---\n\n",
            "<span class='stab portability' title='Upstream/original documentation'>📜</span><br>"
        )
    };
}
#[doc(inline)]
pub use _reexport· as _reexport;
