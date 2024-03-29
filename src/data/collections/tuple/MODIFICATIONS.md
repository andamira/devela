This is derived work from the
[`tupl`](https://crates.io/crates/tupl/0.4.0) crate,
including the following modifications:

- rename the `Tuple` trait to `ExtTuple`.
- implement for tuples of arity 15 by default.
- support arities of 31, 63, 95 or 127 via features.
- add the `ARITY` associated constant with the arity of the tuple.
- add the `MAX_ARITY` associated constant with the maximum supported arity.
- add a method `fmt` to wrap it in `TupleFmt` allowing to `Display` and `Debug` tuples.
- remove the `GrowableTuple` and `NonEmptyTuple` traits, moving their methods to `ExtTuple`.
- rename methods: `truncate_head` to `split_head`, `truncate_tail` to `split_tail`.
- rename types: `TruncateHead` to `NoHead`, `TruncateTail` to `NoTail`.
- rename the identifiers and other misc. refactoring.
- simplify and unify the generative macros.

