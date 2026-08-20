// devela/src/yard/_doc/test_size_of.rs
//
//! Defines [`_doc_test_size_of!`].
//

#[doc = crate::_tags!(internal assert mem)]
/// Emits compact rustdoc metadata for a type's checked stack size.
#[doc = crate::_doc_meta!{location("yard")}]
///
/// The visible fragment shows the expected `size_of::<T>()` value.
/// A hidden doctest calls [`test_size_of!`][crate::test_size_of]
/// to verify the layout during doctests.
///
/// It can also explicitly record whether wrapping `T` in `Option` preserves
/// its stack size. Omitting the niche suffix leaves that relation unspecified.
///
/// # Forms
/// - `Type = N`: checks `devela::Type`.
/// - `Type = N|B`: also documents and checks the expected bit count.
/// - `Type = N; niche Option`: also checks `Option<Type>` has the same size.
/// - `Type = N; niche !Option`: also checks `Option<Type>` has a different size.
/// - `Label: Type = N`: shows `Type` but uses `Label` as the metadata label.
/// - `abs Label: path::Type = N`: uses the type path exactly as written.
///
/// The niche suffixes check size preservation only:
/// - `niche Option`: `size_of::<Option<T>>() == size_of::<T>()`.
/// - `niche !Option`: `size_of::<Option<T>>() != size_of::<T>()`.
///
/// All forms accept outer attributes before the type entry. These attributes
/// are shown in the visible metadata and re-emitted before the hidden doctest.
///
/// # Examples
/// ```ignore
/// #[doc = crate::_doc_test_size_of!(RasterFormat = 4)]
/// #[doc = crate::_doc_test_size_of!(RasterFormat = 4|32)]
///
/// #[doc = crate::_doc_test_size_of!(u8 = 1|8; niche !Option)]
/// #[doc = crate::_doc_test_size_of!(NonZeroU8 = 1|8; niche Option)]
///
/// #[doc = crate::_doc_test_size_of!(PcmRawBuf_Slice: PcmRawBuf<&[u8]> = 24)]
/// #[doc = crate::_doc_test_size_of!(EventWindowCompact: EventWindow = 16|128; niche Option)]
/// #[doc = crate::_doc_test_size_of!(abs Local: crate::Local = 8)]
/// ```
///
/// Conditional size metadata:
/**
```ignore
#[doc = crate::_doc_test_size_of!(
    #[cfg(target_pointer_width = "64")]
    Handle = 8|64; niche Option
)]
#[doc = crate::_doc_test_size_of!(
    #[cfg(target_pointer_width = "32")]
    Handle = 4|32; niche Option
)]
```
**/
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_test_size_of· {
    // Unnamed public-root type:
    //
    // #[doc = _doc_test_size_of!(RasterFormat = 4)]
    // #[doc = _doc_test_size_of!(EventWindow = 16|128; niche Option)]
    // #[doc = _doc_test_size_of!(EventWindow = 16|128; niche !Option)]
    ($(#[$meta:meta])* $ty:ty = $bytes:literal
     $(| $bits:literal)? ; niche ! $wrap:ident $(,)?) => {
        $crate::_doc_test_size_of!(@doc [$(#[$meta])*] [! $wrap]
            stringify!($ty), concat!("devela::", stringify!($ty)),
            stringify!($ty), stringify!($bytes) $(, stringify!($bits))?
        )
    };
    ($(#[$meta:meta])* $ty:ty = $bytes:literal
     $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?) => {
        $crate::_doc_test_size_of!(@doc [$(#[$meta])*] [$($wrap)?]
            stringify!($ty), concat!("devela::", stringify!($ty)),
            stringify!($ty), stringify!($bytes) $(, stringify!($bits))?
        )
    };
    // Named public-root type:
    //
    // #[doc = _doc_test_size_of!(PcmRawBuf_Slice: PcmRawBuf<&[u8]> = 24)]
    // #[doc = _doc_test_size_of!(PcmRawBuf_Slice: PcmRawBuf<&[u8]> = 24|192)]
    // #[doc = _doc_test_size_of!(EventWindowCompact: EventWindow = 16|128; niche Option)]
    // #[doc = _doc_test_size_of!(EventWindowCompact: EventWindow = 16|128; !niche Option)]
    ($(#[$meta:meta])* $name:ident : $ty:ty = $bytes:literal
     $(| $bits:literal)? ; niche ! $wrap:ident $(,)?) => {
        $crate::_doc_test_size_of!(@doc [$(#[$meta])*] [! $wrap]
            stringify!($name), concat!("devela::", stringify!($ty)),
            stringify!($ty), stringify!($bytes) $(, stringify!($bits))?
        )
    };
    ($(#[$meta:meta])* $name:ident : $ty:ty = $bytes:literal
     $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?) => {
        $crate::_doc_test_size_of!(@doc [$(#[$meta])*] [$($wrap)?]
            stringify!($name), concat!("devela::", stringify!($ty)),
            stringify!($ty), stringify!($bytes) $(, stringify!($bits))?
        )
    };
    // Explicit path escape hatch:
    //
    // #[doc = _doc_test_size_of!(abs PcmRawBuf_Slice: crate::PcmRawBuf<&[u8]> = 24)]
    // #[doc = _doc_test_size_of!(abs PcmRawBuf_Slice: ::devela::PcmRawBuf<&[u8]> = 24)]
    // #[doc = _doc_test_size_of!(abs EventWindow: crate::EventWindow = 16|128; niche Option)]
    // #[doc = _doc_test_size_of!(abs EventWindow: crate::EventWindow = 16|128; !niche Option)]
    ($(#[$meta:meta])* abs $name:ident : $ty:ty = $bytes:literal
     $(| $bits:literal)? ; niche ! $wrap:ident $(,)?) => {
        $crate::_doc_test_size_of!(@doc [$(#[$meta])*] [! $wrap]
            stringify!($name), stringify!($ty), stringify!($ty),
            stringify!($bytes) $(, stringify!($bits))?
        )
    };
    ($(#[$meta:meta])* abs $name:ident : $ty:ty = $bytes:literal
     $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?) => {
        $crate::_doc_test_size_of!(@doc [$(#[$meta])*] [$($wrap)?]
            stringify!($name), stringify!($ty), stringify!($ty),
            stringify!($bytes) $(, stringify!($bits))?
        )
    };
    // Core doc emitter.
    (@doc [$(#[$meta:meta])*] [$($niche:tt)*]
        $label:expr, $test_ty:expr, $shown_ty:expr, $bytes:expr $(, $bits:expr)?
    ) => {
        concat!(
            "\n\n",
            "<sup class='_doc_test_size_of' title='stack size, checked by hidden doctest'>",
            "📦 `size_of::<", $shown_ty, ">() == ", $bytes, "` bytes",
            $(" / ", $bits, " bits",)?
            $crate::_doc_test_size_of!(@meta $(#[$meta])*),
            "</sup>",
            $crate::_doc_test_size_of!(@niche_doc [$($niche)*]),
            "\n\n",
            "<div hidden class='devela-hide-next'></div>\n\n",
            "```rust\n",
            $( "# ", stringify!(#[$meta]), "\n", )*
            "# devela::test_size_of!(assert ", $test_ty, " = ", $bytes,
            $("|", $bits,)?
            $crate::_doc_test_size_of!(@niche_test [$($niche)*]),
            ");\n",
            "```\n",
        )
    };
    // Niche-size documentation badge.
    (@niche_doc []) => { "" };
    (@niche_doc [Option]) => {
        concat!("\n", "<sup class='_doc_niche' ",
            "title='checked by hidden doctest: Option<T> has the same stack size as T'>",
            "⚗️`Option<T>` &#x1F7F0; `T`", "</sup>" // = emoji
        )
    };
    (@niche_doc [! Option]) => {
        concat!("\n", "<sup class='_doc_niche' ",
            "title='checked by hidden doctest: Option<T> does not have the same stack size as T'>",
            "⚗️`Option<T>` &#x2757;&#x1F7F0; `T`", "</sup>" // != emoji
        )
    };
    // Niche-size doctest suffix.
    (@niche_test []) => { "" };
    (@niche_test [Option]) => { "; niche Option" };
    (@niche_test [! Option]) => { "; niche !Option" };
    // Attribute printer helpers.
    (@meta) => { "" };
    (@meta $(#[$meta:meta])+) => {
        concat!("  &#35;&#xfe0f;&#x20e3; ", $(stringify!(#[$meta]), " ",)+) // [#] emoji
    };
}
#[doc(inline)]
pub use _doc_test_size_of· as _doc_test_size_of;
