// devela::num::unit::helpers

// Implements TryFrom for `$to_prim` from the `$unit` type `$from_prim` value
macro_rules! impl_try_from {
    ($unit:ty, $from_prim:ty => $($to_prim:ty),+) => {
        $( impl_try_from![@$unit, $from_prim => $to_prim]; )+
    };

    (@$unit:ty, $from_prim:ty => $to_prim:ty) => {
        impl TryFrom<$unit> for $to_prim {
            type Error = $crate::Overflow;
            fn try_from(from: $unit) -> Result<$to_prim, Self::Error> { crate::paste! {
                $crate::Cast($from_prim::from(from)).[< checked_cast_to_ $to_prim>]()
            }}
        }
    };
}
pub(super) use impl_try_from;
