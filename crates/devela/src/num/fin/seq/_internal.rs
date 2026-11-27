//
//! Private dispatch for [`seq!`][crate::seq].
//

#[doc(hidden)]
#[macro_export]
macro_rules! __seq· {
    (%generic $Name:ident; $ops:tt; $($T:ident),+) => {
        $(
            $crate::__seq!(%carrier $T);
            impl $Name<$T> { $crate::__seq!(%unpack $T; $ops); }
        )+
    };
    (%unpack $T:ident; [$($op:ident),*]) => {
        $crate::__seq!(%ops $T; $($op),*);
    };
    (%carrier u8) => {};
    (%carrier u16) => {};
    (%carrier u32) => {};
    (%carrier u64) => {};
    (%carrier u128) => {};
    (%carrier usize) => {};
    (%carrier $bad:ident) => {
        compile_error!(concat!("seq!: unsupported primitive carrier `", stringify!($bad), "`"));
    };
    (%ops $T:ident;) => {};
    (%ops $T:ident; $op:ident $(, $rest:ident)*) => {
        $crate::__seq!(%op $T; $op);
        $crate::__seq!(%ops $T; $($rest),*);
    };
    (%op $T:ident; thue_morse) => { $crate::__seq_impl_digit!($T; thue_morse); };
    (%op $T:ident; prouhet) => { $crate::__seq_impl_digit!($T; prouhet); };
    (%op $T:ident; gray) => { $crate::__seq_impl_digit!($T; gray); };
    (%op $T:ident; van_der_corput) => { $crate::__seq_impl_digit!($T; van_der_corput); };
    (%op $T:ident; fibonacci) => { $crate::__seq_impl_recurrence!($T; fibonacci); };
    (%op $T:ident; all) => {
        $crate::__seq!(%ops $T;
            thue_morse, prouhet, gray, van_der_corput, fibonacci);
    };
    (%op $T:ident; $bad:ident) => {
        compile_error!(concat!("seq!: unknown operation `", stringify!($bad), "`"));
    };
}
#[doc(hidden)]
pub use __seq· as __seq;
