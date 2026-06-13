# devela changelog

[0.29.0-wip] unreleased
=======================

> .
> —

```
```

## Key changes:

------------------------------------------------------------------------------

# Project

### examples
- update examples: `web_api`, `web_workers`:
  - add optional JS bundling and wasm-opt paths for web builds.

---

# Modules

#### data::layout::buffer
- new macro `buffer_ring!`.
- update `buffer_linear!`:
  - new impl `option` methods: `swap_remove_prim`, `swap_remove_copy_prim`.
  - fix impl `option` methods: `truncate`, `swap_remove`, `from_array_unchecked`.
- rename: `BufferStaticExample` to `BufferLinearStaticExample`, `BufferViewExample` to `BufferLinearViewExample`, `BufferAllocExample` to `BufferLinearAllocExample`.

##### sys::os::browser::web
- modularize browser JS bindings.


[0.29.0]: https://github.com/andamira/devela/releases/tag/v0.29.0
