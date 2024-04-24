// devela::mem::pin
//
//!
//

use super::Pin;

/// A wrapper for structurally pinned data.
///
/// Up to 8 generics can be supplied for 8 structurally pinned fields.
#[rustfmt::skip]
pub struct Pinned<A, B = (), C = (), D = (), E = (), F = (), G = (), H = ()> {
    a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H
}

// Constructors
impl<A> Pinned<A> {
    /// Create a new `Pinned` with a single field
    #[inline] #[rustfmt::skip]
    pub fn from_a(a: A) -> Self {
        Pinned { a, b: (), c: (), d: (), e: (), f: (), g: (), h: () }
    }
    /// Add field `B` to a `Pinned`
    #[inline] #[rustfmt::skip]
    pub fn with_b<B>(self, b: B) -> Pinned<A, B> {
        Pinned { a: self.a,
            b, c: (), d: (), e: (), f: (), g: (), h: ()
        }
    }
}
impl<A, B> Pinned<A, B> {
    /// Add field `C` to a `Pinned`
    #[inline] #[rustfmt::skip]
    pub fn with_c<C>(self, c: C) -> Pinned<A, B, C> {
        Pinned {
            a: self.a, b: self.b,
            c, d: (), e: (), f: (), g: (), h: ()
        }
    }
}
impl<A, B, C> Pinned<A, B, C> {
    /// Add field `D` to a `Pinned`
    #[inline] #[rustfmt::skip]
    pub fn with_d<D>(self, d: D) -> Pinned<A, B, C, D> {
        Pinned {
            a: self.a, b: self.b, c: self.c,
            d, e: (), f: (), g: (), h: (),
        }
    }
}
impl<A, B, C, D> Pinned<A, B, C, D> {
    /// Add field `E` to a `Pinned`
    #[inline] #[rustfmt::skip]
    pub fn with_e<E>(self, e: E) -> Pinned<A, B, C, D, E> {
        Pinned {
            a: self.a, b: self.b, c: self.c, d: self.d,
            e, f: (), g: (), h: ()
        }
    }
}
impl<A, B, C, D, E> Pinned<A, B, C, D, E> {
    /// Add field `F` to a `Pinned`
    #[inline] #[rustfmt::skip]
    pub fn with_f<F>(self, f: F) -> Pinned<A, B, C, D, E, F> {
        Pinned {
            a: self.a, b: self.b, c: self.c, d: self.d, e: self.e,
            f, g: (), h: (),
        }
    }
}
impl<A, B, C, D, E, F> Pinned<A, B, C, D, E, F> {
    /// Add field `G` to a `Pinned`
    #[inline] #[rustfmt::skip]
    pub fn with_g<G>(self, g: G) -> Pinned<A, B, C, D, E, F, G> {
        Pinned {
            a: self.a, b: self.b, c: self.c, d: self.d, e: self.e, f: self.f,
            g, h: (),
        }
    }
}
impl<A, B, C, D, E, F, G> Pinned<A, B, C, D, E, F, G> {
    /// Add field `H` to a `Pinned`
    #[inline] #[rustfmt::skip]
    pub fn with_h<H>(self, h: H) -> Pinned<A, B, C, D, E, F, G, H> {
        Pinned {
            a: self.a, b: self.b, c: self.c, d: self.d, e: self.e, f: self.f, g: self.g,
            h,
        }
    }
}

// Getters
impl<A, B, C, D, E, F, G, H> Pinned<A, B, C, D, E, F, G, H> {
    /// Get a `Pin<&mut A>`
    pub fn a(self: Pin<&mut Self>) -> Pin<&mut A> {
        // SAFETY: This is okay because `a` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.a) }
    }
    /// Get a `Pin<&mut B>`
    pub fn b(self: Pin<&mut Self>) -> Pin<&mut B> {
        // SAFETY: This is okay because `b` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.b) }
    }
    /// Get a `Pin<&mut C>`
    pub fn c(self: Pin<&mut Self>) -> Pin<&mut C> {
        // SAFETY: This is okay because `c` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.c) }
    }
    /// Get a `Pin<&mut D>`
    pub fn d(self: Pin<&mut Self>) -> Pin<&mut D> {
        // SAFETY: This is okay because `d` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.d) }
    }
    /// Get a `Pin<&mut E>`
    pub fn e(self: Pin<&mut Self>) -> Pin<&mut E> {
        // SAFETY: This is okay because `e` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.e) }
    }
    /// Get a `Pin<&mut F>`
    pub fn f(self: Pin<&mut Self>) -> Pin<&mut F> {
        // SAFETY: This is okay because `f` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.f) }
    }
    /// Get a `Pin<&mut G>`
    pub fn g(self: Pin<&mut Self>) -> Pin<&mut G> {
        // SAFETY: This is okay because `g` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.g) }
    }
    /// Get a `Pin<&mut H>`
    pub fn h(self: Pin<&mut Self>) -> Pin<&mut H> {
        // SAFETY: This is okay because `h` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|this| &mut this.h) }
    }
}
