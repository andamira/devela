// devela::data::codec::encode::combinators
//
//! Defines
//! [`CodecBe`], [`CodecIf`], [`CodecLe`], [`CodecFlags`], [`CodecJoin`], [`CodecLenValue`].
//

use crate::{
    iif, BitOr, Debug, Decodable, Deref, Encodable, EncodableLen, FmtResult, FmtWrite, Formatter,
    IoError, IoErrorKind, IoRead, IoResult, IoTake, IoWrite, NonZero, PhantomData, TryFromIntError,
};

pub use {
    cond::CodecIf,
    endian::{CodecBe, CodecLe},
    flags::CodecFlags,
    join::CodecJoin,
    len::{CodecLen, CodecLenValue},
};

#[rustfmt::skip]
mod endian {
    use super::*;

    /// Encodes and decodes a number in big-endian order.
    ///
    /// # Example
    /// ```
    /// use devela::{Encodable, CodecBe};
    ///
    /// let mut buf = [0u8; 32];
    /// let len = CodecBe::new(1u16).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], &[0, 1], "the most significant byte comes first");
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::<u8>::new();
    /// CodecBe::new(1u16).encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[0, 1]);
    /// # }
    /// ```
    #[must_use]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CodecBe<W> {
        num: W,
    }
    impl<W> CodecBe<W> {
        /// Creates a new [`CodecBe`] combinator.
        pub const fn new(num: W) -> Self { Self { num } }
    }

    /// Encodes and decodes a number in little-endian order.
    ///
    /// # Examples
    /// ```
    /// use devela::{Encodable, CodecLe};
    ///
    /// let mut buf = [0u8; 2];
    /// let len = CodecLe::new(1u16).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], &[1, 0], "the least significant byte comes first");
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::<u8>::new();
    /// CodecLe::new(1u16).encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[1, 0]);
    /// # }
    /// ```
    #[must_use]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CodecLe<E> {
        num: E,
    }
    impl<W> CodecLe<W> {
        /// Creates a new [`CodecLe`] combinator.
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
                impl TryFrom<usize> for CodecBe<$T> {
                    type Error = TryFromIntError;
                    fn try_from(value: usize) -> Result<Self, Self::Error> {
                        <$T>::try_from(value).map(Self::new)
                    }
                }
                impl TryFrom<usize> for CodecLe<$T> {
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
            $(  // Be
                impl From<$T> for CodecBe<$T> { fn from(num: $T) -> Self { Self { num } } }
                impl From<CodecBe<$T>> for $T { fn from(be: CodecBe<$T>) -> Self { be.num } }
                impl<W: IoWrite> Encodable<W> for CodecBe<$T> {
                    fn encode(&self, writer: &mut W) -> IoResult<usize> {
                        writer.write(&self.num.to_be_bytes()) } }
                impl<R: IoRead> Decodable<R> for CodecBe<$T> {
                    type Output = $T;
                    fn decode(reader: &mut R) -> IoResult<$T> {
                        let mut buf = [0u8; size_of::<$T>()];
                        reader.read_exact(&mut buf)?;
                        Ok(<$T>::from_be_bytes(buf)) } }
                // Le
                impl From<$T> for CodecLe<$T> { fn from(num: $T) -> Self { Self { num } } }
                impl From<CodecLe<$T>> for $T { fn from(be: CodecLe<$T>) -> Self { be.num } }
                impl<W: IoWrite> Encodable<W> for CodecLe<$T> {
                    fn encode(&self, writer: &mut W) -> IoResult<usize> {
                        writer.write(&self.num.to_le_bytes()) } }
                impl<R: IoRead> Decodable<R> for CodecLe<$T> {
                    type Output= $T;
                    fn decode(reader: &mut R) -> IoResult<$T> {
                        let mut buf = [0u8; size_of::<$T>()];
                        reader.read_exact(&mut buf)?;
                        Ok(<$T>::from_le_bytes(buf)) } }
            )+
        };
        (non0: $($T:ty),+) => {
            $(  // Be
                impl From<NonZero<$T>> for CodecBe<NonZero<$T>> {
                    fn from(num: NonZero<$T>) -> Self { Self { num } } }
                impl From<CodecBe<NonZero<$T>>> for NonZero<$T> {
                    fn from(be: CodecBe<NonZero<$T>>) -> Self { be.num } }
                impl<W: IoWrite> Encodable<W> for CodecBe<NonZero<$T>> {
                    fn encode(&self, writer: &mut W) -> IoResult<usize> {
                        writer.write(&self.num.get().to_be_bytes()) } }
                impl<R: IoRead> Decodable<R> for CodecBe<NonZero<$T>> {
                    type Output = NonZero<$T>;
                    fn decode(reader: &mut R) -> IoResult<NonZero<$T>> {
                        let mut buf = [0u8; size_of::<$T>()];
                        reader.read_exact(&mut buf)?;
                        let num = <$T>::from_be_bytes(buf);
                        let non_zero = NonZero::<$T>::new(num)
                            .ok_or(IoError::new(IoErrorKind::InvalidData,
                                    "Decoded zero for NonZero type"))?;
                        Ok(non_zero) } }
                // Le
                impl From<NonZero<$T>> for CodecLe<NonZero<$T>> {
                    fn from(num: NonZero<$T>) -> Self { Self { num } } }
                impl From<CodecLe<NonZero<$T>>> for NonZero<$T> {
                    fn from(be: CodecLe<NonZero<$T>>) -> Self { be.num } }
                impl<W: IoWrite> Encodable<W> for CodecLe<NonZero<$T>> {
                    fn encode(&self, writer: &mut W) -> IoResult<usize> {
                        writer.write(&self.num.get().to_be_bytes()) } }
                impl<R: IoRead> Decodable<R> for CodecLe<NonZero<$T>> {
                    type Output = NonZero<$T>;
                    fn decode(reader: &mut R) -> IoResult<NonZero<$T>> {
                        let mut buf = [0u8; size_of::<$T>()];
                        reader.read_exact(&mut buf)?;
                        let num = <$T>::from_le_bytes(buf);
                        let non_zero = NonZero::<$T>::new(num)
                            .ok_or(IoError::new(IoErrorKind::InvalidData,
                                    "Decoded zero for NonZero type"))?;
                        Ok(non_zero) } }
            )+
        }
    }
    impl_endian![];
}
#[rustfmt::skip]
mod cond {
    use super::*;

    /// Encodes and decodes conditionally.
    ///
    /// # Example
    /// ```
    /// use devela::{Encodable, CodecIf, CStr};
    ///
    /// let non_empty = |s:&&CStr| !s.is_empty();
    /// let mut buf = [0u8; 64];
    /// let len = CodecIf::new(c"hello", non_empty).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"hello\0", "A non-empty CStr includes the null terminator");
    ///
    /// let mut buf = [0u8; 64];
    /// let len = CodecIf::new(c"", non_empty).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"", "An empty CStr does not produce any output");
    /// ```
    #[must_use]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CodecIf<E, F> { encodable: E, condition: F }
    impl<E, F: Fn(&E) -> bool> CodecIf<E, F> {
        /// Creates a new [`CodecIf`] combinator.
        pub const fn new(encodable: E, condition: F) -> Self { Self { encodable, condition } }
    }
    impl<E, F> AsRef<E> for CodecIf<E, F> { fn as_ref(&self) -> &E { &self.encodable } }
    impl<E, F> Deref for CodecIf<E, F> {
        type Target = E;
        fn deref(&self) -> &Self::Target { self.as_ref() }
    }
    impl<E: Encodable<W>, W: IoWrite, F: Fn(&E) -> bool> Encodable<W> for CodecIf<E, F> {
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

    /// Encodes and decodes a sequence of flags as a single byte.
    ///
    /// # Examples
    /// ```
    /// use devela::{Encodable, CodecFlags};
    ///
    /// let mut buf = [0u8; 1];
    /// let len = CodecFlags::new([true, false, false, true, false, false, false, false])
    ///     .encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], &[0b_1001_0000]);
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::new();
    /// CodecFlags::new([true, false, false, true, false, false, false, false])
    ///     .encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[0b_1001_0000]);
    /// # }
    /// ```
    #[must_use]
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CodecFlags([bool; 8]);
    impl CodecFlags {
        /// Creates a new [`CodecFlags`] combinator.
        pub const fn new(flags: [bool; 8]) -> Self { Self(flags) }
    }
    impl Debug for CodecFlags {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write!(f, "CodecFlags({:08b})", u8::from(*self))
        }
    }
    impl Deref for CodecFlags {
        type Target = [bool; 8];
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl AsRef<[bool; 8]> for CodecFlags {
        fn as_ref(&self) -> &[bool; 8] { &self.0 }
    }
    impl From<u8> for CodecFlags {
        fn from(from: u8) -> Self {
            let mut slice = [false; 8];
            slice.iter_mut().enumerate().rev().for_each(|(i, v)| { *v = (from & (1 << i)) != 0; });
            Self(slice)
        }
    }
    impl From<CodecFlags> for u8 {
        fn from(from: CodecFlags) -> Self {
            from.0.into_iter().rev().enumerate()
                .filter_map(|(i, v)| v.then_some(1 << i)).fold(0u8, BitOr::bitor)
        }
    }
    impl<W: IoWrite> Encodable<W> for CodecFlags {
        fn encode(&self, encoder: &mut W) -> IoResult<usize> {
            u8::from(*self).encode(encoder)
        }
    }
}

#[rustfmt::skip]
mod join {
    use super::*;

    /// Encodes and decodes an iterator of encodables as a sequence with an optional `separator`.
    ///
    /// # Example
    /// ```
    /// use devela::{Encodable, CodecJoin};
    ///
    /// let compact_map = [ (c"hello", 1u8), (c"world", 2u8) ];
    /// let mut buf = [0u8; 64];
    /// let len = CodecJoin::new(&compact_map).encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"hello\0\x01world\0\x02");
    ///
    /// let mut buf = [0u8; 64];
    /// let array = ["hello", "world", "another"];
    /// let len = CodecJoin::with(&array, ", ").encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], b"hello, world, another");
    ///
    /// # #[cfg(feature = "alloc")] { use devela::Vec;
    /// let mut buf = Vec::new();
    /// // Note if you'd use '/' (a char) it would get encoded as [47, 0, 0, 0].
    /// let len = CodecJoin::with(&array, "/").encode(&mut buf).unwrap();
    /// assert_eq!(&buf, b"hello/world/another");
    /// # }
    /// ```
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CodecJoin<E, S> {
        encodable_iter: E,
        separator: Option<S>,
    }
    impl<E> CodecJoin<E, ()> {
        /// Creates a new [`CodecJoin`] combinator, without a separator.
        pub const fn new(encodable_iter: E) -> Self {
            Self { encodable_iter, separator: None::<()> }
        }
    }
    impl<E, S> CodecJoin<E, S> {
        /// Creates a new [`CodecJoin`] combinator with a `separator`.
        pub const fn with(encodable_iter: E, separator: S) -> Self {
            Self { encodable_iter, separator: Some(separator) }
        }
    }
    impl<E, S, W: IoWrite> Encodable<W> for CodecJoin<E, S>
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
                    iif![is_first; is_first = false; total += sep.encode(writer)?];
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
    /// use devela::{IoWrite, Encodable, CodecLen};
    ///
    /// let encodable = c"hello, world!";
    /// let mut encoder = CodecLen::new();
    /// encodable.encode(&mut encoder).unwrap();
    /// assert_eq!(encoder.size(), 14, "13 bytes from the ASCII string + 1 for the null terminator");
    /// ```
    #[must_use]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CodecLen { size: usize, }
    impl CodecLen {
        /// Creates a new [`CodecLen`].
        pub const fn new() -> Self { Self { size: 0 } }
        /// Returns the computed encoded size.
        pub const fn size(&self) -> usize { self.size }
    }
    impl From<CodecLen> for usize {
        fn from(encoder: CodecLen) -> usize { encoder.size }
    }
    impl FmtWrite for CodecLen {
        fn write_str(&mut self, s: &str) -> FmtResult<()> { self.size += s.len(); Ok(()) }
    }
    impl IoWrite for CodecLen {
        fn write(&mut self, slice: &[u8]) -> IoResult<usize> { self.size += slice.len(); Ok(self.size) }
        fn flush(&mut self) -> IoResult<()> { Ok(()) }
    }

    /// Encodes and decodes a length prefixed value ([TLV]).
    ///
    /// # Examples
    /// ```
    /// use devela::{Decodable, Encodable, CodecBe, CodecLe, CodecLenValue};
    ///
    /// // Encoding using a u16 len prefix
    /// let mut buf = [0u8; 64];
    /// let len = CodecLenValue::<_, u16, CodecBe<u16>>::new("hello")
    ///     .encode(&mut &mut buf[..])
    ///     .unwrap();
    /// assert_eq!(&buf[..len], &[0, 5, b'h', b'e', b'l', b'l', b'o'], "A big-endian u16 len");
    ///
    /// // Decoding
    /// # #[cfg(feature = "alloc")] { use devela::String;
    /// let mut reader = &buf[..];
    /// let decoded: String = CodecLenValue::<String, u16, CodecBe<u16>>::decode(&mut reader)
    ///     .unwrap();
    /// assert_eq!(decoded, "hello");
    /// # }
    /// ```
    /// [TLV]: https://en.wikipedia.org/wiki/Type–length–value
    #[must_use]
    #[doc(alias("length", "prefix", "TLV"))]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct CodecLenValue<E, LEN, CodecEndian> {
        encodable: E,
        phantom: PhantomData<(LEN, CodecEndian)>,
    }
    impl<E, LEN, CodecEndian> CodecLenValue<E, LEN, CodecEndian> {
        /// Creates a new TLV combinator.
        pub const fn new(encodable: E) -> Self { Self { encodable, phantom: PhantomData } }
    }
    impl<E, LEN, CodecEndian, W: IoWrite> Encodable<W> for CodecLenValue<E, LEN, CodecEndian>
    where
        E: Encodable<W> + EncodableLen,
        CodecEndian: From<LEN> + Encodable<W>,
        LEN: TryFrom<usize>,
    {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            let len = self.encodable.encoded_size()?;
            let len_encoded: LEN = LEN::try_from(len)
                .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Length conversion failed"))?;
            let mut total = CodecEndian::from(len_encoded).encode(writer)?; // encode the length
            total += self.encodable.encode(writer)?; // encode the actual content
            Ok(total)
        }
    }

    impl<D, O, LEN, CodecEndian, R> Decodable<R> for CodecLenValue<D, LEN, CodecEndian>
    where
        for<'i> D: Decodable<IoTake<&'i mut R>, Output = O>, // The TLV payload type to be decoded.
        CodecEndian: Decodable<R, Output = LEN>, // The type used to decode the length prefix.
        LEN: TryInto<usize>, // The type of the length prefix.
        R: IoRead, // The underlying reader type which implements IoRead.
        for<'r> &'r mut R: IoRead, // Ensure that &mut R itself implements IoRead
    {
        type Output = O;
        fn decode(reader: &mut R) -> IoResult<Self::Output> {
            let len_encoded: LEN = CodecEndian::decode(reader)?;
            let len: usize = len_encoded.try_into().map_err(|_| {
                IoError::new(IoErrorKind::InvalidData, "Invalid length value")
            })?;
            let mut limited_reader = reader.take(len as u64);
            D::decode(&mut limited_reader)
        }
    }

    // // An alternative that depends on cloning the reader.
    // impl<E, LEN, CodecEndian, R> Decodable<R> for CodecLenValue<E, LEN, CodecEndian>
    // where
    //     E: Decodable<IoTake<R>>,
    //     CodecEndian: Decodable<R, Output = LEN>,
    //     LEN: TryInto<usize>,
    //     R: IoRead + Clone,
    // {
    //     type Output = E::Output;
    //
    //     fn decode(reader: &mut R) -> IoResult<Self::Output> {
    //         let len_encoded: LEN = CodecEndian::decode(reader)?;
    //         let len: usize = len_encoded
    //             .try_into()
    //             .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid length value"))?;
    //         let mut limited_reader = reader.clone().take(len as u64);
    //         E::decode(&mut limited_reader)
    //     }
    // }

    // // An alternative that requires the reader to be of the exact length of len+value.
    // impl<D, LEN, CodecEndian, R: IoRead> Decodable<R> for CodecLenValue<D, LEN, CodecEndian>
    // where
    //     D: Decodable<R>,
    //     CodecEndian: Decodable<R, Output = LEN>,
    //     LEN: TryInto<usize>,
    // {
    //     type Output = D::Output;
    //     fn decode(reader: &mut R) -> IoResult<Self::Output> {
    //         let len_encoded: LEN = CodecEndian::decode(reader)?;
    //         let len: usize = len_encoded
    //             .try_into()
    //             .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid length value"))?;
    //
    //         /// Manually limit the read by tracking bytes read
    //         struct LimitedReader<'a, R: IoRead> {
    //             reader: &'a mut R,
    //             remaining: usize,
    //         }
    //         impl<R: IoRead> IoRead for LimitedReader<'_, R> {
    //             fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
    //                 iif![self.remaining == 0; return Ok(0)];
    //                 let len = buf.len().min(self.remaining);
    //                 let n = self.reader.read(&mut buf[..len])?;
    //                 self.remaining -= n;
    //                 Ok(n)
    //             }
    //         }
    //         let limited_reader = LimitedReader { reader, remaining: len };
    //         D::decode(limited_reader.reader)
    //     }
    // }
}
