<!-- devela/src/_doc/practice/macros/common.md -->

Macros sit at one of Rust's more interesting boundaries:
code can help shape the code that the compiler will later understand.

They operate during compilation, transforming Rust syntax into more Rust syntax.
This makes them useful for much more than textual shorthand: a macro can describe
families of items, encode small domain-specific grammars, adapt one syntactic form
into another, or generate a structure specialized to the needs of its caller.

For macros that a library can define, Rust provides two main families:
[declarative macros][crate::_doc::practice::macros::declarative] and
[procedural macros][crate::_doc::practice::macros::procedural].

Declarative macros, defined with `macro_rules!`, describe transformations by
matching token patterns and transcribing their captures. They are especially well
suited to finite, structural grammars where the possible forms are known in advance.

Procedural macros are Rust functions that receive and produce token streams.
They can inspect and construct syntax more freely, and may appear as
function-like macros, attributes, or derives.

Compiler-provided built-in macros form another practical category at the call site.
They participate in expansion too, but are not defined by crates
through either of the mechanisms above.

## Choosing the mechanism

devela generally prefers the smallest mechanism
that can express the transformation clearly.

A declarative macro is usually preferred when the transformation can be stated
as a grammar over known Rust syntax. Generator macros such as `arena!`, `pool!`,
or `mods_out!` can therefore expose a small semantic language
and materialize a much larger user-shaped implementation.

A procedural macro becomes useful when the transformation needs capabilities
that `macro_rules!` cannot directly express. [`paste!`][crate::paste], for
example, can construct new identifiers. [`mods_in!`][crate::mods_in] is
procedural because `mod_ name;` must become a path attribute containing a newly
constructed string literal such as `#[path = "name/_.rs"]`.

The two forms can also complement each other. [`macro_apply`][crate::macro_apply]
is a thin procedural adapter that lets an ordinary declarative macro operate in
attribute position. In such cases the procedural layer opens a syntactic door,
while the declarative macro can continue to own the transformation itself.

The choice is therefore less about which kind of macro is "more powerful" and
more about where the required transformation naturally lives.

## Expansion

It is tempting to imagine compilation as:
```text
source → expand every macro → compile the result
```

but Rust's expansion process is more intertwined than that.

A more useful approximation is:
```text
                           ┌─────────────────────────────┐
                           │                             │
source                     ▼                             │
  │                     resolve                          │
  ▼                 macros & imports                     │
parse                      │                             │
  │                        ▼                             │
  │                 choose an invocation                 │
  │                        │                             │
  │            ┌───────────┴────────────┐                │
  │            ▼                        ▼                │
  │      declarative macro       procedural macro        │
  │       match + expand         run TokenStream fn      │
  │            │                        │                │
  │            └───────────┬────────────┘                │
  │                        ▼                             │
  │                integrate new syntax ─────────────────┘
  │
  ▼
fully expanded crate
  │
  ▼
later resolution, type checking, MIR, code generation, …
```

The central part is a loop. Expansion can reveal new macro invocations,
imports, modules and other syntax, so the compiler repeatedly resolves what it can,
expands it, incorporates the result, and continues until the crate is fully expanded.

Name resolution therefore happens partly *during* expansion. The compiler does
not first discover the final meaning of every name and only then expand every macro;
each process supplies information needed by the other.

This is one reason macro problems can sometimes feel temporal: a definition may
exist in the crate that will eventually be produced, while still not being
available at the point where another expansion needs to resolve it.

## Outer and inner expansion

Nested syntax brings another useful distinction.

Given something shaped like:
```rust
outer!(inner!())
```

it should not generally be read as an ordinary function call
where `inner!` must finish before `outer!` begins.

Macro arguments are token trees. An outer macro can receive the tokens
representing `inner!()` and decide whether to reproduce, discard, rearrange or
otherwise transform them. The nested invocation may only become an invocation
for the compiler to expand after the outer expansion has emitted it again.

Some compiler built-ins perform eager expansion,
but this is the exception rather than the basic model.

Attributes and derives add their own ordering rules as well. An attribute
macro may transform an item before macros contained within that item's syntax
have expanded, and macros appearing inside attribute values have a defined
relationship to the other attributes on the item.

The useful rule is therefore not "macros expand from the inside out" or
"macros expand from the outside in". Expansion is contextual, iterative,
and interleaved with the name resolution needed to keep it moving.

## Phase boundaries

Several different activities are casually called "compile-time",
but keeping them separate makes macro behavior easier to reason about.

Cargo build scripts run before compilation of the crate they configure.
They can emit configuration flags and environment values, generate files,
inspect the build environment, and influence the compilation that follows.
They are code generation around the compilation process,
not macro expansion within the crate.

A procedural-macro crate is itself compiled before a crate that uses it.
The compiler can then invoke those compiled procedural macros
while expanding the dependent crate.

Declarative macros, procedural-macro invocations, built-in macros and the
syntax they generate all participate in the expansion of that crate.

So there are at least three related but distinct moments:
```text
Cargo/build preparation
        ↓
proc-macro dependencies available
        ↓
crate parsing ↔ resolution ↔ macro expansion
        ↓
later compilation
```

Confusing these boundaries can lead to assumptions
about what information or syntax should already exist at a particular point.

## Resolution and bootstrap macros

Because macro expansion and macro resolution depend on one another, an
indirectly introduced macro is not always interchangeable with a definition
that is directly available from the beginning.

This matters particularly for infrastructure used by other macro machinery
or by attributes involved early in compilation. devela therefore keeps a few
bootstrap facilities directly available rather than relying on the module
machinery that those facilities themselves help construct.

This can otherwise lead to errors such as "resolution is stuck":
the compiler knows that further expansion might eventually reveal a candidate,
but cannot make enough progress to establish what the current invocation means.

When a macro participates in bootstrapping another part of the macro or module system,
its *time of availability* can be as important as its final path in the expanded crate.

## Hygiene

Expansion also carries information about where tokens came from.

This is broadly known as *macro hygiene*. It determines, among other things,
which identifiers refer to names at the macro definition site and which refer
to names at the invocation site.

Declarative and procedural macros have different hygiene behavior and
different escape hatches. Declarative macros have special facilities such as
`$crate` for referring back to the crate that defined them. Procedural macros
operate on tokens and spans and currently require more deliberate care around
generated names and paths.

Hygiene explains several macro limitations that otherwise look unrelated.

## Language evolution

Some macro techniques work around limitations of Rust's current macro system.

Rust continues to evolve macro hygiene, the positions in which procedural attributes
may operate, declarative metavariable capabilities, identifier construction,
and related parts of expansion and name resolution.
