//
//! Defines [`handle!`].
//

#[doc = crate::_tags!(construction id)]
/// Defines a compact handle from arbitrary numeric components.
#[doc = crate::_doc_meta!{
    location("data/id/handle", macro handle),
}]
/// Each declared component is stored through [`MaybeNiche`], allowing primitive
/// and niche-aware integer representations to share the same generated API.
/// Component names have no built-in semantics: the receiving store or resolver
/// decides what fields such as `index`, `generation`, `revision`, or `scope` mean.
///
/// Constructors validate only the numeric representation of each component.
/// They do not establish whether the resulting handle
/// resolves to live or valid external state.
///
/// The generated handle is copyable, contains no references or ownership, and
/// exposes checked conversion to and from each component's primitive carrier.
///
/// # Syntax
/// Each component declares its primitive carrier and may optionally declare
/// a distinct storage representation:
///
/// - `name: Prim;` uses `Prim` as both carrier and representation.
/// - `name: Prim + Repr;` uses `Prim` as the carrier and `Repr` as the representation.
///
/// Primitive carriers are used by `from_prim`, `*_prim`, and `into_prim`.
/// Representations are used by `new`, direct component accessors, and `into_parts`.
///
/// Each representation must be supported by [`MaybeNiche`]
/// with the declared primitive carrier.
///
/// [`BittenU8<B>`][crate::BittenU8] can be used for byte components
/// whose valid domain is a smaller contiguous range starting at zero,
/// while retaining `u8` as the primitive carrier.
///
/// # Example
/// ```
/// use devela::{NonMaxU16, handle};
///
/// handle! {
///     [
///         index: u16 + NonMaxU16;
///         revision: u16;
///     ]
///     /// A compact versioned reference.
///     pub EntityRef;
/// }
///
/// let handle = EntityRef::from_prim(7, 3).unwrap();
/// assert_eq!(handle.get_index_prim(), 7);
/// assert_eq!(handle.get_revision_prim(), 3);
/// assert_eq!(handle.into_prim(), (7, 3));
/// ```
///
/// See also [`HandleExample`].
///
/// [`MaybeNiche`]: crate::MaybeNiche
/// [`HandleExample`]: crate::HandleExample
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! handle· {
    (
        [ $($fields:tt)* ]
        $(#[$attr:meta])*
        $vis:vis $Handle:ident $(;)?
    ) => {
        $crate::__handle! { %parse_fields
            [] [$($fields)*] [$(#[$attr])* $vis $Handle]
        }
    };
}
#[doc(inline)]
pub use handle· as handle;

#[cfg(test)]
mod _test {
    use crate::{BittenU8, HandleExample, PoolHandleExample, Value32, ValueKind4};
    use crate::{const_assert, handle, unwrap};

    #[test]
    fn components() {
        let handle = HandleExample::from_prim(7, 15, 3).unwrap();
        assert_eq![handle.get_index_prim(), 7];
        assert_eq![handle.get_kind_prim(), 15];
        assert_eq![handle.get_revision_prim(), 3];
        assert_eq![handle.get_index_usize(), Ok(7)];
        assert_eq![handle.get_kind_usize(), Ok(15)];
        assert_eq![handle.get_revision_usize(), Ok(3)];
        assert_eq![handle.into_prim(), (7, 15, 3)];
        let (index, kind, revision) = handle.into_parts();
        assert_eq![index.get(), 7];
        assert_eq![kind, 15];
        assert_eq![revision, 3];
        let raw = handle.raw();
        assert_eq!(HandleExample::from_raw(raw), handle);
        assert_eq!(HandleExample::try_from_raw(raw), Ok(handle));
    }
    #[test]
    fn rejects_invalid_niche_value() {
        assert![HandleExample::from_prim(u8::MAX, 0, 0).is_err()];
    }
    #[test]
    fn scalar_pack_roundtrip() {
        let handle = HandleExample::from_prim(7, 15, 3).unwrap();
        assert_eq!(HandleExample::PACK_BITS, 32);
        let packed = handle.try_pack_u32().unwrap();
        assert_eq!(packed, 0x0003_0F07);
        assert_eq!(HandleExample::try_unpack_u32(packed), Ok(handle));
    }
    #[test]
    fn packed_can_use_a_smaller_carrier() {
        let handle = HandleExample::from_prim(7, 3, 0).unwrap();
        let packed = handle.try_pack_u16().unwrap();
        assert_eq!(packed, 0x0307);
        assert_eq!(HandleExample::try_unpack_u16(packed), Ok(handle));
    }
    #[test]
    fn packed_bridges_to_value_payload() {
        let handle = PoolHandleExample::from_prim(7, 3).unwrap();
        // u8 index + u16 generation.
        assert_eq!(PoolHandleExample::PACK_BITS, 24);
        assert!(PoolHandleExample::PACK_BITS <= Value32::PAYLOAD_BITS);
        let payload = handle.try_pack_u32().unwrap();
        let value = Value32::try_from_parts(ValueKind4::Ref, payload).unwrap();
        assert_eq!(value.kind(), ValueKind4::Ref);
        let decoded = PoolHandleExample::try_unpack_u32(value.payload()).unwrap();
        assert_eq!(decoded, handle);
    }
    #[test]
    fn unpack_rejects_noncanonical_pack() {
        // HandleExample owns only 32 packed bits.
        assert!(HandleExample::try_unpack_u64(1_u64 << 40).is_err());
        // index is NonMaxU8.
        assert!(HandleExample::try_unpack_u32(0x0000_00FF).is_err());
    }
    #[test]
    const fn scalar_pack_is_const() {
        const H: HandleExample = unwrap![ok HandleExample::from_prim(7, 15, 3)];
        const P: u32 = unwrap![some H.try_pack_u32()];
        const H2: HandleExample = unwrap![ok HandleExample::try_unpack_u32(P)];
        const_assert!(eq P, 0x0003_0F07);
        const_assert!(H.eq(H2));
    }
    #[test]
    fn bitten_support() {
        handle! { [index: u8 + BittenU8<3>;] pub BittenHandle; }

        let zero = BittenHandle::from_prim(0).unwrap();
        let max = BittenHandle::from_prim(31).unwrap();

        // Accepted domain.
        assert_eq!(zero.get_index_prim(), 0);
        assert_eq!(max.get_index_prim(), 31);
        assert_eq!(max.get_index().get(), 31);
        assert_eq!(max.get_index_usize(), Ok(31));
        assert_eq!(max.into_prim(), (31,));
        // BittenU8<3> represents exactly 0..=31.
        assert!(BittenHandle::from_prim(32).is_err());
        assert!(BittenHandle::from_prim(u8::MAX).is_err());
        // The niche survives through MaybeNiche and the generated handle.
        assert_eq!(size_of::<BittenHandle>(), 1);
        assert_eq!(size_of::<Option<BittenHandle>>(), 1);
        // Primitive packing still validates the representation on reconstruction.
        let packed = max.try_pack_u8().unwrap();
        assert_eq!(packed, 31);
        assert_eq!(BittenHandle::try_unpack_u8(packed), Ok(max));
        assert!(BittenHandle::try_unpack_u8(32).is_err());
    }
}
