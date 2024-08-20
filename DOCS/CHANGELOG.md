# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## Unreleased [0.22.0-wip]

### Added
- show build environment variables if `__dbg` feature is enabled.
- new features: `_char7`, `_char8`, `_char16`, `_char24`, `_char32`.
- new `Float` and `ExtFloat` method `eval_poly` to evaluate polynomials.
- new `Float` and `ExtFloatConst` consts: `LOW_MARGIN`, `MEDIUM_MARGIN`, `HIGH_MARGIN`.
- new struct `Env` that namespaces `std::env` functions and constants.
- new `Graph` methods: `edge_exists_unchecked`, `edge_remove`.
- new type aliases: `AllocMapFx`, `AllocSetFx`.
- reexport `core::cell` types.
- impl `Num` for niche types.
- new `num::algebra` module.
- new `CellOption` trait.
- new traits: `MemPod`.
- new lints.

### Removed
- remove feature `num_geom`.
- move `num::geom::prim` submodule to separate crate `cuadra`.

### Changed
- refactor build script.
- rename `unsafe_dyn` feature to `unsafe_layout`.
- method `Graph::edge_exists` no loger panics.
- modify how features `_non_value_u8`, `_non_value_u16` are enabled for `Char*` types.
- make `data::dst` types use `MemPod` instead of `bytemuck::Pod`.
- move `num::geom::algebra` to `num::algebra::linear`.
- make `bytemuck` an optional dependency.

### Fixed
- fix build script utility call paths, add missing `_tuple*` features.

## [0.21.2] 2024-08-09

### Added
- add convenience fns: `out_dir` and `out_dir_path` to the build script.
- add new `build::utils` module.

### Changed
- move build codegen output from `/build/out/` to `{OUT_DIR}/build/`.
- show debug information about `OUT_DIR` and the code-generated files.
- move `println` to the `utils` module.
- update `.gitignore`.

### Fixed
- this should fix compilation in *docs.rs*.

## [0.21.1] 2024-08-08

### Changed
- rename build script's `construct` folder to `build`.
- update `.gitignore`.

### Fixed
- include `build` folder.
- fix docs.

## [0.21.0] 2024-08-08

### Added
- add features: `num_geom`, `num_float`, `num_int`, `num_rand`, `sys`, `safe_sys`, `unsafe_thread`.
- add features: `_num_all`, `_float_all`, `_float_f[32|64]`, `_int_all`, `_int_[iu][all|8|16|32|64|128|size]`.
- add features: `_bit_all`, `_bit_[iu][8|16|32|64|128|size]`.
- add features: `_ascii_all`, `_ascii_u[8|16|32|64|128|size]`.
- add features: `_non_value_all`, `_non_value_[iu][8|16|32|64|128|size]`.
- add features: `_non_range_all`, `_non_range_[iu][8|16|32|64|128|size]`.
- add features: `_range_all`, `_non_range_[iu][8|16|32|64|128|size]`.
- add features: `_sort_all`, `_sort_[iu][8|16|32|64|128|size]`.
- add features: `_cmp_all`, `_cmp_[iu][8|16|32|64|128|size]`.
- add features: `_default`, `_min_docs`, `_max_docs`, `_max`, `_docsrs_stable`,`__dbg`, `__no_test` `nightly_stabilized`.
- add features: `_node_all`, `_node_u[8|16|32|64|128|size]`.
- add compilation flags for reflection purposes, named `_some_*`.
- enable `doc_notable_trait` unstable feature with `nightly_doc`.
- add traits: `ExtFuture`, `ExtFloatConst`, `ExtLog`, `NumVector`.
- add functions: `future_block`, `hash_pengy`.
- add type: `Prompt`.
- add type: `Pinned`.
- add type: `Timecode`.
- add types: `NodeU*`, `EdgeU*` `GraphU*`.
- add types: `LoggerConfig`, `LoggerPrint`, `Logging`.
- add types: `Divisor`, `GcdExt`, `ValueQuant`.
- add types: `Extent`, `Extent2d`, `Extent3d`.
- add types: `Angle`, `AngleDirection`, `AngleKind`
- add types: `Vector`, `Vector2d`, `Vector3d`, `VecVector`.
- add types: `Point`, `Point2d`, `Point3d`, `Points`, `VecPoints`.
- add types: `HasherFnv`, `HasherFx`, `HasherFx32`, `HasherFx64`, `HasherBuildFnv`, `HasherBuildFx`.
- add types: `TupleIter`, `TupleIterRef`, `TupleIterMut`, `TupleElement`, `TupleElementRef`, `TupleElementMut`.
- add `ByteSized` constants: `PTR_BITS`, `LITTLE_ENDIAN`, `BIG_ENDIAN`.
- add unsigned `gcd_ext` and `gcd_ext_euc` methods to `Int`.
- add `Floating` and `ExtFloat` method: `neg_abs`.
- add `*_assign` methods to `Num` and `NumRef`.
- add `NumError` variant: `NoInverse`.
- add `DataError` variants: `NodeLinkNotSet`, `NodeLinkNotUnique`.
- add new arms to `array_init:` `init`, `init_heap`.
- add methods to `BareBox`: `as_ref`.
- add methods to `Array`: `as_bare_slice`.
- add methods to `Int`: `midpoint`, `modulo*`.
- add methods to `Int`, `NumInt`, `NumRefInt`: `midpoint`.
- add methods to `ExtAny`: `downcast_ref`, `downcast_mut`.
- add methods to `Tuple:`: `nth_ref`, `nth_mut`, `into_iter`, `iter_ref`, `iter_mut`.
- add methods to `Cast` and `PrimitiveCast`: `wrapping_cast_*`, for `[iu]size_[up|down]`.
- re-export more items from: `std::io`, `core::ptr`.
- re-export: `String`, `ToString`, `Rc`, `RcWeak`.
- add build script for debugging purposes.
- add lints table in manifest.

### Removed
- remove features: `num_all`, `ui`, `ui_all`, `num_float`, `num_int`, `num_niche_impls`, `num_niche_range`, `result`, `safe_result`, `fig`, `safe_fig`, `io`, `io_safe`, `os`, `os_safe`, `rend`, `safe_rend`, `safe_ui`, `safe_ui_term`.
- remove dependencies: `either`, `crossterm`, `miniquad`.
- remove deprecated `ident_total_count` macro.
- remove `repr(C)` attribute from niche types.
- remove `Mem` trait bound from `ByteSized`.
- remove `gfx` module.

### Changed
- bump rust version to 1.80.1.
- move `result` module inside `code`.
- move `io` moule and `os` submodules to `sys`.
- move `rustdoc-header.html` file to `/DOCS/`.
- move `_deps::{code, alloc, std}` to `::{_libcore, _liballoc, _libstd}`.
- rename `Range` to `InRange`.
- rename `Biting` to `Bitwise`.
- rename `Mem` trait to `ExtMem`.
- rename `Text` trait to `StrOwn`.
- rename `_docs` module to `_info`.
- rename `ExtTuple` trait to `Tuple`.
- rename `copy` function to `io_copy`.
- rename `StringEgc` to `EgcString`.
- rename `NonSpecific*` to `NonValue*`.
- rename mem::`BitSize` to mem::`BitSized`.
- rename mem::`ByteSize` to mem::`ByteSized`.
- rename `NonEdge*` aliases to `NonExtreme*`.
- rename `ByteSized::PTR_SIZE` constant to `PTR_BYTES`.
- rename `os` module to `sys`, make submodules private.
- rename `work` module to `exec`, make submodules private.
- rename `Floating` wrapper to `Float` and make it own `self`.
- rename `is_aarch64_feature_detected` to `detect_aarch64`.
- rename `is_x86_feature_detected` to `detect_x86`.
- rename `DataError` variant: `EmptyNode` to `NodeEmpty`.
- rename private feature `_exclude_example` to `__excluded`.
- rename methods: `into_array_const` to `into_array_copy`, `into_inner_const` to `into_inner_copy`, `from_array_const` to `from_array_copy`, `as_array_const` to `as_array_copy`, `as_tuple_const` to `as_tuple_copy`.
- improve `cdbg`, allow to customize the location path and print fmt.
- improve `enumset`, allow to specify the visibility and attributes of the set.
- make `Compare` methods: `pmin`, `pmax` and `pclamp` return `Option`.
- change `TaskWakerNoop` struct into `task_waker_noop` fn.
- add more consts to `Float` and `ExtFloat`.
- add the `?Sized` trait bound for `ExtAny` auto-impls.
- change `From<float>` for `Sign` to return the zero sign.
- change default `Tuple` arity to 12.
- change tuple capability features to: `_tuple_arity_24`, `_tuple_arity_36`, `_tuple_arity_48`, `_tuple_arity_72`.
- add bounds for `color_gamma_*` functions.
- make some methods const: `Slice::<u8>::trim_leading_bytes`, `StringU*::from_bytes_n[left|right]`, `Timecode::split_nanos_u[32|64]`.
- make some methods *optionally* const: `Slice::{lsplit, msplit_[left|right]}`, `Ascii::digits_str`, `StringU*::from_bytes_n[left|right][_unchecked]`.
- feature-gate `Compare` impls for primitives.
- feature-gate `StackU*` method `own_resize_default_truncate`.
- feature-gate `Float` methods: `clamp_total`, `max_total`, `min_total`.
- feature-gate `ExtFloat` methods: `clamp_total`, `max_total`, `min_total`.
- make optionally *const* `Int` methods: `is_prime`, `prime_nth`, `prime_pi`,
- move the enabling of all module's sub-features to the root module feature.
- update CI actions/checkout@v3 -> v4.
- update `devela_macros`.

### Fixed
- fix `exec` re-exports.
- fix `Bitsized` impl for niche types.
- fix feature-gating of `Compare` methods: `is_normal`, `is_subnormal`.
- make `_info/examples` parseable by `rustfmt`.
- add missing `NumInt` `sqrt` implementations.
- make no-std io `memrchr` function private.


## [0.20.0] 2024-03-12

### Added
- add traits: `NumToStr`, `ConstDefault`, `DataQueue`, `DataDeque`, `DataStack`, `DataDesta`, `ExtTuple`, `ExtArray`.
- add types: `Destaque`, `DestaqueIter`, `Stack`, `StackIter`, `Mismatch`, `Own`, `GfxError`, `GfxResult`, `Pnm`, `Array2d`, `OptRes`, `TupleFmt`, `ArrayFmt`, `UninitArray`, `Ascii`, `MilliToHour`, `NanoToSec`, `SecToYear`.
- add features: `fig`, `num_all`, `safe_code`, `safe_data`, `safe_fig`, `safe_gfx`, `safe_mem`, `safe_num`, `safe_os`, `safe_result`, `safe_text`, `safe_time`, `safe_ui`, `safe_ui_term`, `safe_work`, `unsafe_array`, `unsafe_async`, `unsafe_const`, `unsafe_dyn`, `unsafe_niche`, `unsafe_slice`, `unsafe_str`, `nightly_coro`, `nightly_doc`, `_exclude_example`, `ui_window`, `ui_events`, `num_niche_range`.
- add capability features: `_capability_max`, `_tuple_arity_31`, `_tuple_arity_63`, `_tuple_arity_95`, `_tuple_arity_127`.
- add RNGS: `Xabc`, `XorShift8`, `XorShift8Custom`, `XorShift16`, `XorShift32`, `XorShift64`, `XorShift128`, `XorShift128p`, `Xyza8a`, `Xyza8b`.
- add `Floating` constants: `FRAC_1_PHI`, `NEG_FRAC_1_PHI`, `SQ_PHI`, `FRAC_NEG_1_PHI`.
- add `no_std` reimplementations of `std::error` and `std::io` types and traits.
- add non-optional dependency `either` and re-export its items.
- add optional dependencies: `memchr`, `wide`, `crossterm`, `miniquad`, `rand_core`.
- add macros: `assert_eq_all`, `assert_approx_eq_all`, `unwrap`.
- add `Array` methods: `new_boxed`, `into_slice`, `into_vec`.
- add examples: `cargo-script`, `bitfield`, `enumset`.
- add fns: `mem_copy`, `serr` and `sok`.
- add type aliases: `isize_down`, `usize_down`.
- add niche number aliases: `NonEdge*`.
- add `Bare` type alias of `unit`.
- add module: `fig`.
- add script `tools/rustfmt` and `rustfmt_excluded_files` list.
- add new `DataError` variants: `MismatchedLength`, `InvalidAxisLength.
- add new `TextError` variants: `CharConversion`, `OutOfBounds`.
- add new `Floating` and `ExtFloat` methods: `sign`, `is_zero`, `is_sign_positive_nonzero`, `is_sign_negative_nonzero`.
- re-export proc-macros: `ident_total`, `ident_unique`, `ident_total_unique`.
- re-export crate-defined result-related types from `result`.
- re-export core types from `data::iter`.
- re-export core tpes from `num::cmp`.
- re-export `Default` from `code`.

### Removed
- remove `devela_depend`, simplify system for optional dependencies.
- remove features: `fullest`, `safest`, `unsafest`, `full_unsafe`, `data_unsafe`, `mem_unsafe`, `num_unsafe`, `os_unsafe`, `text_unsafe`, `ui_unsafe`, `ui_term_unsafe`, `work_unsafe`, `dep`, `dep_interop`.
- remove `IntBuf` type and `IntBufAble` trait.
- remove re-exported macro: `option_unwrap`.
- remove type aliases: `NonMax*` `NonMin*, DirectArray, BoxedArray.
- remove file `.gitattributes`.
- remove `CharConversionError`.
- remove `asci_` standalone fns.
- deprecate`ident_total_count` macro.

### Changed
- bump rust version to 1.76.0.
- make `num::cmp` non-public.
- move `cmp` from `data` to `num`.
- move `ops` from `num` to `code`.
- move `any` from `data` to `code`.
- move docs html header to `src/_doc`.
- move `.git_hooks` to `tools/git_hooks`.
- move traits: `Apply`, `Also` to `result`.
- make `bytemuck` a non-optional dependency.
- rename `src/Doc.md` to `src/_doc/features.md`.
- rename `::{_alloc, _core, _std}` to `::_deps::{alloc, core, std}`.
- rename the `__doc` module to `_docs` and the `_dep` module to `_deps`.
- rename the `error` module to `result`, and the `render` module to `gfx`.
- rename `AnyExt`, `OptionExt`, `ResultExt`, `SliceExt`, `SliceExtMut`, `StrExt` and `StringExt` to `ExtAny`, `ExtOption`, `ExtResult`, `ExtSlice`, `ExtSliceMut`, `ExtStr` and `ExtString` and `ExtVec` respectively.
- rename `LiteCoroutine`, `LiteCoroutineExecutor`, `LiteCoroutineWaiter` and `LiteCoroutineWaker` to `Coro`, `CoroRun` `CoroYield` and `TaskWakerNoop`, respectively.
- rename `ArrayStringError` to `TextError`, `ArrayU*String` to `StringU*`, `ArrayU8NonNulString` to `StringNonul`, `ArrayU8NonNulEgc` to `EgcNonul`.
- rename `DataErrors` to `DataError` and `NumErrors` to `NumError`.
- rename `num_int_niche` feature to `num_niche_impls`.
- rename `*fence` functions to `atomic_*fence`.
- rename `ascii_` re-exported macros to `str_`.
- prefix allocated collections with `Alloc`.
- rename aliases: `Dst*U` to `Dst*Usize`.
- rename mem::`Size` to mem::`ByteSize`.
- rename feature `docsrs` to `_docsrs`.
- rename `FloatOps` to `ExtFloat`.
- rename feature `full` to `all`.
- rename `Direct` to `BareBox`.
- rename `Slicing` to `Slice`.
- rename `Sorting` to `Sort`.
- rename `Primiting` to `Cast`.
- rename `Comparing` to `Compare`.
- document selected examples from `_docs`.
- derive common traits for `Boxed`, `Cast`.
- always compile `num` submodules in general.
- always compile `text::fmt`, `work::async::coro`.
- make `text` `chars` methods not depend on `alloc`.
- make `bitfield` and `enumset` examples standalone.
- recreate the `pre-commit` script to leverage `rustfmt`.
- update `enumset`: rename `LEN` associated constant to `ENUM_VARIANTS`, and add `enum_variants` method.
- update the `cdbg` macro with column number.
- update and improve the `array_init` macro.
- update the `cswap` macro with `xor` swap.
- improve the `pre-commit` git hook.

### Fixed
- refactor root modules, simplify and dissassociate root-level feature-gating.
- update and add more tests for macros: `cfor`, `mem_size_of_expr`.
- allow to forbid unsafe at the individual module level.
- fix `data::dst` feature-gates.

## [0.19.0] 2024-01-24

### Added
- new features: `docsrs`, `num_int_niche`.
- add optional dependency `unicode-width`.
- new root modules: `io`, `render`, `ui`.
- new fns `mem::ptr_in_stack`, `factorial_*`, `bytes_from_bits`.
- new structs `Priming`, `Comparing`, `Slicing`, `Sorting`, `Biting`, `Int`, `Frac`.
- new traits: `DataArray`, `PrimitiveCast`, `BitOps`, `NumInt`, `NumRefInt`, `VecExt`.
- new macros: `bitfield`, `enumset`, `ident_const_index`, `ident_total_count`, `init_array`.
- new `NumErrors` variants: `MismatchedSizes`, `NonNegativeRequired`, `PositiveRequired`.
- new color constants: `COLOR_LUMINANCE_[RED|GREEN|BLUE]`.
- complete `Floating` and `FloatOps` methods and constants.
- re-export `panic` from `error`, and `hint` from `code`.
- new type aliases: `isize_up`, `usize_up`.
- new enums: `DataErrors`, `Sign`.

### Removed
- remove `os::linux` functionality.
- remove features: `linux`, `linux_unsafe`, `unsafe_linux`, `unsafe_os`, `ops`, `ops_unsafe`, `unsafe_ops`, `unsafe_code`, `unsafe_color`, `unsafe_result`, `unsafe_time`, `code_usafe`, `color_unsafe`, `result_unsafe`, `time_unsafe`.
- remove standalone fns from: `num::ops`, `mem::slice`, `data::convert::collection`.
- remove linux specific tools and github actions.
- remove the `Underflow` variant from `NumErrors`.
- remove the prelude.

### Changed
- bump rust version to `1.75.0`.
- move `term` module to `ui`.
- move `any` module to `data`.
- move `color` module to `render`.
- move `num` and `ops` modules to `num`.
- move `data::slice` to `mem::slice`.
- move `error::{Also, Apply}` to `code`.
- move `num::convert` module to `code`.
- move niche number types to `num::niche`.
- move `data::array` to `data::collections::array`.
- move corresponding standalone fns to `Sorting`, `Comparing` and `Casting`.
- move `cmp` fns to `Comparing` methods: `clamp*`, `max*`, `min*`, `pclamp`, `pmax`, `pmin`.
- rename `meta` module to `code`.
- rename `task` module to `work`.
- rename `result` module to `error`.
- rename `NumError` and `NumResult` to `NumErrors` and `NumResult`, respectively, and move them to the num module.
- rename `AnsiColor3` to `AnsiColor3b` and `AnsiColor8` to `AnsiColor8b`.
- rename fns: `div_half_*` to `div_ties_*`.
- rename `term` features to `ui_term` features.
- rename `Fp` to `Floating` and `FloatExt` to `FloatOps`.
- rename `Floating` and `FloatOps` float parameters to `x`, `y` `z`.
- rename `Floating` and `FloatOps` method: `round` to `round_ties_away`.
- rename `FromPrimitives` to `PrimitiveJoin` and `IntoPrimitives` to `PrimitiveSplit`.
- rename fn `mem_ptr_ratio` and method `Size::ptr_ratio` to `ptr_size_ratio`.
- rename re-exported time types: `Duration` to `SystemDuration`, `Instant` to `SystemInstant`.
- re-export `BitwisePrimitives` and `CastPrimitives` traits from `prelude`.
- change `Ansi::print` method and `ansip` macro to depend on `std`.
- improve `Floating` type and `FloatOps` trait to be partially available without `std` or `libm` features, reverting to the taylor versions when there's no better option.
- impl `DataCollection` for `array`, `Vec`, `VecDeque`, `OrderedMap`, `OrderedSet`, `UnorderedMap`, `UnorderedSet`.
- impl `Error` and `Display` for `NumErrors`.
- make `ascii_calc_digit_*` functions public.
- make `DataCollection` always available.
- make `prime_number_theorem` compilable without `libm` nor `std`.
- make `devela_macros` a non-optional dependency, and update it.
- update color gamma fns, rename to `color_gamma_[apply|remove]_f*`, add `f64` versions.
- update `DataCollection` methods to return `DataResult`; add new methods `collection_contains`, `collection_count`.
- update `IntoPrimitives` related method to take `self`, remove one generic argument.
- update `CastPrimitives` with missing `usize` and `isize` methods.
- update `Collection` trait.
- update crate description.
- update `devela_depend`.

### Fixed
- fix `bytemuck` dependency when `data` is enabled.
- update and improve CI tests.
- add many missing attributes.
- fix and update many docs.

## [0.18.1] 2023-11-08

#### Fixed
- fix `FloatExt` re-export.
- fix `DERIVED.md` links.

## [0.18.0] 2023-11-03

### Added
- new `ops` gcd fns: `gcd_*`, `gcd_ext_*` and `gcd_ext_euc_*`, `lcm_*`.
- new `ops` base fns: `count_digits`, `count_digits_sign`, `count_digits_base`, `count_digits_base_sign`, `digital_root`, `digital_root_base`.
- new `ops` factor fns: `factors_*`, `factors_proper_*`, `factors_prime_*`, `factors_prime_unique_*`, `factors_prime_buf_*`, `factors_prime_unique_buf_*`, `factors_prime_unique_plus_buf_*`.
- new `ops` sort fns: `sort_bubble`, `sort_insertion`, `sort_merge`, `sort_quick_lomuto`, `sort_quick_hoare`, `sort_quick_3way`, `sort_selection`, `sort_shaker`, `sort_counting`, `sort_counting_buf`.
- new `mem` macro: `cswap`.
- add new fn to `LinuxTerminal` size: `pixels_per_cell`.
- new trait `FloatExt` and struct `Fp`.

### Changed
- changed result of `LinuxTerminalSize` fns: `cells` and `pixels` to `[u16; 2]`.
- rename `const_for` to `cfor` and make it always available.
- make `ops::cmp` float fns always available.
- add `libm` feature to docs.rs compilation.

#### Fixed
- add missing rustdoc header file.
- fix reversed order of `LinuxTerminalSize::cells`.

## [0.17.0] 2023-10-27

### Added
- new types: `DstArray`, `DstValue`, `DstStack`, `DstQueue`, `DstQueueIter`, `DstQueueIterMut`, `DstQueuePopHandle`, `DstStackIter`, `DstStackIterMut`.
- new `ops` fns: `div_rem_*`, `div_ceil_*`, `div_floor_*`, `div_half_away_*`, `div_half_towards_*`, `div_half_even_*`, `div_half_odd_*`, `sqrt_*`, `sqrt_ceil_*`, `sqrt_floor_*`, `scale_*`, `lerp_*`.
- new aliases: `DstQueueU`, `DstStackU`, `DstValueU`, `DstVecU`.
- new trait: `DstBuf`.
- impl more traits for `Array:` `PartialOrd`, `Ord`, `AsRef`, `AsMut`, `Borrow`, `BorrowMut`, `Hash`.
- new features: `color`, `color_unsafe`, `unsafe_color`, `ops`, `ops_unsafe`, `unsafe_ops`.
- add katex docs header to support mathematical notation.
- new optional dependency: `libm`.
- new modules: `color`, `ops`.

### Removed
- remove features: `nightly_docs`, `cmp`, `cmp_unsafe`, `unsafe_cmp`, `convert`, `convert_unsafe`, `unsafe_convert`.
- remove `az` optional dependency.

### Changed
- rename `Collection` to `DataCollection`.
- make `Array` private field public to the crate.
- rename `depend` feature to `dep` and `depend` module to `_dep`.
- allow warnings for broken intra-doc links when `dep` is disabled.
- move to `ops` the modules: `cmp`, `convert`.
- update cargo aliases for docs.
- bump `devela_depend`.

### Fixed
- fix test for `mem_size_of_expr`.
- remove redundant re-exports of optional dependencies.
- update cargo aliases.

## [0.16.0] 2023-10-24

### Added
- add `mem_size_of_expr` macro.
- new type `NumErr`, new type alias `NumResult`.
- re-export `BinaryHeap` from `data::PriorityQueue`.
- new time module, and features: `time`, `time_unsafe`, `unsafe_time`.
- add `min_fsize`, `max_fsize`, `clamp_fsize` `total_cmp_fsize`.
- new struct `Array`, and aliases `BoxedArray`, `DirectArray`.
- new trait `Collection`.

### Removed
- remove features: `ascii`, `ascii_unsafe`, `unsafe_ascii`, `char`, `char_unsafe`, `unsafe_char`, `fmt`, `fmt_unsafe`, `unsafe_fmt`, `future`, `future_unsafe`, `unsafe_future`, `option`, `option_unsafe`, `unsafe_option`, `slice`, `slice_unsafe`, `unsafe_slice`, `str`, `str_unsafe`, `unsafe_str`, `sync`, `sync_unsafe`, `unsafe_sync`, `texts`, `texts_unsafe`, `thread`, `thread_unsafe`, `unsafe_thread`.
- remove trait `AltDebug`.

### Changed
- rename `HashMap`, `HashSet` as `UnorderedMap`, `UnorderedSet`.
- rename `BTreeMap`, `BTreeSet` as `OrderedMap`, `OrderedSet`.
- change the result type of Num methods from `Option` to `Result`.
- move to `text` the root modules: `ascii`, `char`, `fmt` `str`.
- move to `task` the root modules: `future`, `sync`, `thread`.
- move the traits `Also`, `Apply` to `result`.
- move to `data` the root module: `slice`.
- rename `collections` module to `data`.
- rename `codegen` module to `meta`.
- rename `string` module to `text`.
- make `Storage` always compiled.

### Fixed
- re-export missing `result::{NeverOk, NeverErr} types.

## [0.15.0] - 2023-10-19

### Added
- new modules: `collections`, `future`, `task`.
- new features: `collections`, `depend`, `future`, `future_unsafe`, `unsafe_future`, `task`, `task_unsafe`, `unsafe_task`, `async`, `async_unsafe`, `term`, `term_unsafe`, `unsafe_term`, `collections_unsafe`, `unsafe_collections`.
- new type aliases: `Egc16`, `Egc32`, `Egc64`, `Egc128`, `NonNulEgc8`, `NonNulEgc16`, `NonNulEgc32`, `NonNulEgc64`, `NonNulEgc128`.
- new structs: `StringEgc`, `ArrayU8Egc`, `ArrayU8NonNulEgc`, `LiteCoroutine`, `LiteCoroutineExecutor`, `LiteCoroutineWaiter`, `LiteCoroutineWaker`, `Direct`, `Boxed`.
- re-export std's items from `collections`, `future`, `mem`, `task`, `vec`.
- new traits `AnyExt`, `Egc`, `Num`, `NumRef`, `Mem`, `Size`, `BitSize`, `Storage`.
- add new optional dependencies `devela_depend`, `hashbrown`.
- new fns: `mem_ptr_ratio`.
- new type aliases: `fsize`, `NoNum`.

### Removed
- remove macros: `manifest_dir`, `sfb`.
- remove features: `lean`, `leanest`, `chars`, `strings`, `os`, `os_unsafe`.
- remove unneded conversion impls from chars to strings.

### Changed
- bump MSRV to `1.73.0`.
- add features `linux`, `term` to `full`.
- optional dependencies can be enabled directly or via `depend`.
- rename `mem` fns by prefixing them with `mem_`: `mem_as_bytes`, `mem_as_bytes_mut`, `mem_as_bytes_sized`.
- rename `Num` trait fns with the `num_` prefix; rename fn `get` to `num_into`; add fn `num_from`.
- update `bytemuck`, enable more features.

### Fixed
- new private helper macro `reexport` for re-exported items.
- do not enable the empty `default` feature.
- improve re-exports rustdoc tags.
- fix `core::num` re-exports.
- improve many docs.
- improve CIs.

## [0.14.0] - 2023-10-04

### Added
- new single module features: `ascii`, `char`, `cmp`, `convert`, `fmt`, `mem`, `num`, `ops`, `os`, `str`, `string`, `sync`, `option`, `result`, `path`, `ascii_unsafe`, `char_unsafe`, `cmp_unsafe`, `convert_unsafe`, `fmt_unsafe`, `mem_unsafe`, `num_unsafe`, `ops_unsafe`, `os_unsafe`, `str_unsafe`, `string_unsafe`, `sync_unsafe`,  `option_unsafe`, `result_unsafe`, `path_unsafe`, `unsafe_linux`, `unsafe_ops`, `unsafe_sync`, `unsafe_option`, `unsafe_result`, `unsafe_path`, `linux`, `linux_unsafe`, `codegen`, `codegen_unsafe`, `unsafe_codegen`, `thread`, `unsafe_thread`, `thread_unsafe`.
- new multiple module features: `fullest`, `full_unsafe`, `lean`, `lean_unsafe`, `leanest`, `strings`, `chars`, `strings_unsafe`, `chars_unsafe`, `texts`, `texts_unsafe`.
- new functions `char_utf8_2bytes_len`, `char_utf8_3bytes_len`, `char_utf8_4bytes_len`.

### Removed
- remove deprecated macro `bdbg`.

### Changed
- make `StrExt` and `StringExt` `new_counter` method depend on `ascii`.
- update modules to depend on their homonymous features: `ascii`, `char`.
- do not require `unsafe` for `linux` structs, just for the unsafe methods.
- make dependencies optional: `az`, `bytemuck`, `atomic`, `portable-atomic`, `const-str`.
- replace the shell script tools with a more powerful rust script.
- move macros: `sf`, `sfb` to the `codegen` module.
- add itself as a dev-dependency.

### Fixed
- make private the `Range*::XOR_VALUE` constant.
- add missing must_use and inline attributes.
- add missing unsafe safeguarded features.
- re-export `option_unwrap` from `all`.
- simplify documentation on features.
- add global warning `missing_docs`.
- update some docs.

## [0.13.0] - 2023-09-29

### Added
- new enum `AsciiChar` ported from unstable `std`.
- new fns: `char_byte_len`, `char_is_7b`, `char_is_noncharacter`, `ascii_usize_digits`, `count_digits`, `count_digits_unchecked`.
- new `char*` methods: `from_ascii_char`, `to_ascii_char`.
- new module: `string`.
- new trait: `StringExt`.
- new enum: `ArrayStringError`.
- new features: `unsafe_ascii`, `unsafe_string`.
- new structs: `ArrayU8String`, `ArrayU16String`, `ArrayU32String`, `ArrayU8NonNulString`.
- new type alises: `String16`, `String32`, `String64`, `String128`, `NonNulString128`, `NonNulString16`, `NonNulString32`, `NonNulString64`, `NonNulString128`.
- new `StrExt` method: `new_counter`.

### Changed
- rename `os::terminal` and `os::linux::terminal` submodules  to `term`.

### Fixed
- add missing inlines.

## [0.12.0] - 2023-09-27

### Added
- new `const_for` macro.
- new fn `ascii_calc_digit`.
- new `LinuxTerminal` method `size`.
- new features: `linux`, `unsafe_char`.
- new `LinuxTermios` method `get_winsize`.
- new function `linux_sig_handler_no_return`.
- new `Ansi` methods `CURSOR_NEXT_LINE*`, `CURSOR_PREV_LINE*`.
- new `Ansi` methods: `CURSOR_PREV_LINE_N`, `CURSOR_NEXT_LINE_N`, `CURSOR_LEFT_N`, `CURSOR_RIGHT_N`, `CURSOR_UP_N`, `CURSOR_MOVE_N`.
- new `Ansi` const methods: `COLORS`, `BRIGHT_COLORS`, `COLORS_BRIGHT_FG`, `COLORS_BRIGHT_BG`, `COLORS256`, `COLOR256_FG`, `COLOR256_BG`, `RGB`, `RGB_FG`, `RGB_BG`, `GRAY`.
- new `Ansi` consts: `CSI`, `GRAY*`, `GRAY*_BG`, `ERASE_LINE`, `ERASE_LINE_END`, `ERASE_LINE_START`, `ERASE_SCREEN_START`, `ERASE_SCREEN_END`.
- new structs: `LinuxTerminalSize`, `AnsiColor8`, `Char7`, `Char8`, `Char16`, `Char24`, `Char32`, `CharConversionError`.
- new enums: `AnsiColor3`, `NeverOk`, `NeverErr`.

### Changed
- move `iif` macro to `codegen` module.
- update `os_*print*` macros to work in `std`.
- use const byte arrays instead of const slices for `Ansi` escape codes.
- rename ansi constant `CLEAR_SCREEN` to `ERASE_SCREEN`.
- rename fns `u*_to_ascii` to `ascii_u*_digits`.
- rename fns `ascii_d*` to `ascii_*digit`.
- rename `bdbg` to `cdbg`, again.

### Fixed
- use `u8_to_ascii` instead of `ascii_3digit` for `u8` types.
- add missing inlines for `Ansi` const fns.

## [0.11.0] - 2023-09-22

### Added
- new structs `LINUX_SYS_X86_64`, `LINUX_SYS_X86`, `LINUX_SYS_ARM`, `LINUX_SYS_AARCH64`, `LINUX_SYS_RISCV`, `LINUX_SIGACTION`, `LinuxSigaction`, `LINUX_TERMIOS_CFLAG`, `LINUX_TERMIOS_IFLAG`, `LINUX_TERMIOS_LFLAG`, `LINUX_TERMIOS_OFLAG`, `LinuxTerminal`.
- new functions: `linux_sys_getrand`, `linux_sys_getpid`, `linux_sys_rt_sigaction`, `linux_getpid`, `linux_handle_signals`, `linux_random_bytes`, `linux_random_u8`, `linux_random_u16`, `linux_random_u32`, `linux_random_u64`, `linux_random_i128`.
- new macros: `ansi`, `ansib`, `ansip`.
- new static: `LINUX_TERMINAL_STATE`.
- add `linux_sys_nanosleep` for `riscv`.
- add missing `LINUX_ERRNO` values.
- new type alias `LINUX_SYS`.
- new `check_miri.sh` script.
- new dependencies: `const-str`, `atomic`, `portable-atomic`.
- new macros: `os_print`, `os_println`, `os_eprint`, `os_eprintln`.
- add scripts to get linux syscalls and error numbers as rust constants.
- add CI actions to get `syscall.h` and `errno.h` values for each target.
- new `sync` module and `atomic` submodule.
- new `os::linux::terminal` module.
- new `LinuxTermios` methods: `enable_raw_mode`, `disable_raw_mode`, `is_terminal`, `get_state`, `set_state`.
- new `LinuxSigaction` method `new`.
- re-export all `core::sync::atomic` types; re-export `Ordering` as `AtomicOrdering`.
- re-export renamed `const-str` macros from `ascii`, `str` and `option` modules.
- re-export `atomic::Atomic` and all `portable-atomic` types.
- re-export `core::num::NonZero*` types from `num` module.

### Removed
- remove the already deprecated macros `cdbg`, `rfs`.
- remove the already deprecated feature `no-std`.
- remove fns: `linux_enable_raw_mode`, `linux_disable_raw_mode`, `linux_is_terminal`.
- remove struct `AnsiColor`, and move its constants to `Ansi`.
- remove the specific print methods from `Ansi`.

### Changed
- rename `SysTimeSpec` to `LinuxTimespec`, `SysTermios` to `LinuxTermios`.
- rename `ERRNO` to `LINUX_ERRNO`, `IOCTL` to `LINUX_IOCTL`.
- rename `Ansi` associated constants `*_ALTERNATIVE_SCREEN` to `*_ALT_SCREEN`.
- rename `Ansi` associated functions `*_alternative_screen` to `*_alt_screen`.
- modify `LinuxSigaction` fields: remove `unsafe` and wrap `sa_restorer` with `Option`.
- rename `_features` module to `_doc`.
- move check scripts to `/tools/`.

### Fixed
- fix `ioctl` and `nanosleep` syscalls for `aarch64`.
- fix `nanosleep` syscall for `arm`.
- fix `ioctl` syscall for `riscv`.
- refactor modules: `ascii`, `codegen`, `option`, `result`, `str`.
- ensure dependencies doesn't include default features.
- update .gitattributes to show all languages.
- move lengthy `target_arch` rustdoc tags to the doc-comments.
- update documentation on re-exported types.

## [0.10.0] - 2023-09-13

### Added
- new features: `unsafe_mem`, `unsafe_str`.
- new modules : `ascii`, `os::terminal`, `str`, `_features`.
- new fns in `ascii:`: `ascii_d[1-4]`, `u[BITS]_to_ascii`.
- new fns in `mem:` `as_bytes`, `as_bytes_mut`, `as_bytes_sized`.
- new fns in `os::linux`: `sys_ioctl`.
- new fns in `os::terminal`: `is_terminal`, `enable_raw_terminal`, `disable_raw_terminal`, `eprint`, `eprintln` `get_char`, `get_dirty_char`, `get_line`, `get_utf8_bytes`, `get_str`, `pause_until_char`, `println`, `prompt`.
- new structs in `os::linux`: `ERRNO`, `FILENO`, `IOCTL`, `SysTermios`.
- new struct in `os::terminal`: `Ansi`, `AnsiColor`.
- new methods for `SysTimeSpecs`: `as_ptr` and `as_mut_ptr`.
- new trait `StrExt`.
- new macro: `sfb`.

### Removed
- remove fns: `linux_enable_raw_mode`, `linux_disable_raw_mode`, `linux_is_terminal`.

### Changed
- rename and deprecate `rfs` macro for `sf`.
- change `SysTimeSpec` to accept arguments in `new` fn, and derive `Default`.
- move `os::linux` file descriptor constnts to `FILENO` struct.
- make `bytemuck` non-optional.

### Fixed
- use `isize` instead of `i32` for `os::linux` `errno` constants.
- use `c_int` and `c_ulong` instead of `i32` and `u64` for `fd` constants and syscall args.
- remove cfg check for `os::linux` to be `no_std` compatible.
- move documentation on features to `_features`.
- move derived works info to a separate file.
- avoid loading `sys_nanosleep` in risc-v.
- fix `az` and `bytemuck` re-exports.
- fix arm assembly.

## [0.9.0] - 2023-09-08

### Added
- new features: `full`, `unsafe_os`.
- add `full` to `nightly_docs` feature.
- new `os` module, with `linux` submodule.
- new functions: `sys_exit`, `sys_read`, `sys_write`, `sys_nanosleep`, `get_byte`, `print`, `print_bytes`.
- new struct `SysTimeSpec`.
- add `bytemuck`'s feature `extern_crate_std` to `std`.

### Removed
- remove the `Copy` trait from `IntBuf`.
- remove the `boxed` module and `bx` fn.
- remove the `string` module and `S` macro.
- remove `unsafe_num` from the `bytemuck` feature.
- remove `bytemuck` from the `unsafe` feature.

### Changed
- deprecate and rename `cdbg` macro to `bdbg`.

### Fixed
- fix the `iif` module.
- improve the `num` docs.
- improve docs related to features.
- refactor manifest and update comments.
- separate lengthy module level docs into markdown files.

## [0.8.0] - 2023-08-29

### Added
- add features: `safest` and `unsafe`.
- new `deprecate_feature` macro.
- new `sleep4` macro.

### Removed
- remove `num::*` module re-export from the prelude.
- remove fns `counter_string`, `indent`, macro `iformat` (move to `textos`).

### Changed
- deprecate feature `no-std` for `no_std`.
- bump MSRV to `1.72.0`.

### Fixed
- fix cargo aliases.
- fix external re-exports.

## [0.7.1] - 2023-08-23

### Changed
- unhide the macros from the root.
- re-export `paste` with wrapped documentation.

## [0.7.0] - 2023-08-23

### Added
- new number types: `NonRange*`, `Range*`.
- new `SliceExtMut` trait.
- new `OptionExt` methods: `fmt_or`, `fmt_or_else`, `fmt_or_empty`.
- new structs: `OptionFmtOr`, `OptionFmtOrElse`, `OptionFmtOrEmpty`.
- new `codegen` module; re-export the `compile` and `paste` macros from there.
- add categories: `development-tools`, `no-std::no-alloc`.

### Removed
- avoid re-exporting external macros from the root and hide the root documentation for the rest.

### Changed
- implement `InBufAble` for `NonRange*` and `Range*`.
- rename feature `unsafe_non_specific` to `unsafe_num`
- move mutable methods from `SliceExt` to the new `SliceExtMut` trait.
- implement the slice extension traits for slice references, arrays and vecs.
- re-export used bytemuck types in `all`.
- include `bytemuck` in `unsafe`.
- bump MSRV to `1.71.1`.
- rename `unsafe_*` features: `unsafe_int_buf` to `unsafe_fmt`, `unsafe_cmp_float` to `unsafe_cmp`, `unsafe_uninit_array` to `unsafe_convert`.
- remove `std` requirement for `cdbg` macro.
- bump `devela_macros` to `0.5.0`.
  - re-export new macros: `compile_attr` and `cif`.

### Fixed
- improve `Debug` impl for `NonSpecific*` and `NonRange*`.
- fix unsafe features safeguarding.
- improve `num` tests.

## [0.6.1] - 2024-08-08

### Changed
- implement `IntBufable` for `NonZero*` and `NonSpecific*`.
- make traits sealed: `OptionExt`, `ResultExt`, `SliceExt`.
- update the `prelude` module to re-export `core::num::NonZero*` and `devela::num::*`.
- update the `all` module to inline every item except foreign ones.
- update information about licensing and derived works.

## [0.6.0] - 2023-08-06

### Added
- new features: `nightly_docs`, `unsafe_cmp_float`, `unsafe_int_buf`,
  `unsafe_uninit_array`, `unsafe_non_specific`.
- add `bytemuck` as an optional dependency.
- new const functions for comparing primitives: `total_cmp_*`, `max_*`, `min_*`,
  `clamp_*`.
  - includes const unsafe and non-const safe versions of functions for comparing
    floating-point primitives.
- add `IntBuf` struct.
- new traits: `FromPrimitives`, `IntoPrimitives`, `SliceExt`,  `IntBufAble`.
- add additional targets for testing `x86_64-unknown-linux-gnu`,
  `x86_64-pc-windows-msvc`, `x86_64-apple-darwin`, `x86_64-unknown-none`,
  `i686-unknown-linux-gnu`, `aarch64-unknown-none`.

### Removed
- delete `safest` feature.
- remove `safe` from the default features.

### Changed

- bump MSRV to `v1.71.0`.
- rename modules and reorganize items in a similar fashion to the rust standard library.
- constify and rename `subslice_left` to `slice_lsplit`, `subslice_right` to
  `slice_rsplit` and `subslice_middle` to `slice_msplit_right`.
- new `slice_msplit_left` function.
- improve documentation, specially regarding features and safety.

## [0.5.3] - 2023-07-24

### Changed
- implement `Default` for `NonMax*`.

## [0.5.2] - 2023-07-24

### Added
- new `safest` feature.

### Changed
- improve `NonSpecific` types:
  - implement `From` and `TryFrom` traits.
  - add `NonMax*` and `NonMin*` aliases.

## [0.5.1] - 2023-06-22

### Added
- new `NonSpecific*` wrappers over the `NonZero*` primitives.
- new `unsafe` feature.

## [0.5.0] - 2023-06-09

### Added
- new traits `AltDebug`, `OptionExt`, `ResultExt`.
- new macros `S`, `iformat`.
- new `indent` function.

### Changed
- improve `iif` macro adding suport to `if let` expressions and empty else clauses.

## [0.4.1] - 2023-05-30

### Fixed
- fix the `cdbg` macro.

## [0.4.0] - 2023-05-23

### Added
- new functions: `slice_into_vec`, `try_slice_into_vec`, `try_vec_into_vec`,
  `vec_into_vec`, `slice_into_array`.
- new macros: `manifest_dir`, `cdbg`.

### Changed
- update MSRV to `1.63.0`.

## [0.3.0] - 2023-05-09

### Added
- re-export the `az` crate and the `paste` macro.

### Changed
- improve the `compile` attribute macro to support the `not()` option.

## [0.2.0] - 2023-05-07

### Added
- new dependecy `devela_macros`.
- new `compile` attribute macro.

## [0.1.10] - 2023-03-29

### Fixes
- fix `alloc` compilation.

## [0.1.9] - 2023-03-29

### Added
- add `format_buf` function and macro.

## [0.1.8] - 2023-03-17

### Added
- add `alloc` and `no-std` features.

### Changed
- bump MSRV to `1.60.0`.

## [0.1.7] - 2023-03-11

### Added
- add `Also` & `Apply` traits.

## [0.1.6] - 2023-03-09

### Added
- new functions `subslice_left`, `subslice_mid`, `subslice_right`.

## [0.1.5] - 2023-03-03

### Added
- new `rfs` macro that allows skipping rust formatting.

## [0.1.4] - 2023-02-18

### Added
- add `nighly` feature

### Changed
- update the `iif` macro to support the absence of a false branch.

## [0.1.3] - 2023-02-17

### Fixed
- fix `no_std` mode.

## [0.1.2] - 2023-01-10

### Added
- add `safe` feature.

### Fixes
- minor fixes and updates.

## [0.1.1] - 2023-01-10

### Added
- add `bx` function.
- add cargo categories.

### Changed
- enable cargo publish.
- update docs.

## [0.1.0] - 2022-12-17

### Added
- add functions `pclamp`, `pmax`, `pmin`, `project_root_path`,
  `project_root_path_string`.
- add macro `iif`.


[unreleased]: https://github.com/andamira/devela/compare/v0.21.2...HEAD
[0.21.2]: https://github.com/andamira/devela/releases/tag/v0.21.2
[0.21.1]: https://github.com/andamira/devela/releases/tag/v0.21.1
[0.21.0]: https://github.com/andamira/devela/releases/tag/v0.21.0
[0.20.0]: https://github.com/andamira/devela/releases/tag/v0.20.0
[0.19.0]: https://github.com/andamira/devela/releases/tag/v0.19.0
[0.18.1]: https://github.com/andamira/devela/releases/tag/v0.18.1
[0.18.0]: https://github.com/andamira/devela/releases/tag/v0.18.0
[0.17.0]: https://github.com/andamira/devela/releases/tag/v0.17.0
[0.16.0]: https://github.com/andamira/devela/releases/tag/v0.16.0
[0.15.0]: https://github.com/andamira/devela/releases/tag/v0.15.0
[0.14.0]: https://github.com/andamira/devela/releases/tag/v0.14.0
[0.13.0]: https://github.com/andamira/devela/releases/tag/v0.13.0
[0.12.0]: https://github.com/andamira/devela/releases/tag/v0.12.0
[0.11.0]: https://github.com/andamira/devela/releases/tag/v0.11.0
[0.10.0]: https://github.com/andamira/devela/releases/tag/v0.10.0
[0.9.0]: https://github.com/andamira/devela/releases/tag/v0.9.0
[0.8.0]: https://github.com/andamira/devela/releases/tag/v0.8.0
[0.7.1]: https://github.com/andamira/devela/releases/tag/v0.7.1
[0.7.0]: https://github.com/andamira/devela/releases/tag/v0.7.0
[0.6.1]: https://github.com/andamira/devela/releases/tag/v0.6.1
[0.6.0]: https://github.com/andamira/devela/releases/tag/v0.6.0
[0.5.3]: https://github.com/andamira/devela/releases/tag/v0.5.3
[0.5.2]: https://github.com/andamira/devela/releases/tag/v0.5.2
[0.5.1]: https://github.com/andamira/devela/releases/tag/v0.5.1
[0.5.0]: https://github.com/andamira/devela/releases/tag/v0.5.0
[0.4.1]: https://github.com/andamira/devela/releases/tag/v0.4.1
[0.4.0]: https://github.com/andamira/devela/releases/tag/v0.4.0
[0.3.0]: https://github.com/andamira/devela/releases/tag/v0.3.0
[0.2.0]: https://github.com/andamira/devela/releases/tag/v0.2.0
[0.1.10]: https://github.com/andamira/devela/releases/tag/v0.1.10
[0.1.9]: https://github.com/andamira/devela/releases/tag/v0.1.9
[0.1.8]: https://github.com/andamira/devela/releases/tag/v0.1.8
[0.1.7]: https://github.com/andamira/devela/releases/tag/v0.1.7
[0.1.6]: https://github.com/andamira/devela/releases/tag/v0.1.6
[0.1.5]: https://github.com/andamira/devela/releases/tag/v0.1.5
[0.1.4]: https://github.com/andamira/devela/releases/tag/v0.1.4
[0.1.3]: https://github.com/andamira/devela/releases/tag/v0.1.3
[0.1.2]: https://github.com/andamira/devela/releases/tag/v0.1.2
[0.1.1]: https://github.com/andamira/devela/releases/tag/v0.1.1
[0.1.0]: https://github.com/andamira/devela/releases/tag/v0.1.0

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
