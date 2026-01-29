// devela_base_std::num::dom::real::float
//
//!
//

pub mod dom; // Numeric domains and value representations
pub mod prob; // Probability theory and statistical inference

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            dom::_all::*,
            prob::_all::*,
        };
    }
}
