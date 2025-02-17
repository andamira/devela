// devela::data::codec::encode::combinators
//
//! Defines
//! [`EncodeBe`], [`EncodeIf`], [`EncodeLe`], [`EncodeFlags`], [`EncodeJoin`], [`EncodeLenValue`].
//

use crate::{
    BitOr, Debug, Deref, Encodable, EncodableLen, FmtResult, FmtWrite, Formatter, IoError,
    IoErrorKind, IoResult, IoWrite, NonZero, PhantomData, TryFromIntError,
};

pub use {
    cond::EncodeIf,
    endian::{EncodeBe, EncodeLe},
    flags::EncodeFlags,
    join::EncodeJoin,
    len::{EncodeLen, EncodeLenValue},
};

#[rustfmt::skip]
mod endian {
    use super::*;

    /// Encodes a number in big-endian order.
    ///
    /// # Example
    /// ```
    /// use devela::{Encodable, EncodeBe};
    ///
    /// let mut buf = [0u8; 32];
    /// let len = EncodeBe::new(1u16).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], &[0, 1], "the most significant byte comes first");
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::<u8>::new();
    /// EncodeBe::new(1u16).encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[0, 1]);
    /// # }
    /// ```
    #[must_use]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EncodeBe<W> {
        num: W,
    }
    impl<W> EncodeBe<W> {
        /// Creates a new [`EncodeBe`] combinator.
        pub const fn new(num: W) -> Self { Self { num } }
    }

    /// Encodes a number in little-endian order.
    ///
    /// # Examples
    /// ```
    /// use devela::{Encodable, EncodeLe};
    ///
    /// let mut buf = [0u8; 2];
    /// let len = EncodeLe::new(1u16).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], &[1, 0], "the least significant byte comes first");
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::<u8>::new();
    /// EncodeLe::new(1u16).encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[1, 0]);
    /// # }
    /// ```
    #[must_use]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EncodeLe<E> {
        num: E,
    }
    impl<W> EncodeLe<W> {
        /// Creates a new [`EncodeLe`] combinator.
        pub const fn new(num: W) -> Self { Self { num } }
    }
    macro_rules! impl_endian {
        () => {
            impl_endian![int: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128];
            impl_endian![float: f32, f64];
        };
        (int: $($T:ty),+) => {
            impl_endian!(prim: $($T),+);
            impl_endian!(non0: $($T),+);
            $(
                impl TryFrom<usize> for EncodeBe<$T> {
                    type Error = TryFromIntError;
                    fn try_from(value: usize) -> Result<Self, Self::Error> {
                        <$T>::try_from(value).map(Self::new)
                    }
                }
                impl TryFrom<usize> for EncodeLe<$T> {
                    type Error = TryFromIntError;
                    fn try_from(value: usize) -> Result<Self, Self::Error> {
                        <$T>::try_from(value).map(Self::new)
                    }
                }
            )+
        };
        (float: $($T:ty),+) => {
            impl_endian!(prim: $($T),+);
        };
        (prim: $($T:ty),+) => {
            $(
                impl From<$T> for EncodeBe<$T> { fn from(num: $T) -> Self { Self { num } } }
                impl From<EncodeBe<$T>> for $T { fn from(be: EncodeBe<$T>) -> Self { be.num } }
                impl<W: IoWrite> Encodable<W> for EncodeBe<$T> {
                    fn encode(&self, writer: &mut W) -> IoResult<usize> {
                        writer.write(&self.num.to_be_bytes())
                    }
                }
                impl From<$T> for EncodeLe<$T> { fn from(num: $T) -> Self { Self { num } } }
                impl From<EncodeLe<$T>> for $T { fn from(be: EncodeLe<$T>) -> Self { be.num } }
                impl<W: IoWrite> Encodable<W> for EncodeLe<$T> {
                    fn encode(&self, writer: &mut W) -> IoResult<usize> {
                        writer.write(&self.num.to_le_bytes())
                    }
                }
            )+
        };
        (non0: $($T:ty),+) => {
            $(
                impl From<NonZero<$T>> for EncodeBe<NonZero<$T>> {
                    fn from(num: NonZero<$T>) -> Self { Self { num } }
                }
                impl From<EncodeBe<NonZero<$T>>> for NonZero<$T> {
                    fn from(be: EncodeBe<NonZero<$T>>) -> Self { be.num }
                }
                impl<W: IoWrite> Encodable<W> for EncodeBe<NonZero<$T>> {
                    fn encode(&self, writer: &mut W) -> IoResult<usize> {
                        writer.write(&self.num.get().to_be_bytes())
                    }
                }
                impl From<NonZero<$T>> for EncodeLe<NonZero<$T>> {
                    fn from(num: NonZero<$T>) -> Self { Self { num } }
                }
                impl From<EncodeLe<NonZero<$T>>> for NonZero<$T> {
                    fn from(be: EncodeLe<NonZero<$T>>) -> Self { be.num }
                }
                impl<W: IoWrite> Encodable<W> for EncodeLe<NonZero<$T>> {
                    fn encode(&self, writer: &mut W) -> IoResult<usize> {
                        writer.write(&self.num.get().to_be_bytes())
                    }
                }
            )+
        }
    }
    impl_endian![];
}
#[rustfmt::skip]
mod cond {
    use super::*;

    /// Encodes conditionally an encodable.
    ///
    /// # Example
    /// ```
    /// use devela::{Encodable, EncodeIf, CStr};
    ///
    /// let non_empty = |s:&&CStr| !s.is_empty();
    /// let mut buf = [0u8; 64];
    /// let len = EncodeIf::new(c"hello", non_empty).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"hello\0", "A non-empty CStr includes the null terminator");
    ///
    /// let mut buf = [0u8; 64];
    /// let len = EncodeIf::new(c"", non_empty).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"", "An empty CStr does not produce any output");
    /// ```
    #[must_use]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EncodeIf<E, F> { encodable: E, condition: F }
    impl<E, F: Fn(&E) -> bool> EncodeIf<E, F> {
        /// Creates a new [`EncodeIf`] combinator.
        pub const fn new(encodable: E, condition: F) -> Self { Self { encodable, condition } }
    }
    impl<E, F> AsRef<E> for EncodeIf<E, F> { fn as_ref(&self) -> &E { &self.encodable } }
    impl<E, F> Deref for EncodeIf<E, F> {
        type Target = E;
        fn deref(&self) -> &Self::Target { self.as_ref() }
    }
    impl<E: Encodable<W>, W: IoWrite, F: Fn(&E) -> bool> Encodable<W> for EncodeIf<E, F> {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            if (self.condition)(&self.encodable) {
                self.encodable.encode(writer)
            } else {
                Ok(0)
            }
        }
    }
}
#[rustfmt::skip]
mod flags {
    use super::*;

    /// Encodes a sequence of flags as a single byte.
    ///
    /// # Examples
    /// ```
    /// use devela::{Encodable, EncodeFlags};
    ///
    /// let mut buf = [0u8; 1];
    /// let len = EncodeFlags::new([true, false, false, true, false, false, false, false])
    ///     .encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], &[0b_1001_0000]);
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::new();
    /// EncodeFlags::new([true, false, false, true, false, false, false, false])
    ///     .encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[0b_1001_0000]);
    /// # }
    /// ```
    #[must_use]
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EncodeFlags([bool; 8]);
    impl EncodeFlags {
        /// Creates a new [`EncodeFlags`] combinator.
        pub const fn new(flags: [bool; 8]) -> Self { Self(flags) }
    }
    impl Debug for EncodeFlags {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write!(f, "EncodeFlags({:08b})", u8::from(*self))
        }
    }
    impl Deref for EncodeFlags {
        type Target = [bool; 8];
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl AsRef<[bool; 8]> for EncodeFlags {
        fn as_ref(&self) -> &[bool; 8] { &self.0 }
    }
    impl From<u8> for EncodeFlags {
        fn from(from: u8) -> Self {
            let mut slice = [false; 8];
            slice.iter_mut().enumerate().rev().for_each(|(i, v)| { *v = (from & (1 << i)) != 0; });
            Self(slice)
        }
    }
    impl From<EncodeFlags> for u8 {
        fn from(from: EncodeFlags) -> Self {
            from.0.into_iter().rev().enumerate()
                .filter_map(|(i, v)| v.then_some(1 << i)).fold(0u8, BitOr::bitor)
        }
    }
    impl<W: IoWrite> Encodable<W> for EncodeFlags {
        fn encode(&self, encoder: &mut W) -> IoResult<usize> {
            u8::from(*self).encode(encoder)
        }
    }
}

#[rustfmt::skip]
mod join {
    use super::*;

    /// Encodes an iterator of encodables as a sequence with an optional `separator`.
    ///
    /// # Example
    /// ```
    /// use devela::{Encodable, EncodeJoin};
    ///
    /// let compact_map = [ (c"hello", 1u8), (c"world", 2u8) ];
    /// let mut buf = [0u8; 64];
    /// let len = EncodeJoin::new(&compact_map).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"hello\0\x01world\0\x02");
    ///
    /// let mut buf = [0u8; 64];
    /// let array = ["hello", "world", "another"];
    /// let len = EncodeJoin::with(&array, ", ").encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"hello, world, another");
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::new();
    /// let array = ["hello", "world", "another"];
    /// EncodeJoin::with(&array, '/').encode(&mut buf).unwrap();
    /// assert_eq!(&buf, b"hello/world/another");
    /// # }
    /// ```
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EncodeJoin<E, S> {
        encodable_iter: E,
        separator: Option<S>,
    }
    impl<E> EncodeJoin<E, ()> {
        /// Creates a new [`EncodeJoin`] combinator, without a separator.
        pub const fn new(encodable_iter: E) -> Self {
            Self { encodable_iter, separator: None::<()> }
        }
    }
    impl<E, S> EncodeJoin<E, S> {
        /// Creates a new [`EncodeJoin`] combinator with a `separator`.
        pub const fn with(encodable_iter: E, separator: S) -> Self {
            Self { encodable_iter, separator: Some(separator) }
        }
    }
    impl<E, S, W: IoWrite> Encodable<W> for EncodeJoin<E, S>
    where
        E: Clone + IntoIterator,
        E::Item: Encodable<W>,
        S: Encodable<W>,
    {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            let mut total = 0;
            if let Some(sep) = &self.separator {
                let mut is_first = true;
                for encodable in self.encodable_iter.clone() {
                    if is_first {
                        is_first = false;
                    } else {
                        total += sep.encode(writer)?;
                    }
                    total += encodable.encode(writer)?;
                }
            } else {
                for encodable in self.encodable_iter.clone() {
                    total += encodable.encode(writer)?;
                }
            }
            Ok(total)
        }
    }
}
#[rustfmt::skip]
mod len {
    use super::*;

    /// A dummy writer that counts bytes instead of actually writing them.
    ///
    /// Note that this encoder runs all the same encoding logic as any other encoder,
    /// so it will trigger the same side effects that other encoders would trigger
    /// (e.g Allocations).
    ///
    /// # Example
    /// ```
    /// use devela::{IoWrite, Encodable, EncodeLen};
    ///
    /// let encodable = c"hello, world!";
    /// let mut encoder = EncodeLen::new();
    /// encodable.encode(&mut encoder).unwrap();
    /// assert_eq!(encoder.size(), 14, "13 bytes from the ASCII string + 1 for the null terminator");
    /// ```
    #[must_use]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EncodeLen { size: usize, }
    impl EncodeLen {
        /// Creates a new [`EncodeLen`].
        pub const fn new() -> Self { Self { size: 0 } }
        /// Returns the computed encoded size.
        pub const fn size(&self) -> usize { self.size }
    }
    impl From<EncodeLen> for usize {
        fn from(encoder: EncodeLen) -> usize { encoder.size }
    }
    impl FmtWrite for EncodeLen {
        fn write_str(&mut self, s: &str) -> FmtResult<()> { self.size += s.len(); Ok(()) }
    }
    impl IoWrite for EncodeLen {
        fn write(&mut self, slice: &[u8]) -> IoResult<usize> { self.size += slice.len(); Ok(self.size) }
        fn flush(&mut self) -> IoResult<()> { Ok(()) }
    }

    /// Encodes a length prefixed value ([TLV](https://en.wikipedia.org/wiki/Type–length–value)).
    ///
    /// # Examples
    /// ```
    /// use devela::{Encodable, EncodeLenValue, TryFromIntError};
    ///
    /// let mut buf = [0u8; 64];
    /// let len = EncodeLenValue::<_, u8>::new("hello").encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"\x05hello", "A single byte indicates the length of the string");
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::new();
    /// EncodeLenValue::<_, u8>::new("hello").encode(&mut buf).unwrap();
    /// assert_eq!(&buf, b"\x05hello");
    /// # }
    /// ```
    #[must_use]
    #[doc(alias("length", "prefix", "TLV"))]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct EncodeLenValue<E, LEN> {
        encodable: E,
        phantom: PhantomData<LEN>,
    }
    impl<E, LEN> EncodeLenValue<E, LEN> {
        /// Creates a new TLV combinator.
        pub const fn new(encodable: E) -> Self { Self { encodable, phantom: PhantomData } }
    }
    impl<E, LEN, W: IoWrite> Encodable<W> for EncodeLenValue<E, LEN>
    where
        E: Encodable<W> + EncodableLen,
        LEN: Encodable<W> + TryFrom<usize>,
    {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            let len = self.encodable.encoded_size()?;
            let len_encoded: LEN = LEN::try_from(len)
                .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Length conversion failed"))?;
            let mut total = len_encoded.encode(writer)?; // Encode the length using `LEN`
            total += self.encodable.encode(writer)?; // Encode the actual content
            Ok(total)
        }
    }
}
