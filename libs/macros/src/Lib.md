## Conditional compilation

Each form of conditional compilation takes a compilation predicate that
evaluates to `true` or `false`.

These are the [`#[compile]`][compile()] and [`#[compile_attr]`][compile_attr()]
attributes and the [`cif!`][cif()] macro.

They are similar to the [`#[cfg]`][1] and [`#[cfg_attr]`][2] attributes and
the [`cfg!`][3] macro, except they use *compilation predicates*.

There is also the [`#[compile_doc]`][compile_doc()] macro to conditionally
compile documentation blocks depending on predicates.

### Panics
Compilation macros will panic if they encounter an unrecognized predicate.
As opposed to configuration macros (`cfg!`) that return `false`
for unrecognized predicates without signaling an error.

### Compilation predicates

The following compilation predicates are supported:

- unary:
  - A bare predicate returns `true` only if it is the **`true`** literal.
  - A bare predicate returns `false` if it's the **`false`** literal or it's *empty*.
  - `not()`: returns `true` only if the predicate does **not** evaluate to **`true`**.

- binary:
  - `equal()`: returns `true` if both predicates are evaluated as **equal**.
  - `xor()`: returns `true` if **only one** predicate **is `true`**, but **not both**.

  - `eq()`: returns `true` if both predicates are **number literals** and Left **==** Right.
  - `ne()`: returns `true` if both predicates are **number literals** and Left **!=** Right.
  - `ge()`: returns `true` if both predicates are **number literals** and Left **>=** Right.
  - `gt()`: returns `true` if both predicates are **number literals** and Left **>** Right.
  - `le()`: returns `true` if both predicates are **number literals** and Left **<=** Right.
  - `lt()`: returns `true` if both predicates are **number literals** and Left **<** Right.

- non-binary:
  - `any()`: returns `true` if **any** predicate **is `true`**.
  - `all()`: returns `true` if **all** predicates **are `true`**.
  - `none()`: returns `true` if there is **no given** predicate.
  - `some()`: returns `true` if there is **some given** predicate.
  - `diff()`: returns `true` if **any** predicate has a **different text**.
  - `same()`: returns `true` if **all** the predicates have the **same text**.
  - `xany()`: returns `true` if there are **any `true`** predicates, but **not all**.
  - `xodd()`: returns `true` if there is an **odd number** of `true` predicates.
  - `xone()`: returns `true` if there is just **one `true`** predicate, but **no more**.

- pointer width:
  - `pointer_width_eq(width)`: returns `true` if current pointer width **==** the given width.
  - `pointer_width_ne(width)`: returns `true` if current pointer width **!=** the given width.
  - `pointer_width_ge(width)`: returns `true` if current pointer width **>=** the given width.
  - `pointer_width_gt(width)`: returns `true` if current pointer width **>** the given width.
  - `pointer_width_le(width)`: returns `true` if current pointer width **<=** the given width.
  - `pointer_width_lt(width)`: returns `true` if current pointer width **<** the given width.

- endianness:
  - `little_endian()`: returns `true` if current architecture is little-endian.
  - `big_endian()`: returns `true` if current architecture is big-endian.

When more than 1 predicate is supported, they are separated by commas.

[1]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
[2]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
[3]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-macro
