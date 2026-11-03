//
//!
//

crate::mods_in! {
    mod constants; // NumConst
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            constants::NumConst,
        };
    }
}
