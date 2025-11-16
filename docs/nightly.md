

Legend:
- ` ` = not enabled / can't be enabled
- `i` = not enabled (incomplete_features).
- `·` = enabled by non-specific cfg flag (`nightly_stable_[1_91|1_92|later]`)
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
- <https://doc.rust-lang.org/nightly/unstable-book/the-unstable-book.html>
- <https://github.com/rust-lang/rust/blob/master/library/core/src/lib.rs>
- <https://github.com/rust-lang/rust/blob/master/library/alloc/src/lib.rs>
- <https://github.com/rust-lang/rust/blob/master/library/std/src/lib.rs>

# 1.91 will be stable on 2025-10-30 ([`nightly_stable_1_91`](https://releases.rs/docs/1.91.0/))
> - <https://github.com/rust-lang/rust/milestone/135>
<!-- > - <https://blog.rust-lang.org/2025/10/30/Rust-1.91.0> -->
<!-- > - <https://github.com/rust-lang/rust/releases/tag/1.91.0> -->

- 1.91: `·`[array_repeat](https://github.com/rust-lang/rust/issues/126695)
- 1.91: `·`[as_array_of_cells](https://github.com/rust-lang/rust/pull/144054)
- 1.91: `·`[const_array_each_ref](https://github.com/rust-lang/rust/pull/143383)
- 1.91: `s`[const_pathbuf_osstring_new](https://github.com/rust-lang/rust/pull/145464)
- 1.91: `·`[const_type_id](https://github.com/rust-lang/rust/pull/144133)
- 1.91: `·`[duration_constructors_lite](https://github.com/rust-lang/rust/pull/145135)
- 1.91: `·`[ip_from](https://github.com/rust-lang/rust/pull/141744)
- 1.91: `·`[iter_chain](https://github.com/rust-lang/rust/pull/144963)
- 1.91: ` `[loongarch32 inline asm](https://github.com/rust-lang/rust/pull/144402)
- 1.91: `s`[panic_payload_as_str](https://github.com/rust-lang/rust/issues/125175)
- 1.91: `s`[path_add_extension](https://github.com/rust-lang/rust/pull/145209)
- 1.91: `s`[path_file_prefix](https://github.com/rust-lang/rust/pull/144870) <!-- & 129114 -->
- 1.91: `·`[round_char_boundary](https://github.com/rust-lang/rust/issues/93743)
- 1.91: `·`[strict_overflow_ops](https://github.com/rust-lang/rust/pull/144682)
- 1.91: `·`[strict_provenance_atomic_ptr](https://github.com/rust-lang/rust/issues/99108)
- 1.91: `·`[unsigned_signed_diff](https://github.com/rust-lang/rust/pull/144900)
- 1.91: ` `[unsigned_bigint_helpers](https://github.com/rust-lang/rust/pull/144494)

# 1.92 will be stable on 2025-12-11 ([`nightly_stable_1_92`](https://releases.rs/docs/1.92.0/))
<!-- > - <https://github.com/rust-lang/rust/milestone/136> -->
<!-- > - <https://blog.rust-lang.org/2025/12/11/Rust-1.92.0> -->
<!-- > - <https://github.com/rust-lang/rust/releases/tag/1.92.0> -->

- 1.92: `a`[btree_entry_insert](https://github.com/rust-lang/rust/pull/144871)
- 1.92: `·`[const_slice_rotate](https://github.com/rust-lang/rust/pull/146841)
- 1.92: `a`[file_with_nul](https://github.com/rust-lang/rust/pull/145664)
- 1.92: `s`[rwlock_downgrade](https://github.com/rust-lang/rust/pull/143191)

# 1.93 will be stable on 2026-01-22 ([`nightly_stable_1_93`](https://releases.rs/docs/1.93.0/))
<!-- > - <https://github.com/rust-lang/rust/milestone/136> -->
<!-- > - <https://blog.rust-lang.org/2026/01/22/Rust-1.93.0> -->
<!-- > - <https://github.com/rust-lang/rust/releases/tag/1.93.0> -->

- 1.93: `·`[core_slice_as_array](https://github.com/rust-lang/rust/issues/133508) // as_array*
- 1.93: ` `[extern_system_varargs](https://github.com/rust-lang/rust/pull/145954)
- 1.93: `·`[duration_from_nanos_u128](https://github.com/rust-lang/rust/pull/148587)
- 1.93: `·`[fmt_from_fn](https://github.com/rust-lang/rust/pull/145915)
- 1.93: ` `[-Zjump-tables=bool](https://github.com/rust-lang/rust/issues/116592)
- 1.93: `a`[vec_into_raw_parts](https://github.com/rust-lang/rust/pull/148827)
- 1.93: `a`[vec_deque_pop_if](https://github.com/rust-lang/rust/issues/135889)
- 1.93: `s`[stdarch_s390x_feature_detection](https://github.com/rust-lang/rust/pull/145656)

# … will be stable later ([`nightly_stable_later`](https://releases.rs/#ongoing-stabilization-prs))

- 1.??: `·`[alloc_layout_extra](https://github.com/rust-lang/rust/pull/148769)
- 1.??: `·`[array_windows](https://github.com/rust-lang/rust/pull/148814)
- 1.??: `·`[assert_matches](https://github.com/rust-lang/rust/pull/137487)
- 1.??: `·`[atomic_try_update](https://github.com/rust-lang/rust/issues/135894)
- 1.??: `·`[breakpoint](https://github.com/rust-lang/rust/pull/142325)
- 1.??: `a`[btree_extract_if](https://github.com/rust-lang/rust/pull/145471)
- 1.??: `·`[cfg_select](https://github.com/rust-lang/rust/issues/115585)
- 1.??: `·`[cfg_version](https://github.com/rust-lang/rust/pull/141766)
- 1.??: `·`[char_max_len](https://github.com/rust-lang/rust/pull/145610)
- 1.??: `·`[const_array_from_ref](https://github.com/rust-lang/rust/issues/90206)
        `·` `const_slice_from_ref`
- 1.??: `·`[const_char_classify](https://github.com/rust-lang/rust/pull/138129)
        `·` `const_sockaddr_setters`
- 1.??: `·`[const_mul_add](https://github.com/rust-lang/rust/pull/148052)
- 1.??: ` `[c-style varargs](https://github.com/rust-lang/rust/pull/144066)
- 1.??: `·`[debug_closure_helpers](https://github.com/rust-lang/rust/issues/117729)
- 1.??: `·`[derive_coerce_pointee](https://github.com/rust-lang/rust/pull/133820)
- 1.??: ` `[flags for doctest cross compilation](https://github.com/rust-lang/rust/pull/137096)
- 1.??: ` `[fn_align](https://github.com/rust-lang/rust/pull/140261)
- 1.??: `·`[frontmatter](https://github.com/rust-lang/rust/pull/148051)
- 1.??: `·`[if_let_guard](https://github.com/rust-lang/rust/pull/141295)
- 1.??: `·`[impl_trait_in_assoc_type](https://github.com/rust-lang/rust/pull/120700)
- 1.??: `·`[integer_sign_cast](https://github.com/rust-lang/rust/pull/137026)
- 1.??: `·`[isqrt](https://github.com/rust-lang/rust/pull/131391)
- 1.??: `·`[macro_metavar_expr](https://github.com/rust-lang/rust/pull/122808) closed
  - [decide about macro_metavar_expr](https://github.com/rust-lang/rust/issues/137581)
  - [RFC Named macro capture groups](https://github.com/rust-lang/rfcs/pull/3649)
- 1.??: `·`[macro_metavar_expr_concat](https://github.com/rust-lang/rust/issues/124225)
- 1.??: `·`[maybe_uninit_write_slice](https://github.com/rust-lang/rust/pull/148048)
- 1.??: `·`[more_qualified_paths](https://github.com/rust-lang/rust/pull/141922)
- 1.??: `a`[new_zeroed_alloc](https://github.com/rust-lang/rust/issues/129396)
- 1.??: `·`[offset_of_enum](https://github.com/rust-lang/rust/issues/143954)
- 1.??: `·`[offset_of_slice](https://github.com/rust-lang/rust/pull/139673)
- 1.??: `s`[once_wait](https://github.com/rust-lang/rust/pull/136360)
- 1.??: `·`[peekable_next_if_map](https://github.com/rust-lang/rust/pull/148941)
- 1.??: `·`[substr_range](https://github.com/rust-lang/rust/pull/141266)
- 1.??: `·`[supertrait_item_shadowing](https://github.com/rust-lang/rust/pull/148605)
- 1.??: `·`[unsafe_cell_from_mut](https://github.com/rust-lang/rust/pull/131261)
- 1.??: ` `[Return Type Notation](https://github.com/rust-lang/rust/pull/138424)

# … may be stable even later

- 1.??: `F`[allocator_api](https://github.com/rust-lang/rust/issues/32838)
        = `nightly_allocator` flag
- 1.??: `D`[autodiff](https://github.com/rust-lang/rust/issues/124509)
          `nightly_autodiff` flag
- 1.??: `F`[bigint_helper_methods](https://github.com/rust-lang/rust/issues/85532)
        = `nightly_bigint` flag
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
- 1.??: `F`[coroutines](https://github.com/rust-lang/rust/issues/43122)
        = `nightly_coro` flag
- 1.??: `F`[doc_cfg](https://github.com/rust-lang/rust/issues/43781)
        = `nightly_doc` flag
- 1.??: `F`[doc_notable_trait](https://github.com/rust-lang/rust/issues/45040)
        = `nightly_doc` flag
- 1.??: `F`[f16|f128](https://github.com/rust-lang/rust/issues/116909)
        = `nightly_float` flag
- 1.??: ` `[float_algebraic](https://github.com/rust-lang/rust/issues/136469)
- 1.??: ` `[generic_atomic](https://github.com/rust-lang/rust/issues/130539)
- 1.??: ` `[integer_atomics](https://github.com/rust-lang/rust/issues/99069)
- 1.??: ` `[maybe_uninit_slice](https://github.com/rust-lang/rust/issues/63569)
- 1.??: ` `[mpmc_channel](https://github.com/rust-lang/rust/issues/126840)
- 1.??: ` `[passing unstable flags only on nightly](https://github.com/rust-lang/cargo/issues/14733)
- 1.??: `F`[portable_simd](https://github.com/rust-lang/rust/issues/86656)
        = `nightly_simd` flag
- 1.??: ` `[thread_local](https://github.com/rust-lang/rust/issues/29594)

# Experimental features:
- [local_waker](https://github.com/rust-lang/rust/issues/118959)
- [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
 ` `- <https://rust-lang.github.io/rfcs/2580-ptr-meta.html>
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

# Pitfalls
- [const_evaluatable_unchecked](https://github.com/rust-lang/rust/issues/76200)

# Meta tracking issues:
- [arbitrary_self_types](https://github.com/rust-lang/rust/issues/44874)
  - [stabilize arbitrary self types v2](https://github.com/rust-lang/rust/pull/135881)
- [cargo config search](https://github.com/rust-lang/cargo/issues/9769)
- [const fn](https://github.com/rust-lang/rust/issues/57563)
- [feature-metadata](https://github.com/rust-lang/cargo/issues/14157)
- [impl Trait](https://github.com/rust-lang/rust/issues/63066)
- [int_roundings](https://github.com/rust-lang/rust/issues/88581)
- [MaybeUninit methods for arrays](https://github.com/rust-lang/rust/issues/96097)
- [num_midpoint](https://github.com/rust-lang/rust/issues/110840)
- [Reproducible Build bugs](https://github.com/rust-lang/rust/issues/129080)
- [unchecked_* on integers](https://github.com/rust-lang/rust/issues/85122)

# Rust project Goals:
- <https://rust-lang.github.io/rust-project-goals/>
  - <https://rust-lang.github.io/rust-project-goals/2025h2/>
- [const traits](https://github.com/rust-lang/rust-project-goals/issues/106)
- [expanded const generics](https://github.com/rust-lang/rust-project-goals/issues/100)
- [Linux building on stable](https://github.com/rust-lang/rust-project-goals/issues/116)
- [next generation trait solver](https://github.com/rust-lang/rust-project-goals/issues/113)
- [optimize clippy & linting](https://github.com/rust-lang/rust-project-goals/issues/114)
- [stabilize cargo-script](https://github.com/rust-lang/rust-project-goals/issues/119)
- [stabilize doc_cfg](https://github.com/rust-lang/rust-project-goals/issues/120)

