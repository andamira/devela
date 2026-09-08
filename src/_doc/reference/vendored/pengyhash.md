This is derived work from the [pengyhash] v0.2 algorithm,
including the following modifications:

- make it a struct implementing `Hasher`.
- make the algorithm *const*.
- make the seed 64-bit.
- add extra methods.

[pengyhash]: https://github.com/tinypeng/pengyhash/blob/70a23e40a2be2e784a68078213b7675055f21949/pengyhash.c
