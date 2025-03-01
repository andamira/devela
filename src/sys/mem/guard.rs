#!/usr/bin/env -S rust-script -c
//! ```cargo
//! [dependencies]
//! devela = { path = "../../../../devela", features = ["std"]}
//! ```
//!
//! Defines the [`Current`] and [`CurrentGuard`]
//

use ::devela::{
    any_type_name, define_static_map, transmute, Any, Deref, DerefMut, Hash, Hasher, HasherFx, Mem,
    PhantomData, RefCell,
};

define_static_map![typeid KeyCurrentMap];

// Stores the current pointers for concrete types.
thread_local! {
    static CURRENT_PTR_MAP: RefCell<KeyCurrentMap<u64, usize, 64>>
        = RefCell::new(KeyCurrentMap::new());
}

/// A guard that temporarily sets a global current pointer for a given type.
///
/// When dropped, it restores the previous pointer (or removes it if none existed).
///
/// This is useful for tracking the current instance of a type within a thread.
#[cfg_attr(doc, doc = ::devela::doc_!(vendor: "current"))]
pub struct CurrentGuard<'a, T: Any> {
    _val: &'a mut T,
    old_ptr: Option<usize>,
}
impl<T: Any> CurrentGuard<'_, T> {
    /// Creates a new *current guard* for the given value.
    ///
    /// When this guard is dropped, the previous pointer (if any) will be restored.
    ///
    /// # Safety
    /// - This function modifies global state within a thread-local map.
    /// - It ensures that only one mutable reference to a given `T` exists at a time.
    /// - Improper use may lead to stale pointers if lifetimes are not respected.
    ///
    /// # Example
    /// ```
    /// # use devela::{CurrentGuard};
    /// # struct MyType { data: u64} impl MyType { fn new() -> Self { Self { data:0 } }}
    /// let mut my_value = MyType::new();
    /// let guard = CurrentGuard::new(&mut my_value);
    /// ```
    pub fn new(val: &mut T) -> CurrentGuard<T> {
        let ptr = val as *mut T as usize;
        let old_ptr = CURRENT_PTR_MAP.with(|current| {
            let mut map = current.borrow_mut();
            if let Some(entry) = map.get_mut_type::<T>() {
                Some(Mem::replace(entry, ptr))
            } else {
                map.insert_type::<T>(ptr).ok();
                None
            }
        });
        CurrentGuard { old_ptr, _val: val }
    }
}
impl<T: Any> Drop for CurrentGuard<'_, T> {
    fn drop(&mut self) {
        CURRENT_PTR_MAP.with(|current| {
            let mut map = current.borrow_mut();
            match self.old_ptr {
                None => {
                    map.remove_type::<T>();
                }
                Some(old_ptr) => {
                    if let Some(entry) = map.get_mut_type::<T>() {
                        *entry = old_ptr;
                    } else {
                        map.insert_type::<T>(old_ptr).ok();
                    }
                }
            }
        });
    }
}

/// A marker object representing the current instance of a type `T`.
///
/// This struct does not hold any actual value but instead allows access to
/// a globally tracked instance of `T`, typically managed through `CurrentGuard`.
///
/// The primary purpose of `Current<T>` is to:
/// - Provide safe, structured access to a global instance of `T`.
/// - Prevent direct global mutable access in safe code.
///
/// This is not a normal smart pointer, as it does not own the value.
/// Instead, it interacts with thread-local state.
#[cfg_attr(doc, doc = ::devela::doc_!(vendor: "current"))]
pub struct Current<T>(PhantomData<T>);

impl<T: Any> Current<T> {
    /// Creates a new `Current<T>` marker object.
    ///
    /// # Safety
    /// - This function does not initialize an actual value.
    /// - Dereferencing without an active `CurrentGuard` leads to undefined behavior.
    /// - Ensure that `CurrentGuard` is properly managed before using this.
    ///
    /// # Example
    /// ```ignore
    /// let current = unsafe { Current::<MyType>::new() };
    /// ```
    pub unsafe fn new() -> Current<T> {
        Current(PhantomData)
    }

    /// Retrieves an exclusive reference to the current instance of `T`, if set.
    ///
    /// # Safety
    /// - May return a dangling reference if the associated `CurrentGuard<T>` has been dropped.
    /// - The caller must ensure the reference is only used while the `CurrentGuard<T>` is alive.
    /// - If the same `T` is set multiple times, the reference may be stale.
    ///
    /// # Example
    /// ```ignore
    /// let mut my_value = MyType::new();
    /// let guard = CurrentGuard::new(&mut my_value);
    /// let current = unsafe { Current::<MyType>::new().current() };
    /// assert!(current.is_some());
    /// ```
    #[rustfmt::skip]
    pub unsafe fn current(&mut self) -> Option<&mut T> {
        let ptr: Option<usize> = CURRENT_PTR_MAP.with(|current| current.borrow().get_type::<T>());
        let ptr = match ptr {
            None => { return None; }
            Some(x) => x,
        };
        // SAFETY: Caller must ensure `T` is still valid.
        Some(unsafe { &mut *(ptr as *mut T) })
    }

    /// Retrieves an exclusive reference to the current instance of `T`, or panics.
    ///
    /// # Safety
    /// Same as [`current`][Self::current]: the caller must ensure the reference remains valid.
    ///
    /// # Panics
    /// Panics if no instance of `T` is currently set.
    ///
    /// # Example
    /// ```ignore
    /// let mut my_value = MyType::new();
    /// let guard = CurrentGuard::new(&mut my_value);
    /// let current = unsafe { Current::<MyType>::new().current_unwrap() };
    /// ```
    pub unsafe fn current_unwrap(&mut self) -> &mut T {
        // SAFETY: Only safe if a current instance of `T` exists.
        match unsafe { self.current() } {
            None => panic!("No current `{}` is set", any_type_name::<T>()),
            Some(x) => x,
        }
    }
}
impl<T: Any> Deref for Current<T> {
    type Target = T;

    #[inline(always)] #[rustfmt::skip]
    fn deref<'a>(&'a self) -> &'a T {
        // SAFETY:
        // - `Current` does not contain any actual value.
        // - It is safe to transmute `&Current<T>` to `&mut Current<T>` because it only acts
        //   as an access point for the global instance.
        // WARNING:
        // - This must only be called when `CurrentGuard<T>` is active.
        // - If no instance is set, `current_unwrap()` will panic.
        #[allow(mutable_transmutes, reason = "`Current` does not contain anything")]
        unsafe { transmute::<&Current<T>, &'a mut Current<T>>(self).current_unwrap() }
    }
}
impl<T: Any> DerefMut for Current<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY:
        // - The caller must ensure that a `CurrentGuard<T>` exists.
        // - If no guard is active, `current_unwrap()` will panic.
        unsafe { self.current_unwrap() }
    }
}

/* example script */

#[allow(unused, reason = "example script")]
fn main() {
    struct State {
        text: String,
    }
    impl State {
        fn print() {
            let ctx = unsafe { &*Current::<State>::new() };
            println!("{}", ctx.text);
            // E0716: temporary-value-dropped-while-borrowed
            // unsafe { &mut *Current::<State>::new() }.text = "world!".to_string();
            let ctx2 = unsafe { &mut *Current::<State>::new() };
            ctx2.text = "world!".to_string();
        }
        fn bar() {
            let mut bar = State { text: "good bye".to_string() };
            let guard = CurrentGuard::new(&mut bar);
            State::print();
            State::print();
            drop(guard);
        }
    }
    let mut ctx = State { text: "hello".to_string() };
    {
        let guard = CurrentGuard::new(&mut ctx);
        State::print();
        State::print();
        State::bar();
        drop(guard);
    }
    ctx.text = "hi!".to_string();
}
