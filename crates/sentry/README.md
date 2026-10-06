# devela_sentry

Downstream validation of **devela**'s public API, behavior, portability, and generated code.

`devela_sentry` exercises devela from outside the repository's main Cargo workspace,
so it can validate assumptions that cannot be tested faithfully from inside the main crate.

It defines its own small workspace for validation and probe packages
and is not intended for publication.


## Structure

- `src/` and `tests/` validate public API and behavioral contracts as an external consumer.
- `examples/` contain manually runnable downstream cases and compiler diagnostics.
- `probes/` contain focused compiler, target, layout, and code-generation experiments.

Probe packages represent shared compilation environments
rather than individual experiments. A package may contain multiple
modules or binaries with the same dependency and feature requirements.

Current probe environments:

- `probes/arch` — architecture-specific instruction experiments.
- `probes/log` — cross-platform diagnostic-output portability.
- `probes/num` — numeric code-generation experiments.


## Inspecting probes

Run the shared inspector from a probe package:

```sh
cd probes/num

../inspect.sh host
../inspect.sh avr
../inspect.sh host avr
../inspect.sh mcu
../inspect.sh all
```

The inspector accepts individual targets and target groups,
and can inspect either a library or a binary target. See:

```sh
../inspect.sh --help
```

Generated assembly is written under `probes/out/`;
Cargo build artifacts are kept under `probes/target/`.
Both are ignored by Git.

Individual probe environments may add specialized reports
without duplicating the compilation machinery. For example:

```sh
cd probes/arch
./check-asm.sh avr
./check-asm.sh mcu
```

The generic inspector owns compilation and generated artifacts;
probe-local tools interpret those artifacts for a particular experiment.
