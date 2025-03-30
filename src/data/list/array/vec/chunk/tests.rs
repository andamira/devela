// devela::data::list::array::vec::chunk

use crate::{FloatConst, String, Vec, VecChunk, vec_ as vec};

#[test]
fn new() {
    let chunk: VecChunk<i32> = VecChunk::default();
    assert!(chunk.is_null());
}

#[test]
fn default() {
    let chunk: VecChunk<i32> = VecChunk::default();
    assert!(chunk.is_null());
}

#[test]
fn is_null() {
    let empty: VecChunk<i32> = VecChunk::default();
    assert!(empty.is_null());

    let non_empty = empty.append(1);
    assert!(!non_empty.is_null());
}

#[test]
fn append() {
    let chunk = VecChunk::default().append(1).append(2).append(3);
    assert_eq!(chunk.as_vec(), vec![1, 2, 3]);

    // Test that original chunk remains unchanged (persistence)
    let chunk1 = VecChunk::default().append(1);
    let chunk2 = chunk1.clone().append(2);
    assert_eq!(chunk1.as_vec(), vec![1]);
    assert_eq!(chunk2.as_vec(), vec![1, 2]);
}

#[test]
fn concat() {
    let chunk1 = VecChunk::default().append(1).append(2);
    let chunk2 = VecChunk::default().append(3).append(4);
    let combined = chunk1.clone().concat(chunk2.clone());

    assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);

    // Test concatenation with empty chunks
    let empty = VecChunk::default();
    assert_eq!(empty.clone().concat(chunk1.clone()).as_vec(), chunk1.as_vec());
    assert_eq!(chunk1.clone().concat(empty.clone()).as_vec(), chunk1.as_vec());
    assert_eq!(empty.clone().concat(empty).as_vec(), Vec::<i32>::new());
}

#[test]
fn as_vec() {
    // Test empty chunk
    let empty: VecChunk<i32> = VecChunk::default();
    assert_eq!(empty.as_vec(), Vec::<i32>::new());

    // Test single element
    let single = VecChunk::default().append(42);
    assert_eq!(single.as_vec(), vec![42]);

    // Test multiple elements
    let multiple = VecChunk::default().append(1).append(2).append(3);
    assert_eq!(multiple.as_vec(), vec![1, 2, 3]);

    // Test complex structure with concatenation
    let chunk1 = VecChunk::default().append(1).append(2);
    let chunk2 = VecChunk::default().append(3).append(4);
    let complex = chunk1.concat(chunk2);
    assert_eq!(complex.as_vec(), vec![1, 2, 3, 4]);
}

#[test]
fn structural_sharing() {
    let chunk1 = VecChunk::default().append(1).append(2);
    let chunk2 = chunk1.clone().append(3);
    let chunk3 = chunk1.clone().append(4);

    // Verify that modifications create new structures while preserving the original
    assert_eq!(chunk1.as_vec(), vec![1, 2]);
    assert_eq!(chunk2.as_vec(), vec![1, 2, 3]);
    assert_eq!(chunk3.as_vec(), vec![1, 2, 4]);
}

#[test]
fn with_different_types() {
    // Test with strings
    let string_chunk =
        VecChunk::default().append(String::from("hello")).append(String::from("world"));
    assert_eq!(string_chunk.as_vec().len(), 2);

    // Test with floating point numbers - using standard constants
    let float_chunk = VecChunk::default().append(f64::PI).append(f64::E);
    assert_eq!(float_chunk.as_vec(), vec![f64::PI, f64::E]);

    // Test with boolean values
    let bool_chunk = VecChunk::default().append(true).append(false).append(true);
    assert_eq!(bool_chunk.as_vec(), vec![true, false, true]);
}

#[test]
fn transform() {
    // Test transform on empty chunk
    let empty: VecChunk<i32> = VecChunk::default();
    let transformed_empty = empty.transform(|x| x * 2);
    assert_eq!(transformed_empty.as_vec(), Vec::<i32>::new());

    // Test transform on single element
    let single = VecChunk::default().append(5);
    let doubled = single.transform(|x| x * 2);
    assert_eq!(doubled.as_vec(), vec![10]);

    // Test transform on multiple elements
    let multiple = VecChunk::default().append(1).append(2).append(3);
    let doubled = multiple.transform(|x| x * 2);
    assert_eq!(doubled.as_vec(), vec![2, 4, 6]);

    // Test transform with string manipulation
    let string_chunk =
        VecChunk::default().append(String::from("hello")).append(String::from("world"));
    let uppercase = string_chunk.transform(|s| s.to_uppercase());
    assert_eq!(uppercase.as_vec(), vec!["HELLO", "WORLD"]);

    // Test chaining multiple transforms
    let numbers = VecChunk::default().append(1).append(2).append(3);
    let result = numbers.transform(|x| x * 2).transform(|x| x + 1).transform(|x| x * 3);
    assert_eq!(result.as_vec(), vec![9, 15, 21]);
}

#[test]
fn transform_flatten() {
    // Test transform_flatten on empty chunk
    let empty: VecChunk<i32> = VecChunk::default();
    let transformed_empty = empty.transform_flatten(|x| VecChunk::new(x * 2));
    assert_eq!(transformed_empty.as_vec(), Vec::<i32>::new());

    // Test transform_flatten on single element
    let single = VecChunk::default().append(5);
    let doubled = single.transform_flatten(|x| VecChunk::new(x * 2));
    assert_eq!(doubled.as_vec(), vec![10]);

    // Test expanding each element into multiple elements
    let numbers = VecChunk::default().append(1).append(2);
    let expanded = numbers.transform_flatten(|x| VecChunk::default().append(x + 1).append(x));
    assert_eq!(expanded.as_vec(), vec![2, 1, 3, 2]);

    // Test with nested chunks
    let chunk = VecChunk::default().append(1).append(2).append(3);
    let nested = chunk.transform_flatten(|x| {
        if x % 2 == 0 {
            // Even numbers expand to [x, x+1]
            VecChunk::default().append(x).append(x + 1)
        } else {
            // Odd numbers expand to [x]
            VecChunk::new(x)
        }
    });
    assert_eq!(nested.as_vec(), vec![1, 2, 3, 3]);

    // Test chaining transform_flatten operations
    let numbers = VecChunk::default().append(1).append(2);
    let result = numbers
        .transform_flatten(|x| VecChunk::default().append(x).append(x))
        .transform_flatten(|x| VecChunk::default().append(x).append(x + 1));
    assert_eq!(result.as_vec(), vec![1, 2, 1, 2, 2, 3, 2, 3]);

    // Test with empty chunk results
    let chunk = VecChunk::default().append(1).append(2);
    let filtered = chunk.transform_flatten(|x| {
        if x % 2 == 0 {
            VecChunk::new(x)
        } else {
            VecChunk::default() // Empty chunk for odd numbers
        }
    });
    assert_eq!(filtered.as_vec(), vec![2]);
}

#[test]
fn prepend() {
    let chunk = VecChunk::default().prepend(1).prepend(2).prepend(3);
    assert_eq!(chunk.as_vec(), vec![3, 2, 1]);

    // Test that original chunk remains unchanged (persistence)
    let chunk1 = VecChunk::default().prepend(1);
    let chunk2 = chunk1.clone().prepend(2);
    assert_eq!(chunk1.as_vec(), vec![1]);
    assert_eq!(chunk2.as_vec(), vec![2, 1]);

    // Test mixing prepend and append
    let mixed = VecChunk::default()
        .prepend(1) // [1]
        .append(2) // [1, 2]
        .prepend(3); // [3, 1, 2]
    assert_eq!(mixed.as_vec(), vec![3, 1, 2]);
}

#[test]
fn from_iterator() {
    // Test collecting from an empty iterator
    let empty_vec: Vec<i32> = vec![];
    let empty_chunk: VecChunk<i32> = empty_vec.into_iter().collect();
    assert!(empty_chunk.is_null());

    // Test collecting from a vector
    let vec = vec![1, 2, 3];
    let chunk: VecChunk<_> = vec.into_iter().collect();
    assert_eq!(chunk.as_vec(), vec![1, 2, 3]);

    // Test collecting from a range
    let range_chunk: VecChunk<_> = (1..=5).collect();
    assert_eq!(range_chunk.as_vec(), vec![1, 2, 3, 4, 5]);

    // Test collecting from map iterator
    let doubled: VecChunk<_> = vec![1, 2, 3].into_iter().map(|x| x * 2).collect();
    assert_eq!(doubled.as_vec(), vec![2, 4, 6]);
}

#[test]
fn concat_optimization() {
    // Create a collected chunk
    let collected: VecChunk<i32> = vec![1, 2, 3].into_iter().collect();

    // Concat a single element
    let result = collected.concat(VecChunk::Single(4));

    // Verify the result
    assert_eq!(result.as_vec(), vec![1, 2, 3, 4]);

    // Verify it's still a Collect variant (not a Concat)
    match result {
        VecChunk::Collect(_) => (), // This is what we want
        _ => panic!("Expected Collect variant after optimization"),
    }
}
