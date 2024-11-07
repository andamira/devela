// devela::num::geom::shape::angle::impl
//
//!
//

mod core_traits;

#[cfg(all(_int_·, _float_·))]
crate::items! {
    mod int;
    #[cfg(test)]
    mod test_int;
}

#[cfg(any(feature = "std", _float_·))]
mod float;
