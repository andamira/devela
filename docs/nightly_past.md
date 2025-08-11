# Past releases

Legend:

- ` ` = not known to be explicitly used
- `✓` = known to be explicitly used
- `→` = intended to be used soon
- `…` = considered for later
- `u` = unstable feature

# 1.89 became stable on [2025-08-07](https://releases.rs/docs/1.89.0/))
> - <https://github.com/rust-lang/rust/milestone/133>
> - <https://blog.rust-lang.org/2025/08/07/Rust-1.89.0>
> - <https://github.com/rust-lang/rust/releases/tag/1.89.0>

- 1.89: ` `[±LoongArch features](https://github.com/rust-lang/rust/pull/135015)
- 1.89: ` `[Allow storing format_args! in variable](https://github.com/rust-lang/rust/pull/140748)
- 1.89: ` `[avx512_target_feature](https://github.com/rust-lang/rust/pull/138940)
- 1.89: `→`[const_array_as_mut_slice](https://github.com/rust-lang/rust/pull/140066)
- 1.89: `→`[file_lock](https://github.com/rust-lang/rust/pull/136794)
- 1.89: `…`[keylocker_x86](https://github.com/rust-lang/rust/pull/140766)
- 1.89: ` `[non_null_from_ref](https://github.com/rust-lang/rust/pull/140511)
- 1.89: ` `[os_string_pathbuf_leak](https://github.com/rust-lang/rust/pull/137992)
- 1.89: `✓`[remove wasm legacy abi](https://github.com/rust-lang/rust/pull/133952)
- 1.89: ` `[repr128](https://github.com/rust-lang/rust/pull/138285)
- 1.89: ` `[resolve_const_param_in_non_trivial_anon_const](https://github.com/rust-lang/rust/pull/142157)
- 1.89: `→`[result_flattening](https://github.com/rust-lang/rust/pull/141072)
- 1.89: ` `[sha512_sm_x86](https://github.com/rust-lang/rust/pull/140767)
- 1.89: ` `[stdarch_x86_avx512](https://github.com/rust-lang/rust/issues/138940)
- 1.89: ` `[unstable: iter_macro](https://github.com/rust-lang/stdarch/pull/1819)

# 1.88 became stable on [2025-06-26](https://releases.rs/docs/1.87.0/)
> - <https://github.com/rust-lang/rust/milestone/132>
> - <https://blog.rust-lang.org/2025/06/26/Rust-1.88.0>
> - <https://github.com/rust-lang/rust/releases/tag/1.88.0>

- 1.88: `u`[async_drop](https://github.com/rust-lang/rust/pull/123948)
- 1.88: ` `[c_str_module](https://github.com/rust-lang/rust/pull/137439)
- 1.88: `→`[cfg_boolean_literals](https://github.com/rust-lang/rust/pull/138632)
- 1.88: `→`[cell_update](https://github.com/rust-lang/rust/pull/134446)
- 1.88: `→`[const_cell](https://github.com/rust-lang/rust/pull/137928)
- 1.88: `✓`[const_swap_nonoverlapping](https://github.com/rust-lang/rust/pull/137280)
- 1.88: `u`[exact_div](https://github.com/rust-lang/rust/pull/139163)
- 1.88: `→`[let_chains](https://github.com/rust-lang/rust/pull/132833)
- 1.88: ` `[naked_functions](https://github.com/rust-lang/rust/pull/134213)
- 1.88: ` `[±proc_macro_span](https://github.com/rust-lang/rust/pull/139865)
- 1.88: `→`[slice_as_chunks](https://github.com/rust-lang/rust/pull/139656)

# 1.87 became stable on [2025-05-15](https://releases.rs/docs/1.87.0/)
> - <https://github.com/rust-lang/rust/milestone/130>
> - <https://blog.rust-lang.org/2025/05/15/Rust-1.87.0>
> - <https://github.com/rust-lang/rust/releases/tag/1.87.0>

- 1.87: `→`[anonymous_pipe](https://github.com/rust-lang/rust/pull/135822)
- 1.87: ` `[asm_goto](https://github.com/rust-lang/rust/pull/133870)
- 1.87: ` `[box_uninit_write](https://github.com/rust-lang/rust/pull/137850)
- 1.87: ` `[const_copy_from_slice](https://github.com/rust-lang/rust/pull/138098)
- 1.87: ` `[const_slice_flatten](https://github.com/rust-lang/rust/pull/134995)
- 1.87: `→`[const_str_from_utf8](https://github.com/rust-lang/rust/pull/136668)
- 1.87: ` `[const_vec_string_slice](https://github.com/rust-lang/rust/pull/137319)
- 1.87: ` `[extract_if](https://github.com/rust-lang/rust/pull/137109)
- 1.87: ` `[hash_extract_if](https://github.com/rust-lang/rust/pull/134655)
- 1.87: ` `[integer_sign_cast](https://github.com/rust-lang/rust/pull/137026)
- 1.87: `→`[num_midpoint_signed](https://github.com/rust-lang/rust/pull/134340)
- 1.87: ` `[os_str_display](https://github.com/rust-lang/rust/pull/137336)
- 1.87: ` `[precise_capturing_in_traits](https://github.com/rust-lang/rust/pull/138128)
- 1.87: ` `[ptr_sub_ptr](https://github.com/rust-lang/rust/pull/137121)
        ` ` `const_ptr_sub_ptr`
- 1.87: ` `[slice_take](https://github.com/rust-lang/rust/pull/137829)
        ` ` `split_off` (new name)
- 1.87: ` `[string_extend_from_within](https://github.com/rust-lang/rust/pull/137569)
- 1.87: ` `[unbounded_shifts](https://github.com/rust-lang/rust/pull/137393)
- 1.87: ` `[unsigned_is_multiple_of](https://github.com/rust-lang/rust/pull/137383)
- 1.87: ` `[Allow `*const W<dyn A>` -> `*const dyn A` ptr cast](https://github.com/rust-lang/rust/pull/136127)
- 1.87: ` `[IoErrorKind::InvalidFilename](https://github.com/rust-lang/rust/pull/134076)
- 1.87: ` `[Reduce fmt width & precision to 16 bits](https://github.com/rust-lang/rust/pull/136932)
- 1.87: `→`[Undeprecate env::home_dir](https://github.com/rust-lang/rust/pull/137327)
- 1.87: ` `[Update to LLVM 20](https://github.com/rust-lang/rust/pull/135763)
- 1.87: ` `[Update stdarch](https://github.com/rust-lang/rust/pull/136831) CHECK

# 1.86 became stable on [2025-04-03](https://releases.rs/docs/1.86.0/)
> - <https://blog.rust-lang.org/2025/04/03/Rust-1.86.0>
> - <https://github.com/rust-lang/rust/releases/tag/1.86.0>
> - <https://github.com/rust-lang/rust/milestone/128>

- 1.86: `✓`[const_black_box](https://github.com/rust-lang/rust/pull/135414)
- 1.86: `→`[const_is_char_boundary](https://github.com/rust-lang/rust/pull/134016)
        `→` `const_str_split_at`
- 1.86: `→`[const_mut_cursor](https://github.com/rust-lang/rust/pull/136634)
- 1.86: ` `[float_next_up_down](https://github.com/rust-lang/rust/pull/135661)
- 1.86: ` `[get_disjoint_mut](https://github.com/rust-lang/rust/pull/134633)
- 1.86: ` `[map_many_mut](https://github.com/rust-lang/rust/pull/136152)
- 1.86: ` `[non_zero_count_ones](https://github.com/rust-lang/rust/pull/136663)
- 1.86: ` `[target_feature_11](https://github.com/rust-lang/rust/pull/134090)
- 1.86: `→`[trait_upcasting](https://github.com/rust-lang/rust/pull/134367)
- 1.86: ` `[vec_pop_if](https://github.com/rust-lang/rust/pull/135488)
- 1.86: `✓`[iter::FromCoroutine](https://github.com/rust-lang/rust/pull/135687)

# 1.85 became stable on [2025-02-20](https://releases.rs/docs/1.85.0/)
> - <https://blog.rust-lang.org/2025/02/20/Rust-1.85.0>
> - <https://github.com/rust-lang/rust/releases/tag/1.85.0>
> - <https://github.com/rust-lang/rust/milestone/127>

- 1.85: `✓`[2024 edition](https://github.com/rust-lang/rust/issues/117258)
- 1.85: ` `[async_closure](https://github.com/rust-lang/rust/pull/132706)
- 1.85: `✓`[const_align_of_val](https://github.com/rust-lang/rust/pull/133762)
        `✓` `const_size_of_val`
- 1.85: ` `[const_collections_with_hasher](https://github.com/rust-lang/rust/pull/133696)
        ` ` `build_hasher_default_const_new`
- 1.85: `✓`[const_float_methods](https://github.com/rust-lang/rust/issues/130843)
- 1.85: ` `[const_maybe_uninit_write](https://github.com/rust-lang/rust/pull/131713)
- 1.85: ` `[const_nonnull_new](https://github.com/rust-lang/rust/pull/134116)
- 1.85: `✓`[const_swap](https://github.com/rust-lang/rust/pull/134757)
- 1.85: ` `[coverage_attribute](https://github.com/rust-lang/rust/pull/130766)
- 1.85: ` `[diagnostics::do_not_recommend](https://github.com/rust-lang/rust/pull/132056)
- 1.85: ` `[Extend impls for tuples 1-12](https://github.com/rust-lang/rust/pull/132187)
- 1.85: ` `[extended_varargs_abi_support](https://github.com/rust-lang/rust/pull/116161)
- 1.85: `✓`[home_dir:fix&undeprecate](https://github.com/rust-lang/rust/pull/132515)
- 1.85: `✓`[noop_waker](https://github.com/rust-lang/rust/issues/98286) work::async::waker
- 1.85: `→`[num_midpoint:unsigned,float](https://github.com/rust-lang/rust/pull/131784)
- 1.85: `✓`[ptr_fn_addr_eq](https://github.com/rust-lang/rust/pull/133678)

# 1.84 became stable on [2024-01-09](https://releases.rs/docs/1.84.0/)
> - <https://blog.rust-lang.org/2025/01/09/Rust-1.84.0>
> - <https://github.com/rust-lang/rust/releases/tag/1.84.0>
> - <https://github.com/rust-lang/rust/milestone/126>

- 1.84: ` `[const_atomic_from_ptr](https://github.com/rust-lang/rust/pull/131717)
- 1.84: ` `[const_char_encode_utf16](https://github.com/rust-lang/rust/pull/132153)
- 1.84: `✓`[const_float_methods(unstable)](https://github.com/rust-lang/rust/pull/130568)
- 1.84: ` `[const_make_ascii](https://github.com/rust-lang/rust/pull/131496)
- 1.84: `→`[const_maybe_uninit_assume_init](https://github.com/rust-lang/rust/pull/131274)
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

# 1.83 became stable on [2024-11-28](https://releases.rs/docs/1.83.0/)
> - <https://blog.rust-lang.org/2024/11/28/Rust-1.83.0>
> - <https://github.com/rust-lang/rust/releases/tag/1.83.0>
> - <https://github.com/rust-lang/rust/milestone/125>

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

# 1.82 became stable on [2024-10-17](https://releases.rs/docs/1.82.0/)
> - <https://blog.rust-lang.org/2024/10/17/Rust-1.82.0>
> - <https://github.com/rust-lang/rust/releases/tag/1.82.0>
> - <https://github.com/rust-lang/rust/milestone/123>

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
- 1.82: ` `[raw_ref_op](https://github.com/rust-lang/rust/pull/127679) (&raw const, &raw mut)
- 1.82: ` `[ready_into_inner](https://github.com/rust-lang/rust/pull/116528)
- 1.82: ` `[thread_spawn_unchecked](https://github.com/rust-lang/rust/pull/129161)
- 1.82: `✓`[unsafe_extern_blocks](https://github.com/rust-lang/rust/pull/127921)
- 1.82: `✓`[unsafe_attributes](https://github.com/rust-lang/rust/pull/128771)
