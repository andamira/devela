

Legend:
- ` ` = not enabled / can't be enabled
- `i` = not enabled (incomplete_features).
- `·` = enabled by non-specific cfg flag (`nightly_stable_[1_XX|later]`)
- `a` = enabled by non-specific cfg flag (and depends on `alloc`)
- `s` = enabled by non-specific cfg flag (and depends on `std`)
- `F` = enabled by some specific cfg flag, shown afterwards.
- `D` = disabled, feature previously enabled as `F`
- `✓` = enabled already

Usage example:
```sh
RUSTFLAGS="--cfg nightly_coro --cfg nightly_stable_later" cargo +nightly b
```

See also
- <https://github.com/rust-lang/rust/blob/main/RELEASES.md>
- <https://doc.rust-lang.org/nightly/unstable-book/the-unstable-book.html>
- <https://github.com/rust-lang/rust/blob/master/library/core/src/lib.rs>
- <https://github.com/rust-lang/rust/blob/master/library/alloc/src/lib.rs>
- <https://github.com/rust-lang/rust/blob/master/library/std/src/lib.rs>

# 1.95 will be stable on [2026-04-16](https://releases.rs/docs/1.95.0/) `nightly_stable_1_95`
> - <https://github.com/rust-lang/rust/milestone/140>
<!-- > - <https://blog.rust-lang.org/2026/04/16/Rust-1.95.0> -->
<!-- > - <https://github.com/rust-lang/rust/releases/tag/1.95.0> -->

- 1.95: `·`[assert_matches](https://github.com/rust-lang/rust/pull/137487)
- 1.95: `·`[atomic_try_update](https://github.com/rust-lang/rust/issues/135894)
- 1.95: `·`[cfg_select](https://github.com/rust-lang/rust/pull/149783)
- 1.95: `·`[cold_path](https://github.com/rust-lang/rust/pull/151576)
- 1.95: `·`[if_let_guard](https://github.com/rust-lang/rust/pull/141295)
- 1.95: ` `[importing path-segment keyword with renaming](https://github.com/rust-lang/rust/pull/146972)
- 1.95: ` `[improve pattern matching semantics](https://github.com/rust-lang/rust/pull/150681)
- 1.96: `·`[min_const_control_flow](https://github.com/rust-lang/rust/pull/152253)
- 1.95: `·`[more_conversion_trait_implsa](https://github.com/rust-lang/rust/pull/145504)
- 1.95: ` `[rustdoc: hide deprecated items](https://github.com/rust-lang/rust/pull/151091)
- 1.95: `a`[push_mut](https://github.com/rust-lang/rust/pull/151785)
- 1.95: `·`[new_range_inclusive_api](https://github.com/rust-lang/rust/pull/150522) (new_range_api)
- 1.95: `·`[ptr_as_ref_unchecked](https://github.com/rust-lang/rust/pull/151995)
- 1.95: ` `[rustdoc: sort stable items first](https://github.com/rust-lang/rust/pull/149460)
- 1.95: ` `[`TryFrom<integer>` for bool](https://github.com/rust-lang/rust/pull/147400)

# 1.96 will be stable on [2026-05-28](https://releases.rs/docs/1.96.0/) `nightly_stable_1_96`
> - <https://github.com/rust-lang/rust/milestone/142>
<!-- > - <https://blog.rust-lang.org/2026/05/28/Rust-1.96.0> -->
<!-- > - <https://github.com/rust-lang/rust/releases/tag/1.96.0> -->

- 1.96: `·`[control_flow_ok](https://github.com/rust-lang/rust/pull/152911)
- 1.96: `·`[Lint unused_features](https://github.com/rust-lang/rust/pull/152164)
- 1.96: `·`[new_range_api](https://github.com/rust-lang/rust/pull/154620) (new_range_api)
- 1.96: `·`[new_range_from_api](https://github.com/rust-lang/rust/pull/153380) (new_range_api)
- 1.96: `·`[new_range_to_inclusive_api](https://github.com/rust-lang/rust/pull/152304) (new_range_api)

# 1.97 will be stable on [2026-07-09](https://releases.rs/docs/1.97.0/) `nightly_stable_1_97`
<!-- > - <https://github.com/rust-lang/rust/milestone/???> -->
<!-- > - <https://blog.rust-lang.org/2026/07/09/Rust-1.97.0> -->
<!-- > - <https://github.com/rust-lang/rust/releases/tag/1.97.0> -->

# … will be stable later ([`nightly_stable_later`](https://releases.rs/#ongoing-stabilization-prs))

- 1.??: `·`[breakpoint](https://github.com/rust-lang/rust/pull/142325)
- 1.??: `a`[btree_extract_if](https://github.com/rust-lang/rust/pull/145471)
- 1.??: ` `[c-style varargs](https://github.com/rust-lang/rust/pull/144066)
- 1.??: `·`[cfg_target_has_atomic_equal_alignment](https://github.com/rust-lang/rust/pull/155006)
- 1.??: `·`[cfg_version](https://github.com/rust-lang/rust/pull/141766)
- 1.??: `·`[const_array_from_ref](https://github.com/rust-lang/rust/issues/90206)
        `·` `const_slice_from_ref`
- 1.??: `·`[const_char_classify](https://github.com/rust-lang/rust/pull/138129)
        `·` `const_sockaddr_setters`
- 1.??: `·`[debug_closure_helpers](https://github.com/rust-lang/rust/issues/117729)
- 1.??: `·`[derive_coerce_pointee](https://github.com/rust-lang/rust/pull/133820)
- 1.??: `F`[doc_cfg](https://github.com/rust-lang/rust/pull/150055)
        = `nightly_doc` flag
- 1.??: ` `[flags for doctest cross compilation](https://github.com/rust-lang/rust/pull/137096)
- 1.??: `·`[float_bits_const](https://github.com/rust-lang/rust/pull/154065)
- 1.??: `·`[fn_align](https://github.com/rust-lang/rust/pull/140261)
- 1.??: `·`[frontmatter](https://github.com/rust-lang/rust/pull/148051)
- 1.??: `·`[impl_trait_in_assoc_type](https://github.com/rust-lang/rust/pull/120700)
- 1.??: `·`[int_format_into](https://github.com/rust-lang/rust/pull/152544)
- 1.??: `·`[integer_sign_cast](https://github.com/rust-lang/rust/pull/137026)
- 1.??: `·`[isolate_most_least_significant_one](https://github.com/rust-lang/rust/pull/155130)
- 1.??: `·`[isqrt](https://github.com/rust-lang/rust/pull/131391)
- 1.??: `·`[likely_unlikely](https://github.com/rust-lang/rust/issues/151619)
- 1.??: `·`[macro_metavar_expr](https://github.com/rust-lang/rust/pull/122808) closed
  - [decide about macro_metavar_expr](https://github.com/rust-lang/rust/issues/137581)
  - [RFC Named macro capture groups](https://github.com/rust-lang/rfcs/pull/3649) <
- 1.??: `·`[macro_metavar_expr_concat](https://github.com/rust-lang/rust/issues/124225)
- 1.??: ` `[Mergeable rustdoc cross-crate info](https://github.com/rust-lang/rust/pull/152902)
- 1.??: `·`[more_qualified_paths](https://github.com/rust-lang/rust/pull/141922)
- 1.??: `a`[new_zeroed_alloc](https://github.com/rust-lang/rust/issues/129396)
- 1.??: `·`[new_range_api](https://github.com/rust-lang/rust/issues/125687)
- 1.??: `·`[new_range_api](https://github.com/rust-lang/rust/pull/152304)
- 1.??: `·`[offset_of_enum](https://github.com/rust-lang/rust/issues/143954)
- 1.??: `·`[offset_of_slice](https://github.com/rust-lang/rust/pull/139673)
- 1.??: `s`[once_wait](https://github.com/rust-lang/rust/pull/136360)
- 1.??: `·`[proc_macro_value](https://github.com/rust-lang/rust/pull/151973) in `devela_base_macros`
- 1.??: ` `[Return Type Notation](https://github.com/rust-lang/rust/pull/138424)
- 1.??: `·`[refcell_try_map](https://github.com/rust-lang/rust/pull/152092)
- 1.??: ` `[stabilize inner tool attributes](https://github.com/rust-lang/rust/pull/154770)
- 1.??: `·`[str_as_str](https://github.com/rust-lang/rust/pull/152971)
- 1.??: `·`[substr_range](https://github.com/rust-lang/rust/pull/141266)
- 1.??: `·`[supertrait_item_shadowing](https://github.com/rust-lang/rust/pull/148605)
- 1.??: ` `[tcp_deferaccept](https://github.com/rust-lang/rust/pull/154834)
- 1.??: `·`[uint_bit_width](https://github.com/rust-lang/rust/pull/155131)
- 1.??: `·`[unsafe_cell_from_mut](https://github.com/rust-lang/rust/pull/131261)
- 1.??: `a`[vec_deque_truncate_front](https://github.com/rust-lang/rust/pull/151379)
- 1.??: ` `[-Zinstrument-mcount](https://github.com/rust-lang/rust/pull/152122)

# … may be stable even later

- 1.??: `F`[allocator_api](https://github.com/rust-lang/rust/issues/32838)
        = `nightly_allocator` flag
- 1.??: `F`[coroutines](https://github.com/rust-lang/rust/issues/43122)
        = `nightly_coro` flag
- 1.??: `F`[doc_notable_trait](https://github.com/rust-lang/rust/issues/45040)
        = `nightly_doc` flag
- 1.??: `F`[f16|f128](https://github.com/rust-lang/rust/issues/116909)
        = `nightly_float` flag
- 1.??: `F`[portable_simd](https://github.com/rust-lang/rust/issues/86656)
        = `nightly_simd` flag

- 1.??: `D`[autodiff](https://github.com/rust-lang/rust/issues/124509)
          `nightly_autodiff` flag

- 1.??: ` `[cargo-script](https://github.com/rust-lang/cargo/pull/16569)

- 1.??: ` `[signed_bigint_helpers](https://github.com/rust-lang/rust/issues/151989)
- 1.??: ` `[const bigint_helper_methods](https://github.com/rust-lang/rust/issues/152015)
- 1.??: ` `[widening bigint_helper_methods](https://github.com/rust-lang/rust/issues/152016)

- 1.??: ` `[box_into_inner](https://github.com/rust-lang/rust/issues/80437)
- 1.??: ` `[cfg(accessible(::path::to::thing))](https://github.com/rust-lang/rust/issues/64797)
- 1.??: ` `[cfg(version(..))](https://github.com/rust-lang/rust/issues/64796)
- 1.??: ` `[concat_bytes](https://github.com/rust-lang/rust/issues/87555)
- 1.??: ` `[concat_idents](https://github.com/rust-lang/rust/issues/29599)
- 1.??: ` `[const_async_blocks](https://github.com/rust-lang/rust/issues/85368)
- 1.??: ` `[const_closures](https://github.com/rust-lang/rust/issues/106003)
- 1.??: ` `[const_convert](https://github.com/rust-lang/rust/issues/143773)
- 1.??: ` `[const_for](https://github.com/rust-lang/rust/issues/87575)
          (depends on const_trait_impl)
- 1.??: ` `[const_str_from_utf8](https://github.com/rust-lang/rust/issues/91006)
- 1.??: ` `[const_trait_impl](https://github.com/rust-lang/rust/issues/143874)
- 1.??: ` `[core_float_math](https://github.com/rust-lang/rust/issues/137578)
- 1.??: ` `[custom inner attributes](https://github.com/rust-lang/rust/issues/54726)
- 1.??: ` `[float_algebraic](https://github.com/rust-lang/rust/issues/136469)
- 1.??: ` `[generic_atomic](https://github.com/rust-lang/rust/issues/130539)
- 1.??: ` `[int_format_into](https://github.com/rust-lang/rust/issues/138215)
- 1.??: ` `[integer_atomics](https://github.com/rust-lang/rust/issues/99069)
- 1.??: ` `[mpmc_channel](https://github.com/rust-lang/rust/issues/126840)
- 1.??: ` `[passing unstable flags only on nightly](https://github.com/rust-lang/cargo/issues/14733)
- 1.??: ` `[thread_local](https://github.com/rust-lang/rust/issues/29594)

# Experimental features:
- [local_waker](https://github.com/rust-lang/rust/issues/118959)
- [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
 ` `- <https://rust-lang.github.io/rfcs/2580-ptr-meta.html>
    - [related issues](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3A%22F-ptr_metadata%22)
- [stmt_expr_attributes](https://github.com/rust-lang/rust/issues/15701)
- [type_alias_impl_trait|impl_trait_in_assoc_type](https://github.com/rust-lang/rust/issues/63063)

# Projects
- on track:
  - [Associated Const Equality (ACE)](https://github.com/orgs/rust-lang/projects/60/views/1)
  - [Generic Const Items (GCI)](https://github.com/orgs/rust-lang/projects/62/)
  - [Inherent Associated Types (IAT)](https://github.com/orgs/rust-lang/projects/64)
  - [Lazy Type Aliases (LTA)](https://github.com/orgs/rust-lang/projects/59/)
- other
  - [type alias impl trait (TAIT) stabilization](https://github.com/orgs/rust-lang/projects/22/)
  - [safer transmute](https://github.com/rust-lang/rfcs/pull/2981)
    - <https://github.com/jswrenn/project-safe-transmute/blob/rfc/rfcs/0000-safe-transmute.md#safe-transmute-rfc>
    - <https://jswrenn.github.io/safer-transmute-rustdoc/rfc/core/convert/transmute/index.html>
    - <https://jswrenn.github.io/transmutation-foundation/>

# Pitfalls
- [const_evaluatable_unchecked](https://github.com/rust-lang/rust/issues/76200)

# Meta tracking issues:
- [arbitrary_self_types](https://github.com/rust-lang/rust/issues/44874)
  - [stabilize arbitrary self types v2](https://github.com/rust-lang/rust/pull/135881)
- [branch hints](https://github.com/rust-lang/rust/issues/26179)
- [cargo config search](https://github.com/rust-lang/cargo/issues/9769)
- [const fn](https://github.com/rust-lang/rust/issues/57563)
- [feature-metadata](https://github.com/rust-lang/cargo/issues/14157)
- [impl Trait](https://github.com/rust-lang/rust/issues/63066)
- [int_roundings](https://github.com/rust-lang/rust/issues/88581)
- [MaybeUninit methods for arrays](https://github.com/rust-lang/rust/issues/96097)
- [Reproducible Build bugs](https://github.com/rust-lang/rust/issues/129080)
- [User control over cargo warnings](https://github.com/rust-lang/cargo/issues/12235)
- [doc_cfg](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3A%22F-doc_cfg%22)

# Rust project Goals:
- <https://rust-lang.github.io/rust-project-goals/>
  - <https://rust-lang.github.io/rust-project-goals/2025h2/>
- [const traits](https://github.com/rust-lang/rust-project-goals/issues/106)
- [expanded const generics](https://github.com/rust-lang/rust-project-goals/issues/100)
- [Linux building on stable](https://github.com/rust-lang/rust-project-goals/issues/116)
- [next generation trait solver](https://github.com/rust-lang/rust-project-goals/issues/113)
- [optimize clippy & linting](https://github.com/rust-lang/rust-project-goals/issues/114)
- [reborrow traits](https://github.com/rust-lang/rust-project-goals/issues/106)
- [reflection and comptime](https://github.com/rust-lang/rust-project-goals/issues/406)
- [stabilize cargo-script](https://github.com/rust-lang/rust-project-goals/issues/119)
- [stabilize doc_cfg](https://github.com/rust-lang/rust-project-goals/issues/120)
