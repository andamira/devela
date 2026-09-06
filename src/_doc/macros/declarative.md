<!-- devela/src/_doc/macros/declarative.md -->

Declarative macros are Rust's pattern-based macro system, defined with `macro_rules!`.

They work especially well for structural transformations: recognizing a small
grammar, forwarding parts of it, repeating syntax, or generating families of
related items. devela uses them both for small conveniences and for larger
generators such as `arena!`, `pool!`, and `mods_out!`.

## Basic anatomy

A `macro_rules!` definition contains one or more **arms**:

```rust
macro_rules! example {
    ($name:ident : $ty:ty) => {
        struct $name($ty);
    };
}
```

The left side is the **matcher** and the right side is the **transcriber** or expansion.

Captured values such as `$name` are **metavariables**.
Their fragment specifier tells the matcher what kind of Rust syntax to accept:

```rust
$name:ident
$ty:ty
$value:expr
$path:path
$meta:meta
$tokens:tt
```

Repetitions use `*`, `+`, or `?`:

```rust
$( $item:ident ),*
```

The fragment kind matters. Prefer the narrowest one that describes the intended grammar;
use `tt` when syntax should deliberately remain opaque and be passed elsewhere.

## Designing the grammar

A public macro invocation is an API.

Prefer a small syntax whose parameters correspond to real choices in the
generated abstraction. Implementation details can be normalized after the
public form has been recognized.

A useful pattern is to parse a stable prefix and keep the remaining input
as token trees:

```rust
(
    $(#[$attr:meta])*
    $vis:vis struct $name:ident;
    $($rest:tt)*
) => {
    /* ... */
};
```

The known part is checked immediately, while `$rest` can be forwarded to
another stage.

Avoid using `tt` everywhere just because it accepts more input. A more precise
fragment usually gives the caller a better error and makes the macro's grammar
easier to understand.

## Recursive dispatch

A macro can consume part of its input and recursively process the rest. This is
useful for optional sections or groups whose order is flexible.

```rust
macro_rules! example {
    (@groups $name:ident;) => {};

    (@groups $name:ident; first($($args:tt)*); $($rest:tt)*) => {
        example!(@first $name ($($args)*));
        example!(@groups $name; $($rest)*);
    };

    (@groups $name:ident; second($($args:tt)*); $($rest:tt)*) => {
        example!(@second $name ($($args)*));
        example!(@groups $name; $($rest)*);
    };
}
```

This separates two jobs:

- dispatch arms recognize and consume the grammar;
- handler arms perform the corresponding expansion.

For larger macros, this can be easier to follow than trying to describe
every combination in a single matcher.

## Public and internal forms

Internal routing syntax such as `@impl`, `%generate`, or `%normalize` is useful,
but it is not necessarily part of the intended public macro language.

devela uses underscore prefixes to distinguish helper macros by reachability.
`_name!` denotes an internal helper that is not exported for downstream use,
while `__name!` denotes a public but hidden helper that
downstream macro expansions may need to resolve.

Small macros can keep public and internal arms together. Larger public macros
should move substantial implementation protocols into a helper
with the appropriate reachability.

```text
arena! → __arena!
```

`arena!` owns the user-facing grammar, while `__arena!` handles normalization,
dispatch, backend selection, and related machinery. Because an expansion of
`arena!` may invoke it through `$crate`, `__arena!` must remain publicly
reachable even though it is not part of the documented API.

Helpers needed only while expanding devela itself use the single-underscore form instead.

## Exporting and re-exporting

`#[macro_export]` exports a declarative macro from the crate root, regardless of
the source module containing its definition.

devela commonly uses a middle-dot suffix for this definition artifact:

```rust
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! example· {
    /* ... */
}

#[doc(inline)]
pub use example· as example;
```

`example!` is the intended name. `example·!` exists because of the mechanics of
exporting and routing the macro.

This also keeps documentation placement separate from visibility.
`#[doc(inline)]` is added when a re-export should become the visible documented
surface; an intermediate public re-export does not need to make that decision yet.

When an exported macro refers back to the crate that defined it,
`$crate` provides the appropriate path:

```rust
$crate::__example!(...)
```

This avoids depending on names imported by the caller.

## Source layout

Large macro definitions benefit from a few simple visual conventions.

Use normal rustdoc above the macro for its public documentation.
Within the definition, block comments can separate families of arms:

```rust
macro_rules! example {
    /* public forms */

    (...) => { ... };

    /* internal expansion */

    (%normalize ...) => { ... };
}
```

Short line comments are useful for explaining the particular purpose of an arm.
They are most useful when they explain *why the arm exists*,
rather than simply restating its matcher.

Sentinel tokens such as `@`, `%`, or `#` can make internal protocols easy to
recognize at a glance.

## Notes and useful tricks

### Prefix symbols

Many non-alphanumeric tokens can be matched directly before a metavariable:

```rust
(@$name:ident ...)
(%$name:ident ...)
(#$name:ident ...)
```
Such symbols are handy for internal dispatch forms because they are visually
distinct from normal public syntax. But they are conventions, not namespaces:
their meaning belongs to the macro that uses them.

Useful prefixes include:
```
@ # ? ! : ; , . ~ | ^ & > < = / + - * %
```
as well as empty delimiter groups and literals.

### Remembering that a fixed token was present

Sometimes an optional part of a matcher contains only fixed syntax:

```rust
$(+const)?
```

Later expansion cannot refer to `+const` as a metavariable, so there is nothing
inside the repetition whose presence can be replayed.

One workaround used in devela is to capture an otherwise unexpected optional fragment:

```rust
$(+const $($_marker:lifetime)?)?
```

The lifetime is not intended to appear in valid input. Its capture gives the
repetition a metavariable whose nesting records whether the surrounding fixed
syntax matched.

The same technique is used for optional structural markers in macros such as `mods_out!`.

### Generating another `macro_rules!`

A macro that generates another declarative macro has to distinguish its own `$`
tokens from those intended for the generated definition.

[`macro_dollar!`][crate::macro_dollar] supplies a literal dollar token:

```rust
macro_dollar! { ($d:tt) => {
    macro_rules! generated {
        ($d($item:tt)*) => {
            stringify!($d($item)*)
        };
    }
}}
```

This works for nested repetitions and for constructs
such as the generated macro's own `$crate`.

### Constructing identifiers

Stable `macro_rules!` cannot generally create a new identifier by joining pieces
of other identifiers.

devela uses [`paste!`][crate::paste] when identifier construction is actually needed.
Keep its scope small where possible: wrapping only the portion that needs generated
names is easier to read than putting a whole large expansion inside `paste!`.

### Expanding complete syntax

A macro invocation expands as a syntactic construct; it cannot generally
be used to contribute an arbitrary partial sequence of tokens
to a surrounding array, tuple, or similar construct.

For example, `punroll!` builds its complete array or tuple in the arm performing
the repetition rather than trying to have another invocation emit only the
interior elements.

This is an easy limitation to run into when first decomposing a large macro.

## Language evolution

Some of these techniques work around limitations of today's declarative macro
system rather than expressing an ideal API.

Rust continues to develop macro hygiene, metavariable expressions, identifier
construction, and related facilities. Workarounds should therefore stay
localized where possible, making them easier to remove when a direct language
feature becomes available.

## Further reading

* [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
* [Macro follow-set ambiguity](https://doc.rust-lang.org/reference/macro-ambiguity.html)
* [The Little Book of Rust Macros](https://veykril.github.io/tlborm/decl-macros.html)

