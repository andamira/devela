<!-- devela/src/_doc/design/principles.md -->

# Design principles

devela is a foundation for systems and creative tools in Rust. These principles
guide the design of its parts and explain the choices that recur across the
library and its supporting crates.

They are the current basis for design decisions. When they pull in different
directions, the aim is the smallest coherent design that preserves their intent.
Important exceptions need a reason that can be recorded and revisited.
These principles do not settle every case; specific problems still need
judgment and evidence.

## Build from useful parts

An abstraction should represent a useful concept, with only the state and
requirements that concept needs. More involved facilities can build on those
parts while leaving them useful on their own.

This is how the library is meant to grow. Common mechanisms can serve several
domains, and new layers add capabilities without replacing their foundations.
Composing a few reusable parts leaves room for uses that were not anticipated
when those parts were written.

## Keep choices independent

Meaning, representation, storage ownership, memory layout, access capabilities,
execution strategy, and environment are separate questions wherever they can
vary independently. Choosing one should not unnecessarily settle the others.

For example, `pool!` lets the caller choose index and generation representations
independently of fixed or allocating storage. Both forms access values through
handles containing a slot index and a generation.

Start with the least capability sufficient for the task. A form with fewer
requirements should be complete and useful on its own. Greater capability can
add precision, scale, or expressive range while keeping the simpler form
available. Independent capabilities and explicit adaptations are more useful
than treating backends as a single ladder from lesser to greater.

Compile-time specialization and runtime choice should also connect. Prefer a
path from concrete forms with `const` operations where possible, through generic
interfaces, to finite runtime alternatives, and finally to broader type erasure
as needed. When a runtime family is finite, keep a path back to its concrete
forms where practical. These transitions should be explicit and, where possible,
reversible.

## Make representations accessible

A foundational type should offer stable ways to inspect its shape, state,
storage, or components when these are meaningful parts of the abstraction.
It may support several useful views of the same value.

Access can include borrowed inspection, exclusive access, recovering owned
storage, or taking a value apart and rebuilding it. Each operation should expose
the representation the type promises, while preserving its invariants and
safety boundaries. This does not require public fields or unrestricted access
to implementation details.

Rebuilding a value from its canonical representation must preserve that
representation exactly. Invalid representations are rejected. An unchecked
path requires an explicit safety boundary.

Convenient construction serves a different purpose. A constructor may clamp,
wrap, quantize, recognize reserved values, or otherwise normalize its input when
that behavior is useful and explicit. The accepted inputs, semantic values, and
canonical representations need not have the same domains. A successful canonical
reconstruction, however, must not silently substitute a different representation.

Compact storage is valuable when it improves capacity, layout, or the guarantees
of a type. Natural types are preferable for ordinary access when narrowing them
brings no material benefit. Index widths, niches, packing, and specialization
may belong in the abstraction or its generated variants when they are meaningful
choices. Compact transport and serialized forms can otherwise remain separate,
with clear conversions to and from the semantic form.

When type or compile-time information already determines a value, prefer leaving
out the redundant stored state over encoding it in fewer bits.

## Keep requirements explicit

Core abstractions begin with `core`, so their baseline is usable without `std`
or an allocator. When an operation can use fixed buffers or storage supplied by
the caller, that path should remain available. Allocating conveniences are added
through features. This lets applications choose where memory comes from and how
much they are prepared to use.

The baseline must work on stable Rust. Nightly capabilities can be offered as
optional additions. Allocation, the standard library, and platform facilities
are introduced where they provide something the selected capability needs.

A feature may imply another only when it has no substantial useful existence
without it. Features remain separate when their combinations represent
meaningful supported forms. Sharing a module is not enough to require sharing
all of its dependencies.

Core concepts are defined by their role in the library. Platform bindings,
foreign interfaces, and backend adapters translate between those concepts and
external systems where possible. Keeping that translation at the boundary
allows the same core to remain useful in other environments.

## State contracts and costs

State changes should be visible in the API. Advancing a cursor, taking an
iteration step, changing phase, or progressing through time should make clear
how the state moves from one point to the next.

An API should make it clear what the caller must provide and what the operation
guarantees. Panics belong to programming errors such as broken invariants or
violated preconditions. Failures expected during normal use need a fallible
path.

When performance, validation, or safety requirements materially change an
operation's contract, distinct APIs should make the choice explicit.

Borrowing or reinterpreting existing data has different costs and effects from
copying, allocating, materializing, reordering storage, or accessing data with
side effects. Names, signatures, or documentation should make those differences
visible. Convenience should not hide a cost that matters to the caller.

## Generate what is needed

Some abstractions admit many useful combinations of index types, handle forms,
storage backends, and optional fields. Publishing every combination separately
makes both the library and its compilation harder to manage.

Generator macros let the caller select a form and generate the corresponding
types and implementations. A small set of common parts supports the variations
that are actually needed. This is intended to retain concrete, monomorphized
code while limiting how much must be generated and compiled in advance.

Macro parameters should be few and correspond to real choices, such as an index
type, handle encoding, optional field, or validation contract. Generated APIs
need a stable, readable structure that can be documented and debugged. Their
meaning should remain clear without requiring the caller to trace the expansion.

Macros can build on other macros when each layer stays small and understandable,
and the expansion leads to direct underlying operations.

Build scripts, feature reflection, code generation, and documentation tools are
part of the library's design. They help keep it consistent and reduce repetition.
They should remain inspectable and deterministic, serving the public API.

## Give things a clear place

An item's name, module, and opening description should express what it is.
Its domain and meaning determine its home. Convenience and implementation
history should give way when they obscure that meaning.

A namespace can be established before it contains much code when its role is
already clear. Such a place is useful when it makes the map easier to understand
and gives later work a natural home.

An item's home and the places from which it is accessible serve different
purposes. A parent module may re-export a few significant or broadly useful
items, favoring mature ones that help readers enter that part of the library.
Each parent makes its own selection; a re-export need not continue through every
ancestor. The selection should express the branch's character. Familiar upstream
items should not crowd out the items that best represent that branch.

Public naming families should stay consistent. Enum names are singular, and
prefixes or suffixes distinguish meaningful differences such as a storage
backend or index width. A departure from an established pattern needs a real
conceptual distinction. Aliases can help people find an item while leaving its
domain and canonical name clear.

## Write for the reader

API documentation explains what an abstraction represents, what it guarantees,
and how to use it correctly. Relationships to nearby concepts belong there when
they help the reader make a choice. Design debates, rejected alternatives, and
maintenance instructions have their own homes in comments, notes, or decision
records.

The layers of documentation should complement one another. An opening doc-line
identifies the role of an item or module. A parent module explains how its
children relate. Each child develops its own guarantees, boundaries, and use.
Readers should gain something at each step without meeting the same introduction
again.

Emphasis also needs a purpose. Badges, notable markers, and featured examples
should identify differences that help someone understand or use an item.
Something being important or widespread does not by itself make it useful to
highlight everywhere.

## Learn from experience

Decisions that materially affect compilation, code size, runtime behavior, or
the extent of a generated API need evidence that can be tested or measured. The
intended benefit of a design should be checked against what it costs in practice.

Before 1.0, the project gives priority to a clearer and more consistent library
over preserving every existing API. Renames, moves, removals, and other breaking
changes are acceptable when they improve that structure. Deprecation shims are
optional during this stage.

The project aims to keep its minimum supported Rust version close to stable
Rust. That minimum may advance quickly when newer capabilities allow a cleaner
implementation.

Important decisions deserve a record of their reasons, consequences, and the
conditions that might lead to reconsidering them. The record can be brief;
what matters is being able to understand the choice later.

These principles should change when repeated experience reveals a clearer idea,
an assumption that no longer holds, or a tension they do not yet address. Local
decisions need not all become general rules. The underlying purpose and broad
design principles should change slowly, while conventions and techniques can
respond more readily to what the work teaches. Revisions should preserve the
reasons for important changes and improve the document's usefulness for future
decisions.
