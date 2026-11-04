//
//! Private dispatch for [`int!`][crate::int].

#[doc(hidden)]
#[macro_export]
macro_rules! __int· {
    (%impl_generic name[$Name:ident] carriers[$($Carrier:ident),+] ops $ops:tt) => {
        $(
            impl $Name<$Carrier> {
                /// Creates an integer with the selected primitive carrier.
                pub const fn new(value: $Carrier) -> Self { Self(value) }
                /// Returns the primitive carrier value.
                pub const fn get(self) -> $Carrier { self.0 }
            }
            $crate::__int!(%dispatch_carrier
                vis[pub] name[$Name] target[$Name<$Carrier>]
                carrier[$Carrier] ops $ops
            );
        )+
    };

    /* carrier normalization */

    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i8] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i8] signed[i8] unsigned[u8]
            up[i16] iup[i16] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i16] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i16] signed[i16] unsigned[u16]
            up[i32] iup[i32] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i32] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i32] signed[i32] unsigned[u32]
            up[i64] iup[i64] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i64] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i64] signed[i64] unsigned[u64]
            up[i128] iup[i128] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[i128] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[i128] signed[i128] unsigned[u128]
            up[i128] iup[i128] up_wide[false] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[isize] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[signed] carrier[isize] signed[isize] unsigned[usize]
            up[$crate::isize_up] iup[$crate::isize_up] up_wide[true] ops[$($op),*]);
    };

    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u8] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u8] signed[i8] unsigned[u8]
            up[u16] iup[i16] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u16] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u16] signed[i16] unsigned[u16]
            up[u32] iup[i32] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u32] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u32] signed[i32] unsigned[u32]
            up[u64] iup[i64] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u64] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u64] signed[i64] unsigned[u64]
            up[u128] iup[i128] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[u128] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[u128] signed[i128] unsigned[u128]
            up[u128] iup[i128] up_wide[false] ops[$($op),*]);
    };
    (%dispatch_carrier vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[usize] ops[$($op:ident),*]) => {
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[unsigned] carrier[usize] signed[isize] unsigned[usize]
            up[$crate::usize_up] iup[$crate::isize_up] up_wide[true] ops[$($op),*]);
    };
    (%dispatch_carrier
        vis[$vis:vis] name[$Name:ident] target[$Target:ty] carrier[$bad:tt] ops[$($op:ident),*]
    ) => {
        compile_error!(concat!("int!: unsupported primitive carrier `", stringify!($bad), "`"));
    };

    /* operation-family dispatch */

    (%ops vis[$vis:vis] name[$Name:ident] target[$Target:ty]
        kind[$kind:ident] carrier[$Carrier:ty] signed[$Signed:ty] unsigned[$Unsigned:ty]
        up[$Up:ty] iup[$IUp:ty] up_wide[$up_wide:tt] ops[]
    ) => {};
    (%ops vis[$vis:vis] name[$Name:ident] target[$Target:ty]
        kind[$kind:ident] carrier[$Carrier:ty] signed[$Signed:ty] unsigned[$Unsigned:ty]
        up[$Up:ty] iup[$IUp:ty] up_wide[$up_wide:tt] ops[base $(, $rest:ident)*]
    ) => {
        $crate::__int_impl_base!(
            vis[$vis] name[$Name] target[$Target]
            kind[$kind] carrier[$Carrier] signed[$Signed] unsigned[$Unsigned]
            up[$Up] iup[$IUp] up_wide[$up_wide]
        );
        $crate::__int!(%ops vis[$vis] name[$Name] target[$Target]
            kind[$kind] carrier[$Carrier] signed[$Signed] unsigned[$Unsigned]
            up[$Up] iup[$IUp] up_wide[$up_wide] ops[$($rest),*]);
    };
    (%ops vis[$vis:vis] name[$Name:ident] target[$Target:ty]
        kind[$kind:ident] carrier[$Carrier:ty] signed[$Signed:ty] unsigned[$Unsigned:ty]
        up[$Up:ty] iup[$IUp:ty] up_wide[$up_wide:tt] ops[$bad:ident $(, $rest:ident)*]
    ) => {
        compile_error!(concat!("int!: unknown operation family `", stringify!($bad), "`"));
    };
}
#[doc(hidden)]
pub use __int· as __int;
