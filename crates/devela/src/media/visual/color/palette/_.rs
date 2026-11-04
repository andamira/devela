//
//!
//

crate::mods_in! {
    mod define; // Palette
    // mod ase;
    // mod gpl; // TODO WIP
    // mod hex;
    // mod kpl;
    // mod pal;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::Palette;
        // pub use super::{
        //     // ase::*,
        //     gpl::*,
        //     // hex::*,
        //     // kpl::*,
        //     // pal::*,
        // };
    }
}
