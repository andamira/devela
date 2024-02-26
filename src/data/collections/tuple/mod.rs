// devela::data::collections::tuple
//
//!
//

use crate::code::ident_total;
use core::fmt;

#[rustfmt::skip]
mod private { pub trait Sealed {} }

/// Extension trait providing convenience methods for [tuples][tuple].
///
/// It's implemented for tuples of arity 106 or less.
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
	($t1:ident $(, $tn:ident)*) => {
		impl<HEAD, $t1 $(, $tn)*, TAIL> private::Sealed for (HEAD, $t1, $($tn,)* TAIL) {}
		impl<HEAD, $t1 $(, $tn)*, TAIL> ExtTuple for (HEAD, $t1, $($tn,)* TAIL) {
            type Head = HEAD;
            type Tail = TAIL;
			type NoHead = ($t1, $($tn,)* TAIL,);
			type NoTail = (HEAD, $t1, $($tn,)*);
            type Append<APPEND> = (HEAD, $t1, $($tn,)* TAIL, APPEND);
            type Prepend<PREPEND> = (PREPEND, HEAD, $t1, $($tn,)* TAIL);

			const LEN: usize = { ident_total![$($tn)*] + 2};

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
#[rustfmt::skip]
impl_tuple![ // 26 * 4 = 104 automatically + 3 manually = 107 tuple sizes
    // separate HEAD
    CZ,CY,CX,CW,CV,CU,CT,CS,CR,CQ,CP,CO,CN,CM,CL,CK,CJ,CI,CH,CG,CF,CE,CD,CC,CB,CA,
    BZ,BY,BX,BW,BV,BU,BT,BS,BR,BQ,BP,BO,BN,BM,BL,BK,BJ,BI,BH,BG,BF,BE,BD,BC,BB,BA,
    AZ,AY,AX,AW,AV,AU,AT,AS,AR,AQ,AP,AO,AN,AM,AL,AK,AJ,AI,AH,AG,AF,AE,AD,AC,AB,AA,
    Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A
    // separate TAIL
];

#[rustfmt::skip]
mod manual_impls {
    use super::{ExtTuple, private::Sealed};

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
}
