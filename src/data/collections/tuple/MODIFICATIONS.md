This is derived work from the
[`tupl`](https://crates.io/crates/tupl/0.4.0) crate,
including the following modifications:

- rename the `Tuple` trait to `ExtTuple`.
- implement for tuples up to arity `106`.
- add the `LEN` associated constant with the arity of the tuple.
- remove the `GrowableTuple` and `NonEmptyTuple` traits, moving their methods to `ExtTuple`.
- rename methods: `mut_head` to `head_mut`, `mut_tail` to `tail_mut`.
- rename methods: `truncate_head` to `split_head`, `truncate_tail` to `split_tail`.
- rename types: `TruncateHead` to `NoHead`, `TruncateTail` to `NoTail`.
- rename the identifiers and other misc. refactoring.
- simplify and unify the generative macros.

