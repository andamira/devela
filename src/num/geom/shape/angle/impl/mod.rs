// devela::num::geom::shape::angle::impl
//
//!
//

use crate::items;

mod core_traits;

#[cfg(all(_some_int, _some_float))]
items! {
    mod int;
    #[cfg(test)]
    mod test_int;
}

#[cfg(any(feature = "std", _some_float))]
mod float;
