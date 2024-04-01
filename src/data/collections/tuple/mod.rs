// devela::data::collections::tuple
//
//!
//

use core::fmt;

/// A formatting wrapper for [`ExtTuple`]s, implementing `Display` and `Debug`.
#[repr(transparent)]
pub struct TupleFmt<'a, T: ExtTuple>(&'a T);

// Private traits for tuples with elements that implement Debug or Display.
trait TupleDebug: ExtTuple {
    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
trait TupleDisplay: ExtTuple {
    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<'a, T: TupleDebug> fmt::Debug for TupleFmt<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_debug(f)
    }
}
impl<'a, T: TupleDisplay> fmt::Display for TupleFmt<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_display(f)
    }
}

// Marker trait to prevent downstream implementations of the `ExtTuple` trait.
#[rustfmt::skip]
mod private { pub trait Sealed {} }

/// Extension trait providing convenience methods for [tuples][tuple].
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// # Features
/// By default it's implemented for tuples of arity of 15 or less.
/// It supports increased arities of 31, 63, 95 and 127 by enabling the
/// corresponding feature: `_tuple_arity_[31|63|95|127]`,
/// which in turn will increase compilation times.
pub trait ExtTuple: private::Sealed {
    /// The first element of this tuple.
    type Head;
    /// The last element of this tuple.
    type Tail;
    /// This tuple without its head.
    type NoHead;
    /// This tuple without its tail.
    type NoTail;
    /// This tuple with an extra element `T` appended to it.
    type Append<T>;
    /// This tuple with an extra element `T` prepended to it.
    type Prepend<T>;

    /// The arity of this tuple (the number of contained elements).
    const ARITY: usize;

    /// The maximum arity supported by the current compilation options.
    ///
    /// See the available [features](#features) to increase this number.
    #[rustfmt::skip] const MAX_ARITY: usize = {
        if cfg!(not(feature = "_tuple_arity_31")) { 15
        } else if cfg!(all(feature = "_tuple_arity_31", not(feature = "_tuple_arity_63"))) { 31
        } else if cfg!(all(feature = "_tuple_arity_63", not(feature = "_tuple_arity_95"))) { 63
        } else if cfg!(all(feature = "_tuple_arity_95", not(feature = "_tuple_arity_127"))) { 95
        } else { 127 }
    };

    /// The arity of this tuple (the number of contained elements).
    #[inline]
    #[must_use]
    fn arity(&self) -> usize {
        Self::ARITY
    }

    /// Wraps the tuple in a [`TupleFmt`] for formatting purposes.
    #[rustfmt::skip]
    fn fmt(&self) -> TupleFmt<Self> where Self: Sized { TupleFmt(self) }

    /// Returns a shared reference to the head of this tuple.
    #[must_use]
    fn head(&self) -> &Self::Head;
    /// Returns a shared reference to the tail of this tuple.
    #[must_use]
    fn tail(&self) -> &Self::Tail;
    /// Returns an exclusive reference to the head of this tuple.
    #[must_use]
    fn head_mut(&mut self) -> &mut Self::Head;
    /// Returns an exclusive reference to the tail of this tuple.
    #[must_use]
    fn tail_mut(&mut self) -> &mut Self::Tail;

    /// Returns this tuple with the head element splitted from the rest.
    #[must_use]
    fn split_head(self) -> (Self::Head, Self::NoHead);
    /// Returns this tuple with the tail element splitted from the rest.
    #[must_use]
    fn split_tail(self) -> (Self::NoTail, Self::Tail);

    /// Returns this tuple without the head.
    #[must_use]
    fn no_head(self) -> Self::NoHead;
    /// Returns this tuple without the head.
    #[must_use]
    fn no_tail(self) -> Self::NoTail;

    /// Appends the given `value` to this tuple.
    #[must_use]
    fn append<T>(self, value: T) -> Self::Append<T>;
    /// Prepends the given `value` to this tuple.
    #[must_use]
    fn prepend<T>(self, value: T) -> Self::Prepend<T>;
}

// Manual impls for arities <= 2 of: ExtTuple, TupleDebug, TupleDisplay.
#[rustfmt::skip]
mod manual_impls {
    use super::{ExtTuple, private::Sealed, TupleDebug, TupleDisplay};
    use core::fmt;

    // arity 0
    impl Sealed for () {}
    impl ExtTuple for () {
        const ARITY: usize = 0;
        type Head = ();
        type Tail = ();
        type NoHead = ();
        type NoTail = ();
        type Append<T> = (T,);
        type Prepend<T> = (T,);
        fn head(&self) -> &Self::Head { self }
        fn tail(&self) -> &Self::Tail { self }
        fn head_mut(&mut self) -> &mut Self::Head { self }
        fn tail_mut(&mut self) -> &mut Self::Tail { self }
        fn split_head(self) -> (Self::Head, Self::NoHead) { ((), ()) }
        fn split_tail(self) -> (Self::NoTail, Self::Tail) { ((), ()) }
        fn no_head(self) -> Self::NoHead {}
        fn no_tail(self) -> Self::NoTail {}
        fn append<T>(self, value: T) -> Self::Append<T> { (value,) }
        fn prepend<T>(self, value: T) -> Self::Prepend<T> { (value,) }
    }
    impl TupleDebug for () {
        fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("").finish()
        }
    }
    impl TupleDisplay for () {
        fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "()")
        }
    }

    // arity 1
    impl<HEADTAIL> Sealed for (HEADTAIL,) {}
    impl<HEADTAIL> ExtTuple for (HEADTAIL,) {
        const ARITY: usize = 1;
        type Head = HEADTAIL;
        type Tail = HEADTAIL;
        type NoHead = ();
        type NoTail = ();
        type Append<T> = (HEADTAIL, T);
        type Prepend<T> = (T, HEADTAIL);
        fn head(&self) -> &Self::Head { &self.0 }
        fn tail(&self) -> &Self::Tail { &self.0 }
        fn head_mut(&mut self) -> &mut Self::Head { &mut self.0 }
        fn tail_mut(&mut self) -> &mut Self::Tail { &mut self.0 }
        fn split_head(self) -> (Self::Head, Self::NoHead) { (self.0, ()) }
        fn split_tail(self) -> (Self::NoTail, Self::Tail) { ((), self.0) }
        fn no_head(self) -> Self::NoHead {}
        fn no_tail(self) -> Self::NoTail {}
        fn append<T>(self, value: T) -> Self::Append<T> { (self.0, value) }
        fn prepend<T>(self, value: T) -> Self::Prepend<T> { (value, self.0) }
    }
    impl<HEADTAIL: fmt::Debug> TupleDebug for (HEADTAIL,) {
        fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("").field(&self.0).finish()
        }
    }
    impl<HEADTAIL: fmt::Display> TupleDisplay for (HEADTAIL,) {
        fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({},)", &self.0)
        }
    }

    // arity 2
    impl<HEAD, TAIL> super::private::Sealed for (HEAD, TAIL) {}
    impl<HEAD, TAIL> ExtTuple for (HEAD, TAIL) {
        const ARITY: usize = 2;
        type Head = HEAD;
        type Tail = TAIL;
        type NoHead = (TAIL,);
        type NoTail = (HEAD,);
        type Append<T> = (HEAD, TAIL, T);
        type Prepend<T> = (T, HEAD, TAIL);
        fn head(&self) -> &Self::Head { &self.0 }
        fn tail(&self) -> &Self::Tail { &self.1 }
        fn head_mut(&mut self) -> &mut Self::Head { &mut self.0 }
        fn tail_mut(&mut self) -> &mut Self::Tail { &mut self.1 }
        fn split_head(self) -> (Self::Head, Self::NoHead) { (self.0, (self.1,)) }
        fn split_tail(self) -> (Self::NoTail, Self::Tail) { ((self.0,), self.1) }
        fn no_head(self) -> Self::NoHead { (self.1,) }
        fn no_tail(self) -> Self::NoTail { (self.0,) }
        fn append<T>(self, value: T) -> Self::Append<T> { (self.0, self.1, value) }
        fn prepend<T>(self, value: T) -> Self::Prepend<T> { (value, self.0, self.1) }
    }
    impl<HEAD: fmt::Debug, TAIL: fmt::Debug> TupleDebug for (HEAD, TAIL) {
        fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("").field(&self.0).field(&self.1).finish()
        }
    }
    impl<HEAD: fmt::Display, TAIL: fmt::Display> TupleDisplay for (HEAD, TAIL) {
        fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", &self.0, &self.1)
        }
    }
}

// Helper macro to implement ExtTuple for arities >2
//
// $t1: the first generic argument after the HEAD
// $tn: the rest of the generic arguments before the TAIL
macro_rules! impl_tuple {
	() => {};
	($t1:ident $(, $tn:ident )*) => {
		impl<HEAD, $t1 $(, $tn)*, TAIL> private::Sealed for (HEAD, $t1, $($tn,)* TAIL) {}
        #[allow(non_snake_case)]
		impl<HEAD, $t1 $(, $tn)*, TAIL> ExtTuple for (HEAD, $t1, $($tn,)* TAIL) {
			const ARITY: usize = { $crate::code::ident_total![$($tn)*] + 3};

            type Head = HEAD;
            type Tail = TAIL;
			type NoHead = ($t1, $($tn,)* TAIL,);
			type NoTail = (HEAD, $t1, $($tn,)*);
            type Append<APPEND> = (HEAD, $t1, $($tn,)* TAIL, APPEND);
            type Prepend<PREPEND> = (PREPEND, HEAD, $t1, $($tn,)* TAIL);

            fn head(&self) -> &Self::Head { &self.0 }
            fn tail(&self) -> &Self::Tail { let (.., tail) = self; tail }
			fn head_mut(&mut self) -> &mut Self::Head { &mut self.0 }
            fn tail_mut(&mut self) -> &mut Self::Tail { let (.., tail) = self; tail }

			fn split_head(self) -> (Self::Head, Self::NoHead) {
				let (head, $t1, $($tn,)* tail) = self; (head, ($t1, $($tn,)* tail))
			}
			fn split_tail(self) -> (Self::NoTail, Self::Tail) {
				let (head, $t1, $($tn,)* tail) = self; ((head, $t1 $(,$tn)*), tail)
			}
			fn no_head(self) -> Self::NoHead {
				let (_, $t1, $($tn,)* tail) = self; ($t1, $($tn,)* tail,)
            }
			fn no_tail(self) -> Self::NoTail {
				let (head, $t1, $($tn,)* _) = self; (head, $t1, $($tn,)*)
            }
			fn append<APPEND>(self, value: APPEND) -> Self::Append<APPEND> {
				let (head, $t1, $($tn,)* tail) = self; (head, $t1, $($tn,)* tail, value)
			}
			fn prepend<PREPEND>(self, value: PREPEND) -> Self::Prepend<PREPEND> {
				let (head, $t1, $($tn,)* tail) = self; (value, head, $t1, $($tn,)* tail)
			}
		}
		impl_tuple!($($tn),*);
    };
}
#[rustfmt::skip] // max arity 15 (= 13 + 2); (16 tuples including ())
#[cfg(not(feature = "_tuple_arity_31"))]
impl_tuple![M,L,K,J,I,H,G,F,E,D,C,B,A];

#[rustfmt::skip] // max arity 31 (= 26 * 1 + 3 + 2); (32 tuples including ())
#[cfg(all(feature = "_tuple_arity_31", not(feature = "_tuple_arity_63")))]
impl_tuple![
    AC,AB,AA, // + 3
    Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A // 26 * 1
];
#[rustfmt::skip] // max arity 63 (= 26 * 2 + 9 + 2); (64 tuples including ())
#[cfg(all(feature = "_tuple_arity_63", not(feature = "_tuple_arity_95")))]
impl_tuple![
    BI,BH,BG,BF,BE,BD,BC,BB,BA, // + 9
    AZ,AY,AX,AW,AV,AU,AT,AS,AR,AQ,AP,AO,AN,AM,AL,AK,AJ,AI,AH,AG,AF,AE,AD,AC,AB,AA, // 26 * 2
    Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A
];
#[rustfmt::skip] // max arity 95 (= 26 * 3 + 15 + 2); (96 tuples including ())
#[cfg(all(feature = "_tuple_arity_95", not(feature = "_tuple_arity_127")))]
impl_tuple![
    CO,CN,CM,CL,CK,CJ,CI,CH,CG,CF,CE,CD,CC,CB,CA, // + 15
    BZ,BY,BX,BW,BV,BU,BT,BS,BR,BQ,BP,BO,BN,BM,BL,BK,BJ,BI,BH,BG,BF,BE,BD,BC,BB,BA, // 26 *3
    AZ,AY,AX,AW,AV,AU,AT,AS,AR,AQ,AP,AO,AN,AM,AL,AK,AJ,AI,AH,AG,AF,AE,AD,AC,AB,AA,
    Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A
];
#[rustfmt::skip] // max arity 127 (= 26 * 4 + 21 + 2); (128 tuples including ())
#[cfg(feature = "_tuple_arity_127")]
impl_tuple![
    DU,DT,DS,DR,DQ,DP,DO,DN,DM,DL,DK,DJ,DI,DH,DG,DF,DE,DD,DC,DB,DA, // + 21
    CZ,CY,CX,CW,CV,CU,CT,CS,CR,CQ,CP,CO,CN,CM,CL,CK,CJ,CI,CH,CG,CF,CE,CD,CC,CB,CA, // 26 *4
    BZ,BY,BX,BW,BV,BU,BT,BS,BR,BQ,BP,BO,BN,BM,BL,BK,BJ,BI,BH,BG,BF,BE,BD,BC,BB,BA,
    AZ,AY,AX,AW,AV,AU,AT,AS,AR,AQ,AP,AO,AN,AM,AL,AK,AJ,AI,AH,AG,AF,AE,AD,AC,AB,AA,
    Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A
];

// Helper macro for implementing TupleDebug and TupleDisplay
//
// # Arguments
// - a list of indices from 2 to (arity - 2)
// - optionally prepend +26 | +52 | +78 | +104 to avoid repeating lower numbers
macro_rules! itup {
    ($($i:tt),+) => {
        itup!(@TupleDebug, Debug, fmt_debug, $($i),+);
        itup!(@TupleDisplay, Display, fmt_display, $($i),+);
    };
    (+26 $($i:tt),+) => { // 26*1
        itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26, $($i),+);
    };
    (+52 $($i:tt),+) => { // 26*2
        itup!(+26
        27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52, $($i),+);
    };
    (+78 $($i:tt),+) => { // 26*3
        itup!(+52
        53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78, $($i),+);
    };
    (+104 $($i:tt),+) => { // 26*4
        itup!(+78
        79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,
        $($i),+);
    };

    // NOTE: we can't be recursive because we can't unabiguosly separate the last token,
    // we can only remove the first one, but order is important and we can't reverse it…
    (@$trait:ident, $fmt:ident, $fn:ident, /*$macro:ident,*/ $($i:tt),+) => { $crate::code::paste! {
        impl<HEAD: fmt::$fmt, $([<_$i>]: fmt::$fmt),+, LAST: fmt::$fmt> $trait
            for (HEAD, $([<_$i>],)+ LAST) {
            fn $fn(&self, f: &mut fmt::Formatter) -> fmt::Result {
                itup![@body:$fmt f, self: 0, 1, $($i),*]
            }
        }
    }};
    (@body: Debug $fmt:ident, $self:ident: $($n:tt),+) => {
        $fmt.debug_tuple("") $( .field(&$self.$n) )+ .finish()
    };
    (@body: Display $f:ident, $self:ident: $first:tt, $($rest:tt),+) => {{
        write!($f, "({}", &$self.$first)?;
        $( write!($f, ", {}", &$self.$rest)?; )+
        write!($f, ")")
    }};
}

#[rustfmt::skip] #[cfg(not(feature = "_tuple_arity_31"))]
mod _arity_15 {
    use super::*;
    itup!(2);
    itup!(2,3);
    itup!(2,3,4);
    itup!(2,3,4,5);
    itup!(2,3,4,5,6);
    itup!(2,3,4,5,6,7);
    itup!(2,3,4,5,6,7,8);
    itup!(2,3,4,5,6,7,8,9);
    itup!(2,3,4,5,6,7,8,9,10);
    itup!(2,3,4,5,6,7,8,9,10,11);
    itup!(2,3,4,5,6,7,8,9,10,11,12);
}
#[rustfmt::skip] #[cfg(feature = "_tuple_arity_31")]
mod _arity_31 {
    use super::*;
    itup!(2,3,4,5,6,7,8,9,10,11,12,13);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25);
    itup!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26);
    itup!(+26 27);
    itup!(+26 27,28);
}
#[rustfmt::skip] #[cfg(feature = "_tuple_arity_63")]
mod _arity_63 {
    use super::*;
    itup!(+26 27,28,29);
    itup!(+26 27,28,29,30);
    itup!(+26 27,28,29,30,31);
    itup!(+26 27,28,29,30,31,32);
    itup!(+26 27,28,29,30,31,32,33);
    itup!(+26 27,28,29,30,31,32,33,34);
    itup!(+26 27,28,29,30,31,32,33,34,35);
    itup!(+26 27,28,29,30,31,32,33,34,35,36);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51);
    itup!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52);
    itup!(+52 53);
    itup!(+52 53,54);
    itup!(+52 53,54,55);
    itup!(+52 53,54,55,56);
    itup!(+52 53,54,55,56,57);
    itup!(+52 53,54,55,56,57,58);
    itup!(+52 53,54,55,56,57,58,59);
    itup!(+52 53,54,55,56,57,58,59,60);
    itup!(+52 53,54,55,56,57,58,59,60,61);
}
#[rustfmt::skip] #[cfg(feature = "_tuple_arity_95")]
mod _arity_95 {
    use super::*;
    itup!(+52 53,54,55,56,57,58,59,60,61,62);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77);
    itup!(+52 53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78);
    itup!(+78 79);
    itup!(+78 79,80);
    itup!(+78 79,80,81);
    itup!(+78 79,80,81,82);
    itup!(+78 79,80,81,82,83);
    itup!(+78 79,80,81,82,83,84);
    itup!(+78 79,80,81,82,83,84,85);
    itup!(+78 79,80,81,82,83,84,85,86);
    itup!(+78 79,80,81,82,83,84,85,86,87);
    itup!(+78 79,80,81,82,83,84,85,86,87,88);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93);
}
#[rustfmt::skip] #[cfg(feature = "_tuple_arity_127")]
mod _arity_127 {
    use super::*;
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103);
    itup!(+78 79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104);
    itup!(+104 105);
    itup!(+104 105,106);
    itup!(+104 105,106,107);
    itup!(+104 105,106,107,108);
    itup!(+104 105,106,107,108,109);
    itup!(+104 105,106,107,108,109,110);
    itup!(+104 105,106,107,108,109,110,111);
    itup!(+104 105,106,107,108,109,110,111,112);
    itup!(+104 105,106,107,108,109,110,111,112,113);
    itup!(+104 105,106,107,108,109,110,111,112,113,114);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117,118);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117,118,119);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124);
    itup!(+104 105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125);
}
