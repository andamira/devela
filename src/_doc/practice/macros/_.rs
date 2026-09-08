// devela/src/_doc/macros/_.rs
//
//!
//

#![doc = include_str!("./common.md")]

/// # Declarative macros.
pub mod declarative {
    #![doc = include_str!("./declarative.md")]
}

/// # Procedural macros.
pub mod procedural {
    #![doc = include_str!("./procedural.md")]
    #![doc = include_str!("./devela_macros_compile.md")] // conditional compilation (symlinked)
    // #![doc = include_str!("./devela_macros_derive.md")] // declarative macro adapters
}
