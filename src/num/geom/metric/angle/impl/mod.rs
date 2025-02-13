// devela::num::geom::metric::angle::impl
//
//!
//

mod core_traits;

#[cfg(all(_int··, _float··))]
crate::items! {
    mod int;
    #[cfg(test)]
    mod test_int;
}

#[cfg(any(feature = "std", _float··))]
mod float;
