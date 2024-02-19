This module adds support for storing dynamically-sized types within fixed-size
allocations

- The `DstValue` type provides a fixed size (7 word in the current version)
  buffer in which a trait object or array can be stored, without resorting
  to a heap allocation.
- The `DstQueue` and `DstStack` types provide collection types
  (first-in-first-out and last-in-first-out).

# Examples

## An unboxed any
As a quick example - The following wraps a 64-bit integer up in an inline DST
using the `Any` trait.

```rust
# use devela::all::{Any, DstArray, DstValue};
let dst = DstValue::<dyn Any, DstArray<usize, 2>>::new(1234u64, |p| p)
    .ok().expect("Integer did not fit in allocation");
println!("dst as u64 = {:?}", dst.downcast_ref::<u64>());
println!("dst as i8 = {:?}", dst.downcast_ref::<i8>());
```

## Stack-allocated closures
The following snippet shows how small (`'static`) closures can be returned using
this crate

```rust
# use devela::all::{DstArrayUsize, DstQueue, DstValue};
let c = DstValue::<dyn Fn(i32) -> i32, DstArrayUsize<1>>::new(|n| n * 3, |v| v)
    .ok().expect("Not enough size!");
assert_eq!(c(6), 18);

let mut q = DstQueue::<dyn Fn(i32) -> i32, DstArrayUsize<8>>::new();
assert![q.push_back(|n| n * 3, |v| v).is_ok()];
assert![q.push_back(|n| n - 3, |v| v).is_ok()];
assert_eq![18, q.front().unwrap()(6)];
assert_eq![21, q.pop_front().unwrap()(7)];
assert_eq![5, q.pop_front().unwrap()(8)];
```

## Custom allocation sizes/types
If you need larger alignment, you can use a different type for the backing array.
(Note, that metadata uses at least one slot in the array)

This code panics, because i128 requires 8/16 byte alignment (usually)
```should_panic
# use devela::all::{Any, DstArray, DstValue};
let v: DstValue<dyn Any, DstArray<u8, 32>> =
    DstValue::new(123i128, |p| p as _).unwrap();
```

This works, because the backing buffer has sufficient alignment
```rust
# use devela::all::{Any, DstArray, DstValue};
let v: DstValue<dyn Any, DstArray<u128, 2>> =
    DstValue::new(123i128, |p| p as _).unwrap();
```
