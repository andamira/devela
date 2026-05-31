// devela::examples::code::enumint
//
//! Shows how to use the [`enumint!`] declarative macro.
//!
//! # Examples
//! This will create the [`EnumintI8Example`] type.
//! ```
//! # use devela::enumint;
//! enumint![
//!     /// An example compact `i8` enum from -126 to 126, generated with [`enumint!`].
//!     pub EnumintI8Example, i8, -126, 126
//! ];
//! fn main() {
//!     assert_eq!(EnumintI8Example::VALUES, 253);
//!     assert_eq!(EnumintI8Example::NICHES, 3);
//!     // We can nest 3 Options before the memory representation increases:
//!     assert_eq!(size_of::<EnumintI8Example>(), 1); // 0 niches used
//!     assert_eq!(size_of::<Option<EnumintI8Example>>(), 1); // 1 niche used
//!     assert_eq!(size_of::<Option<Option<EnumintI8Example>>>(), 1); // 2 niches used
//!     assert_eq!(size_of::<Option<Option<Option<EnumintI8Example>>>>(), 1); // all 3 niches used
//!     assert_eq!(size_of::<Option<Option<Option<Option<EnumintI8Example>>>>>(), 2);
//! }
//! ```
//
// Note that having a huge number of variants needs a lot of resources. E.g.:
// enumint![pub EnumintI8Example, i8, -126, 126]; // +0.2s to compile
// enumint![pub EnumintU16Example, u16, 0, 16384]; // +30s in safe mode, +10.0s unsafe
// enumint![pub EnumintI16Example, i16, -16384, 16384]; // +30s +25GB RAM in unsafe mode

use devela::enumint;

enumint![
    /// An example compact `i8` enum from -126 to 126, generated with [`enumint!`].
    pub EnumintI8Example, i8, -126, 126
];

fn main() {
    assert_eq!(EnumintI8Example::VALUES, 253);
    assert_eq!(EnumintI8Example::NICHES, 3);
    // We can nest 3 Options before the memory representation increases:
    assert_eq!(size_of::<EnumintI8Example>(), 1); // 0 niches used
    assert_eq!(size_of::<Option<EnumintI8Example>>(), 1); // 1 niche used
    assert_eq!(size_of::<Option<Option<EnumintI8Example>>>(), 1); // 2 niches used
    assert_eq!(size_of::<Option<Option<Option<EnumintI8Example>>>>(), 1); // all 3 niches used
    assert_eq!(size_of::<Option<Option<Option<Option<EnumintI8Example>>>>>(), 2);
}
