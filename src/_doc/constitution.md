<!-- devela/src/_doc/constitution.md -->

> This constitution is authoritative for current design decisions,
> but remains revisable as devela evolves.

### Contents

0. [Nature of this document](#0-nature-of-this-document)
1. [Identity](#1-identity)

    <details>
      <summary>1.1–1.2</summary>
      <ul>
        <li><a href="#11-purpose">1.1 Purpose</a></li>
        <li><a href="#12-the-prime-directive">1.2 The prime directive</a></li>
      </ul>
    </details>
2. [Design posture](#2-design-posture)

    <details>
      <summary>2.1–2.9</summary>
      <ul>
        <li><a href="#21-smallest-honest-abstraction">2.1 Smallest honest abstraction</a></li>
        <li><a href="#22-orthogonality-first">2.2 Orthogonality first</a></li>
        <li><a href="#23-core-to-complex-emergence">2.3 Core-to-complex emergence</a></li>
        <li><a href="#24-explicit-transitions">2.4 Explicit transitions</a></li>
        <li><a href="#25-no_std-posture">2.5 <code>no_std</code> posture</a></li>
        <li><a href="#26-stable-rust-first">2.6 Stable Rust first</a></li>
        <li><a href="#27-grateful-regradation">2.7 Grateful regradation</a></li>
        <li><a href="#28-representational-permeability">2.8 Representational permeability</a></li>
        <li><a href="#29-static-dynamic-continuity">2.9 Static-dynamic continuity</a></li>
      </ul>
    </details>
3. [Structural rules](#3-structural-rules)

    <details>
      <summary>3.1–3.6</summary>
      <ul>
        <li><a href="#31-feature-boundaries">3.1 Feature boundaries</a></li>
        <li><a href="#32-api-policy">3.2 API policy</a></li>
        <li><a href="#33-allocation-posture">3.3 Allocation posture</a></li>
        <li><a href="#34-boundary-discipline">3.4 Boundary discipline</a></li>
        <li><a href="#35-taxonomy-before-fill">3.5 Taxonomy-before-fill</a></li>
        <li><a href="#36-peeling-reconstruction-and-cost-visibility">3.6 Peeling, reconstruction, and cost visibility</a></li>
      </ul>
    </details>
4. [Generative strategy](#4-generative-strategy)

    <details>
      <summary>4.1–4.5</summary>
      <ul>
        <li><a href="#41-generator-macro-principle">4.1 Generator-macro principle</a></li>
        <li><a href="#42-anti-lattice-rule">4.2 Anti-lattice rule</a></li>
        <li><a href="#43-macro-generation-guidelines">4.3 Macro generation guidelines</a></li>
        <li><a href="#44-thin-macro-layering">4.4 Thin macro layering</a></li>
        <li><a href="#45-tooling-as-structure">4.5 Tooling as structure</a></li>
      </ul>
    </details>
5. [Surface semantics](#5-surface-semantics)

    <details>
      <summary>5.1–5.6</summary>
      <ul>
        <li><a href="#51-taxonomy-contract">5.1 Taxonomy contract</a></li>
        <li><a href="#52-semantic-precedence">5.2 Semantic precedence</a></li>
        <li><a href="#53-selective-upward-exposure">5.3 Selective upward exposure</a></li>
        <li><a href="#54-naming-and-surface-conventions">5.4 Naming and surface conventions</a></li>
        <li><a href="#55-naming-family-consistency">5.5 Naming family consistency</a></li>
        <li><a href="#56-documentation-posture">5.6 Documentation posture</a></li>
      </ul>
    </details>
6. [Governance](#6-governance)

    <details>
      <summary>6.1–6.5</summary>
      <ul>
        <li><a href="#61-evidence-and-measurement">6.1 Evidence and measurement</a></li>
        <li><a href="#62-pre-10-compatibility-posture">6.2 Pre-1.0 compatibility posture</a></li>
        <li><a href="#63-stable-edge-msrv">6.3 Stable-edge MSRV</a></li>
        <li><a href="#64-decision-records">6.4 Decision records</a></li>
        <li><a href="#65-constitutional-evolution">6.5 Constitutional evolution</a></li>
      </ul>
    </details>


### 0. Nature of this document

This document defines devela's design constraints, decision rules, and enduring
project posture. It is normative but not exhaustive: a living record of
principles, policies, and boundaries as the crate-family evolves.

Identity and design posture are expected to change slowly. Structural rules,
strategies, and policies may evolve more readily as implementation experience
reveals better distinctions.

When principles pull in different directions, prefer the smallest coherent
tradeoff that preserves their intent. Material exceptions or changes in
direction should be made explicit and, when significant, recorded.

This is not a module catalog, a tutorial, or a substitute for measurement and
domain-specific judgment.

### 1. Identity

#### 1.1 Purpose

devela is a foundation crate-family for building systems and creative tooling
with stable Rust, where core abstractions remain small, composable, and portable
across `no_std`/`alloc`/`std` and across backends.

#### 1.2 The prime directive

Prefer *kernel atoms* that compose over *precompiled combinations* that sprawl.

devela grows through layering, composition, and generation
rather than by shipping every useful permutation upfront.

### 2. Design posture

#### 2.1 Smallest honest abstraction

Represent the minimal state that truly exists; everything else is layered.

#### 2.2 Orthogonality first

Avoid coupling concerns that can vary independently, especially semantic meaning,
representation, storage ownership, memory layout, access capability, and execution strategy.

#### 2.3 Core-to-complex emergence

Build primitives that become engines when composed.

#### 2.4 Explicit transitions

When state evolves through time, cursor movement, iteration, or phase change,
the API should make the transition visible.

#### 2.5 `no_std` posture

Default to `core`; opt into `alloc` or `std` only where they buy real leverage.

#### 2.6 Stable Rust first

Nightly Rust must not be required for the baseline supported surface.
Nightly capabilities may exist as opt-in augmentation.

#### 2.7 Grateful regradation

Begin with the least sufficient capacities and treat every supported grade as
a complete expressive medium in its own right.

Greater capability should add depth, precision, scale, or subtlety without
invalidating simpler forms. Prefer independent capability dimensions and
explicit adaptation over rigid backend tiers.

#### 2.8 Representational permeability

Abstractions may add meaning and enforce invariants, but should not
unnecessarily conceal the representation through which they operate.

Foundational types should expose stable, canonical views of their shape, layout,
storage, state, or components when these are enduring parts of the model.
Prefer reversible layering: semantic structures should be peelable into
smaller honest atoms and reconstructible from validated atoms.

A semantic type may admit several useful representational lenses without
surrendering its own identity or forcing one physical form on every use.

Representational permeability does not require public fields,
unrestricted mutation, or exposure of incidental implementation details.
Invariants, capabilities, and safety boundaries remain explicit.

Canonical semantic access need not use the narrowest physical representation.
Prefer natural types and forms when narrowing would not materially improve the
abstraction's retained structural density, capacity, or invariants.

When representation width, niche choice, packing, or specialization is itself
a meaningful structural degree of freedom, compact forms may belong directly
in the abstraction or in generated variants. Otherwise, narrower serialized
or transported forms should remain explicit representations with coherent
conversion to and from the semantic form.

Where specialization can derive information from type or compile-time context,
prefer eliminating redundant stored state over merely encoding it more narrowly.

#### 2.9 Static-dynamic continuity

When a concept admits both compile-time specialization and runtime choice,
prefer a graded path from concrete const-capable forms, through trait-generic
forms, to finite tagged representations, and only then to broader erasure.

Transitions should be explicit and, where practical, reversible. Finite dynamic
families should permit re-entry into type-specialized code without making
dynamism the default representation.

### 3. Structural rules

#### 3.1 Feature boundaries

A feature may imply another only when the first has no substantial
user-meaningful existence without the second.

Features remain separate when their combinations
represent meaningful supported layers.

#### 3.2 API policy

Panicking APIs should correspond to programmer error, such as violated
invariants or documented preconditions. Failure expected during normal
operation should be expressible through a fallible API.

When performance, safety, or validation posture materially changes an operation's
contract, prefer explicitly distinct APIs over a single ambiguous one.

#### 3.3 Allocation posture

When an operation can be expressed with fixed buffers or caller-provided
storage, devela prefers that route in core abstractions.

Allocating convenience is additive, gated, and never the only path to functionality.

#### 3.4 Boundary discipline

Core abstractions should be defined by devela's own semantics, not by the shape
of foreign APIs or backend quirks.

Integrations, FFI layers, platform bindings, and backend adapters should live at
the edge and translate inward when possible, rather than pulling external naming
and structure into the kernel.

#### 3.5 Taxonomy-before-fill

A module or namespace may exist before it is materially populated
when its place in the taxonomy is already clear.

Empty or lightly populated scaffolding is acceptable when it reduces
future friction and preserves a coherent long-term map.

#### 3.6 Peeling, reconstruction, and cost visibility

Where meaningful, foundational types should provide coherent paths
for borrowed inspection, exclusive access, ownership recovery,
decomposition, and checked reconstruction.

Peeling operations should reveal canonical semantics
rather than accidental implementation detail.

Reconstruction should validate the invariants required by the abstraction;
unchecked reconstruction may exist only behind an explicit safety boundary.

Distinguish zero-copy views and reinterpretations from operations that allocate,
copy, materialize, reorder physical storage, or perform effectful access.
Such costs and semantic changes should be visible in names, signatures,
or documentation rather than hidden behind foundational convenience.

Construction and canonical reconstruction are distinct concerns.

An abstraction may provide ergonomic constructors that project a broader
input domain through clamping, wrapping, quantization, reserved values,
or other normalization when that behavior is useful and explicit.

Canonical reconstruction, however, must preserve its stated representation exactly:
non-admitted representations are rejected rather than silently normalized.

The domain accepted by convenience constructors, the semantic value domain,
and the canonical representation domain need not coincide.

### 4. Generative strategy

#### 4.1 Generator-macro principle

devela uses *generator macros* to resolve the tension
between breadth of coverage and compile time.

Instead of precompiling large combinations of generic components,
devela favors macros that define and implement tailored items
for the exact shape needed by the user.

This preserves monomorphized performance
while containing compile-time and public-surface explosion.

#### 4.2 Anti-lattice rule

Do not ship a combinatorial lattice of pre-made variants.

Avoid publishing every combination of index sizes, handle styles, ownership
modes, and backends as separate ready-made artifacts. Prefer a small set of
canonical atoms plus a generative path that materializes the required variant.

#### 4.3 Macro generation guidelines

Generator macros exist to materialize *user-shaped* structure,
not to obscure semantics.

Generated surfaces should remain readable in documentation,
debuggable in practice, and stable in structure.

Parameters should be few and correspond to real degrees of freedom,
such as index type, handle encoding, optional fields, or bounds-checking posture.

#### 4.4 Thin macro layering

Macros may compose with other macros when each layer remains thin and readable,
and the composition ultimately resolves to a direct underlying operation.

#### 4.5 Tooling as structure

Build scripts, code generation, feature reflection, and documentation tooling
are part of devela's architectural surface when they preserve coherence,
reduce repetition, or keep generated APIs honest.

They should remain inspectable, deterministic,
and subordinate to the semantic public surface.

### 5. Surface semantics

#### 5.1 Taxonomy contract

Every item has a home by domain and meaning, not by convenience.

Naming, module placement, and doc-lines should encode what an item *is*
before how it is implemented.

#### 5.2 Semantic precedence

When convenience, implementation history, and semantic clarity disagree,
semantic clarity wins.

#### 5.3 Selective upward exposure

Canonical placement and discoverability are separate concerns.

A parent namespace may re-export a small selection of significant,
representative, or broadly useful items from its descendants
without changing their semantic home.

Selection is reconsidered at every level and is intentionally lossy:
an item exposed by one parent need not continue through every ancestor.
Prefer a few mature gateway items over mirroring descendant namespaces.

Upward exposure should reveal the character of a branch,
not reproduce its contents. Familiar upstream re-exports
should not crowd out items that better represent devela's own surface.

#### 5.4 Naming and surface conventions

Public names aim for semantic clarity and minimal drift: singular enum names,
consistent prefixes and suffixes for materially distinct backends or storage
strategies (`Vec*`, `*U8`, …), and tight doc-lines that state role and adjacency.

Aliases may exist for discoverability, but they do not drive taxonomy.

#### 5.5 Naming family consistency

Once a public naming family establishes a semantic pattern, new items should
conform to it unless divergence encodes a real conceptual distinction.

Avoid parallel naming dialects for the same conceptual layer.

#### 5.6 Documentation posture

Public documentation addresses the user of an abstraction,
not the process by which its design was reached.

Describe what an abstraction represents, what guarantees it provides, how it
relates to neighboring concepts, and the distinctions a user needs to apply it
correctly. Design deliberation, rejected alternatives, and instructions to
future maintainers belong in comments, notes, or decision records instead.

Documentation should be layered without unnecessary repetition:

- A doc-line identifies the domain and role of an item or module.
- A parent module distinguishes its children and provides the local conceptual map.
- A child module develops its own semantics, guarantees, boundaries, and usage.

Prefer complementary layers over restating the same introductory sentence at
each level. Public prose should present established semantics directly rather
than narrating the architectural reasoning that produced them.

Documentation prominence should remain selective and discriminative.
Badges, notable markers, featured examples, and similar emphasis should
highlight distinctions that materially help users understand or use an item.

Importance or ubiquity alone does not justify prominence;
emphasis that appears everywhere ceases to communicate useful distinction.

### 6. Governance

#### 6.1 Evidence and measurement

Design decisions that materially affect compile time, code size, runtime behavior,
or generated surface area should admit measurement or testing.

#### 6.2 Pre-1.0 compatibility posture

Before 1.0, devela prioritizes internal coherence, taxonomy,
and surface regularity over backward compatibility.

Breaking refactors, renames, moves, and removals are acceptable
when they reduce conceptual drift, collapse accidental complexity,
or improve the long-term shape of the crate-family.

During this phase, deprecation shims are optional rather than required.

#### 6.3 Stable-edge MSRV

devela aims to remain near the latest stable Rust release.

Its MSRV advances aggressively when needed to preserve
a clean and modern implementation baseline.

#### 6.4 Decision records

Important choices are worth recording when their reasoning,
consequences, or conditions for reconsideration might otherwise be lost.

The form need not be fixed or formal. What matters is preserving enough context
to help the project remain coherent across long spans of evolution.

#### 6.5 Constitutional evolution

This constitution should change when repeated design experience reveals a clearer
principle, an invalid assumption, or a recurring tension not yet expressed.

It should not absorb every local decision. Amendments should increase the
document's ability to guide future choices rather than merely record the past.

Material revisions should preserve enough of the reasoning behind the change
to make the constitution's evolution intelligible.

