// devela::num::grain::niche::impls
//
//! Implements `BitSized`, `ConstInit` and `MemPod` for `NonValue*`.
//

#[cfg(all(feature = "unsafe_layout", not(feature = "safe_mem")))]
use crate::MemPod;
#[cfg(feature = "bit")]
use crate::{BitSized, ByteSized};
#[allow(unused, reason = "±unsafe")]
use crate::{
    NonValueI8, NonValueI16, NonValueI32, NonValueI64, NonValueI128, NonValueIsize, NonValueU8,
    NonValueU16, NonValueU32, NonValueU64, NonValueU128, NonValueUsize,
};

macro_rules! _impl_traits_for_non_value {
    () => {
        _impl_traits_for_non_value![
            u8, u16, u32, u64, u128, usize,
            i8, i16, i32, i64, i128, isize,
        ];
    };
    ($($IP:ty),+ $(,)?) => { crate::paste! {
        $(
            _impl_traits_for_non_value!(@
                [<NonValue $IP:camel>],
                [<NonExtreme $IP:camel>],
                $IP
                );
            )+
    }};
    (@$nv:ident, $ne:ident, $IP:ty) => {

        // BitSized for NonValue*
        #[cfg(feature = "bit")]
        impl<const V: $IP> BitSized<{<$IP>::BYTE_SIZE * 8}> for $nv<V> {}

        // MemPod for NonValue*
        #[cfg(feature = "unsafe_layout")]
        #[cfg(not(any(feature = "safe_mem", feature = "safe_num")))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
        #[cfg_attr(nightly_doc, doc(cfg(not(feature = "safe_mem"))))]
        #[cfg_attr(nightly_doc, doc(cfg(not(feature = "safe_num"))))]
        unsafe impl<const V: $IP> MemPod for Option<$nv<V>> {}
    };
}
_impl_traits_for_non_value![];
