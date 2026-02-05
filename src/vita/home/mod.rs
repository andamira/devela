// devela::vita::home
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_VITA_HOME!()] // public
#![doc = crate::_doc!(modules: crate::vita; home)]
#![doc = crate::_doc!(flat:"vita")]
#![doc = crate::_doc!(hr)]
//

// pub mod build; // architecture, construction, built environment
// pub mod cook; // cooking, meals, kitchen, gastronomy
// pub mod dwell; // habitation, domestic life
// pub mod land; // landscape, terrain
// pub mod urban; // cities, infrastructure

crate::structural_mods! { // _pub_mods
    _pub_mods {
        // pub use super::{
        //     build::_all::*,
        //     cook::_all::*,
        //     dwell::_all::*,
        //     land::_all::*,
        //     urban::_all::*,
        // };
    }
    _reexports {
        // pub use devela_base_core::vita::home::*;
    }
}
