// devela_base_core::code::util::structural
//
//! Defines the [`structural_mods!`] macro.
//

/// Defines a standardized module structure with automatic re-exports and visibility handling.
///
/// This macro generates a set of structural modules that help organize exports based on their
/// intended visibility and usage patterns.
/// It reduces boilerplate and ensures consistency throught the workspace crates.
///
/// <div class="warning">
///
/// **Order Matters:** The modules must be specified in the exact order shown in the examples
/// below, though all modules are optional. The macro parsing is order-sensitive, and incorrect
/// ordering will result in compilation errors or unexpected behavior. The `_all` module is
/// automatically generated based on the presence of `_mods` and `_pub_mods` modules.
/// </div>
///
/// # Generated Modules
///
/// The macro can generate these optional modules:
///
/// - `_mods`: Public items from non-public modules. Items should be `pub`.
/// - `_pub_mods`: Public items from public modules. Items should be `pub` and have `doc(inline)`.
/// - `_crate_internals`: Crate-private items (`pub(crate)` visibility) that bubble up to the top.
/// - `_workspace_internals`: Workspace-visible items (`pub` but hidden from external crates).
/// - `_always`: Items that bypass normal modules feature-gating and are always available.
/// - `_hidden`: Public but hidden items (`pub` + `doc(hidden)`).
///
/// Additionally, it always generates an `_all` module that aggregates exports from
/// `_mods` and `_pub_mods` when they are present.
///
/// # Usage Patterns
/// ```ignore
/// # use devela_base_core::structural_mods
/// # mod some_module {}
/// # mod other_module {}
/// # pub mod public_module { pub(super) mod _all {} }
/// # mod internal_utils {}
/// # mod workspace_tools {}
/// structural_mods! {
///     _mods {
///         pub use super::{some_module::*, other_module::*};
///     }
///     _pub_mods {
///         #[doc(inline)]
///         pub use super::public_module::_all::*;
///     }
///     _crate_internals {
///         pub(crate) use super::internal_utils::*;
///     }
///     _workspace_internals {
///         pub use super::workspace_tools::*;
///     }
/// }
/// ```
/// # Notes
/// - Modules are generated with `#[allow(unused_imports)]` to prevent noise from
///   intra-crate visibility boundaries
///
/// <div class="warning">
///
/// **Important Usage Note**: The correct functionality of these structural modules depends on
/// strict adherence to the established guidelines for the module chain throughout the entire crate
/// hierarchy.
///
/// Each module must properly re-export its structural components upward, culminating in
/// the root `lib.rs` file, to ensure consistent visibility and proper aggregation of exports. The
/// system relies on a disciplined approach to visibility modifiers and re-export patterns at each
/// level of the module structure to maintain the intended public API surface and internal
/// organization. Deviation from these patterns in intermediate modules may break the automatic
/// aggregation mechanism and compromise the consistency of the final exported interface.
/// </div>
//
// We use the pattern `$(_mods$($has_mods:lifetime)?)?` where the optional lifetime parameter
// serves as a marker to conditionally include the module in `_all`. The lifetime is never
// actually used, simply allowing us to detect the presence of the module during expansion.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _structural_mods {
    (
        // Items inside should be pub.
        // Public items from non-public modules. They bubble up and show up in the current module.
        $( $(_mods$($has_mods:lifetime)?)? { $($block_mods:tt)* } )?
        //
        // Items inside should be pub & doc(inline).
        // Public items from public modules. They bubble up but are hidden in the current module.
        $( $(_pub_mods$($has_pub_mods:lifetime)?)? { $($block_pub_mods:tt)* } )?
        //
        // Items inside should be pub(crate).
        // They are re-exported from the root of the crate.
        $( _crate_internals { $($block_crate_internals:tt)* } )?
        //
        // Items inside should be pub & doc(hidden).
        // They are publicly re-exported from the root of the crate except for the top crate.
        $( _workspace_internals { $($block_workspace_internals:tt)* } )?
        //
        // Items inside should be pub.
        // They bubble up side-stepping certain feature-gates.
        $( _always { $($block_always:tt)* } )?
        //
        // Items inside should be pub & doc(hidden).
        // They are publicly reexported from the root of the crate.
        $( _hidden { $($block_hidden:tt)* } )?
    ) => {
        $(
            #[allow(unused_imports)]
            pub use _mods::*;
            mod _mods { #![allow(unused_imports)]
                $($block_mods)*
            }
        )?
        $(
            #[allow(unused_imports)]
            #[doc(hidden, no_inline)]
            pub use _pub_mods::*;
            mod _pub_mods { #![allow(unused_imports)]
                $($block_pub_mods)*
            }
        )?
        $(
            #[allow(unused_imports)]
            pub(crate) use _crate_internals::*;
            pub(super) mod _crate_internals { #![allow(unused_imports)]
                $($block_crate_internals)*
            }
        )?
        $(
            #[allow(unused_imports)]
            #[doc(hidden, no_inline)]
            pub use _workspace_internals::*;
            pub(crate) mod _workspace_internals { #![allow(unused_imports)]
                $($block_workspace_internals)*
            }
        )?
        pub(super) mod _all { #![allow(unused_imports)]
            $($($($has_mods)?
                #[doc(inline)]
                pub use super::_mods::*;
            )?)?
            $($($($has_pub_mods)?
                #[doc(inline)]
                pub use super::_pub_mods::*;
            )?)?
        }
        $(
            #[allow(unused_imports)]
            #[doc(hidden, no_inline)]
            pub use _always::*;
            pub(super) mod _always { #![allow(unused_imports)]
                $($block_always)*
            }
        )?
        $(
            #[allow(unused_imports)]
            pub use _hidden::*;
            pub(super) mod _hidden { #![allow(unused_imports)]
                $($block_hidden)*
            }
        )?
    };
}
#[doc(inline)]
pub use _structural_mods as structural_mods;
