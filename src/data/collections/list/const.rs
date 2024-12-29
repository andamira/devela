// devela::data::collections::list:const
//
// This is a modified version of:
// [`const_list`](https://crates.io/crates/const_list/0.1.0)

use crate::ConstDefault;

/// A linked list node in a `ConstList`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ConstListItem<'a, T: 'a> {
    /// The item represented by this node.
    first: T,
    /// The rest of the list.
    rest: &'a ConstList<'a, T>,
}

/// An immutable, append-only, linear, functional, non-contiguous, list.
///
/// A safe, predictable, and lightweight structure, suitable where immutability
/// is an asset and compile-time guarantees matter more than list manipulation.
///
/// # Example
/// ```
/// # use devela::ConstList;
/// const MY_LIST: ConstList<'static, i32> = ConstList::new()
///     .push(2)
///     .push(4)
///     .push(8);
///
/// assert_eq!(8, *MY_LIST.pop().0.unwrap());
/// ```
#[must_use]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ConstList<'a, T: 'a>(Option<ConstListItem<'a, T>>);

impl<T> ConstDefault for ConstList<'_, T> {
    const DEFAULT: Self = Self::new();
}

impl<'a, T: 'a> ConstList<'a, T> {
    /// Creates a new, empty list.
    pub const fn new() -> Self {
        Self(None)
    }

    /// Gets a reference to the item at the provided index in this list, if any.
    #[must_use]
    pub const fn get(&self, index: usize) -> Option<&T> {
        if let Some(value) = &self.0 {
            if index == 0 {
                Some(&value.first)
            } else {
                value.rest.get(index - 1)
            }
        } else {
            None
        }
    }

    /// Determines the length of this list.
    #[must_use]
    pub const fn len(&self) -> usize {
        if let Some(value) = &self.0 {
            value.rest.len() + 1
        } else {
            0
        }
    }

    /// Whether the list is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// Pushes a new item onto the beginning of this list,
    /// producing a new list head.
    pub const fn push(&'a self, value: T) -> Self {
        ConstList(Some(ConstListItem { first: value, rest: self }))
    }

    /// Removes the first item (if any) from this list, and produces
    /// the rest of the list.
    pub const fn pop(&'a self) -> (Option<&'a T>, &'a Self) {
        if let Some(value) = &self.0 {
            (Some(&value.first), value.rest)
        } else {
            (None, self)
        }
    }

    /// Creates an iterator over the contents of the list.
    pub const fn iter(&self) -> ConstListIterator<T> {
        ConstListIterator { target: self }
    }
}

impl<'a, T> IntoIterator for &'a ConstList<'a, T> {
    type Item = &'a T;
    type IntoIter = ConstListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ConstListIterator { target: self }
    }
}

#[doc = crate::TAG_ITERATOR!()]
/// Iterates over the contents of a `ConstList`.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConstListIterator<'a, T> {
    /// The current list head.
    target: &'a ConstList<'a, T>,
}
impl<'a, T> Iterator for ConstListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.target.pop();
        self.target = rest;
        first
    }
}
