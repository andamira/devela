// devela::data::codec::encode::combinators
//
//! Defines
//! [`CodecBe`], [`CodecIf`], [`CodecLe`], [`CodecFlags`], [`CodecJoin`], [`CodecLenValue`].
//
// TOC
pub use {
    cond::CodecIf,
    endian::{CodecBe, CodecLe},
    flags::CodecFlags,
    join::CodecJoin,
    len::{CodecLen, CodecLenValue},
};

use crate::{
    _TAG_CODEC, BitOr, Debug, Decodable, Deref, Encodable, EncodableLen, FmtResult, FmtWrite,
    Formatter, IoError, IoErrorKind, IoRead, IoResult, IoTake, IoWrite, NonZero, PhantomData,
    TryFromIntError, is,
};
crate::_use! {basic::from_utf8}

#[rustfmt::skip]
mod endian {
    use super::*;

    #[doc = _TAG_CODEC!()]
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
    /// # #[cfg(feature = "alloc")] {
    /// let mut buf = Vec::<u8>::new();
    /// CodecBe::new(1u16).encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[0, 1]);
    /// # }
    /// ```
    #[doc = crate::_doc!(vendor: "encode")]
    #[must_use]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CodecBe<W> {
        num: W,
    }
    impl<W> CodecBe<W> {
        /// Creates a new [`CodecBe`] combinator.
        pub const fn new(num: W) -> Self { Self { num } }
    }

    #[doc = _TAG_CODEC!()]
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
    /// # #[cfg(feature = "alloc")] {
    /// let mut buf = Vec::<u8>::new();
    /// CodecLe::new(1u16).encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[1, 0]);
    /// # }
    /// ```
    #[doc = crate::_doc!(vendor: "encode")]
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
            impl_endian![int: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize];
            impl_endian![prim: usize]; impl_endian![non0: usize];
            impl_endian![float: f32, f64];
            // impl_endian![float: f16, f128]; // TODO
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

    #[doc = _TAG_CODEC!()]
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
    #[doc = crate::_doc!(vendor: "encode")]
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

    #[doc = _TAG_CODEC!()]
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
    /// # #[cfg(feature = "alloc")] {
    /// let mut buf = Vec::new();
    /// CodecFlags::new([true, false, false, true, false, false, false, false])
    ///     .encode(&mut buf).unwrap();
    /// assert_eq!(&buf, &[0b_1001_0000]);
    /// # }
    /// ```
    #[doc = crate::_doc!(vendor: "encode")]
    #[must_use]
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CodecFlags([bool; 8]);
    impl CodecFlags {
        /// Creates a new [`CodecFlags`] combinator from 8 `bool`s.
        pub const fn new(flags: [bool; 8]) -> Self { Self(flags) }
        /// Creates a new [`CodecFlags`] from a slice of `bool`s.
        ///
        /// Takes the first 8 `bool`s and fills missing with `false`.
        pub fn from_slice(slice: &[bool]) -> Self {
            let mut flags = [false; 8];
            for (i, &b) in slice.iter().take(8).enumerate() { flags[i] = b; }
            Self(flags)
        }
        /// Creates a new [`CodecFlags`] from a slice of arbitrary types.
        ///
        /// The closure `f` is run for each element.
        pub fn from_iter<T, I, F>(iter: I, mut f: F) -> Self
        where
            I: IntoIterator<Item = T>,
            F: FnMut(T) -> bool,
        {
            let mut flags = [false; 8];
            for (i, v) in iter.into_iter().take(8).enumerate() {
                flags[i] = f(v);
            }
            Self(flags)
        }
    }
    impl Debug for CodecFlags {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write!(f, "CodecFlags({:08b})", u8::from(*self)) }
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
            Self(slice) }
    }
    impl From<CodecFlags> for u8 {
        fn from(from: CodecFlags) -> Self {
            from.0.into_iter().rev().enumerate()
                .filter_map(|(i, v)| v.then_some(1 << i)).fold(0u8, BitOr::bitor) }
    }
    impl<W: IoWrite> Encodable<W> for CodecFlags {
        fn encode(&self, encoder: &mut W) -> IoResult<usize> {
            u8::from(*self).encode(encoder) }
    }
}
#[rustfmt::skip]
mod join {
    use super::*;

    #[doc = _TAG_CODEC!()]
    /// Encodes and decodes an iterator as a sequence with an optional `separator`.
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
    /// # #[cfg(feature = "alloc")] {
    /// let mut buf = Vec::new();
    /// // Note if you'd use '/' (a char) it would get encoded as [47, 0, 0, 0].
    /// let len = CodecJoin::with(&array, "/").encode(&mut buf).unwrap();
    /// assert_eq!(&buf, b"hello/world/another");
    /// # }
    /// ```
    #[doc = crate::_doc!(vendor: "encode")]
    #[must_use]
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
                    is![is_first; is_first = false; total += sep.encode(writer)?];
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

    #[doc = _TAG_CODEC!()]
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
    #[doc = crate::_doc!(vendor: "encode")]
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

    #[doc = _TAG_CODEC!()]
    /// Encodes and decodes a length prefixed value
    /// (<abbr title = "Type-Length-Value">[TLV]</abbr>).
    ///
    /// Encodes a value by **prefixing it with its length** using a fixed-size integer.
    ///
    /// The length must either be a `u8`, or explicitly encoded in either big-endian
    /// by using [`CodecBe`] or little-endian by using [`CodecLe`].
    ///
    /// [TLV]: https://en.wikipedia.org/wiki/Type–length–value
    ///
    /// # Examples
    /// ```
    /// use devela::{Decodable, Encodable, CodecBe, CodecLenValue};
    ///
    /// // Non-allocating
    /// let mut buf = [0u8; 64];
    /// let len = CodecLenValue::<_, u8>::new("hello").encode(&mut &mut buf[..]).unwrap();
    /// assert_eq!(&buf[..len], &[5, b'h', b'e', b'l', b'l', b'o']);
    /// //
    /// let mut reader = &buf[..];
    /// let decoded: &str = CodecLenValue::<&str, u8>::decode(&mut &mut reader).unwrap();
    /// assert_eq!(decoded, "hello");
    ///
    /// # #[cfg(feature = "alloc")] {
    /// let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
    /// incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud
    /// exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure
    /// dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.";
    /// // Allocating
    /// let mut buf = Vec::new();
    /// let len = CodecLenValue::<_, CodecBe<u16>>::new(lorem).encode(&mut buf).unwrap();
    /// assert_eq!(&buf[..7], &[1, 78, b'L', b'o', b'r', b'e', b'm'], "A big-endian u16 len");
    /// assert_eq![len, 336];
    /// //
    /// let mut reader = buf.as_slice();
    /// let decoded: String = CodecLenValue::<String, CodecBe<u16>>::decode(&mut reader).unwrap();
    /// assert_eq!(decoded, lorem);
    /// # }
    /// ```
    ///
    /// The length must fit the given type:
    /// ```
    /// # #[cfg(feature = "alloc")] {
    /// use devela::{Encodable, CodecLe, CodecLenValue};
    ///
    /// let mut buf = Vec::new();
    /// assert![CodecLenValue::<_, u8>::new("*".repeat(451)).encode(&mut buf).is_err()];
    /// assert![CodecLenValue::<_, CodecLe<u16>>::new("*".repeat(451)).encode(&mut buf).is_ok()];
    /// # }
    /// ```
    #[doc = crate::_doc!(vendor: "encode")]
    #[must_use]
    #[doc(alias("length", "prefix", "TLV"))]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct CodecLenValue<E, CodecEndian> {
        encodable: E,
        phantom: PhantomData<CodecEndian>,
    }
    impl<E, CodecEndian> CodecLenValue<E, CodecEndian> {
        /// Creates a new <abbr title = "Type-Length-Value">TLV</abbr> combinator.
        pub const fn new(encodable: E) -> Self { Self { encodable, phantom: PhantomData, } }
    }
    impl<E, CodecEndian, W: IoWrite> Encodable<W> for CodecLenValue<E, CodecEndian>
    where
        E: Encodable<W> + EncodableLen,
        CodecEndian: From<<CodecEndian as CodecEndianLen>::Len> + Encodable<W> + CodecEndianLen,
        <CodecEndian as CodecEndianLen>::Len: TryFrom<usize>,
    {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            let len = self.encodable.encoded_size()?;
            let len = <<CodecEndian as CodecEndianLen>::Len as TryFrom<usize>>::try_from(len)
                .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Length conversion failed"))?;
            let mut total = CodecEndian::from(len).encode(writer)?;
            total += self.encodable.encode(writer)?;
            Ok(total)
        }
    }
    impl<D, O, CodecEndian, R> Decodable<R> for CodecLenValue<D, CodecEndian>
    where
        // D must be decodable from an IoTake wrapping &mut R.
        for<'i> D: Decodable<IoTake<&'i mut R>, Output = O>,
        // CodecEndian decodes the length prefix from R and its output is the associated Len.
        CodecEndian: Decodable<R, Output = <CodecEndian as CodecEndianLen>::Len> + CodecEndianLen,
        R: IoRead,
        for<'r> &'r mut R: IoRead,
    {
        type Output = O;
        fn decode(reader: &mut R) -> IoResult<Self::Output> {
            let len_encoded = CodecEndian::decode(reader)?;
            let len: usize = len_encoded.try_into().map_err(|_| {
                IoError::new(IoErrorKind::InvalidData, "Invalid length value")
            })?;
            let mut limited_reader = reader.take(len as u64);
            D::decode(&mut limited_reader)
        }
    }

    // Specialized implementation for &str.
    impl<'a, CodecEndian> Decodable<&'a mut &'a [u8]> for CodecLenValue<&'a str, CodecEndian>
    where
        CodecEndian: CodecEndianLen + Decodable<&'a mut &'a [u8],
            Output = <CodecEndian as CodecEndianLen>::Len>,
        <CodecEndian as CodecEndianLen>::Len: TryInto<usize>,
    {
        type Output = &'a str;
        fn decode(reader: &mut &'a mut &'a [u8]) -> IoResult<Self::Output> {
            fn decode_str<'a>(buf: &mut &'a [u8], len: usize) -> IoResult<&'a str> {
                if buf.len() < len {
                    return Err(IoError::new(IoErrorKind::UnexpectedEof, "Not enough bytes"));
                }
                let (s_bytes, rest) = buf.split_at(len);
                *buf = rest;
                from_utf8(s_bytes)
                    .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid UTF-8"))
            }
            let len_encoded = CodecEndian::decode(reader)?;
            let len: usize = len_encoded.try_into().map_err(|_| {
                IoError::new(IoErrorKind::InvalidData, "Invalid length value")
            })?;
            decode_str(reader, len)
        }
    }

    #[doc = _TAG_CODEC!()]
    /// A private helper trait to tie a length type to the endian codec.
    ///
    /// This trait ensures that `CodecLenValue` only accepts explicit endianness encoders
    /// (`CodecBe<T>` or `CodecLe<T>`) for encoding and decoding length prefixes
    /// using unsized primitives, or simply `u8` where endianess is irrelevant.
    ///
    /// Justifications:
    /// - Only endianness matters when encoding a fixed-length integer.
    /// - Prevents accidental misuse by requiring an explicit choice.
    /// - Keeps the API clean and avoids unnecessary complexity.
    trait CodecEndianLen { type Len: TryInto<usize> + TryFrom<usize>; }
    macro_rules! impl_codec_endian_len { ($($T:ty),+) => { $(
        impl CodecEndianLen for CodecLe<$T> { type Len = $T; }
        impl CodecEndianLen for CodecBe<$T> { type Len = $T; }
    )+ }; }
    impl_codec_endian_len![u8, u16, u32, usize];
    impl CodecEndianLen for u8 { type Len = u8; }
}
