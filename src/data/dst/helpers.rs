// helper functions

use crate::_dep::bytemuck::Pod;
use core::{mem, ptr, slice};

type BufSlice<T> = [mem::MaybeUninit<T>];

pub(crate) fn decompose_pointer<T: ?Sized>(mut ptr: *const T) -> (*const (), usize, [usize; 3]) {
    let addr = ptr as *const ();
    let rv = mem_as_slice(&mut ptr);
    let mut vals = [0; 3];
    assert!(
        rv[0] == addr as usize,
        "BUG: Pointer layout is not (data_ptr, info...)"
    );
    vals[..rv.len() - 1].copy_from_slice(&rv[1..]);
    (addr, rv.len() - 1, vals)
}

pub(crate) fn mem_as_slice<T>(ptr: &mut T) -> &mut [usize] {
    assert!(mem::size_of::<T>() % mem::size_of::<usize>() == 0);
    assert!(mem::align_of::<T>() % mem::align_of::<usize>() == 0);
    let words = mem::size_of::<T>() / mem::size_of::<usize>();
    // SAFETY: Points to valid memory (a raw pointer)
    unsafe { slice::from_raw_parts_mut(ptr as *mut _ as *mut usize, words) }
}

/// Re-construct a fat pointer
pub(crate) unsafe fn make_fat_ptr<T: ?Sized, W: Pod>(
    data_ptr: *mut (),
    meta_vals: &BufSlice<W>,
) -> *mut T {
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Raw {
        ptr: *const (),
        meta: [usize; 4],
    }
    union Inner<T: ?Sized> {
        ptr: *mut T,
        raw: Raw,
    }
    let mut rv = Inner {
        raw: Raw {
            ptr: data_ptr,
            meta: [0; 4],
        },
    };
    assert!(meta_vals.len() * mem::size_of::<W>() % mem::size_of::<usize>() == 0);
    assert!(meta_vals.len() * mem::size_of::<W>() <= 4 * mem::size_of::<usize>());
    ptr::copy(
        meta_vals.as_ptr() as *const u8,
        rv.raw.meta.as_mut_ptr() as *mut u8,
        meta_vals.len() * mem::size_of::<W>(),
    );
    let rv = rv.ptr;
    assert_eq!(rv as *const (), data_ptr as *const ());
    rv
}

/// Write metadata (abstraction around `ptr::copy`)
pub(crate) fn store_metadata<W: Pod>(dst: &mut BufSlice<W>, meta_words: &[usize]) {
    let n_bytes = core::mem::size_of_val(meta_words);
    assert!(
        n_bytes <= dst.len() * mem::size_of::<W>(),
        "nbytes [{}] <= dst.len() [{}] * sizeof [{}]",
        n_bytes,
        dst.len(),
        mem::size_of::<W>()
    );
    unsafe {
        ptr::copy(
            meta_words.as_ptr() as *const u8,
            dst.as_mut_ptr() as *mut u8,
            n_bytes,
        );
    }
}

pub(crate) fn round_to_words<T>(len: usize) -> usize {
    (len + mem::size_of::<T>() - 1) / mem::size_of::<T>()
}

/// Calls a provided function to get a fat pointer version of `v` (and checks that the returned pointer is sane)
pub(crate) fn check_fat_pointer<U, T: ?Sized>(v: &U, get_ref: impl FnOnce(&U) -> &T) -> &T {
    let ptr: &T = get_ref(v);
    assert_eq!(
        ptr as *const _ as *const u8, v as *const _ as *const u8,
        "MISUSE: Closure returned different pointer"
    );
    assert_eq!(
        mem::size_of_val(ptr),
        mem::size_of::<U>(),
        "MISUSE: Closure returned a subset pointer"
    );
    ptr
}

/// Push items to a list using a generator function to get the items
/// - `meta`  - Metadata slot (must be 1 usize long)
/// - `data`  - Data slot, must be at least `count * sizeof(T)` long
/// - `count` - Number of items to insert
/// - `gen`   - Generator function (is passed the current index)
/// - `reset_slot` - A slot updated with `reset_value` when a panic happens before push is complete
/// - `reset_value` - Value used in `reset_slot`
///
/// This provides a panic-safe push as long as `reset_slot` and `reset_value` undo the allocation operation
pub(crate) unsafe fn list_push_gen<T, W: Pod>(
    meta: &mut BufSlice<W>,
    data: &mut BufSlice<W>,
    count: usize,
    mut gen: impl FnMut(usize) -> T,
    reset_slot: &mut usize,
    reset_value: usize,
) {
    /// Helper to drop/zero all pushed items, and reset data structure state if there's a panic
    struct PanicState<'a, T>(*mut T, usize, &'a mut usize, usize);
    impl<'a, T> core::ops::Drop for PanicState<'a, T> {
        fn drop(&mut self) {
            if self.0.is_null() {
                return;
            }
            // Reset the state of the data structure (leaking items)
            *self.2 = self.3;
            // Drop all partially-populated items
            unsafe {
                while self.1 != 0 {
                    ptr::drop_in_place(&mut *self.0);
                    ptr::write_bytes(self.0 as *mut u8, 0, mem::size_of::<T>());
                    self.0 = self.0.offset(1);
                    self.1 -= 1;
                }
            }
        }
    }

    let mut ptr = data.as_mut_ptr() as *mut T;
    let mut clr = PanicState(ptr, 0, reset_slot, reset_value);
    for i in 0..count {
        let val = gen(i);
        ptr::write(ptr, val);
        ptr = ptr.offset(1);
        clr.1 += 1;
    }
    // Prevent drops and prevent reset
    clr.0 = ptr::null_mut();
    // Save the length once everything has been written
    store_metadata(meta, &[count]);
}
