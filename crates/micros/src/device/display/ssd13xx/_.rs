//
//! SSD1306, SSD1315…
//

crate::mods_in! {
    mod device;
    mod i2c;
    mod write;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            device::Ssd13xx,
            i2c::Ssd13xxI2c,
            write::Ssd13xxWrite,
        };
    }
}
