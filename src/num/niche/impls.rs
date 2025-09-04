// devela::num::niche::impls
//
//! Implements `BitSized`, `ConstDefault` and `MemPod` for `NonValue*`.
//

#[cfg(all(feature = "unsafe_layout", not(feature = "safe_mem")))]
use crate::MemPod;
#[cfg(feature = "bit")]
use crate::{BitSized, ByteSized};
use crate::{
    ConstDefault, NonExtremeI8, NonExtremeI16, NonExtremeI32, NonExtremeI64, NonExtremeI128,
    NonExtremeIsize, NonExtremeU8, NonExtremeU16, NonExtremeU32, NonExtremeU64, NonExtremeU128,
    NonExtremeUsize, paste,
};
#[allow(unused, reason = "±unsafe")]
use crate::{
    NonValueI8, NonValueI16, NonValueI32, NonValueI64, NonValueI128, NonValueIsize, NonValueU8,
    NonValueU16, NonValueU32, NonValueU64, NonValueU128, NonValueUsize,
};

macro_rules! impl_for_non_value {
    () => {
        impl_for_non_value![
            u8, u16, u32, u64, u128, usize,
            i8, i16, i32, i64, i128, isize,
        ];
    };
    ($($IP:ty),+ $(,)?) => { paste! {
        $(
            impl_for_non_value!(@
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

        // ConstDefault for NonExtreme*
        impl ConstDefault for $ne {
            /// # Features
            /// Makes use of the `unsafe_niche` feature if enabled.
            const DEFAULT: Self = {
                #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                if let Some(v) = Self::new(<$IP>::DEFAULT) { v } else { unreachable![] }

                #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                // SAFETY: the default primitive value is always 0, and their MAX is never 0.
                unsafe { $ne::new_unchecked(<$IP>::DEFAULT) }
            };
        }

        // MemPod for NonValue*
        #[cfg(feature = "unsafe_layout")]
        #[cfg(not(any(feature = "safe_mem", feature = "safe_num")))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
        #[cfg_attr(nightly_doc, doc(cfg(not(feature = "safe_mem"))))]
        #[cfg_attr(nightly_doc, doc(cfg(not(feature = "safe_num"))))]
        unsafe impl<const V: $IP> MemPod for Option<$nv<V>> {}
    };
}
impl_for_non_value![];
