

# 1.83 became stable on 2024-11-28:
- 1.83: ✓[char::MIN](https://github.com/rust-lang/rust/pull/130154)
- 1.83:  [const_cell_into_inner](https://github.com/rust-lang/rust/pull/130972)
- 1.83: ✓[const_char_encode_utf8](https://github.com/rust-lang/rust/pull/131463)
- 1.83:  [const_extern_fn](https://github.com/rust-lang/rust/pull/129753)
- 1.83: ✓[const_float_bits_conv](https://github.com/rust-lang/rust/pull/129555)
- 1.83: ✓[const_float_classify](https://github.com/rust-lang/rust/pull/130157)
- 1.83: ✓[const_intrinsic_copy](https://github.com/rust-lang/rust/pull/130762)
- 1.83: ✓[const_maybe_uninit_as_mut_ptr](https://github.com/rust-lang/rust/pull/130542)
- 1.83: ✓[const_mut_refs|const_refs_to_cell](https://github.com/rust-lang/rust/pull/129195)
- 1.83:  [const_refs_to_static](https://github.com/rust-lang/rust/pull/129759)
- 1.83:  [const_result](https://github.com/rust-lang/rust/pull/131287)
- 1.83: ✓[const_slice_from_raw_parts_mut](https://github.com/rust-lang/rust/pull/130403)
- 1.83: ✓[const_slice_split_at_mut](https://github.com/rust-lang/rust/pull/130428)
- 1.83:  [duration_consts_float](https://github.com/rust-lang/rust/pull/131289)
- 1.83:  [entry_insert](https://github.com/rust-lang/rust/pull/130290)
- 1.83: ✓[io_error_more](https://github.com/rust-lang/rust/pull/128316)
- 1.83: ✓[r#ident](https://github.com/rust-lang/rust/pull/126452)
- 1.83: ✓[rustdoc: table of contents](https://github.com/rust-lang/rust/pull/120736)
- 1.83: ✓[waker_getters](https://github.com/rust-lang/rust/pull/129919)

# 1.84 will be stable on 2024-01-09: (nightly_stable_next1)
- 1.84: ·[const_atomic_from_ptr](https://github.com/rust-lang/rust/pull/131717)
- 1.84: ·[const_char_encode_utf16](https://github.com/rust-lang/rust/pull/132153)
- 1.84: →[const_float_methods(unstable)](https://github.com/rust-lang/rust/pull/130568)
- 1.84: ·[const_make_ascii](https://github.com/rust-lang/rust/pull/131496)
- 1.84: ·[const_maybe_uninit_assume_init](https://github.com/rust-lang/rust/pull/131274)
- 1.84: ·[const_option_ext](https://github.com/rust-lang/rust/pull/132966)
- 1.84: ·[const_pin](https://github.com/rust-lang/rust/issues/76654)
- 1.84: ·[const_ptr_is_null](https://github.com/rust-lang/rust/pull/133116)
- 1.84: ·[const_unicode_case_lookup](https://github.com/rust-lang/rust/pull/132948)
- 1.84: ?[Do not run lints that cannot emit](https://github.com/rust-lang/rust/pull/125116)
- 1.84: ?[inline assembly s390x](https://github.com/rust-lang/rust/pull/131258)
- 1.84: ?[inline assembly Arm64EC ](https://github.com/rust-lang/rust/pull/131781)
- 1.84: ?[Ipv6Addr::is_(unicast_link|unique)_local](https://github.com/rust-lang/rust/pull/129238)
- 1.84: ?[Provenance(Strict,Exposed)](https://github.com/rust-lang/rust/pull/130350)
- 1.84: ·[pin_deref_mut](https://github.com/rust-lang/rust/pull/129424)
- 1.84: ·[result_ffi_guarantees](https://github.com/rust-lang/rust/pull/130628)
- 1.84: ?[wasm:multivalue,reference-types,tail-call](https://github.com/rust-lang/rust/pull/131080)

# 1.85 will be stable on 2025-02-20: (nightly_stable_next2)
- 1.85: …[2024 edition](https://github.com/rust-lang/rust/issues/117258)
- 1.85: ·[async_closure](https://github.com/rust-lang/rust/pull/132706)
- 1.85: ·[const_align_of_val](https://github.com/rust-lang/rust/pull/133762)
        · const_size_of_val
- 1.85: ?[const_collections_with_hasher](https://github.com/rust-lang/rust/pull/133696)
        · build_hasher_default_const_new
- 1.85: ·[const_float_methods](https://github.com/rust-lang/rust/issues/117258)
- 1.85: ·[const_maybe_uninit_write](https://github.com/rust-lang/rust/pull/131713)
- 1.85: ·[const_nonnull_new](https://github.com/rust-lang/rust/pull/134116)
- 1.85: ·[const_swap](https://github.com/rust-lang/rust/pull/134757)
- 1.85: ·[coverage_attribute](https://github.com/rust-lang/rust/pull/130766)
- 1.85: ·[diagnostics::do_not_recommend](https://github.com/rust-lang/rust/pull/132056)
- 1.85:  [Extend impls for tuples 1-12](https://github.com/rust-lang/rust/pull/132187)
- 1.85: ·[extended_varargs_abi_support](https://github.com/rust-lang/rust/pull/116161)
- 1.85: ?[home_dir:fix&undeprecate](https://github.com/rust-lang/rust/pull/132515)
- 1.85: ·[noop_waker](https://github.com/rust-lang/rust/issues/98286) work::async::waker
- 1.85: ·[num_midpoint:unsigned,float](https://github.com/rust-lang/rust/pull/131784)
- 1.85: ·[ptr_fn_addr_eq](https://github.com/rust-lang/rust/pull/133678)

# 1.‥… (nightly_stable_later) <https://releases.rs/#ongoing-stabilization-prs>
- 1.??: ·[asm_goto](https://github.com/rust-lang/rust/pull/133870)
- 1.??: ?[box_uninit_write](https://github.com/rust-lang/rust/issues/129397)
- 1.??: ·[cell_update](https://github.com/rust-lang/rust/pull/134446)
- 1.??: ·[const_·array|slice·_from_ref](https://github.com/rust-lang/rust/issues/90206)
- 1.??: ·[const_is_char_boundary](https://github.com/rust-lang/rust/pull/134016)
        · const_str_split_at
- 1.??: x[derive_smart_pointer](https://github.com/rust-lang/rust/pull/133820)
- 1.??: ?[get_disjoint_mut](https://github.com/rust-lang/rust/pull/134633)
- 1.??: ?[hash_extract_if](https://github.com/rust-lang/rust/pull/134655)
- 1.??: ·[impl_trait_in_assoc_type](https://github.com/rust-lang/rust/pull/120700)
- 1.??: ·[isqrt](https://github.com/rust-lang/rust/pull/131391)
- 1.??: ·[let_chains](https://github.com/rust-lang/rust/pull/132833)
- 1.??: ·[macro_metavar_expr](https://github.com/rust-lang/rust/pull/122808)
- 1.??: ·[naked_functions](https://github.com/rust-lang/rust/pull/134213)
- 1.??: ?[new_zeroed_alloc](https://github.com/rust-lang/rust/issues/129396)
- 1.??: ·[num_midpoint_signed](https://github.com/rust-lang/rust/pull/134340)
- 1.??: ·[trait_upcasting](https://github.com/rust-lang/rust/pull/134367)
- 1.??: ·[unbounded_shifts](https://github.com/rust-lang/rust/issues/129375)
- 1.??: ·[unsafe_cell_from_mut](https://github.com/rust-lang/rust/pull/131261)

# .‥… much later
- 1.?? F[allocator_api](https://github.com/rust-lang/rust/issues/32838) `nightly_allocator`
- 1.?? F[autodiff](https://github.com/rust-lang/rust/issues/124509) `nightly_autodiff`
- 1.?? F[bigint_helper_methods](https://github.com/rust-lang/rust/issues/85532) `nightly_bigint`
- 1.??  [box_into_inner](https://github.com/rust-lang/rust/issues/80437)
- 1.??  [cfg(accessible(::path::to::thing))](https://github.com/rust-lang/rust/issues/64797)
- 1.??  [cfg(version(..))](https://github.com/rust-lang/rust/issues/64796)
- 1.??  [const_async_blocks](https://github.com/rust-lang/rust/issues/85368)
- 1.??  [const_closures](https://github.com/rust-lang/rust/issues/106003)
- 1.??  [const_cmp](https://github.com/rust-lang/rust/issues/92391) REMOVED
- 1.??  [const_for](https://github.com/rust-lang/rust/issues/87575) depends on const_trait_impl
- 1.??  [const_str_from_utf8](https://github.com/rust-lang/rust/issues/91006)
- 1.??  [const_trait_impl](https://github.com/rust-lang/rust/issues/67792)
- 1.?? F[coroutines](https://github.com/rust-lang/rust/issues/43122) `nightly_coro`
- 1.?? F[doc_cfg](https://github.com/rust-lang/rust/issues/43781) `nightly_doc`
- 1.?? F[doc_notable_trait](https://github.com/rust-lang/rust/issues/45040) `nightly_doc`
- 1.?? F[f16|f128](https://github.com/rust-lang/rust/issues/116909) `nightly_float`
- 1.??  [generic_atomic](https://github.com/rust-lang/rust/issues/130539)
- 1.??  [integer_atomics](https://github.com/rust-lang/rust/issues/99069)
- 1.??  [mpmc_channel](https://github.com/rust-lang/rust/pull/126839)
- 1.??  [passing unstable flags only on nightly](https://github.com/rust-lang/cargo/issues/14733)
- 1.?? F[portable_simd](https://github.com/rust-lang/rust/issues/86656) `nightly_simd`
- 1.??  [slice_as_array](https://github.com/rust-lang/rust/issues/133508)
- 1.??  [stdarch_x86_avx512](https://github.com/rust-lang/rust/issues/111137)

# Experimental features:
- [stmt_expr_attributes](https://github.com/rust-lang/rust/issues/15701)
- [offset_of_enum](https://github.com/rust-lang/rust/issues/120141)
- [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
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
- https://rust-lang.github.io/rust-project-goals/
- [const traits](https://github.com/rust-lang/rust-project-goals/issues/106)
- [expanded const generics](https://github.com/rust-lang/rust-project-goals/issues/100)
- [Linux building on stable](https://github.com/rust-lang/rust-project-goals/issues/116)
- [next generation trait solver](https://github.com/rust-lang/rust-project-goals/issues/113)
- [optimize clippy & linting](https://github.com/rust-lang/rust-project-goals/issues/114)
- [Rust 2024 Edition](https://github.com/rust-lang/rust-project-goals/issues/117)
- [stabilize cargo-script](https://github.com/rust-lang/rust-project-goals/issues/119)
- [stabilize doc_cfg](https://github.com/rust-lang/rust-project-goals/issues/120)
