// devela::data::codec::encode::impls
//
//! General implementations of [`Decodable`] and [`Encodable`].
//

use crate::{
    CStr, Char, Decodable, Encodable, Fmt, FmtArguments, FmtError, FmtResult, FmtWrite, IoError,
    IoErrorKind, IoRead, IoResult, IoWrite, NoData, Slice, sf,
};
crate::_use! {basic::from_utf8}

#[cfg(feature = "alloc")]
crate::items! {
    use crate::{is, Cow, CString, Box, String, ToOwned, Vec};

    impl<W: IoWrite> Encodable<W> for Vec<u8> {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self.as_slice()) } }
    impl<R: IoRead> Decodable<R> for Vec<u8> {
        type Output = Vec<u8>;
        fn decode(reader: &mut R) -> IoResult<Self::Output> {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            Ok(buf) } }

    impl<W: IoWrite, T: Encodable<W>> Encodable<W> for Box<T> {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            self.as_ref().encode(writer) } }
    impl<R: IoRead, T: Decodable<R>> Decodable<R> for Box<T> {
        type Output = Box<T::Output>;
        fn decode(reader: &mut R) -> IoResult<Self::Output> {
            Ok(Box::new(T::decode(reader)?)) } }

    impl<W: IoWrite> Encodable<W> for String {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self.as_bytes()) } }
    impl<R: IoRead> Decodable<R> for String {
        type Output = String;
        fn decode(reader: &mut R) -> IoResult<Self::Output> {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            from_utf8(&buf).map(|s| s.to_owned())
                .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid UTF-8"))
        }
    }
    impl<W: IoWrite> Encodable<W> for CString {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self.as_bytes()) } }
    impl<R: IoRead> Decodable<R> for CString {
        type Output = CString;
        fn decode(reader: &mut R) -> IoResult<Self::Output> {
            let mut buf = Vec::new();
            let mut byte = [0u8; 1];
            loop {
                reader.read_exact(&mut byte)?;
                is![byte[0] == 0; break];
                buf.push(byte[0]);
            }
            CString::new(buf).map_err(|_|
                IoError::new(IoErrorKind::InvalidData, "Invalid C string")) } }

    // only Encodable (for now?)
    impl<W: IoWrite, T: Clone + Encodable<W>> Encodable<W> for Cow<'_, T> {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            self.as_ref().encode(writer) } }
}

/* fmt */

#[rustfmt::skip]
#[allow(clippy::io_other_error, reason = "not implemented in core version")]
impl<W: IoWrite> Encodable<W> for FmtArguments<'_> {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        // Allows to use `Fmt::write` to format `FmtArguments` directly into `IoWrite`.
        struct InlineFmtAdapter<'a, W: IoWrite> {
            writer: &'a mut W,
            error: Option<IoError>,
            total: usize,
        }
        impl<W: IoWrite> FmtWrite for InlineFmtAdapter<'_, W> {
            fn write_str(&mut self, s: &str) -> FmtResult<()> {
                match self.writer.write(s.as_bytes()) {
                    Ok(n) => { self.total += n; Ok(()) }
                    Err(e) => { self.error = Some(e); Err(FmtError) }
                }
            }
        }
        let mut adapter = InlineFmtAdapter { writer, error: None, total: 0 };
        if Fmt::write(&mut adapter, *self).is_ok() { Ok(adapter.total) }
        else { Err(adapter.error
            .unwrap_or_else(|| IoError::new(IoErrorKind::Other, "Formatting failed"))) }
    }
}

/* primitives: char, byte, bool */

impl<W: IoWrite> Encodable<W> for char {
    fn encode(&self, w: &mut W) -> IoResult<usize> {
        let mut buf = [0; 4];
        let s = self.encode_utf8(&mut buf);
        w.write(s.as_bytes())
    }
}
impl<R: IoRead> Decodable<R> for char {
    type Output = char;
    fn decode(reader: &mut R) -> IoResult<Self::Output> {
        let mut buf = [0u8; 4]; // UTF-8 can be up to 4 bytes
        reader.read_exact(&mut buf[..1])?;
        let len = Char(buf[0]).len_utf8_unchecked();
        reader.read_exact(&mut buf[1..len])?;
        let utf8_str = from_utf8(&buf[..len])
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid UTF-8 sequence"))?;
        utf8_str
            .chars()
            .next()
            .ok_or_else(|| IoError::new(IoErrorKind::UnexpectedEof, "Empty UTF-8 sequence"))
    }
}

impl<W: IoWrite> Encodable<W> for u8 {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        writer.write(Slice::from_ref(self))
    }
}
impl<R: IoRead> Decodable<R> for u8 {
    type Output = u8;
    fn decode(reader: &mut R) -> IoResult<u8> {
        let mut buf = [0u8; size_of::<u8>()];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl<W: IoWrite> Encodable<W> for i8 {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        (*self as u8).encode(writer)
    }
}
impl<R: IoRead> Decodable<R> for i8 {
    type Output = i8;
    fn decode(reader: &mut R) -> IoResult<i8> {
        let mut buf = [0u8; size_of::<i8>()];
        reader.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
}

impl<W: IoWrite> Encodable<W> for bool {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        u8::from(*self).encode(writer)
    }
}
impl<R: IoRead> Decodable<R> for bool {
    type Output = bool;
    fn decode(reader: &mut R) -> IoResult<Self::Output> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        match buf[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(IoError::new(IoErrorKind::InvalidData, "Invalid boolean value")),
        }
    }
}

// only Encodable
/// Allows encoding of `&T` directly, forwarding calls to `T::encode()`.
impl<T: Encodable<W> + ?Sized, W: IoWrite> Encodable<W> for &T {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        T::encode(self, writer)
    }
}
// only Decodable
// impl<R: IoRead, T: Decodable<R>> Decodable<IoTake<R>> for T {
//     type Output = T::Output;
//     fn decode(reader: &mut IoTake<R>) -> IoResult<Self::Output> {
//         T::decode(reader)
//     }
// }

/* slices, arrays */

sf! {
    impl<W: IoWrite, const SIZE: usize> Encodable<W> for [u8; SIZE] {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self)
        }
    }
    impl<R: IoRead, const SIZE: usize> Decodable<R> for [u8; SIZE] {
        type Output = [u8; SIZE];
        fn decode(reader: &mut R) -> IoResult<Self::Output> {
            let mut buf = [0u8; SIZE];
            reader.read_exact(&mut buf)?;
            Ok(buf)
        }
    }

    impl<W: IoWrite> Encodable<W> for str {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self.as_bytes())
        }
    }
    impl<'a> Decodable<&'a mut &[u8]> for &'a str {
        type Output = Self;
        fn decode(reader: &mut &'a mut &[u8]) -> IoResult<Self::Output> {
            // In this specialized case, we assume the reader is already limited
            // to exactly the bytes for the string.
            let s = from_utf8(reader)
                .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid UTF-8"))?;
            // Consume the entire slice.
            **reader = &[];
            Ok(s)
        }
    }

    // only Encodable
    impl<W: IoWrite> Encodable<W> for [u8] {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self)
        }
    }
    // only Encodable
    impl<W: IoWrite> Encodable<W> for CStr {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self.to_bytes_with_nul())
        }
    }
}

/* tuples */

// https://users.rust-lang.org/t/macro-to-impl-trait-for-tuple/79165/3
macro_rules! impl_encodable_for_tuple {
    ($($T:tt)*) => { $crate::paste! {
        impl<WRITER, A, $($T,)*> Encodable<WRITER> for (A,$($T,)*)
        where
            WRITER: IoWrite,
            A: Encodable<WRITER>,
            $($T: Encodable<WRITER>,)*
        {
            fn encode(&self, writer: &mut WRITER) -> IoResult<usize> {
                let (a, $([<$T:lower>],)*) = self;
                let mut total = 0;
                total += a.encode(writer)?;
                $(
                    total += [<$T:lower>].encode(writer)?;
                )*
                Ok(total)
            }

        }
    }};
}
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V W X Y);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V W X);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V W);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O);
impl_encodable_for_tuple!(B C D E F G H I J K L M N);
impl_encodable_for_tuple!(B C D E F G H I J K L M);
impl_encodable_for_tuple!(B C D E F G H I J K L);
impl_encodable_for_tuple!(B C D E F G H I J K);
impl_encodable_for_tuple!(B C D E F G H I J);
impl_encodable_for_tuple!(B C D E F G H I);
impl_encodable_for_tuple!(B C D E F G H);
impl_encodable_for_tuple!(B C D E F G);
impl_encodable_for_tuple!(B C D E F);
impl_encodable_for_tuple!(B C D E);
impl_encodable_for_tuple!(B C D);
impl_encodable_for_tuple!(B C);
impl_encodable_for_tuple!(B);
impl_encodable_for_tuple!();

/* option, unit */

sf! {
    impl<T: Encodable<W>, W: IoWrite> Encodable<W> for Option<T> {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            match self { Some(value) => value.encode(writer), None => Ok(0) }
        }
    }
    impl<W: IoWrite> Encodable<W> for NoData {
        fn encode(&self, _writer: &mut W) -> IoResult<usize> { Ok(0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option() {
        let mut writer = &mut [0u8; 32] as &mut [u8];
        None::<Option<u8>>.encode(&mut writer).unwrap();
        Some(42u8).encode(&mut writer).unwrap();
    }
}
