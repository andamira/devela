//
//! Defines the [`mods_out!`] macro.
//

// NOTE: docs in mods_out.md are included from ../code/util/synth/mods_out.rs

// We use the pattern `$(_mods$($has_mods:lifetime)?)?` where the optional lifetime parameter
// serves as a marker to conditionally include the module in `_all`. The lifetime is never
// actually used, simply allowing us to detect the presence of the module during expansion.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! mods_out· {
    (
        // Items inside should be pub.
        // Public items from non-public modules. They bubble up and show up in the current module.
        $( $(_mods$($has_mods:lifetime)?)? { $($block_mods:tt)* } )?

        // Items inside should be pub & doc(inline).
        // Public items from public modules. They bubble up but are hidden in the current module.
        $( $(_pub_mods$($has_pub_mods:lifetime)?)? { $($block_pub_mods:tt)* } )?

        // Items inside should be pub.
        // Explicit visible re-exports from elsewhere.
        // Also used for selected upward promotion through the module hierarchy.
        $( $(_reexports$($has_reexports:lifetime)?)? { $($block_reexports:tt)* } )?

        // Items inside should be pub(crate).
        // They are re-exported from the root of the current crate.
        $( _crate_internals { $($block_crate_internals:tt)* } )?

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
            #[doc(inline)]
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
pub use mods_out· as mods_out;
