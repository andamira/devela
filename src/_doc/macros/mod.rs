// devela::_doc::macros
//
//! Guidelines and information about macros.
//

// /// # Declarative macros.
// pub mod declarative; // WIP

/// # Procedural macros.
pub mod procedural {
    #![doc = include_str!("./devela_macros_compile.md")] // conditional compilation
    // #![doc = include_str!("./devela_macros_derive.md")] // declarative macro adapters
}
