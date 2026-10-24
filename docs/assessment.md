# Crate assessment

A rough assessment of each crate along two independent dimensions:

- **Maturity** — How developed, tested, and refined is its existing implementation?
- **Coverage** — How much of its intended scope has been implemented?

Both use a **1–5 scale** (maturity / coverage):
- 1 — initial / minimal
- 3 — functional / partial
- 5 — well-established / comprehensive

Scores are judgments, not precise measurements.
They do not imply API stability, production readiness, or completion.
Coverage is relative to the currently intended scope, which may evolve.

## Current assessment

| Crate           | Maturity | Coverage |
| --------------- | :------: | :------: |
| `devela`        |    5     |    3     |
| `devela_macros` |    4     |    5     |
| `devela_bridge` |    2     |    1     |
| `devela_extend` |    1     |    1     |
| `devela_micros` |    3     |    2     |
| `devela_sentry` |    3     |    2     |

**`devela` (5, 3).** Extensive implementation and repeated
architectural refinement, with substantial working functionality.
Much of its envisioned scope remains unimplemented.

**`devela_macros` (4, 5).** A well-developed collection of procedural macros
that largely fulfills its current purpose. Further internal restructuring
is likely, but there are no major known functional gaps.

**`devela_bridge` (2, 1).** A working ABI and binding-generation prototype,
demonstrated with C and Odin. It does not yet expose meaningful portions
of devela's actual API.

**`devela_extend` (1, 1).** A clear purpose and an initial scaffold,
but no substantive implementation yet.

**`devela_micros` (3, 2).** Rapidly maturing hardware support,
with working implementations and a particularly strong set of examples.
Significant gaps remain across devices, MCUs, boards, and computers;
maturity may already be approaching 4.

**`devela_sentry` (3, 2).** Working downstream validation
and compiler/target probing infrastructure. Coverage of
public contracts, targets, and scenarios remains limited.
