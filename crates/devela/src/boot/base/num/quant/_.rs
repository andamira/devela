//
//!
//

mod sign;

crate::boot::meta::mods_out! { // _mods
    _mods {
        pub use super::{
            sign::Sign,
        };
    }
}
