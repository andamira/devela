// devela/sys/hw/mcu/board/generic/_.rs
//
//! Generic boards.
//

crate::mods_in! {
    mod supermini_oled042;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            supermini_oled042::BoardSuperMiniOled042,
        };
    }
}
