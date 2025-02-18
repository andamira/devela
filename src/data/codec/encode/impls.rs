// devela::data::codec::encode::impls
//
//!

use crate::{
    CStr, Encodable, Fmt, FmtArguments, FmtError, FmtResult, FmtWrite, IoError, IoErrorKind,
    IoResult, IoWrite, NoData, Slice,
};

#[cfg(feature = "alloc")]
crate::items! {
    use crate::{Cow, Box, String, Vec};

    impl<W: IoWrite> Encodable<W> for Vec<u8> {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self.as_slice())
        }
    }
    impl<W: IoWrite, T: Encodable<W>> Encodable<W> for Box<T> {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            self.as_ref().encode(writer)
        }
    }
    impl<W: IoWrite, T: Clone + Encodable<W>> Encodable<W> for Cow<'_, T> {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            self.as_ref().encode(writer)
        }
    }
    impl<W: IoWrite> Encodable<W> for String {
        fn encode(&self, writer: &mut W) -> IoResult<usize> {
            writer.write(self.as_bytes())
        }
    }
}

/* fmt */

#[rustfmt::skip]
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

/* primitives */

impl<W: IoWrite> Encodable<W> for char {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        let mut buf = [0; 4];
        let s = self.encode_utf8(&mut buf);
        writer.write(s.as_bytes())
    }
}
impl<W: IoWrite> Encodable<W> for u8 {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        writer.write(Slice::from_ref(self))
    }
}
impl<W: IoWrite> Encodable<W> for i8 {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        (*self as u8).encode(writer)
    }
}
impl<W: IoWrite> Encodable<W> for bool {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        u8::from(*self).encode(writer)
    }
}
impl<T: Encodable<W> + ?Sized, W: IoWrite> Encodable<W> for &T {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        T::encode(self, writer)
    }
}

/* slices, arrays */

impl<W: IoWrite> Encodable<W> for [u8] {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        writer.write(self)
    }
}
impl<W: IoWrite, const SIZE: usize> Encodable<W> for [u8; SIZE] {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        writer.write(self)
    }
}
impl<W: IoWrite> Encodable<W> for str {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        writer.write(self.as_bytes())
    }
}
impl<W: IoWrite> Encodable<W> for CStr {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        writer.write(self.to_bytes_with_nul())
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

#[rustfmt::skip]
impl<T: Encodable<W>, W: IoWrite> Encodable<W> for Option<T> {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        match self { Some(value) => value.encode(writer), None => Ok(0) }
    }
}
#[rustfmt::skip]
impl<W: IoWrite> Encodable<W> for NoData {
    fn encode(&self, _writer: &mut W) -> IoResult<usize> { Ok(0) }
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
