# devela_sentry

Downstream validation of **devela**'s public API and behavior.

It exercises devela as an external consumer in order to catch regressions and
validate assumptions that cannot be tested faithfully from inside the main crate.

It intentionally remains outside the devela workspace as a development
and validation crate rather than a library intended for downstream use.
