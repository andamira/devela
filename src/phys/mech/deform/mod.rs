// devela/src/phys/mech/deform/mod.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_PHYS_MECH_DEFORM!()] // WIP
#![doc = crate::_doc!(modules: crate::phys::mech; deform)]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(hr)]
//

// mod law; // Constitutive laws relating deformation to mechanical response
// mod measure; // Stretch, strain, curvature, and other deformation measures
// mod rest; // Stress-free and mechanically preferred configurations

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            law::_all::*,
            measure::_all::*,
            rest::*,
        };
    }
}
