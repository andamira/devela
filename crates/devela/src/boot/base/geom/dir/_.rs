//
//!
//

mod boundary;

crate::boot::meta::mods_out! { // _mods
    _mods {
        pub use super::boundary::Boundary1d;
    }
}
