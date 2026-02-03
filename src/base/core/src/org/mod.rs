// devela_base_core::org
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_ORG!()] // public, root
#![doc = crate::_DOC_ORG_MODULES!()]
#![doc = crate::_doc!(flat:"org")]
#![doc = crate::_doc!(hr)]
//! > Social and organizational semantics.
//
// safety
#![cfg_attr(base_safe_run, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_ORG_MODULES =
    crate::_doc!(modules: crate; org); // agent, care, econ, gov, inst, moral, role
}

// mod norm; // general norms

// pub mod agent; // intentional entities
// pub mod care; // maintenance, dependency, support
// pub mod econ; // economy, incentives, value flows
// pub mod gov; // governance, rules, authority
// pub mod inst; // institutions, organizations, systems
// pub mod moral; // moral norms, responsibility
// pub mod role; // agents, roles, responsibilities

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        // pub use super::{
        //     agent::_all::*,
        //     care::_all::*,
        //     econ::_all::*,
        //     gov::_all::*,
        //     inst::_all::*,
        //     moral::_all::*,
        //     role::_all::*,
        // };
    }
    _crate_internals {
        pub(crate) use super::_DOC_ORG_MODULES;
    }
}
