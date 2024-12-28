// devela::examples::num::niche
//
//! Shows how to define niche types.
//!
//! Uses the macros: [`impl_non_value!`].
//!
//! # Example
//! ```
//! # use devela::impl_non_value;
//! impl_non_value!(I 8);
//!
//! let a = NonValueI8::<3>::new(2);
//! ```
//

use devela::impl_non_value;

impl_non_value!(I 8);

fn main() {
    assert![NonValueI8::<3>::new(2).is_some()];
    assert![NonValueI8::<3>::new(3).is_none()];
}
