// devela::mem::cache_padded
//
// This is derived work from the `CachePadded` struct in the
// [crossbeam-utils](https://crates.io/crates/crossbeam-utils/0.8.20) crate,
// including the following modifications:
// - rename to CacheAlign.
// - add missing attributes.
// - add fn into_inner_copy.
// - add const ALIGN.
// - bound `unsafe` use.
// - misc. refactor.

/// Pads and aligns a value to the length of a cache line.
///
/// In concurrent programming, sometimes it is desirable to make sure commonly accessed pieces of
/// data are not placed into the same cache line. Updating an atomic value invalidates the whole
/// cache line it belongs to, which makes the next access to the same cache line slower for other
/// CPU cores. Use `CacheAlign` to ensure updating one piece of data doesn't invalidate other
/// cached data.
///
/// # Size and alignment
/// Cache lines are assumed to be N bytes long, depending on the architecture:
/// * On x86-64, aarch64, and powerpc64, N = 128.
/// * On arm, mips, mips64, sparc, and hexagon, N = 32.
/// * On m68k, N = 16.
/// * On s390x, N = 256.
/// * On all others, N = 64.
///
/// Note that N is just a reasonable guess and is not guaranteed to match the actual cache line
/// length of the machine the program is running on. On modern Intel architectures, spatial
/// prefetcher is pulling pairs of 64-byte cache lines at a time, so we pessimistically assume that
/// cache lines are 128 bytes long.
///
/// The size of `CacheAlign<T>` is the smallest multiple of N bytes large enough to accommodate
/// a value of type `T`.
///
/// The alignment of `CacheAlign<T>` is the maximum of N bytes and the alignment of `T`.
///
/// # Examples
/// Alignment and padding:
/// ```
/// # use devela::CacheAlign;
/// let array = [CacheAlign::new(1i8), CacheAlign::new(2i8)];
/// let addr1 = &*array[0] as *const i8 as usize;
/// let addr2 = &*array[1] as *const i8 as usize;
///
/// assert!(addr2 - addr1 >= 32);
/// assert_eq!(addr1 % 32, 0);
/// assert_eq!(addr2 % 32, 0);
/// ```
///
/// When building a concurrent queue with a head and a tail index, it is wise to place them in
/// different cache lines so that concurrent threads pushing and popping elements don't invalidate
/// each other's cache lines:
/// ```
/// # use devela::{CacheAlign, AtomicUsize};
/// struct Queue<T> {
///     head: CacheAlign<AtomicUsize>,
///     tail: CacheAlign<AtomicUsize>,
///     buffer: *mut T,
/// }
/// ```
//
// Starting from Intel's Sandy Bridge, spatial prefetcher is now pulling pairs of 64-byte cache
// lines at a time, so we have to align to 128 bytes rather than 64.
//
// Sources:
// - https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-optimization-manual.pdf
// - https://github.com/facebook/folly/blob/1b5288e6eea6df074758f877c849b6e73bbb9fbb/folly/lang/Align.h#L107
//
// ARM's big.LITTLE architecture has asymmetric cores and "big" cores have 128-byte cache line size.
//
// Sources:
// - https://www.mono-project.com/news/2016/09/12/arm64-icache/
//
// powerpc64 has 128-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_ppc64x.go#L9
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/powerpc/include/asm/cache.h#L26
#[cfg_attr(
    any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "powerpc64",),
    repr(align(128))
)]
// arm, mips, mips64, sparc, and hexagon have 32-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_arm.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mipsle.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips64x.go#L9
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L17
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/hexagon/include/asm/cache.h#L12
#[cfg_attr(
    any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips32r6",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "sparc",
        target_arch = "hexagon",
    ),
    repr(align(32))
)]
// m68k has 16-byte cache line size.
//
// Sources:
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/m68k/include/asm/cache.h#L9
#[cfg_attr(target_arch = "m68k", repr(align(16)))]
// s390x has 256-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_s390x.go#L7
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/s390/include/asm/cache.h#L13
#[cfg_attr(target_arch = "s390x", repr(align(256)))]
// x86, wasm, riscv, and sparc64 have 64-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/dda2991c2ea0c5914714469c4defc2562a907230/src/internal/cpu/cpu_x86.go#L9
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_wasm.go#L7
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/riscv/include/asm/cache.h#L10
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L19
//
// All others are assumed to have 64-byte cache line size.
#[cfg_attr(
    not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips32r6",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "sparc",
        target_arch = "hexagon",
        target_arch = "m68k",
        target_arch = "s390x",
    )),
    repr(align(64))
)]
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct CacheAlign<T> {
    value: T,
}

#[rustfmt::skip]
impl<T> CacheAlign<T> {
    /// The alignment of a cache line in the current platform.
    pub const ALIGN: usize = align_of::<Self>();

    /// Pads and aligns a value to the length of a cache line.
    ///
    /// # Examples
    /// ```
    /// # use devela::CacheAlign;
    /// let padded_value = CacheAlign::new(1);
    /// ```
    #[inline]
    pub const fn new(t: T) -> CacheAlign<T> { CacheAlign::<T> { value: t } }

    /// Returns the inner value.
    ///
    /// # Examples
    /// ```
    /// # use devela::CacheAlign;
    /// let padded_value = CacheAlign::new(7);
    /// let value = padded_value.into_inner();
    /// assert_eq!(value, 7);
    /// ```
    #[inline]
    pub fn into_inner(self) -> T { self.value }

    /// Returns the copied inner value in compile-time.
    #[inline]
    pub const fn into_inner_copy(self) -> T where Self: Copy { self.value }
}

#[rustfmt::skip]
mod impls {
    use crate::{CacheAlign, Debug, Deref, DerefMut, Display, FmtResult, Formatter};

    impl<T> From<T> for CacheAlign<T> {
        fn from(t: T) -> Self { CacheAlign::new(t) }
    }
    impl<T> Deref for CacheAlign<T> {
        type Target = T;
        fn deref(&self) -> &T { &self.value }
    }
    impl<T> DerefMut for CacheAlign<T> {
        fn deref_mut(&mut self) -> &mut T { &mut self.value }
    }
    impl<T: Debug> Debug for CacheAlign<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            f.debug_struct("CacheAlign")
                .field("align", &Self::ALIGN)
                .field("value", &self.value).finish()
        }
    }
    impl<T: Display> Display for CacheAlign<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            Display::fmt(&self.value, f)
        }
    }
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_sync")))]
    unsafe impl<T: Send> Send for CacheAlign<T> {}
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_sync"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_sync")))]
    unsafe impl<T: Sync> Sync for CacheAlign<T> {}
}
