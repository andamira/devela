

Legend:
- `·` = enabled by non-specific feature (`nightly_[next1|next2|later`)
- `a` = enabled by non-specific feature (and depends on `alloc`)
- `s` = enabled by non-specific feature (and depends on `std`)
- `F` = enabled by some specific feature, shown afterwards.
- `✓` = past feature explicitly used

See also
- <https://github.com/rust-lang/rust/blob/master/library/core/src/lib.rs>
- <https://github.com/rust-lang/rust/blob/master/library/alloc/src/lib.rs>
- <https://github.com/rust-lang/rust/blob/master/library/std/src/lib.rs>

# 1.85 will be stable on 2025-02-20: ([`nightly_stable_next1`](https://releases.rs/docs/1.85.0/))

- 1.85: ` `[2024 edition](https://github.com/rust-lang/rust/issues/117258)
- 1.85: `·`[async_closure](https://github.com/rust-lang/rust/pull/132706)
- 1.85: `·`[const_align_of_val](https://github.com/rust-lang/rust/pull/133762)
        `·` const_size_of_val
- 1.85: `s`[const_collections_with_hasher](https://github.com/rust-lang/rust/pull/133696)
      `·` build_hasher_default_const_new
- 1.85: `·`[const_float_methods](https://github.com/rust-lang/rust/issues/130843)
- 1.85: `·`[const_maybe_uninit_write](https://github.com/rust-lang/rust/pull/131713)
- 1.85: `·`[const_nonnull_new](https://github.com/rust-lang/rust/pull/134116)
- 1.85: `·`[const_swap](https://github.com/rust-lang/rust/pull/134757)
- 1.85: `·`[coverage_attribute](https://github.com/rust-lang/rust/pull/130766)
- 1.85: `·`[diagnostics::do_not_recommend](https://github.com/rust-lang/rust/pull/132056)
- 1.85: ` `[Extend impls for tuples 1-12](https://github.com/rust-lang/rust/pull/132187)
- 1.85: `·`[extended_varargs_abi_support](https://github.com/rust-lang/rust/pull/116161)
- 1.85: ` `[home_dir:fix&undeprecate](https://github.com/rust-lang/rust/pull/132515)
- 1.85: `·`[noop_waker](https://github.com/rust-lang/rust/issues/98286) work::async::waker
- 1.85: `·`[num_midpoint:unsigned,float](https://github.com/rust-lang/rust/pull/131784)
- 1.85: `·`[ptr_fn_addr_eq](https://github.com/rust-lang/rust/pull/133678)

# 1.86 will be stable on 2025-04-03 ([`nightly_stable_next2`](https://releases.rs/docs/1.86.0/))

- 1.86: `·`[const_black_box](https://github.com/rust-lang/rust/pull/135414)
- 1.86: `·`[const_is_char_boundary](https://github.com/rust-lang/rust/pull/134016)
       `·` const_str_split_at
- 1.86: `s`[const_mut_cursor](https://github.com/rust-lang/rust/pull/136634)
- 1.86: `·`[float_next_up_down](https://github.com/rust-lang/rust/pull/135661)
- 1.86: ` `[get_disjoint_mut](https://github.com/rust-lang/rust/pull/134633)
        `·` get_many_mut (old name)
- 1.86: `s`[map_many_mut](https://github.com/rust-lang/rust/pull/136152)
- 1.86: `·`[non_zero_count_ones](https://github.com/rust-lang/rust/pull/136663)
- 1.86: `·`[target_feature_11](https://github.com/rust-lang/rust/pull/134090)
- 1.86: `·`[trait_upcasting](https://github.com/rust-lang/rust/pull/134367)
- 1.86: `a`[vec_pop_if](https://github.com/rust-lang/rust/pull/135488)

# … will be stable later ([`nightly_stable_later`](https://releases.rs/#ongoing-stabilization-prs))

- 1.??: `s`[anonymous_pipe](https://github.com/rust-lang/rust/pull/135822)
- 1.??: `·`[asm_goto](https://github.com/rust-lang/rust/pull/133870)
- 1.??: `a`[box_uninit_write](https://github.com/rust-lang/rust/issues/129397)
- 1.??: `·`[cell_update](https://github.com/rust-lang/rust/pull/134446)
- 1.??: `·`[cfg_match](https://github.com/rust-lang/rust/issues/115585)
- 1.??: `·`[const_array_from_ref](https://github.com/rust-lang/rust/issues/90206)
        `·` const_slice_from_ref
- 1.??: `·`[const_slice_flatten](https://github.com/rust-lang/rust/pull/134995)
- 1.??: `·`[const_str_from_utf8](https://github.com/rust-lang/rust/pull/136668)
- 1.??: `·`[derive_coerce_pointee](https://github.com/rust-lang/rust/pull/133820)
- 1.??: `s`[file_lock](https://github.com/rust-lang/rust/pull/136794)
- 1.??: `s`[hash_extract_if](https://github.com/rust-lang/rust/pull/134655)
- 1.??: `·`[impl_trait_in_assoc_type](https://github.com/rust-lang/rust/pull/120700)
- 1.??: `·`[integer_sign_cast](https://github.com/rust-lang/rust/pull/137026)
- 1.??: `·`[isqrt](https://github.com/rust-lang/rust/pull/131391)
- 1.??: `·`[let_chains](https://github.com/rust-lang/rust/pull/132833)
- 1.??: `·`[macro_metavar_expr](https://github.com/rust-lang/rust/pull/122808)
- 1.??: `·`[naked_functions](https://github.com/rust-lang/rust/pull/134213)
- 1.??: `a`[new_zeroed_alloc](https://github.com/rust-lang/rust/issues/129396)
- 1.??: `·`[num_midpoint_signed](https://github.com/rust-lang/rust/pull/134340)
- 1.??: `s`[once_wait](https://github.com/rust-lang/rust/pull/136360)
- 1.??: `·`[unbounded_shifts](https://github.com/rust-lang/rust/issues/129375)
- 1.??: `·`[unsafe_cell_from_mut](https://github.com/rust-lang/rust/pull/131261)

# … should be stable even later in the future

- 1.?? `F`[allocator_api](https://github.com/rust-lang/rust/issues/32838)
       `nightly_allocator`
- 1.?? `F`[autodiff](https://github.com/rust-lang/rust/issues/124509)
          `nightly_autodiff`
- 1.?? `F`[bigint_helper_methods](https://github.com/rust-lang/rust/issues/85532)
       `nightly_bigint`
- 1.?? ` `[box_into_inner](https://github.com/rust-lang/rust/issues/80437)
- 1.?? ` `[cfg(accessible(::path::to::thing))](https://github.com/rust-lang/rust/issues/64797)
- 1.?? ` `[cfg(version(..))](https://github.com/rust-lang/rust/issues/64796)
- 1.?? ` `[const_async_blocks](https://github.com/rust-lang/rust/issues/85368)
- 1.?? ` `[const_closures](https://github.com/rust-lang/rust/issues/106003)
- 1.?? ` `[const_for](https://github.com/rust-lang/rust/issues/87575)
          (depends on const_trait_impl)
- 1.?? ` `[const_str_from_utf8](https://github.com/rust-lang/rust/issues/91006)
- 1.?? ` `[const_trait_impl](https://github.com/rust-lang/rust/issues/67792)
- 1.?? `F`[coroutines](https://github.com/rust-lang/rust/issues/43122)
       `nightly_coro`
- 1.?? `F`[doc_cfg](https://github.com/rust-lang/rust/issues/43781)
       `nightly_doc`
- 1.?? `F`[doc_notable_trait](https://github.com/rust-lang/rust/issues/45040)
       `nightly_doc`
- 1.?? `F`[f16|f128](https://github.com/rust-lang/rust/issues/116909)
       `nightly_float`
- 1.?? ` `[generic_atomic](https://github.com/rust-lang/rust/issues/130539)
- 1.?? ` `[integer_atomics](https://github.com/rust-lang/rust/issues/99069)
- 1.?? ` `[maybe_uninit_slice](https://github.com/rust-lang/rust/issues/63569)
- 1.?? ` `[mpmc_channel](https://github.com/rust-lang/rust/issues/126840)
- 1.?? ` `[passing unstable flags only on nightly](https://github.com/rust-lang/cargo/issues/14733)
- 1.?? `F`[portable_simd](https://github.com/rust-lang/rust/issues/86656)
       `nightly_simd`
- 1.?? ` `[slice_as_array](https://github.com/rust-lang/rust/issues/133508)
- 1.?? ` `[stdarch_x86_avx512](https://github.com/rust-lang/rust/issues/111137)

# Experimental features:
- [stmt_expr_attributes](https://github.com/rust-lang/rust/issues/15701)
- [offset_of_enum](https://github.com/rust-lang/rust/issues/120141)
- [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
 ` `- <https://rust-lang.github.io/rfcs/2580-ptr-meta.html>
- [type_alias_impl_trait|impl_trait_in_assoc_type](https://github.com/rust-lang/rust/issues/63063)

# Meta tracking issues:
- [arbitrary_self_types](https://github.com/rust-lang/rust/issues/44874)
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
- [Rust 2024 Edition](https://github.com/rust-lang/rust-project-goals/issues/117)
- [stabilize cargo-script](https://github.com/rust-lang/rust-project-goals/issues/119)
- [stabilize doc_cfg](https://github.com/rust-lang/rust-project-goals/issues/120)

# Past releases

# 1.84 became stable on [2024-01-09](https://blog.rust-lang.org/2025/01/09/Rust-1.84.0.html):
- 1.84: ` `[const_atomic_from_ptr](https://github.com/rust-lang/rust/pull/131717)
- 1.84: ` `[const_char_encode_utf16](https://github.com/rust-lang/rust/pull/132153)
- 1.84: ` `[const_float_methods(unstable)](https://github.com/rust-lang/rust/pull/130568)
- 1.84: ` `[const_make_ascii](https://github.com/rust-lang/rust/pull/131496)
- 1.84: ` `[const_maybe_uninit_assume_init](https://github.com/rust-lang/rust/pull/131274)
- 1.84: ` `[const_option_ext](https://github.com/rust-lang/rust/pull/132966)
- 1.84: ` `[const_pin](https://github.com/rust-lang/rust/issues/76654)
- 1.84: ` `[const_ptr_is_null](https://github.com/rust-lang/rust/pull/133116)
- 1.84: ` `[const_unicode_case_lookup](https://github.com/rust-lang/rust/pull/132948)
- 1.84: ` `[Do not run lints that cannot emit](https://github.com/rust-lang/rust/pull/125116)
- 1.84: ` `[inline assembly s390x](https://github.com/rust-lang/rust/pull/131258)
- 1.84: ` `[inline assembly Arm64EC ](https://github.com/rust-lang/rust/pull/131781)
- 1.84: ` `[Ipv6Addr::is_(unicast_link|unique)_local](https://github.com/rust-lang/rust/pull/129238)
- 1.84: ` `[Provenance(Strict,Exposed)](https://github.com/rust-lang/rust/pull/130350)
- 1.84: ` `[pin_deref_mut](https://github.com/rust-lang/rust/pull/129424)
- 1.84: ` `[result_ffi_guarantees](https://github.com/rust-lang/rust/pull/130628)
- 1.84: ` `[wasm:multivalue,reference-types,tail-call](https://github.com/rust-lang/rust/pull/131080)

## 1.83 became stable on [2024-11-28](https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html):
- 1.83: `✓`[char::MIN](https://github.com/rust-lang/rust/pull/130154)
- 1.83: ` `[const_cell_into_inner](https://github.com/rust-lang/rust/pull/130972)
- 1.83: `✓`[const_char_encode_utf8](https://github.com/rust-lang/rust/pull/131463)
- 1.83: ` `[const_extern_fn](https://github.com/rust-lang/rust/pull/129753)
- 1.83: `✓`[const_float_bits_conv](https://github.com/rust-lang/rust/pull/129555)
- 1.83: `✓`[const_float_classify](https://github.com/rust-lang/rust/pull/130157)
- 1.83: `✓`[const_intrinsic_copy](https://github.com/rust-lang/rust/pull/130762)
- 1.83: `✓`[const_maybe_uninit_as_mut_ptr](https://github.com/rust-lang/rust/pull/130542)
- 1.83: `✓`[const_mut_refs|const_refs_to_cell](https://github.com/rust-lang/rust/pull/129195)
- 1.83: ` `[const_refs_to_static](https://github.com/rust-lang/rust/pull/129759)
- 1.83: ` `[const_result](https://github.com/rust-lang/rust/pull/131287)
- 1.83: `✓`[const_slice_from_raw_parts_mut](https://github.com/rust-lang/rust/pull/130403)
- 1.83: `✓`[const_slice_split_at_mut](https://github.com/rust-lang/rust/pull/130428)
- 1.83: ` `[duration_consts_float](https://github.com/rust-lang/rust/pull/131289)
- 1.83: ` `[entry_insert](https://github.com/rust-lang/rust/pull/130290)
- 1.83: `✓`[io_error_more](https://github.com/rust-lang/rust/pull/128316)
- 1.83: `✓`[r#ident](https://github.com/rust-lang/rust/pull/126452)
- 1.83: `✓`[rustdoc: table of contents](https://github.com/rust-lang/rust/pull/120736)
- 1.83: `✓`[waker_getters](https://github.com/rust-lang/rust/pull/129919)

## 1.82 became stable on [2024-10-17](https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html):
- 1.82: ` `[asm_const](https://github.com/rust-lang/rust/pull/128570)
- 1.82: ` `[char_indices_offset](https://github.com/rust-lang/rust/pull/129276)
- 1.82: `✓`[const_fn_floating_point_arithmetic](https://github.com/rust-lang/rust/pull/128596)
- 1.82: `✓`[const_int_from_str](https://github.com/rust-lang/rust/pull/124941)
- 1.82: `✓`[const_waker](https://github.com/rust-lang/rust/pull/128228)
- 1.82: ` `[debug_more_non_exhaustive](https://github.com/rust-lang/rust/pull/131109)
- 1.82: ` `[iter::repeat_n](https://github.com/rust-lang/rust/pull/129294)
- 1.82: ` `[is_none_or](https://github.com/rust-lang/rust/pull/129086)
- 1.82: ` `[is_sorted](https://github.com/rust-lang/rust/pull/128279)
- 1.82: ` `[min_exhaustive_patterns](https://github.com/rust-lang/rust/pull/122792)
- 1.82: ` `[new_uninit](https://github.com/rust-lang/rust/pull/129401)
- 1.82: ` `[offset_of_nested](https://github.com/rust-lang/rust/pull/128284)
- 1.82: ` `[raw_ref_op](https://github.com/rust-lang/rust/pull/127679)
- 1.82: ` `[ready_into_inner](https://github.com/rust-lang/rust/pull/116528)
- 1.82: ` `[thread_spawn_unchecked](https://github.com/rust-lang/rust/pull/129161)
- 1.82: ` `[unsafe_extern_blocks](https://github.com/rust-lang/rust/pull/127921)
- 1.82: ` `[unsafe_attributes](https://github.com/rust-lang/rust/pull/128771)

