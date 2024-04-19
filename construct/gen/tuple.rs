// devela construct::gen::tuple
//
//!
//
// We're generating code here because declarative macros are too limited.
// This also seem to offer better performance for large arities.

use std::fs::{create_dir_all, File};
use std::io::{Error, Write};
use std::{write as w0, writeln as w};

#[rustfmt::skip] const MAX_ARITY: usize = {
    if cfg!(not(feature = "_tuple_arity_15")) { 7
    } else if cfg!(all(feature = "_tuple_arity_15", not(feature = "_tuple_arity_31"))) { 15
    } else if cfg!(all(feature = "_tuple_arity_31", not(feature = "_tuple_arity_63"))) { 31
    } else if cfg!(all(feature = "_tuple_arity_63", not(feature = "_tuple_arity_95"))) { 63
    } else if cfg!(all(feature = "_tuple_arity_95", not(feature = "_tuple_arity_127"))) { 95
    } else { 127 }
};

#[rustfmt::skip]
pub(super) fn generate() -> Result<(), Error> {
    create_dir_all("construct/out/").unwrap();
    let path = "construct/out/tuple.rs";
    let mut f = File::create(path)?;

    /* trait definition */
    // --------------------------------------------------------------------------

    w!(f, r#"
    /// Extension trait providing convenience methods for [tuples][tuple].
    ///
    /// Tuples are random-access, sequentially allocated, statically sized,
    /// heterogeneous data structures.
    ///
    /// They enable structured grouping and access to a sequence of different types.
    ///
    /// # Features
    /// By default it's implemented for tuples of arity of 15 or less.
    /// It supports increased arities of 31, 63, 95 and 127 by enabling the
    /// corresponding capability feature: `_tuple_arity_[31|63|95|127]`.
    "#)?;
    w!(f, "pub trait Tuple {{")?;

    // constants
    w!(f, "/// The arity of this tuple (the number of contained elements)")?;
    w!(f, "const ARITY: usize;")?;

    // name fields
    w!(f, "/// The first element of the tuple, at index 0.")?;
    w!(f, "type Head;")?;
    w!(f, "/// The last element of the tuple, at index `ARITY-1`.")?;
    w!(f, "type Tail;")?;

    w!(f, "/// This tuple without its head.")?;
    w!(f, "type NoHead;")?;
    w!(f, "/// This tuple without its tail.")?;
    w!(f, "type NoTail;")?;
    w!(f, "/// This tuple with an extra element `T` appended to it.")?;
    w!(f, "type Append<T>;")?;
    w!(f, "/// This tuple with an extra element `T` prepended to it.")?;
    w!(f, "type Prepend<T>;")?;

    // index fields
    for i in 0..MAX_ARITY {
        w!(f, "/// The type of the element at index {}.", i)?;
        w!(f, "type _{};", i)?;
    }

    w!(f, r#"
    /// The maximum arity supported by the current compilation options.
    ///
    /// See the available [features](#features) to increase this number.
    "#)?;
    w!(f, "const MAX_ARITY: usize = {MAX_ARITY};")?;

    // methods

    w!(f, "/// Returns a shared reference to the head of this tuple.
        #[must_use]
        fn head(&self) -> &Self::Head;")?;
    w!(f, "/// Returns a shared reference to the tail of this tuple.
        #[must_use]
        fn tail(&self) -> &Self::Tail;")?;

    w!(f, "/// Returns an exclusive reference to the head of this tuple.
        #[must_use]
        fn head_mut(&mut self) -> &mut Self::Head;")?;
    w!(f, "/// Returns an exclusive reference to the tail of this tuple.
        #[must_use]
        fn tail_mut(&mut self) -> &mut Self::Tail;")?;

    w!(f, "/// Returns this tuple with the head element splitted from the rest.
        #[must_use]
        fn split_head(self) -> (Self::Head, Self::NoHead);")?;
    w!(f, "/// Returns this tuple with the tail element splitted from the rest.
        #[must_use]
        fn split_tail(self) -> (Self::NoTail, Self::Tail);")?;

    w!(f, "/// Returns this tuple without the head.
        #[must_use]
        fn no_head(self) -> Self::NoHead;")?;
    w!(f, "/// Returns this tuple without the tail.
        #[must_use]
        fn no_tail(self) -> Self::NoTail;")?;

    w!(f, "/// Appends the given `value` to this tuple.
        #[must_use]
        fn append<T>(self, value: T) -> Self::Append<T>;")?;
    w!(f, "/// Prepends the given `value` to this tuple.
        #[must_use]
        fn prepend<T>(self, value: T) -> Self::Prepend<T>;")?;

    w!(f, "/// Returns a shared reference to the `nth` element,
    /// or `None` if `nth >= ARITY`.
    #[allow(clippy::type_complexity)]
    fn nth_ref(&self, nth: usize) -> TupleEnumRef<")?;
        for i in 0..MAX_ARITY { w!(f, "Self::_{i},")?; }
        w!(f, ">;")?;

    w!(f, "/// Returns an exclusive reference to the `nth` element,
    /// or `None` if `nth >= ARITY`.
    #[allow(clippy::type_complexity)]
    fn nth_mut(&mut self, nth: usize) -> TupleEnumMut<")?;
        for i in 0..MAX_ARITY { w!(f, "Self::_{i},")?; }
        w!(f, ">;")?;


    // auto-implementations

    w!(f, "/// Wraps the tuple in a [`TupleFmt`] for formatting purposes.
        fn fmt(&self) -> TupleFmt<Self> where Self: Sized {{ TupleFmt(self) }}")?;

    w!(f, "}}")?; // end impl Tuple


    /* enum definitions */
    // --------------------------------------------------------------------------

    w!(f, "/// All the possible types in a [`Tuple`] as shared references.")?;
    w!(f, "#[non_exhaustive]")?;
    w!(f, "pub enum TupleEnumRef<'a, ")?;
    for i in 0..MAX_ARITY { w!(f, "_{i},")?; }
    w!(f, "> {{")?;
    // variants
    w!(f, "/// No type.")?;
    w!(f, "None,")?;
    for i in 0..MAX_ARITY {
        w!(f, "/// A shared reference to the tuple element at index {i}.")?;
        w!(f, "_{i}(&'a _{i}),")?;
    }
    w!(f, "}}")?;

    w!(f, "/// All the possible types in a [`Tuple`] as exclusive references.")?;
    w!(f, "#[non_exhaustive]")?;
    w!(f, "pub enum TupleEnumMut<'a, ")?;
    for i in 0..MAX_ARITY { w!(f, "_{i},")?; }
    w!(f, "> {{")?;
    // variants
    w!(f, "/// No type.")?;
    w!(f, "None,")?;
    for i in 0..MAX_ARITY {
        w!(f, "/// An exclusive reference to the tuple element at index {i}.")?;
        w!(f, "_{i}(&'a mut _{i}),")?;
    }
    w!(f, "}}")?;


    /* manual implementations for arities of 0 and 1 */
    // --------------------------------------------------------------------------

    /* arity 0 */
    w!(f, r#"impl Tuple for () {{
        const ARITY: usize = 0;

        type Head = ();
        type Tail = ();
        type NoHead = ();
        type NoTail = ();
        type Append<T> = (T,);
        type Prepend<T> = (T,);

        // methods
        fn head(&self) -> &Self::Head {{ self }}
        fn tail(&self) -> &Self::Tail {{ self }}
        fn head_mut(&mut self) -> &mut Self::Head {{ self }}
        fn tail_mut(&mut self) -> &mut Self::Tail {{ self }}
        fn split_head(self) -> (Self::Head, Self::NoHead) {{ ((), ()) }}
        fn split_tail(self) -> (Self::NoTail, Self::Tail) {{ ((), ()) }}
        fn no_head(self) -> Self::NoHead {{}}
        fn no_tail(self) -> Self::NoTail {{}}
        fn append<T>(self, value: T) -> Self::Append<T> {{ (value,) }}
        fn prepend<T>(self, value: T) -> Self::Prepend<T> {{ (value,) }}
    "#)?;
        // index types
        for i in 0..MAX_ARITY { w!(f, "type _{i} = ();")?; }

        // methods
        w!(f, "fn nth_ref(&self, _index: usize) -> TupleEnumRef<")?;
        for i in 0..MAX_ARITY { w!(f, "Self::_{i},")?; } w!(f, "> {{")?;
            w!(f, "TupleEnumRef::None")?;
        w!(f, "}}")?;
        w!(f, "fn nth_mut(&mut self, _index: usize) -> TupleEnumMut<")?;
        for i in 0..MAX_ARITY { w!(f, "Self::_{i},")?; } w!(f, "> {{")?;
            w!(f, "TupleEnumMut::None")?;
        w!(f, "}}")?;


    w!(f, "}}")?;

    w!(f, r#"
    impl TupleDebug for () {{
        fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {{
            f.debug_tuple("").finish()
        }}
    }}
    impl TupleDisplay for () {{
        fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {{
            write!(f, "()")
        }}
    }}
    "#)?;


    /* arity 1 */
    w!(f, r#"impl<_0> Tuple for (_0,) {{
        const ARITY: usize = 1;

        type Head = _0;
        type Tail = _0;
        type NoHead = ();
        type NoTail = ();
        type Append<T> = (_0, T);
        type Prepend<T> = (T, _0);

        // methods
        fn head(&self) -> &Self::Head {{ &self.0 }}
        fn tail(&self) -> &Self::Tail {{ &self.0 }}
        fn head_mut(&mut self) -> &mut Self::Head {{ &mut self.0 }}
        fn tail_mut(&mut self) -> &mut Self::Tail {{ &mut self.0 }}
        fn split_head(self) -> (Self::Head, Self::NoHead) {{ (self.0, ()) }}
        fn split_tail(self) -> (Self::NoTail, Self::Tail) {{ ((), self.0) }}
        fn no_head(self) -> Self::NoHead {{}}
        fn no_tail(self) -> Self::NoTail {{}}
        fn append<T>(self, value: T) -> Self::Append<T> {{ (self.0, value) }}
        fn prepend<T>(self, value: T) -> Self::Prepend<T> {{ (value, self.0) }}
    "#)?;
        // index types
        w!(f, "type _0 = _0;")?;
        for i in 1..MAX_ARITY { w!(f, "type _{i} = ();")?; }

        // methods
        w!(f, "fn nth_ref(&self, index: usize) -> TupleEnumRef<")?;
        for i in 0..MAX_ARITY { w!(f, "Self::_{i},")?; } w!(f, "> {{")?;
            w!(f, "match index {{")?;
                w!(f, "0 => TupleEnumRef::_0(&self.0),")?;
                w!(f, "_ => TupleEnumRef::None,")?;
            w!(f, "}}")?;
        w!(f, "}}")?;
        w!(f, "fn nth_mut(&mut self, index: usize) -> TupleEnumMut<")?;
        for i in 0..MAX_ARITY { w!(f, "Self::_{i},")?; } w!(f, "> {{")?;
            w!(f, "match index {{")?;
                w!(f, "0 => TupleEnumMut::_0(&mut self.0),")?;
                w!(f, "_ => TupleEnumMut::None,")?;
            w!(f, "}}")?;
        w!(f, "}}")?;

    w!(f, "}}")?;

    w!(f, r#"
    impl<_0: Debug> TupleDebug for (_0,) {{
        fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {{
            f.debug_tuple("").field(&self.0).finish()
        }}
    }}
    impl<_0: Display> TupleDisplay for (_0,) {{
        fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {{
            write!(f, "({{}},)", &self.0)
        }}
    }}
    "#)?;


    /* auto implementations for arities >= 2 */
    // --------------------------------------------------------------------------

    for arity in 2..=MAX_ARITY {
        w0!(f,
            "
            // #[rustfmt::skip]
            impl<{0}> Tuple for ({0}) {{",
            (0..arity).map(|i| format!("_{}", i)).collect::<Vec<_>>().join(", ")
        )?;

        // constants
        w!(f, "const ARITY: usize = {arity};")?;

        // name fields
        w!(f, "/// The first element of the tuple, at index 0.")?;
        w!(f, "type Head = _0;")?;
        w!(f, "/// The last element of the tuple, at index `ARITY-1`.")?;
        w!(f, "type Tail = _{};", arity-1)?;

        w!(f, "///.")?;
        w!(f, "type NoHead = (")?; for i in 1..arity { w0!(f, "_{i},")?; } w!(f, ");")?;
        w!(f, "type NoTail = (")?; for i in 0..arity-1 { w0!(f, "_{i},")?; } w!(f, ");")?;
        w!(f, "type Append<T> = (")?; for i in 0..arity { w0!(f, "_{i},")?; } w!(f, "T);")?;
        w!(f, "type Prepend<T> = (T, ")?; for i in 0..arity { w0!(f, "_{i},")?; } w!(f, ");")?;

        // index fields
        w!(f, "/// The type of the element at index 0 (the first field).")?;
        w!(f, "type _0 = _0;")?;
        for i in 1..arity-1 {
            w!(f, "/// The type of the element at index {i}.")?;
            w!(f, "type _{i} = _{i};")?;
        }
        if arity > 1 {
            w!(f, "/// The type of the element at index {} (the last field).", arity-1)?;
            w!(f, "type _{0} = _{0};", arity-1)?;
        }
        for i in arity..MAX_ARITY {
            w!(f, "/// Non-existing element with current arity.")?;
            w!(f, "type _{i} = ();")?;
        }

        /* impl trait methods */

        w!(f, "fn head(&self) -> &Self::Head {{ &self.0 }}")?;
        w!(f, "fn tail(&self) -> &Self::Tail {{ &self.{} }}", arity-1)?;
        w!(f, "fn head_mut(&mut self) -> &mut Self::Head {{ &mut self.0 }}")?;
        w!(f, "fn tail_mut(&mut self) -> &mut Self::Tail {{ &mut self.{} }}", arity-1)?;

        w!(f, "fn split_head(self) -> (Self::Head, Self::NoHead) {{ (self.0, (")?;
            for i in 1..arity { w0!(f, "self.{i},")?; }
        w!(f, ")) }}")?;

        w!(f, "fn split_tail(self) -> (Self::NoTail, Self::Tail) {{ ((")?;
            for i in 0..arity-1 { w0!(f, "self.{i},")?; }
        w!(f, "), self.{}) }}", arity-1)?;

        w!(f, "fn no_head(self) -> Self::NoHead {{ (")?;
            for i in 1..arity { w0!(f, "self.{i},")?; }
        w!(f, ") }}")?;
        w!(f, "fn no_tail(self) -> Self::NoTail {{ (")?;
            for i in 0..arity-1 { w0!(f, "self.{i},")?; }
        w!(f, ") }}")?;

        w!(f, "fn append<T>(self, value: T) -> Self::Append<T> {{ (")?;
            for i in 0..arity { w0!(f, "self.{i},")?; }
        w!(f, "value) }}")?;
        w!(f, "fn prepend<T>(self, value: T) -> Self::Prepend<T> {{ (value, ")?;
            for i in 0..arity { w0!(f, "self.{i},")?; }
        w!(f, ") }}")?;

        w!(f, "fn nth_ref(&self, index: usize) -> TupleEnumRef<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, "> {{")?;
            w!(f, "match index {{")?;
                for i in 0..arity { w0!(f, "{i} => TupleEnumRef::_{i}(&self.{i}),")?; }
                w!(f, "_ => TupleEnumRef::None")?;
            w!(f, "}}")?;
        w!(f, "}}")?;

        w!(f, "fn nth_mut(&mut self, index: usize) -> TupleEnumMut<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, "> {{")?;
            w!(f, "match index {{")?;
                for i in 0..arity { w0!(f, "{i} => TupleEnumMut::_{i}(&mut self.{i}),")?; }
                w!(f, "_ => TupleEnumMut::None")?;
            w!(f, "}}")?;
        w!(f, "}}")?;


        // -----------------------------
        w!(f, "}}")?; // end impl Tuple

        /* impl other traits */

        w!(f, "#[rustfmt::skip]")?;
        w!(f, "impl<")?; for i in 0..arity { w0!(f, "_{i}: Debug,")?; }
        w!(f, "> TupleDebug for (")?; for i in 0..arity { w0!(f, "_{i},")?; }
        w!(f, ") {{\n fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
            w!(f, "f.debug_tuple(\"\")")?;
                for i in 0..arity { w!(f, ".field(&self.{i})")?; }
                w!(f, ".finish()")?;
        w!(f, "}}\n }}")?;

        w!(f, "#[rustfmt::skip]")?;
        w!(f, "impl<")?; for i in 0..arity { w0!(f, "_{i}: Display,")?; }
        w!(f, "> TupleDisplay for (")?; for i in 0..arity { w0!(f, "_{i},")?; }
        w!(f, ") {{\n fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
            w!(f, "write!(f, \"({{}}\", &self.0)?;")?;
                for i in 1..arity { w!(f, "write!(f, \", {{}}\", &self.{i})?;")?; }
            w!(f, "write!(f, \")\")")?;
        w!(f, "}}\n }}")?;
    }

    #[cfg(doc)] // format the source if we're building the docs
    crate::rustfmt_file(path);
    Ok(())
}
