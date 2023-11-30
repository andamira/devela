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
use devela::all::{Any, DstArray, DstValue};

let dst = DstValue::<dyn Any, DstArray<usize, 2>>::new(1234u64, |p| p)
    .ok().expect("Integer did not fit in allocation");
println!("dst as u64 = {:?}", dst.downcast_ref::<u64>());
println!("dst as i8 = {:?}", dst.downcast_ref::<i8>());
```

## Stack-allocated closure!
The following snippet shows how small (`'static`) closures can be returned using
this crate

```rust
use devela::all::{DstArray, DstValue};

fn make_closure(value: u64) -> DstValue<dyn FnMut()->String, DstArray<u64, 2>> {
    DstValue::new(move || format!("Hello there! value={}", value), |p| p as _)
        .ok().expect("Closure doesn't fit")
}
let mut closure = make_closure(666);
assert_eq!( (&mut *closure)(), "Hello there! value=666" );
```

## Custom allocation sizes/types
If you need larger alignment, you can use a different type for the backing array.
(Note, that metadata uses at least one slot in the array)

This code panics, because i128 requires 8/16 byte alignment (usually)
```should_panic
use devela::all::{Any, DstArray, DstValue};

let v: DstValue<dyn Any, DstArray<u8, 32>> =
    DstValue::new(123i128, |p| p as _).unwrap();
```

This works, because the backing buffer has sufficient alignment
```rust
use devela::all::{Any, DstArray, DstValue};

let v: DstValue<dyn Any, DstArray<u128, 2>> =
    DstValue::new(123i128, |p| p as _).unwrap();
```
