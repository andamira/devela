// devela::data::collections::tuple
//
//!
//

use crate::code::{ident_total, paste};
use core::fmt;

/// A formatting wrapper for [`ExtTuple`]s, implementing `Display` and `Debug`.
#[repr(transparent)]
pub struct TupleFmt<T: ExtTuple>(pub T);

// Private traits for tuples with elements that implement Debug or Display.
trait TupleDebug: ExtTuple {
    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
trait TupleDisplay: ExtTuple {
    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

// Marker trait to prevent downstream implementations of the `ExtTuple` trait.
#[rustfmt::skip]
mod private { pub trait Sealed {} }

/// Extension trait providing convenience methods for [tuples][tuple].
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// It's implemented for tuples of arity of 55 or less.
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

    /// The length (arity) of this tuple.
    const LEN: usize;

    /// Wraps the tuple in a [`TupleFmt`] for formatting purposes.
    #[rustfmt::skip]
    fn fmt(self) -> TupleFmt<Self> where Self: Sized { TupleFmt(self) }

    ///
    fn head(&self) -> &Self::Head;
    ///
    fn tail(&self) -> &Self::Tail;
    ///
    fn mut_head(&mut self) -> &mut Self::Head;
    ///
    fn mut_tail(&mut self) -> &mut Self::Tail;

    /// Returns a tuple with the head element splitted from the rest.
    fn split_head(self) -> (Self::Head, Self::NoHead);
    /// Returns a tuple with the tail element splitted from the rest.
    fn split_tail(self) -> (Self::NoTail, Self::Tail);

    /// Returns the tuple without the head.
    fn no_head(self) -> Self::NoHead;
    /// Returns the tuple without the head.
    fn no_tail(self) -> Self::NoTail;

    /// Appends the given `value` to this tuple.
    fn append<T>(self, value: T) -> Self::Append<T>;

    /// Prepends the given `value` to this tuple.
    fn prepend<T>(self, value: T) -> Self::Prepend<T>;
}

// $t1: the first generic argument after the HEAD
// $tn: the rest of the generic arguments before the TAIL
macro_rules! impl_tuple {
	() => {};
	($t1:ident $(, $tn:ident )*) => {
		impl<HEAD, $t1 $(, $tn)*, TAIL> private::Sealed for (HEAD, $t1, $($tn,)* TAIL) {}
		impl<HEAD, $t1 $(, $tn)*, TAIL> ExtTuple for (HEAD, $t1, $($tn,)* TAIL) {
			const LEN: usize = { ident_total![$($tn)*] + 3};

            type Head = HEAD;
            type Tail = TAIL;
			type NoHead = ($t1, $($tn,)* TAIL,);
			type NoTail = (HEAD, $t1, $($tn,)*);
            type Append<APPEND> = (HEAD, $t1, $($tn,)* TAIL, APPEND);
            type Prepend<PREPEND> = (PREPEND, HEAD, $t1, $($tn,)* TAIL);

            fn head(&self) -> &Self::Head { &self.0 }
            fn tail(&self) -> &Self::Tail { let (.., tail) = self; tail }
			fn mut_head(&mut self) -> &mut Self::Head { &mut self.0 }
            fn mut_tail(&mut self) -> &mut Self::Tail { let (.., tail) = self; tail }

            #[allow(non_snake_case)]
			fn split_head(self) -> (Self::Head, Self::NoHead) {
				let (head, $t1, $($tn,)* tail) = self;
				(head, ($t1, $($tn,)* tail))
			}
            #[allow(non_snake_case)]
			fn split_tail(self) -> (Self::NoTail, Self::Tail) {
				let (head, $t1, $($tn,)* tail) = self;
				((head, $t1 $(,$tn)*), tail)
			}
            #[allow(non_snake_case)]
			fn no_head(self) -> Self::NoHead {
				let (_, $t1, $($tn,)* tail) = self;
				($t1, $($tn,)* tail,)
            }
            #[allow(non_snake_case)]
			fn no_tail(self) -> Self::NoTail {
				let (head, $t1, $($tn,)* _) = self;
				(head, $t1, $($tn,)*)
            }
			#[allow(non_snake_case)]
			fn append<APPEND>(self, value: APPEND) -> Self::Append<APPEND> {
				let (head, $t1, $($tn,)* tail) = self;
				(head, $t1, $($tn,)* tail, value)
			}
			#[allow(non_snake_case)]
			fn prepend<PREPEND>(self, value: PREPEND) -> Self::Prepend<PREPEND> {
				let (head, $t1, $($tn,)* tail) = self;
				(value, head, $t1, $($tn,)* tail)
			}
		}

		impl_tuple!($($tn),*);
    };
}
// MAYBE: control arity with features. E.g.:
// _tuple_arity_107 = [_tuple_arity_81]
// _tuple_arity_81 = [_tuple_arity_54]
// _tuple_arity_55 = [_tuple_arity_29]
// _tuple_arity_29 = []
#[rustfmt::skip]
impl_tuple![ // 26 * 2 + 3 = 55 max arity
    // HEAD
    // CZ,CY,CX,CW,CV,CU,CT,CS,CR,CQ,CP,CO,CN,CM,CL,CK,CJ,CI,CH,CG,CF,CE,CD,CC,CB,CA,
    // BZ,BY,BX,BW,BV,BU,BT,BS,BR,BQ,BP,BO,BN,BM,BL,BK,BJ,BI,BH,BG,BF,BE,BD,BC,BB,BA,
    AZ,AY,AX,AW,AV,AU,AT,AS,AR,AQ,AP,AO,AN,AM,AL,AK,AJ,AI,AH,AG,AF,AE,AD,AC,AB,AA,
    Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A
    // TAIL
];

// manual implementations for tuples of arity 0, 1 and 2
#[rustfmt::skip]
mod manual_impls_small_tuples {
    use super::{ExtTuple, private::Sealed, TupleDebug, TupleDisplay};
    use core::fmt;

    // 0 elements
    impl Sealed for () {}
    impl ExtTuple for () {
        type Head = ();
        type Tail = ();
        type NoHead = ();
        type NoTail = ();
        type Append<T> = (T,);
        type Prepend<T> = (T,);
        const LEN: usize = 0;
        fn head(&self) -> &Self::Head { self }
        fn tail(&self) -> &Self::Tail { self }
        fn mut_head(&mut self) -> &mut Self::Head { self }
        fn mut_tail(&mut self) -> &mut Self::Tail { self }
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

    // 1 element
    impl<HEADTAIL> Sealed for (HEADTAIL,) {}
    impl<HEADTAIL> ExtTuple for (HEADTAIL,) {
        type Head = HEADTAIL;
        type Tail = HEADTAIL;
        type NoHead = ();
        type NoTail = ();
        type Append<T> = (HEADTAIL, T);
        type Prepend<T> = (T, HEADTAIL);
        const LEN: usize = 1;
        fn head(&self) -> &Self::Head { &self.0 }
        fn tail(&self) -> &Self::Tail { &self.0 }
        fn mut_head(&mut self) -> &mut Self::Head { &mut self.0 }
        fn mut_tail(&mut self) -> &mut Self::Tail { &mut self.0 }
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

    // 2 elements
    impl<HEAD, TAIL> super::private::Sealed for (HEAD, TAIL) {}
    impl<HEAD, TAIL> ExtTuple for (HEAD, TAIL) {
        type Head = HEAD;
        type Tail = TAIL;
        type NoHead = (TAIL,);
        type NoTail = (HEAD,);
        type Append<T> = (HEAD, TAIL, T);
        type Prepend<T> = (T, HEAD, TAIL);
        const LEN: usize = 2;
        fn head(&self) -> &Self::Head { &self.0 }
        fn tail(&self) -> &Self::Tail { &self.1 }
        fn mut_head(&mut self) -> &mut Self::Head { &mut self.0 }
        fn mut_tail(&mut self) -> &mut Self::Tail { &mut self.1 }
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

mod impl_fmt {
    use super::{TupleDisplay, TupleDebug, TupleFmt, ExtTuple};
    use core::fmt::{Formatter, Result, Debug, Display};
    use crate::code::paste;

    impl<T: TupleDebug> Debug for TupleFmt<T> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            self.0.fmt_debug(f)
        }
    }
    impl<T: TupleDisplay> Display for TupleFmt<T> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            self.0.fmt_display(f)
        }
    }

    // helper macros
    macro_rules! debug_body {
        ($fmt:ident, $self:ident: $($n:tt),+) => {
            $fmt.debug_tuple("") $( .field(&$self.$n) )+ .finish()
        };
    }
    macro_rules! tuple_display {
        ($f:ident, $self:ident: $first:tt, $($rest:tt),+) => {{
            write!($f, "({}", &$self.$first)?;
            $( write!($f, ", {}", &$self.$rest)?; )+
            write!($f, ")")
        }};
    }

    // helper macro for
    // pass a list of indices from 2 to arity-1, to
    macro_rules! impl_fmt {
        ($($i:tt),+) => {
            impl_fmt!(@TupleDebug, Debug, fmt_debug, debug_body, $($i),+);
            impl_fmt!(@TupleDisplay, Display, fmt_display, tuple_display, $($i),+);
        };
        (+26 $($i:tt),+) => {
            impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26, $($i),+);
        };
        (+52 $($i:tt),+) => {
            impl_fmt!(+26
            27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52, $($i),+);
        };

        // NOTE: we can't be recursive because we can't unabiguosly separate the last token,
        // only the first one, but the order is important and we can't reverse it…
        (@$trait:ident, $fmt:ident, $fn:ident, $macro:ident, $($i:tt),+) => { paste! {
            impl<HEAD: $fmt, $([<_$i>]: $fmt),+, LAST: $fmt> $trait for (HEAD, $([<_$i>],)+ LAST) {
                fn $fn(&self, f: &mut Formatter) -> Result {
                    $macro![f, self: 0, 1, $($i),*]
                }
            }
        }};
    }
    impl_fmt!(2);
    impl_fmt!(2,3);
    impl_fmt!(2,3,4);
    impl_fmt!(2,3,4,5);
    impl_fmt!(2,3,4,5,6);
    impl_fmt!(2,3,4,5,6,7);
    impl_fmt!(2,3,4,5,6,7,8);
    impl_fmt!(2,3,4,5,6,7,8,9);
    impl_fmt!(2,3,4,5,6,7,8,9,10);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25);
    impl_fmt!(2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26);
    impl_fmt!(+26 27);
    impl_fmt!(+26 27,28);
    impl_fmt!(+26 27,28,29);
    impl_fmt!(+26 27,28,29,30);
    impl_fmt!(+26 27,28,29,30,31);
    impl_fmt!(+26 27,28,29,30,31,32);
    impl_fmt!(+26 27,28,29,30,31,32,33);
    impl_fmt!(+26 27,28,29,30,31,32,33,34);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51);
    impl_fmt!(+26 27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52);
    impl_fmt!(+52 53);
}
