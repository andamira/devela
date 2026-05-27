// devela::yard::_doc::meta
//
//! Defines [`_doc_meta!`].
//

#[doc = crate::_tags!(internal)]
/// Composes a small rustdoc metadata section for an item.
#[doc = crate::_doc_meta!{location("yard")}]
///
/// This macro centralizes the ad-hoc metadata band used near the top of item
/// documentation. It wraps supported metadata fragments between horizontal
/// rules and dispatches each section to the corresponding helper.
///
/// # Sections
/// - `location(...)`: emits an item location fragment through [`_doc_location!`].
/// - `size_of(...)`: emits checked type-size metadata through [`_doc_size_of!`].
/// - `origin(...)`: emits re-export origin metadata for Rust or dependency items.
///
/// [`_doc_location!`]: crate::_doc_location
/// [`_doc_size_of!`]: crate::_doc_size_of
///
/// # Examples
/// ```ignore
/// #[doc = crate::_doc_meta! { location("media/audio"), size_of(PcmSpec = 8) }]
/// #[doc = crate::_doc_meta! { location(re-exported "code/any"), origin(rust core::any) }]
/// #[doc = crate::_doc_meta! { location(re-exported "code"), origin(crate "hashbrown") }]
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_meta· {
    /* internal dispatcher: end */
    (@items) => { "" };
    (@items ,) => { "" };
    /* public section: location */
    (@items location($($args:tt)*) $(, $($rest:tt)*)?) => {
        concat!(
            $crate::_doc_location!(%from_meta $($args)*),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };
    /* public section: size_of */
    (@items size_of($ty:ty = $bytes:literal $(| $bits:literal)?)
        $(, $($rest:tt)*)?) => {
        concat!(
            $crate::_doc_size_of!($ty = $bytes $(| $bits)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };
    (@items size_of($name:ident : $ty:ty = $bytes:literal $(| $bits:literal)?)
        $(, $($rest:tt)*)?) => {
        concat!(
            $crate::_doc_size_of!($name : $ty = $bytes $(| $bits)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };
    (@items size_of(abs $name:ident : $ty:ty = $bytes:literal $(| $bits:literal)?)
        $(, $($rest:tt)*)?) => {
        concat!(
            $crate::_doc_size_of!(abs $name : $ty = $bytes $(| $bits)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };

    /* public section: origin from Rust core/alloc/std */
    (@items origin(rust $root:ident $(:: $path:ident)* $(;
        renamed($($old:ident as $new:ident),* $(,)?))?) $(, $($rest:tt)*)?) => {
        concat!(" ",
            $crate::_doc_meta!(@emit_origin_rust $root $(:: $path)* ; $($($old as $new),*)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };

    /* public section: origin from external crate */
    (@items origin(crate $dep:literal
        $(; renamed($($old:ident as $new:ident),* $(,)?))?) $(, $($rest:tt)*)?) => {
        concat!(" ",
            $crate::_doc_meta!(@emit_origin_crate $dep; $($($old as $new),*)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };

    /* optional: display name differs from docs.rs crate slug */
    (@items origin(crate $shown:literal => $docs:literal
        $(; renamed($($old:ident as $new:ident),* $(,)?))?) $(, $($rest:tt)*)?) => {
        concat!(" ",
            $crate::_doc_meta!(@emit_origin_crate_as $shown => $docs; $($($old as $new),*)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };

    /* internal emitters */
    (@emit_origin_rust $root:ident $(:: $path:ident)* ; $($renamed:tt)*) => {
        concat!(
            "<sup>re-exported from <a title='location in `", ::core::stringify!($root),
            "`' href=\"https://doc.rust-lang.org/", ::core::stringify!($root), "/",
            $( ::core::stringify!($path), "/", )* "\">", ::core::stringify!($root),
            $("::", ::core::stringify!($path),)* "</a>",
            $crate::_doc_meta!(@emit_renamed $($renamed)*), "</sup>"
        )
    };
    (@emit_origin_crate $dep:literal; $($renamed:tt)*) => {
        concat!(
            "<sup>re-exported from the <a title='docs for `", $dep,
            "`' href=\"https://docs.rs/", $dep, "\">", $dep, "</a> crate",
            $crate::_doc_meta!(@emit_renamed $($renamed)*), ".</sup>"
        )
    };

    (@emit_origin_crate_as $shown:literal => $docs:literal; $($renamed:tt)*) => {
        concat!(
            "<sup>re-exported from the <a title='docs for `", $shown,
            "`' href=\"https://docs.rs/", $docs, "\">", $shown, "</a> crate",
            $crate::_doc_meta!(@emit_renamed $($renamed)*), ".</sup>"
        )
    };
    (@emit_renamed) => { "" };
    (@emit_renamed $($old:ident as $new:ident),+ $(,)?) => {
        concat!( " ", $( "`::", stringify!($old), "` as `", stringify!($new), "` " ),+)
    };

    /* diagnostics */
    (@items $bad:ident($($args:tt)*) $(, $($rest:tt)*)?) => {
        compile_error!(concat!( "unknown _doc_meta! section: `", stringify!($bad), "`"))
    };

    /* public entrypoints last */
    () => { "" };
    ($($rest:tt)+) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_meta!(@items $($rest)+),
            "\n\n---\n\n",
        )
    };
}
#[doc(inline)]
pub use _doc_meta· as _doc_meta;
