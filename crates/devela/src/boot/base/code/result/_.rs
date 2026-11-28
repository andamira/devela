//
//! Root-independent code vocabulary defined by devela.
//

mod mismatch;

mod unwrap;
#[cfg(test)]
mod _test_unwrap;

crate::boot::meta::mods_out! {
    _mods {
        pub use super::{
            mismatch::Mismatch,
            unwrap::unwrap,
        };
    }
}
