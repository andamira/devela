#!/usr/bin/env -S rust-script -c --debug
//! ```cargo
//! [dependencies]
//! devela = { path = "../../../../devela", features = ["std"]}
//! ```
//!
//! Defines the [`Current`] and [`CurrentGuard`] structs.
//

use ::devela::{
    Any, Deref, DerefMut, Hash, Mem, PhantomData, PtrNonNull, RefCell, any_type_name,
    define_static_map, transmute,
};

// Stores the current pointers for concrete types.
define_static_map![typeid KeyCurrentMap];
thread_local! {
    static CURRENT_PTR_MAP: RefCell<KeyCurrentMap<u64, PtrNonNull<u8>, 64>>
        = RefCell::new(KeyCurrentMap::new_cloned(PtrNonNull::<u8>::dangling()));
}

#[doc = crate::_TAG_GUARD!()]
/// A guard that temporarily sets a global current pointer for `T`, restoring the old one on drop.
///
/// When dropped, it restores the previous pointer or sets a placeholder if none existed."
///
/// This is useful for tracking the current instance of a type within a thread.
#[cfg_attr(doc, doc = ::devela::_doc!(vendor: "current"))]
#[derive(Debug)]
pub struct CurrentGuard<'a, T: Any> {
    /// The active instance of `T` for the duration of this guard.
    _current: &'a mut T,
    /// The previous pointer, restored when this guard is dropped.
    prev_ptr: Option<PtrNonNull<T>>,
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
    pub fn new(current: &mut T) -> CurrentGuard<'_, T> {
        let ptr = PtrNonNull::from(&mut *current).cast::<u8>();
        let prev_ptr = CURRENT_PTR_MAP.with(|current| {
            let mut map = current.borrow_mut();
            if let Some(entry) = map.get_mut_type::<T>() {
                Some(Mem::replace(entry, ptr).cast::<T>())
            } else {
                map.insert_type::<T>(ptr.cast()).ok();
                None
            }
        });
        CurrentGuard { prev_ptr, _current: current }
    }
}
impl<T: Any> Drop for CurrentGuard<'_, T> {
    fn drop(&mut self) {
        CURRENT_PTR_MAP.with(|current| {
            let mut map = current.borrow_mut();
            match self.prev_ptr {
                None => map.insert_type::<T>(PtrNonNull::<u8>::dangling().cast()).ok(),
                Some(prev_ptr) => map.insert_type::<T>(prev_ptr.cast()).ok(),
            }
        });
    }
}

#[doc = crate::_TAG_GUARD!()]
/// A marker object representing the current instance of a type `T`.
///
/// This struct does not hold any actual value but instead allows access to
/// a globally tracked instance of `T`, typically managed through `CurrentGuard`.
///
/// The primary purpose of `Current<T>` is to:
/// - Provide safe, structured access to a global instance of `T`.
/// - Prevent direct global mutable access in safe code.
///
/// Not a smart pointer; instead, it acts as a reference handle for thread-local state.
#[cfg_attr(doc, doc = ::devela::_doc!(vendor: "current"))]
#[derive(Debug)]
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
        let ptr: PtrNonNull<u8> = CURRENT_PTR_MAP.with(|current| current.borrow().get_type::<T>())?;
        // SAFETY: The pointer is non-null but may not be valid; ensure `CurrentGuard<T>` is active.
        Some(unsafe { &mut *ptr.cast::<T>().as_ptr() })
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
        // SAFETY: Panics if no `CurrentGuard<T>` exists, preventing invalid access.
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
        // - `Current<T>` is only an access point, not an actual value.
        // - Transmuting `&Current<T>` to `&mut Current<T>` is safe since it's never directly used.
        // - Caller must ensure `CurrentGuard<T>` is active; otherwise, `current_unwrap` panics.
        #[allow(mutable_transmutes, reason = "`Current` only acts as a reference handle")]
        unsafe { transmute::<&Current<T>, &'a mut Current<T>>(self).current_unwrap() }
    }
}
impl<T: Any> DerefMut for Current<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY:
        // - Requires an active `CurrentGuard<T>`, ensuring a valid instance.
        // - If no guard exists, `current_unwrap` panics instead of returning an invalid reference.
        unsafe { self.current_unwrap() }
    }
}

#[allow(unused, reason = "example script")]
fn main() {
    struct State {
        text: String,
    }
    impl State {
        // prints the current text, and changes it to "world!"
        fn print() {
            let mut ctx = unsafe { Current::<State>::new() };
            println!("{}", ctx.text);
            ctx.text = "world!".to_string();
        }
        // changes the text to "good bye" and calls print() two times.
        fn bar() {
            let mut bar = State { text: "good bye".to_string() };
            let guard = CurrentGuard::new(&mut bar);
            State::print(); // good bye
            State::print(); // world!
        }
    }
    let mut ctx = State { text: "hello".to_string() };
    let guard = CurrentGuard::new(&mut ctx);
    State::print(); // hello
    State::print(); // world!
    State::bar(); // good bye world!
}
