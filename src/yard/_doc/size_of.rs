// devela::yard::_doc::size_of
//
//! Defines [`_doc_size_of!`].
//

#[doc = crate::_tags!(internal)]
/// Emits compact rustdoc metadata for a type's checked stack size.
#[doc = crate::_doc_meta!{location("yard")}]
///
/// The visible fragment shows the expected `size_of::<T>()` value.
/// A hidden doctest calls [`test_size_of!`][crate::test_size_of]
/// to verify the size during doctests.
///
/// # Forms
/// - `Type = N`: checks `devela::Type`.
/// - `Label: Type = N`: shows `Type` but uses `Label` as the metadata label.
/// - `abs Label: path::Type = N`: uses the type path exactly as written.
///
/// # Examples
/// ```ignore
/// #[doc = crate::_doc_size_of!(RasterFormat = 4)]
/// #[doc = crate::_doc_size_of!(PcmRawBuf_Slice: PcmRawBuf<&[u8]> = 24)]
/// #[doc = crate::_doc_size_of!(abs Local: crate::Local = 8)]
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_size_of· {
    // Named public-root type:
    //
    // #[doc = _doc_size_of!(PcmRawBuf_Slice: PcmRawBuf<&[u8]> = 24)]
    // #[doc = _doc_size_of!(PcmRawBuf_Slice: PcmRawBuf<&[u8]> = 24|192)]
    ($ty:ty = $bytes:literal $(| $bits:literal)? $(,)?) => {
        $crate::_doc_size_of!(@doc stringify!($ty),
            concat!("devela::", stringify!($ty)), stringify!($ty),
            stringify!($bytes) $(, stringify!($bits))? )
    };
    // Unnamed public-root type:
    //
    // #[doc = _doc_size_of!(RasterFormat = 4)]
    ($name:ident : $ty:ty = $bytes:literal $(| $bits:literal)? $(,)?) => {
        $crate::_doc_size_of!(@doc stringify!($name),
            concat!("devela::", stringify!($ty)), stringify!($ty),
            stringify!($bytes) $(, stringify!($bits))? )
    };
    // Explicit path escape hatch:
    //
    // #[doc = _doc_size_of!(abs PcmRawBuf_Slice: crate::PcmRawBuf<&[u8]> = 24)]
    // #[doc = _doc_size_of!(abs PcmRawBuf_Slice: ::devela::PcmRawBuf<&[u8]> = 24)]
    (abs $name:ident : $ty:ty = $bytes:literal $(| $bits:literal)? $(,)?) => {
        $crate::_doc_size_of!(@doc stringify!($name),
            stringify!($ty), stringify!($ty), stringify!($bytes) $(, stringify!($bits))? )
    };
    // Core doc emitter.
    (@doc $label:expr, $test_ty:expr, $shown_ty:expr, $bytes:expr $(, $bits:expr)?) => {
        concat!(
            "\n\n<sup class='_doc_size_of' title='stack size, checked by hidden doctest'>",
            // "📦 `", $label, ": size_of::<", $shown_ty, ">() == ", $bytes, "` bytes",
            "📦 `size_of::<", $shown_ty, ">() == ", $bytes, "` bytes", "</sup>\n\n",
            "<div hidden class='devela-hide-next'></div>\n\n",
            "```rust\n",
            "# devela::test_size_of!(assert ", $test_ty, " = ", $bytes, ");\n",
            "```\n",
        )
    };
}
#[doc(inline)]
pub use _doc_size_of· as _doc_size_of;
