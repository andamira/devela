// devela::examples::code::enumint
//
//! Shows how to use the [`enumint!`] declarative macro.
//!
//! # Examples
//! This will create the [`ExampleEnumIntI8`] type.
//! ```
//! # use devela::enumint;
//! enumint![pub ExampleEnumIntI8, i8, -126, 125];
//! ```
//
// Note that having a huge number of variants needs a lot of resources. E.g.:
// enumint![pub ExampleEnumIntU16, u16, 0, 16384]; // +5s to compile
// enumint![pub ExampleEnumIntU16, u16, -16384, 16384]; // +17s +25GB to compile

use devela::enumint;

enumint![pub ExampleEnumIntI8, i8, -126, 126];

fn main() {
    assert_eq!(ExampleEnumIntI8::VALUES, 253);
    assert_eq!(ExampleEnumIntI8::NICHES, 3);
    // We can nest 3 Options before the memory representation increases:
    assert_eq!(size_of::<ExampleEnumIntI8>(), 1); // 0 niches used
    assert_eq!(size_of::<Option<ExampleEnumIntI8>>(), 1); // 1 niche used
    assert_eq!(size_of::<Option<Option<ExampleEnumIntI8>>>(), 1); // 2 niches used
    assert_eq!(size_of::<Option<Option<Option<ExampleEnumIntI8>>>>(), 1); // all 3 niches used
    assert_eq!(size_of::<Option<Option<Option<Option<ExampleEnumIntI8>>>>>(), 2);
}
