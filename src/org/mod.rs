// devela::org
//
#![cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#![cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
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

// pub mod agent; // Intentional entities capable of action and coordination.
// pub mod econ; // Exchange, incentives, and flows of value within collectives.
// pub mod gov; // Governance, authority, and rule-based coordination.
// pub mod inst; // Enduring organizational forms and institutional structures.
// pub mod moral; // Normative frameworks of responsibility, duty, and judgment.
// pub mod norm; // Shared social norms and informal behavioral expectations.
// pub mod role; // Roles, responsibilities, and positions within coordinated action.

crate::structural_mods! { // _pub_mods, _reexports, _crate_internals
    _pub_mods {
        // pub use super::{
        //     agent::_all::*,
        //     care::_all::*,
        //     econ::_all::*,
        //     gov::_all::*,
        //     inst::_all::*,
        //     moral::_all::*,
        //     norm::_all::*,
        //     role::_all::*,
        // };
    }
    _reexports {
        // pub use devela_base_core::org::{
        //     *
        // }
    }
    _crate_internals {
        pub(crate) use super::_DOC_ORG_MODULES;
    }
}
