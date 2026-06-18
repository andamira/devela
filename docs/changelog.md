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

## worskspace
- add cargo aliases: `c_<x|a|r><32|64>l`.

## features & flags
- new features: `code_all`, `data_all`, `geom_all`, `lang_all`, `media_all`, `num_all`, `org_all`, `phys_all`, `run_all`, `sys_all`, `text_all`, `ui_all`, `vita_all`, `widget`, `work_all`.
- remove features: `layout`.
- add more feature gates for: `time`, `ui`.

## documentation
- make `all` and `all_` root modules public.
- remove `zall` and `zall_` public root modules.
- clarify public module and hidden scope features.

## examples
- update examples: `web_api`, `web_workers`:
  - add optional JS bundling and wasm-opt paths for web builds.
  - show keyboard events.

---

# Modules

### code::util
- extend `whilst!` with indexed slice iteration.

#### data::access::route
- new types: `Route`, `RouteAnchor`, `RouteName`, `RouteSeg`.

#### data::layout::buffer
- new macro `buffer_ring!`.
- update `buffer_linear!`:
  - new impl `option` methods: `swap_remove_prim`, `swap_remove_copy_prim`.
  - fix impl `option` methods: `truncate`, `swap_remove`, `from_array_unchecked`.
- rename: `BufferStaticExample` to `BufferLinearStaticExample`, `BufferViewExample` to `BufferLinearViewExample`, `BufferAllocExample` to `BufferLinearAllocExample`.

#### media::visual::image
- new types: `ImageInfo`, `ImageFrameInfo`, `ImageFrameSpan`.
- udpate `ImageError`; add new variant: `InsufficientBuffer`.
- update `Pnm`:
  - implement all classic PNM variants P1..P6.
  - remove `alloc` feature-gates.
  - make all methods const.

### sys::io
- re-export missing `IoSeekFrom` from std.

##### sys::os::browser::web
- new type `WebEventKey`.
- move `<from|to>_web_*` conversions from `KeyFfi` to `Key`.
- modularize browser JS bindings.

#### sys::os::linux
- new types: `LinuxFd`, `LinuxFileType`, `LinuxOpenOptions`, `LinuxPipe`, `LinuxPipeFlags`, `LinuxSeekFrom`.
- add private types: `LINUX_AT`.
- delete private types: `AT_FDCWD`.
- make crate-private: `LINUX_FILENO`, `LINUX_O_FLAGS`, `LINUX_S_IFMT`, `LINUX_SEEK`.
- update `Linux`:
  - new methods: `open_fd`, `open_fd_at`, `close_fd`, `read_fd`, `write_fd`, `write_fd_all`, `syscall_openat`.
  - re-implement stdio read and write methods.

#### sys::os::term
- make `ansi` module public.

### text::str
- update `Str`:
  - add methods: `eq`, `starts_with[_char]`, `ends_with[_char]`, `strip_<prefix|suffix>[_char]`, `strip_circumfix[_chars]`.

## ui
- new submodules: `frame`, `route`, `semantic`, `widget`.

### ui::frame
- new types: `UiId`, `UiKey`, `UiScope`, `UiFrame`, `UiPhase`.

### ui::layout
- new types: `Layout1d`, `LayoutReceipt`, `Lunit`.
- new aliases: `UiExt`, `UiPos`, `UiRect`, `UiStride`.

### ui::route
- new types: `HitRegion`, `RouteActive`, `RouteCapture`, `RouteFocus`, `RouteHot`.

### ui::semantic
- new types: `UiRole`, `UiAction`, `UiActions`, `UiFlags`, `UiEntry`, `UiText`.

### ui::view
- new types: `UiLayer`, `UiView`, `UiViewFlags`, `UiViewForm`.

### ui::widget
- new types: `UiButton`, `UiResponse`, `UiResponseFlags`.


[0.29.0]: https://github.com/andamira/devela/releases/tag/v0.29.0
