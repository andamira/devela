// devela_base_core::code::util::structural
//
//! Defines the [`structural_mods!`] macro.
//

#[doc = crate::_TAG_CODE!()]
/// Defines a standardized module structure for organizing visibility and re-exports.
#[doc = crate::_doc!(location: "code/util")]
///
/// This macro generates a set of structural helper modules that centralize export logic
/// according to intended visibility and usage. It reduces boilerplate and enforces a
/// consistent module layout across workspace crates.
///
/// <div class="warning">
///
/// **Usage Constraints:**
/// - Module blocks must be specified in the exact order shown below (all are optional).
/// - This macro defines internal modules with reserved names
///   (`_mods`, `_pub_mods`, `_reexports`, `_crate_internals`, `_workspace_internals`,
///   `_hidden`, and `_all`). Do not define modules with these names in the same scope.
///
/// Violating either rule will result in compilation errors or incorrect aggregation.
/// </div>
///
/// # Generated Modules
///
/// The macro can generate these optional modules:
///
/// - `_mods`: Public items from non-public modules. Items should be `pub`.
/// - `_pub_mods`: Public items from public modules. Items should be `pub` and use `doc(inline)`.
/// - `_reexports`: Public items from other modules, crates, or the std. Items should be `pub`.
/// - `_crate_internals`: Crate-private items (`pub(crate)`) re-exported within the crate.
/// - `_workspace_internals`: Workspace-visible items (`pub`, `doc(hidden)`).
/// - `_hidden`: Public but hidden items (`pub`, `doc(hidden)`).
///
/// An `_all` module is always generated. It aggregates exports from `_mods`, `_pub_mods`,
/// and `_reexports` when present.
///
/// # Usage Patterns
/// ```ignore
/// # use devela_base_core::structural_mods
/// # mod some_module {}
/// # mod other_module {}
/// # pub mod public_module { pub(super) mod _all {} }
/// # mod _reexport {}
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
///     _pub_mods {
///         pub use super::_reexport::*;
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
/// - Generated modules use `#[allow(unused_imports)]` to avoid warnings caused by
///   intra-crate visibility boundaries.
/// - Each module in the hierarchy must forward its structural exports upward to preserve
///   the intended public API surface.
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
        // Items inside should be pub.
        $( $(_reexports$($has_reexports:lifetime)?)? { $($block_reexports:tt)* } )?
        //
        // Items inside should be pub(crate).
        // They are re-exported from the root of the current crate.
        $( _crate_internals { $($block_crate_internals:tt)* } )?
        //
        // Items inside should be pub & doc(hidden).
        // They are publicly re-exported from the root of each crate except for the top crate.
        $( _workspace_internals { $($block_workspace_internals:tt)* } )?
        //
        // Items inside should be pub & doc(hidden).
        // They are publicly re-exported from the root of the crate.
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
            pub use _reexports::*;
            mod _reexports { #![allow(unused_imports)]
                $($block_reexports)*
            }
        )?
        $(
            #[allow(unused_imports)]
            pub(crate) use _crate_internals::*;
            pub(crate) mod _crate_internals { #![allow(unused_imports)]
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
        pub(crate) mod _all { #![allow(unused_imports)]
            $($($($has_mods)?
                #[doc(inline)]
                pub use super::_mods::*;
            )?)?
            $($($($has_pub_mods)?
                #[doc(inline)]
                pub use super::_pub_mods::*;
            )?)?
            $($($($has_reexports)?
                #[doc(inline)]
                pub use super::_reexports::*;
            )?)?
        }
        $(
            #[allow(unused_imports)]
            pub use _hidden::*;
            pub(crate) mod _hidden { #![allow(unused_imports)]
                $($block_hidden)*
            }
        )?
    };
}
#[doc(inline)]
pub use _structural_mods as structural_mods;
