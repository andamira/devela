// devela::_doc::macros
//
//! Guidelines and information about macros.
//

// /// # Declarative macros.
// pub mod declarative; // WIP

/// # Procedural macros.
pub mod procedural {
    #![doc = include_str!("./devela_macros.md")] // conditional compilation
}
