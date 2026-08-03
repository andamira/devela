// devela/src/org/mod.rs
//
#![cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_ORG!()] // public, root
#![doc = crate::_DOC_ORG_MODULES!()]
#![doc = crate::_doc!(flat:"org")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_org", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_ORG_MODULES =
    crate::_doc!(modules: crate; org: _); // agent, care, econ, gov, inst, moral, role
}

// pub mod agent; // Agents participating in intentional and coordinated action.
// pub mod econ; // Exchange, incentives, labor, and allocation of value and resources.
// pub mod gov; // Collective authority, decision, rule, and enforcement.
// pub mod inst; // Persistent organizational forms, services, and procedures.
// pub mod know; // Social production, recognition, circulation, and exclusion of knowledge.
// pub mod moral; // Principles of responsibility, obligation, virtue, and judgment.
// pub mod norm; // Shared expectations, practices, and informal social regulation.
// pub mod role; // Social positions and their associated duties, authority, and scope.

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        // pub use super::{
        //     agent::_all::*,
        //     care::_all::*,
        //     econ::_all::*,
        //     gov::_all::*,
        //     inst::_all::*,
        //     know::_all::*,
        //     moral::_all::*,
        //     norm::_all::*,
        //     role::_all::*,
        // };
    }
    _crate_internals {
        pub(crate) use super::_DOC_ORG_MODULES;
    }
}
