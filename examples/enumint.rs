// devela::examples::enumint
//
//! Shows how to use the [`enumint!`] declarative macro.
//!
//! # Examples
//!
//! This will create the [`ExampleEnum`] and [`ExampleEnumSet`] interrelated types.
//! ```
//! # use devela::code::enumint;
//! enumint![pub ExampleEnumIntU8, i8, -126, 125];
//! ```
//

use devela::code::enumint;

enumint![pub ExampleEnumIntU8, i8, -126, 126];

fn main() {
    assert_eq!(ExampleEnumIntU8::VALID_VALUES, 253);
    assert_eq!(ExampleEnumIntU8::NICHE_VALUES, 3);
    // We can nest 3 Options before the memory representation increases:
    assert_eq!(size_of::<ExampleEnumIntU8>(), 1); // 0 niches used
    assert_eq!(size_of::<Option<ExampleEnumIntU8>>(), 1); // 1 niche used
    assert_eq!(size_of::<Option<Option<ExampleEnumIntU8>>>(), 1); // 2 niches used
    assert_eq!(size_of::<Option<Option<Option<ExampleEnumIntU8>>>>(), 1); // all 3 niches used
    assert_eq!(size_of::<Option<Option<Option<Option<ExampleEnumIntU8>>>>>(), 2);
}
