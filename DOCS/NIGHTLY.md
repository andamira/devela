

Legend:
- ` ` = not enabled / can't be enabled
- `·` = enabled by non-specific cfg flag (`nightly_[next1|next2|later`)
- `a` = enabled by non-specific cfg flag (and depends on `alloc`)
- `s` = enabled by non-specific cfg flag (and depends on `std`)
- `F` = enabled by some specific cfg flag, shown afterwards.

Usage example:
```sh
RUSTFLAGS="--cfg nightly_coro --cfg nightly_stable_next1" cargo +nightly b
```

See also
- <https://doc.rust-lang.org/nightly/unstable-book/the-unstable-book.html>
- <https://github.com/rust-lang/rust/blob/master/library/core/src/lib.rs>
- <https://github.com/rust-lang/rust/blob/master/library/alloc/src/lib.rs>
- <https://github.com/rust-lang/rust/blob/master/library/std/src/lib.rs>

# 1.86 will be stable on 2025-04-03 ([`nightly_stable_next1`](https://releases.rs/docs/1.86.0/))
> - <https://github.com/rust-lang/rust/milestone/128>

- 1.86: `·`[const_black_box](https://github.com/rust-lang/rust/pull/135414)
- 1.86: `·`[const_is_char_boundary](https://github.com/rust-lang/rust/pull/134016)
       `·` `const_str_split_at`
- 1.86: `s`[const_mut_cursor](https://github.com/rust-lang/rust/pull/136634)
- 1.86: `·`[float_next_up_down](https://github.com/rust-lang/rust/pull/135661)
- 1.86: ` `[get_disjoint_mut](https://github.com/rust-lang/rust/pull/134633)
        `·` `get_many_mut` (old name)
- 1.86: `s`[map_many_mut](https://github.com/rust-lang/rust/pull/136152)
- 1.86: `·`[non_zero_count_ones](https://github.com/rust-lang/rust/pull/136663)
- 1.86: `·`[target_feature_11](https://github.com/rust-lang/rust/pull/134090)
- 1.86: `·`[trait_upcasting](https://github.com/rust-lang/rust/pull/134367)
- 1.86: `a`[vec_pop_if](https://github.com/rust-lang/rust/pull/135488)
- 1.86: ` `[iter::FromCoroutine](https://github.com/rust-lang/rust/pull/135687)

# 1.87 will be stable on 2025-05-15 ([`nightly_stable_next2`](https://releases.rs/docs/1.87.0/))
> - <https://github.com/rust-lang/rust/milestone/130>

- 1.87: `·`[const_slice_flatten](https://github.com/rust-lang/rust/pull/134995)
- 1.87: `·`[const_str_from_utf8](https://github.com/rust-lang/rust/pull/136668)
- 1.87: `a`[extract_if](https://github.com/rust-lang/rust/pull/137109)
- 1.87: `s`[hash_extract_if](https://github.com/rust-lang/rust/pull/134655)
- 1.87: `·`[integer_sign_cast](https://github.com/rust-lang/rust/pull/137026)
- 1.87: `s`[file_lock](https://github.com/rust-lang/rust/pull/136794)
- 1.87: `·`[num_midpoint_signed](https://github.com/rust-lang/rust/pull/134340)
- 1.87: `s`[os_str_display](https://github.com/rust-lang/rust/pull/137336)
- 1.87: `·`[ptr_sub_ptr](https://github.com/rust-lang/rust/pull/137121)
        `·` `const_ptr_sub_ptr`
- 1.87: `·`[unbounded_shifts](https://github.com/rust-lang/rust/pull/137393)
- 1.87: `·`[unsigned_is_multiple_of](https://github.com/rust-lang/rust/pull/137383)
- 1.87: ` `[Update to LLVM 20](https://github.com/rust-lang/rust/pull/135763)

# … will be stable later ([`nightly_stable_later`](https://releases.rs/#ongoing-stabilization-prs))

- 1.??: `s`[anonymous_pipe](https://github.com/rust-lang/rust/pull/135822)
- 1.??: `·`[asm_goto](https://github.com/rust-lang/rust/pull/133870)
- 1.??: `·`[assert_matches](https://github.com/rust-lang/rust/pull/137487)
- 1.??: `a`[box_uninit_write](https://github.com/rust-lang/rust/pull/137850)
- 1.??: `·`[cell_update](https://github.com/rust-lang/rust/pull/134446)
- 1.??: `·`[cfg_match](https://github.com/rust-lang/rust/issues/115585)
- 1.??: `·`[const_array_from_ref](https://github.com/rust-lang/rust/issues/90206)
        `·` `const_slice_from_ref`
- 1.??: `·`[const_cell](https://github.com/rust-lang/rust/pull/137928)
- 1.??: `a`[const_vec_string_slice](https://github.com/rust-lang/rust/pull/137319)
- 1.??: `·`[const_swap_nonoverlapping](https://github.com/rust-lang/rust/pull/137280)
- 1.??: `·`[c_str_module](https://github.com/rust-lang/rust/pull/137439)
- 1.??: `·`[derive_coerce_pointee](https://github.com/rust-lang/rust/pull/133820)
- 1.??: ` `[flags for doctest cross compilation](https://github.com/rust-lang/rust/pull/137096)
- 1.??: `·`[impl_trait_in_assoc_type](https://github.com/rust-lang/rust/pull/120700)
- 1.??: `·`[integer_sign_cast](https://github.com/rust-lang/rust/pull/137026)
- 1.??: `·`[isqrt](https://github.com/rust-lang/rust/pull/131391)
- 1.??: `·`[let_chains](https://github.com/rust-lang/rust/pull/132833)
- 1.??: `·`[macro_metavar_expr](https://github.com/rust-lang/rust/pull/122808)
- 1.??: `·`[naked_functions](https://github.com/rust-lang/rust/pull/134213)
- 1.??: `a`[new_zeroed_alloc](https://github.com/rust-lang/rust/issues/129396)
- 1.??: `s`[once_wait](https://github.com/rust-lang/rust/pull/136360)
- 1.??: `·`[slice_take](https://github.com/rust-lang/rust/pull/137829)
        ` ` `split_off` (new name)
- 1.??: `·`[unsafe_cell_from_mut](https://github.com/rust-lang/rust/pull/131261)

# … may be stable even later

- 1.?? `F`[allocator_api](https://github.com/rust-lang/rust/issues/32838)
        = `nightly_allocator`
- 1.?? `F`[autodiff](https://github.com/rust-lang/rust/issues/124509)
          `nightly_autodiff`
- 1.?? `F`[bigint_helper_methods](https://github.com/rust-lang/rust/issues/85532)
        = `nightly_bigint`
- 1.?? ` `[box_into_inner](https://github.com/rust-lang/rust/issues/80437)
- 1.?? ` `[cfg(accessible(::path::to::thing))](https://github.com/rust-lang/rust/issues/64797)
- 1.?? ` `[cfg(version(..))](https://github.com/rust-lang/rust/issues/64796)
- 1.?? ` `[concat_bytes](https://github.com/rust-lang/rust/issues/87555)
- 1.?? ` `[concat_idents](https://github.com/rust-lang/rust/issues/29599)
- 1.?? ` `[const_async_blocks](https://github.com/rust-lang/rust/issues/85368)
- 1.?? ` `[const_closures](https://github.com/rust-lang/rust/issues/106003)
- 1.?? ` `[const_for](https://github.com/rust-lang/rust/issues/87575)
          (depends on const_trait_impl)
- 1.?? ` `[const_str_from_utf8](https://github.com/rust-lang/rust/issues/91006)
- 1.?? ` `[const_trait_impl](https://github.com/rust-lang/rust/issues/67792)
- 1.?? `F`[coroutines](https://github.com/rust-lang/rust/issues/43122)
        = `nightly_coro`
- 1.?? `F`[doc_cfg](https://github.com/rust-lang/rust/issues/43781)
        = `nightly_doc`
- 1.?? `F`[doc_notable_trait](https://github.com/rust-lang/rust/issues/45040)
        = `nightly_doc`
- 1.?? `F`[f16|f128](https://github.com/rust-lang/rust/issues/116909)
        = `nightly_float`
- 1.?? ` `[generic_atomic](https://github.com/rust-lang/rust/issues/130539)
- 1.?? ` `[integer_atomics](https://github.com/rust-lang/rust/issues/99069)
- 1.?? ` `[maybe_uninit_slice](https://github.com/rust-lang/rust/issues/63569)
- 1.?? ` `[mpmc_channel](https://github.com/rust-lang/rust/issues/126840)
- 1.?? ` `[panic_payload_as_str](https://github.com/rust-lang/rust/issues/125175)
- 1.?? ` `[passing unstable flags only on nightly](https://github.com/rust-lang/cargo/issues/14733)
- 1.?? `F`[portable_simd](https://github.com/rust-lang/rust/issues/86656)
        = `nightly_simd`
- 1.?? ` `[slice_as_array](https://github.com/rust-lang/rust/issues/133508)
- 1.?? ` `[stdarch_x86_avx512](https://github.com/rust-lang/rust/issues/111137)
- 1.?? ` `[thread_local](https://github.com/rust-lang/rust/issues/29594)

# Experimental features:
- [local_waker](https://github.com/rust-lang/rust/issues/118959)
- [offset_of_enum](https://github.com/rust-lang/rust/issues/120141)
- [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
 ` `- <https://rust-lang.github.io/rfcs/2580-ptr-meta.html>
- [stmt_expr_attributes](https://github.com/rust-lang/rust/issues/15701)
- [type_alias_impl_trait|impl_trait_in_assoc_type](https://github.com/rust-lang/rust/issues/63063)

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
- [const traits](https://github.com/rust-lang/rust-project-goals/issues/106)
- [expanded const generics](https://github.com/rust-lang/rust-project-goals/issues/100)
- [Linux building on stable](https://github.com/rust-lang/rust-project-goals/issues/116)
- [next generation trait solver](https://github.com/rust-lang/rust-project-goals/issues/113)
- [optimize clippy & linting](https://github.com/rust-lang/rust-project-goals/issues/114)
- [stabilize cargo-script](https://github.com/rust-lang/rust-project-goals/issues/119)
- [stabilize doc_cfg](https://github.com/rust-lang/rust-project-goals/issues/120)

