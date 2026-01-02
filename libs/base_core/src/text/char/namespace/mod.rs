// devela_base_core::text::char::namespace
//
//! Defines the [`Char`] namespace.
//
// TOC
// - struct Char
// - methods over u16

#[cfg(test)]
mod tests;

mod char; // Char<char>
mod u16; // Char<u16>
mod u32; // Char<u32>
mod bytes; // Char<u8 | &[u8] | &[u8; N]>

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// Unicode scalars-related low-level *const* operations.
#[doc = crate::_doc_location!("text/char")]
///
/// # Methods
/// - [over `char`](#methods-over-char)
/// - [over `u32`](#methods-over-u32)
/// - [over `u16`](#methods-over-u16)
/// - [over `u8`](#methods-over-u8)
/// - [over `&[u8]`](#methods-over-u8-slice)
///
/// See also [`Str`][crate::Str].
#[derive(Clone, Copy, Debug)]
pub struct Char<T>(pub T);
