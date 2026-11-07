This macro generates a set of structural helper modules that centralize export logic
according to intended visibility and usage. It reduces boilerplate and enforces a
consistent module layout across workspace crates.

<div class="warning">

**Usage Constraints:**
- Module blocks must be specified in the exact order shown below (all are optional).
- This macro defines internal modules with reserved names
  (`_mods`, `_pub_mods`, `_reexports`, `_crate_internals`, `_hidden`, and `_all`).
  Do not define modules with these names in the same scope.

Violating either rule will result in compilation errors or incorrect aggregation.
</div>

# Generated Modules

The macro can generate these optional modules:

- `_mods`: Public items from non-public modules. Items should be `pub`.
- `_pub_mods`: Public items from public modules. Items should be `pub` and use `doc(inline)`.
- `_reexports`: Public items from other modules, crates, or the std. Items should be `pub`.
- `_crate_internals`: Crate-private items (`pub(crate)`) re-exported within the crate.
- `_hidden`: Public but hidden items (`pub`, `doc(hidden)`).

An `_all` module is always generated. It aggregates exports from `_mods`, `_pub_mods`,
and `_reexports`, when present.


# Usage Patterns
```ignore
use devela::mods_out;

# mod some_module {}
# mod other_module {}
# pub mod public_module { pub(super) mod _all {} }
# mod _reexport {}
# mod internal_utils {}
mods_out! {
    _mods {
        pub use super::{some_module::*, other_module::*};
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::public_module::_all::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
    _crate_internals {
        pub(crate) use super::internal_utils::*;
    }
}
```

# Notes

- Generated modules use `#[allow(unused_imports)]` to avoid warnings
  caused by intra-crate visibility boundaries.
- Each module in the hierarchy must forward its structural exports upward
  to preserve the intended public API surface.


# Re-export policy

`_mods` and `_pub_mods` preserve the structural public API as it propagates
through the module hierarchy.

- `_reexports`: Explicit public re-exports shown in the current module.
  They may come from other modules, workspace crates, dependencies, or
  Rust's libraries. When used to promote items through the module hierarchy,
  the selection may become progressively more selective at higher levels.
