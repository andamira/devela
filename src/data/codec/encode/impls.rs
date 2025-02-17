// devela::data::codec::encode::impls
//
//!

use crate::{
    CStr, Encodable, Fmt, FmtArguments, FmtError, FmtResult, FmtWrite, IoError, IoResult, IoWrite,
    Slice,
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

impl<W: IoWrite> Encodable<W> for FmtArguments<'_> {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        let mut adapter = FmtAdapter { writer, error: None, total: 0 };
        if Fmt::write(&mut adapter, *self).is_ok() {
            Ok(adapter.total)
        } else {
            Err(adapter.error.expect("FmtAdapter always sets error on failure"))
        }
    }
}
/// Acts as a bridge between [`FmtWrite`] and [`IoWrite`].
///
/// It allows to use `Fmt::write` to format `FmtArguments` directly into `IoWrite`.
///
/// This conversion is necessary because:
/// - `FmtWrite` works with `&str` while `IoWrite` works with `&[u8]`.
/// - `FmtWrite` does not return the byte count, but `IoWrite` does.
/// - `FmtWrite` uses `FmtError` but we need to store `IoError`.
struct FmtAdapter<'a, W> {
    writer: &'a mut W,
    error: Option<IoError>,
    /// Track the total number of bytes written
    total: usize,
}
impl<W: IoWrite> FmtWrite for FmtAdapter<'_, W> {
    fn write_str(&mut self, s: &str) -> FmtResult<()> {
        // if let Err(error) = self.writer.write(s.as_bytes()) {
        //     self.error = Some(error);
        //     Err(FmtError)
        // } else {
        //     Ok(())
        // }
        match self.writer.write(s.as_bytes()) {
            Ok(n) => {
                self.total += n;
                Ok(())
            }
            Err(error) => {
                self.error = Some(error);
                Err(FmtError)
            }
        }
    }
}

/* option_result */

impl<T: Encodable<W>, W: IoWrite> Encodable<W> for Option<T> {
    fn encode(&self, writer: &mut W) -> IoResult<usize> {
        match self {
            Some(value) => value.encode(writer),
            None => Ok(0),
        }
    }
}
// RETHINK?
// impl<T: Encodable<W>, W: IoWrite> Encodable<W> for IoResult<T> {
//     fn encode(&self, writer: &mut W) -> IoResult<usize> {
//         match self {
//             Ok(value) => value.encode(writer),
//             Err(err) => Err(err.clone()),
//         }
//     }
// }

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
        #[allow(clippy::cast_sign_loss)] // CHECK REVIEW
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

/* slices */

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

impl<W: IoWrite> Encodable<W> for () {
    fn encode(&self, _writer: &mut W) -> IoResult<usize> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_that_some_option_can_be_encoded() {
        let mut writer = &mut [0u8; 32] as &mut [u8];
        Some(42u8).encode(&mut writer).unwrap();
    }

    #[test]
    fn assert_that_none_encodes_nothing() {
        let mut writer = &mut [0u8; 32] as &mut [u8];
        let option: Option<u8> = None;
        option.encode(&mut writer).unwrap();
    }
}
