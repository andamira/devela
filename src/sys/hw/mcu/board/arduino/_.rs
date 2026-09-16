//
//! Arduino boards.
//

crate::mods_in! {
    mod nano;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            nano::BoardArduinoNano,
        };
    }
}
