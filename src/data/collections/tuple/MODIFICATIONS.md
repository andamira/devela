This is derived work from the
[`tupl`](https://crates.io/crates/tupl/0.4.0) crate,
including the following modifications:

- implement for tuples of arity 12 by default.
- support arities of 24, 36, 48, 60, 72 or 96 via features.
- add the `ARITY` associated constant with the arity of the tuple.
- add the `MAX_ARITY` associated constant with the maximum supported arity.
- add a method `fmt` to wrap it in `TupleFmt` allowing to `Display` and `Debug` tuples.
- remove the `GrowableTuple` and `NonEmptyTuple` traits, moving their methods to `Tuple`.
- rename methods: `truncate_head` to `split_head`, `truncate_tail` to `split_tail`.
- rename types: `TruncateHead` to `NoHead`, `TruncateTail` to `NoTail`.
- add methods: `nth`, `nth_clone`, `nth_ref`, `nth_mut`.
- add associated enums: `TupleElement`, `TupleElementRef`, `TupleElementMut`.
- add one associated type per tuple index field.
- rename the identifiers and other misc. refactoring.
- replace the macro code generation with build script.

