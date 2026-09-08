<!-- devela/src/_doc/design/structure.md -->

# Library structure

devela is organized as a hierarchy of domains. The module tree gives concepts
a place and keeps related facilities together, while re-exports provide other
ways to find and use them.

For example, [`DistBernoulli`] belongs to probability distributions:

```text
devela
└── num
    └── prob
        └── dist
            └── DistBernoulli
```

Its canonical location is therefore:

```rust
devela::num::prob::dist::DistBernoulli
```

That location identifies where the type belongs in the library. It does not
limit where the type can be accessed.

## Re-exports and visibility

Public items are structurally forwarded through their containing modules and
ultimately to the crate root. These intermediate re-exports are normally hidden
from the generated documentation, so the hierarchy stays readable while shorter
paths remain available:

```rust
devela::num::prob::DistBernoulli
devela::num::DistBernoulli
devela::DistBernoulli
```

Separately, selected items can be re-exported **visibly** from other modules.
These visible public locations are deliberate points of discovery: a module can
show facilities that are especially useful or representative there, even when
their canonical location lies elsewhere.

An item may therefore have several visible public locations, while retaining one
canonical home. For example, [`CStr`] is canonically located with the C FFI
facilities, but is also shown from the string module because it is useful in
that context.

As the hierarchy approaches the crate root, visible re-exports become more
selective. A detailed module can present many relevant facilities, its ancestors
can highlight a smaller selection, and the crate root can show only a compact
set of broadly useful entry points. Hidden forwarding remains independent of
that presentation.

## Root access

Because the public surface is forwarded upward, ordinary code can usually import
items directly from `devela`:

```rust
use devela::{DistBernoulli, Probability};
```

The root namespace is therefore much broader than the root documentation makes
visible. The documentation shows only selected re-exports, while the remaining
forwarded items stay directly accessible without filling the crate page with the
complete library inventory.

## Flat views

The documentation also provides two alternative views of the public surface.

[`all`] gathers the forwarded items into one flat namespace:

```rust
use devela::all::*;
```

Unlike the crate root, it leaves out the root module names and public hidden
support items, giving a cleaner flat inventory of the library's usable surface.

[`all_`] preserves the root domains but flattens everything beneath each one.
For example:

```text
devela::all_::_num
devela::all_::_data
devela::all_::_media
```

`devela::all_::_num::DistBernoulli` therefore shows `DistBernoulli` alongside
the rest of the numeric surface, without the intermediate `prob::dist`
hierarchy.

Taken together, the main views are:

```text
devela::num::prob::dist::DistBernoulli   canonical hierarchy
devela::all_::_num::DistBernoulli        domain-flat
devela::all::DistBernoulli               crate-flat
devela::DistBernoulli                    root access
```

They are different views or access paths to the same item. The hierarchy records
where it belongs, the flat views expose broader inventories, and the root offers
the shortest general access path.

In the generated documentation, `◉` links from a root domain to its flat
`all_` view, while `▽` returns to the hierarchical view.

## Item locations

Individual item pages show a `📍` location annotation near their opening
documentation. The marker is followed by the canonical module path and, when
available, the item name; the module and item are linked independently.

This makes the canonical hierarchy easy to recover after reaching an item
through a visible re-export, the crate root, or a flat view such as `all`.

The `all` and `all_` modules are namespace views. They are separate from the
Cargo `all` and `*_all` features, which control which library capabilities are
compiled.

## In the source

The module layout and export flow are kept consistent by [`mods_in!`] and
[`mods_out!`]. `mods_in!` declares modules following devela's source-file layout,
while `mods_out!` gathers their outward-facing exports and distinguishes
structural forwarding from visible re-exports.

[`all`]: crate::all
[`all_`]: crate::all_
[`DistBernoulli`]: crate::num::prob::dist::DistBernoulli
[`Probability`]: crate::num::prob::Probability
[`CStr`]: crate::lang::prog::ffi::c::CStr
[`mods_in!`]: crate::code::util::synth::mods_in
[`mods_out!`]: crate::code::util::synth::mods_out
