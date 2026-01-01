// devela::build::codegen::tuple
//
//! Code generator for the [`Tuple`] trait, `TupleElement*` enums and `TupleIter*` structs.
//
// TOC
// - Tuple trait definition
// - manual Tuple impls for arities < 2
// - automatic Tuple impls for arities >= 2
// - enums definitions
// - iterators definitions and implementations
//

use super::super::Build;
use std::{
    fs::{File, create_dir_all},
    io::{BufWriter, Error, Write},
    write as w0, writeln as w,
};

use tabs::*;
#[rustfmt::skip]
#[allow(unused)]
mod tabs {
    #[doc = "0 tabs, 0 spaces."]  pub const TAB0: &str = "";
    #[doc = "1 tabs, 4 spaces."]  pub const TAB1: &str = "    ";
    #[doc = "2 tabs, 8 spaces."]  pub const TAB2: &str = "        ";
    #[doc = "3 tabs, 12 spaces."] pub const TAB3: &str = "            ";
    #[doc = "4 tabs, 16 spaces."] pub const TAB4: &str = "                ";
    #[doc = "5 tabs, 20 spaces."] pub const TAB5: &str = "                    ";
    #[doc = "6 tabs, 24 spaces."] pub const TAB6: &str = "                        ";
    #[doc = "7 tabs, 28 spaces."] pub const TAB7: &str = "                            ";
    #[doc = "8 tabs, 32 spaces."] pub const TAB8: &str = "                                ";
}

#[rustfmt::skip] const MAX_ARITY: usize = {
    if cfg!(not(feature = "_tuple_24")) { 12
    } else if cfg!(all(feature = "_tuple_24", not(feature = "_tuple_36"))) { 24
    } else if cfg!(all(feature = "_tuple_36", not(feature = "_tuple_48"))) { 36
    } else if cfg!(all(feature = "_tuple_48", not(feature = "_tuple_72"))) { 48
    } else { 72 }
};

#[rustfmt::skip]
pub(crate) fn generate() -> Result<(), Error> {
    let build_out_dir = Build::out_dir().join("build/");
    create_dir_all(&build_out_dir)?;
    let path = build_out_dir.join("tuple.rs");

    // the generated file will be imported from /src/data/collections/tuple/mod.rs
    #[cfg(feature = "__dbg")]
    Build::println(&format!("generated: {}", path.display()));

    let file = File::create(path)?;
    let mut f = BufWriter::new(file);
    // let mut f = BufWriter::with_capacity(100 * 1024, file);

    /* Tuple trait definition */
    // --------------------------------------------------------------------------

    w!(f, r#"
#[doc = crate::_TAG_CODE!()]
#[doc = crate::_TAG_DATA_STRUCTURE!()]
#[doc = crate::_TAG_CODEGEN_BUILD!()]
/// Extension trait providing convenience methods for [tuples][tuple].
#[doc = crate::_doc!(location: "data/list/tuple")]
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// Tuples are random-access, sequentially allocated, statically sized,
/// heterogeneous data structures.
///
/// They enable structured grouping and access to a sequence of different types.
///
/// # Features
/// By default it's implemented for tuples of arity of 12 or less.
/// It supports increased arities of 24, 36, 48 and 72 by enabling the
/// corresponding capability feature: `_tuple_[24|36|48|72]`.
///
/// # Vendored
/// This is adapted work from [tupl][crate::_doc::vendored#tupl]"#)?;
// In sync with code::utils::_doc::doc_!(vendor:)

    w!(f, "{TAB1}#[cfg_attr(nightly_doc, doc(notable_trait))]
    #[cfg_attr(nightly_doc, doc(notable_trait))]
    #[allow(private_bounds)]
    pub trait Tuple: Sealed {{

    /* constants */

    /// The arity of this tuple (the number of contained elements)
    const ARITY: usize;

    /* name fields */
    /// The first element of the tuple, at index 0.
    type Head;
    /// The last element of the tuple, at index `ARITY-1`.
    type Tail;

    /// This tuple without its head.
    type NoHead;
    /// This tuple without its tail.
    type NoTail;
    /// This tuple with an extra element `T` appended to it.
    type Append<T>;
    /// This tuple with an extra element `T` prepended to it.
    type Prepend<T>;
    ")?;

    // index fields
    for i in 0..MAX_ARITY {
        w!(f, "{TAB1}/// The type of the element at index {i}.")?;
        w!(f, "{TAB1}type _{i};")?;
    }

    w!(f, r#"
    /// The maximum arity supported by the current compilation options.
    ///
    /// See the available [features](#features) to increase this number."#)?;
    w!(f, "{TAB1}const MAX_ARITY: usize = {MAX_ARITY};")?;

    /* Tuple methods */

    w!(f, "{TAB1}/// Returns a shared reference to the head of this tuple.
    #[must_use]
    fn head(&self) -> &Self::Head;")?;
    w!(f, "{TAB1}/// Returns a shared reference to the tail of this tuple.
    #[must_use]
    fn tail(&self) -> &Self::Tail;")?;

    w!(f, "{TAB1}/// Returns an exclusive reference to the head of this tuple.
    #[must_use]
    fn head_mut(&mut self) -> &mut Self::Head;")?;
    w!(f, "{TAB1}/// Returns an exclusive reference to the tail of this tuple.
    #[must_use]
    fn tail_mut(&mut self) -> &mut Self::Tail;")?;

    w!(f, "{TAB1}/// Returns this tuple with the head element splitted from the rest.
    #[must_use]
    fn split_head(self) -> (Self::Head, Self::NoHead);")?;
    w!(f, "{TAB1}/// Returns this tuple with the tail element splitted from the rest.
    #[must_use]
    fn split_tail(self) -> (Self::NoTail, Self::Tail);")?;

    w!(f, "{TAB1}/// Returns this tuple without the head.
    #[must_use]
    fn no_head(self) -> Self::NoHead;")?;
    w!(f, "{TAB1}/// Returns this tuple without the tail.
    #[must_use]
    fn no_tail(self) -> Self::NoTail;")?;

    w!(f, "{TAB1}/// Appends the given `value` to this tuple.
    #[must_use]
    fn append<T>(self, value: T) -> Self::Append<T>;")?;
    w!(f, "{TAB1}/// Prepends the given `value` to this tuple.
    #[must_use]
    fn prepend<T>(self, value: T) -> Self::Prepend<T>;")?;

    // nth
    w!(f, "{TAB1}/// Returns the `nth` element, or `None` if `nth >= ARITY`.
    #[allow(clippy::type_complexity)]
    fn nth(self, nth: usize) -> Option<TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">>;")?;

    w!(f, "{TAB1}/// Returns the `nth` element cloned, or `None` if `nth >= ARITY`.
    #[allow(clippy::type_complexity)]
    fn nth_cloned(&self, nth: usize) -> Option<TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">> where ")?;
        for i in 0..MAX_ARITY {
            w0!(f, "Self::_{i}: Clone")?;
            if i == MAX_ARITY-1 { w!(f, ";")?; } else { w0!(f, ",")?; }
        }

    w!(f, "{TAB1}/// Returns a shared reference to the `nth` element,
    /// or `None` if `nth >= ARITY`.
    #[allow(clippy::type_complexity)]
    fn nth_ref(&self, nth: usize) -> Option<TupleElementRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">>;")?;

    w!(f, "{TAB1}/// Returns an exclusive reference to the `nth` element,
    /// or `None` if `nth >= ARITY`.
    #[allow(clippy::type_complexity)]
    fn nth_mut(&mut self, nth: usize) -> Option<TupleElementMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">>;")?;

    // iter
    w0!(f, "{TAB1}/// Returns an iterator over elements of the tuple.
    #[allow(clippy::type_complexity)]
    fn into_iter(self) -> TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">;")?;
    w0!(f, "{TAB1}/// Returns an iterator over shared references to elements of the tuple.
    #[allow(clippy::type_complexity)]
    fn iter_ref(&self) -> TupleIterRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">;")?;
    w!(f, "{TAB1}/// Returns an iterator over exclusive reference to elements of the tuple.
    #[allow(clippy::type_complexity)]
    fn iter_mut(&mut self) -> TupleIterMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">;")?;

    /* Tuple auto-implemented methods */

    w!(f, "{TAB1}/// Returns the arity (number of elements) of this tuple.
    #[must_use]
    fn arity(&self) -> usize {{ Self::ARITY }}")?;

    w!(f, "{TAB1}/// Wraps the tuple in a [`TupleFmt`] for formatting purposes.
    fn fmt(&self) -> TupleFmt<'_, Self> where Self: Sized {{ TupleFmt(self) }}")?;

    w!(f, "}}")?; // end define Tuple


    /* manual implementations of Tuple for arities of 0 and 1 */
    // --------------------------------------------------------------------------

    /* arity 0 */

    w!(f, "impl Sealed for () {{}}")?;

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
        for i in 0..MAX_ARITY { w!(f, "{TAB1}type _{i} = ();")?; }

        /* methods */

        // nth
        w!(f, "\n{TAB1}fn nth(self, _nth: usize) -> Option<TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> {{")?;
            w!(f, "{TAB2}None")?;
        w!(f, "{TAB1}}}")?;
        w!(f, "{TAB1}fn nth_cloned(&self, _nth: usize) -> Option<TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> where ")?;
            for i in 0..MAX_ARITY { w0!(f, "Self::_{i}: Clone,")?; }
            w!(f, "{{")?;
                w!(f, "{TAB2}None")?;
        w!(f, "{TAB1}}}")?;
        w!(f, "{TAB1}fn nth_ref(&self, _nth: usize) -> Option<TupleElementRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> {{")?;
            w!(f, "{TAB2}None")?;
        w!(f, "{TAB1}}}")?;
        w!(f, "{TAB1}fn nth_mut(&mut self, _nth: usize) -> Option<TupleElementMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> {{")?;
            w!(f, "{TAB2}None")?;
            w!(f, "{TAB1}}}")?;

        // iter
        w0!(f, "{TAB1}fn into_iter(self) -> TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        TupleIter {{
            tuple: (")?; for _ in 0..MAX_ARITY { w0!(f, "None, ")?; } w!(f, "),
            front_index: 0,
            back_index: 0,
        }}
    }}")?;
        w0!(f, "{TAB1}fn iter_ref(&self) -> TupleIterRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        TupleIterRef {{
            tuple: (")?; for _ in 0..MAX_ARITY { w0!(f, "&(), ")?; } w!(f, "),
            front_index: 0,
            back_index: 0,
        }}
    }}")?;
        w0!(f, "{TAB1}fn iter_mut(&mut self) -> TupleIterMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        TupleIterMut {{
            tuple: (")?; for _ in 0..MAX_ARITY { w0!(f, "None, ")?; } w!(f, "),
            front_index: 0,
            back_index: 0,
        }}
    }}")?;

    w!(f, "}}")?;

    w!(f, r#"
impl TupleDebug for () {{
    fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()> {{
        f.debug_tuple("").finish()
    }}
}}
impl TupleDisplay for () {{
    fn fmt_display(&self, f: &mut Formatter) -> FmtResult<()> {{
        write!(f, "()")
    }}
}}"#)?;

    /* arity 1 */

    w!(f, "impl<_0> Sealed for (_0,) {{}}")?;

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

        /* methods */

        // nth
        w0!(f, "{TAB1}fn nth(self, nth: usize) -> Option<TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> {{")?;
            w!(f, "{TAB2}match nth {{")?;
                w!(f, "{TAB3}0 => Some(TupleElement::_0(self.0)),")?;
                w!(f, "{TAB3}_ => None,")?;
            w!(f, "{TAB2}}}")?;
        w!(f, "{TAB1}}}")?;
        w0!(f, "{TAB1}fn nth_cloned(&self, nth: usize) -> Option<TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> where ")?;
            for i in 0..MAX_ARITY { w0!(f, "Self::_{i}: Clone,")?; }
            w!(f, "{{")?;
            w!(f, "{TAB2}match nth {{")?;
                w!(f, "{TAB3}0 => Some(TupleElement::_0(self.0.clone())),")?;
                w!(f, "{TAB3}_ => None,")?;
            w!(f, "{TAB2}}}")?;
        w!(f, "{TAB1}}}")?;
        w0!(f, "{TAB1}fn nth_ref(&self, nth: usize) -> Option<TupleElementRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> {{")?;
            w!(f, "{TAB2}match nth {{")?;
                w!(f, "{TAB3}0 => Some(TupleElementRef::_0(&self.0)),")?;
                w!(f, "{TAB3}_ => None,")?;
            w!(f, "{TAB2}}}")?;
        w!(f, "{TAB1}}}")?;
        w0!(f, "{TAB1}fn nth_mut(&mut self, nth: usize) -> Option<TupleElementMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> {{")?;
            w!(f, "{TAB2}match nth {{")?;
                w!(f, "{TAB3}0 => Some(TupleElementMut::_0(&mut self.0)),")?;
                w!(f, "{TAB3}_ => None,")?;
            w!(f, "{TAB2}}}")?;
        w!(f, "{TAB1}}}")?;

        // iter
        w0!(f, "{TAB1}fn into_iter(self) -> TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        TupleIter {{
            tuple: (Some(self.0), ")?;
                for _ in 1..MAX_ARITY { w0!(f, "None, ")?; }
                w!(f, "),
            front_index: 0,
            back_index: 0,
        }}
    }}")?;
        w0!(f, "{TAB1}fn iter_ref(&self) -> TupleIterRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        TupleIterRef {{
            tuple: (&self.0, ")?;
                for _ in 1..MAX_ARITY { w0!(f, "&(), ")?; }
                w!(f, "),
            front_index: 0,
            back_index: 0,
        }}
    }}")?;
        w0!(f, "{TAB1}fn iter_mut(&mut self) -> TupleIterMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        TupleIterMut {{
            tuple: (Some(&mut self.0), ")?;
                for _ in 1..MAX_ARITY { w0!(f, "None, ")?; }
                w!(f, "),
            front_index: 0,
            back_index: 0,
        }}
    }}")?;

    w!(f, "}}")?;

    w!(f, r#"
impl<_0: Debug> TupleDebug for (_0,) {{
    fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()> {{
        f.debug_tuple("").field(&self.0).finish()
    }}
}}
impl<_0: Display> TupleDisplay for (_0,) {{
    fn fmt_display(&self, f: &mut Formatter) -> FmtResult<()> {{
        write!(f, "({{}},)", &self.0)
    }}
}}"#)?;


    /* auto implementations of Tuple for arities >= 2 */
    // --------------------------------------------------------------------------

    for arity in 2..=MAX_ARITY {
        w0!(f, "impl<")?; for i in 0..arity { w0!(f, "_{i},")?; }
        w0!(f, "> Sealed for (")?; for i in 0..arity { w0!(f, "_{i},")?; }
        w!(f, ") {{}}")?;

        w0!(f, "impl<")?; for i in 0..arity { w0!(f, "_{i},")?; }
        w0!(f, "> Tuple for (")?; for i in 0..arity { w0!(f, "_{i},")?; }
        w!(f, ") {{")?;

        // constants
        w!(f, "{TAB1}const ARITY: usize = {arity};")?;

        // name fields
        w!(f, "{TAB1}/// The first element of the tuple, at index 0.")?;
        w!(f, "{TAB1}type Head = _0;")?;
        w!(f, "{TAB1}/// The last element of the tuple, at index `ARITY-1`.")?;
        w!(f, "{TAB1}type Tail = _{};", arity-1)?;

        w!(f, "{TAB1}///.")?;
        w0!(f, "{TAB1}type NoHead = (")?; for i in 1..arity { w0!(f, "_{i},")?; } w!(f, ");")?;
        w0!(f, "{TAB1}type NoTail = (")?; for i in 0..arity-1 { w0!(f, "_{i},")?; } w!(f, ");")?;
        w0!(f, "{TAB1}type Append<T> = (")?; for i in 0..arity { w0!(f, "_{i},")?; } w!(f, "T);")?;
        w0!(f, "{TAB1}type Prepend<T> = (T, ")?; for i in 0..arity { w0!(f, "_{i},")?; } w!(f, ");")?;

        // index fields
        w!(f, "{TAB1}/// The type of the element at index 0 (the first field).")?;
        w!(f, "{TAB1}type _0 = _0;")?;
        for i in 1..arity-1 {
            w!(f, "{TAB1}/// The type of the element at index {i}.")?;
            w!(f, "{TAB1}type _{i} = _{i};")?;
        }
        if arity > 1 {
            w!(f, "{TAB1}/// The type of the element at index {} (the last field).", arity-1)?;
            w!(f, "{TAB1}type _{0} = _{0};", arity-1)?;
        }
        for i in arity..MAX_ARITY {
            w!(f, "{TAB1}/// Non-existing element with current arity.")?;
            w!(f, "{TAB1}type _{i} = ();")?;
        }

        /* Tuple methods */

        w!(f, "{TAB1}fn head(&self) -> &Self::Head {{ &self.0 }}")?;
        w!(f, "{TAB1}fn tail(&self) -> &Self::Tail {{ &self.{} }}", arity-1)?;
        w!(f, "{TAB1}fn head_mut(&mut self) -> &mut Self::Head {{ &mut self.0 }}")?;
        w!(f, "{TAB1}fn tail_mut(&mut self) -> &mut Self::Tail {{ &mut self.{} }}", arity-1)?;

        w0!(f, "{TAB1}fn split_head(self) -> (Self::Head, Self::NoHead) {{ (self.0, (")?;
            for i in 1..arity { w0!(f, "self.{i},")?; }
        w!(f, ")) }}")?;

        w0!(f, "{TAB1}fn split_tail(self) -> (Self::NoTail, Self::Tail) {{ ((")?;
            for i in 0..arity-1 { w0!(f, "self.{i},")?; }
        w!(f, "), self.{}) }}", arity-1)?;

        w0!(f, "{TAB1}fn no_head(self) -> Self::NoHead {{ (")?;
            for i in 1..arity { w0!(f, "self.{i},")?; }
        w!(f, ") }}")?;
        w0!(f, "{TAB1}fn no_tail(self) -> Self::NoTail {{ (")?;
            for i in 0..arity-1 { w0!(f, "self.{i},")?; }
        w!(f, ") }}")?;

        w0!(f, "{TAB1}fn append<T>(self, value: T) -> Self::Append<T> {{ (")?;
            for i in 0..arity { w0!(f, "self.{i},")?; }
        w!(f, "value) }}")?;
        w0!(f, "{TAB1}fn prepend<T>(self, value: T) -> Self::Prepend<T> {{ (value, ")?;
            for i in 0..arity { w0!(f, "self.{i},")?; }
        w!(f, ") }}")?;

        // nth
        w0!(f, "{TAB1}fn nth(self, nth: usize) -> Option<TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">> {{")?;
            w!(f, "{TAB2}match nth {{")?;
                for i in 0..arity {
                    w!(f, "{TAB3}{i} => Some(TupleElement::_{i}(self.{i})),")?; }
                w!(f, "{TAB3}_ => None")?;
            w!(f, "{TAB2}}}")?;
        w!(f, "{TAB1}}}")?;

        w0!(f, "{TAB1}fn nth_cloned(&self, nth: usize) -> Option<TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; } w!(f, ">> where ")?;
            for i in 0..MAX_ARITY { w0!(f, "Self::_{i}: Clone,")?; }
            w!(f, "{{")?;
            w!(f, "{TAB2}match nth {{")?;
                for i in 0..arity {
                    w!(f, "{TAB3}{i} => Some(TupleElement::_{i}(self.{i}.clone())),")?; }
                w!(f, "{TAB3}_ => None")?;
            w!(f, "{TAB2}}}")?;
        w!(f, "{TAB1}}}")?;
        w0!(f, "{TAB1}fn nth_ref(&self, nth: usize) -> Option<TupleElementRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">> {{")?;
            w!(f, "{TAB2}match nth {{")?;
                for i in 0..arity {
                    w!(f, "{TAB3}{i} => Some(TupleElementRef::_{i}(&self.{i})),")?; }
                w!(f, "{TAB3}_ => None")?;
            w!(f, "{TAB2}}}")?;
        w!(f, "{TAB1}}}")?;
        w0!(f, "{TAB1}fn nth_mut(&mut self, nth: usize) -> Option<TupleElementMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w!(f, ">> {{")?;
            w!(f, "{TAB2}match nth {{")?;
                for i in 0..arity {
                    w!(f, "{TAB3}{i} => Some(TupleElementMut::_{i}(&mut self.{i})),")?; }
                w!(f, "{TAB3}_ => None")?;
            w!(f, "{TAB2}}}")?;
        w!(f, "{TAB1}}}")?;

        // iter
        w0!(f, "{TAB1}fn into_iter(self) -> TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        let back_index = self.arity() - 1;
        TupleIter {{
            tuple: (Some(self.0), Some(self.1), ")?;
                for i in 2..arity { w0!(f, "Some(self.{i}), ")?; }
                for _ in arity..MAX_ARITY { w0!(f, "None, ")?; }
                w!(f, "),
            front_index: 0,
            back_index,
        }}
    }}")?;
        w0!(f, "{TAB1}fn iter_ref(&self) -> TupleIterRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        let back_index = self.arity() - 1;
        TupleIterRef {{
            tuple: (&self.0, &self.1, ")?;
                for i in 2..arity { w0!(f, "&self.{i}, ")?; }
                for _ in arity..MAX_ARITY { w0!(f, "&(), ")?; }
                w!(f, "),
            front_index: 0,
            back_index,
        }}
    }}")?;
        w0!(f, "{TAB1}fn iter_mut(&mut self) -> TupleIterMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "Self::_{i},")?; }
        w0!(f, "> {{
        let back_index = self.arity() - 1;
        TupleIterMut {{
            tuple: (Some(&mut self.0), Some(&mut self.1), ")?;
                for i in 2..arity { w0!(f, "Some(&mut self.{i}), ")?; }
                for _ in arity..MAX_ARITY { w0!(f, "None, ")?; }
                w!(f, "),
            front_index: 0,
            back_index,
        }}
    }}")?;

        w!(f, "}}")?; // end impl Tuple

        /* impl other traits */

        w!(f, "#[rustfmt::skip]")?;
        w0!(f, "impl<")?; for i in 0..arity { w0!(f, "_{i}: Debug, ")?; }
        w0!(f, "> TupleDebug for (")?; for i in 0..arity { w0!(f, "_{i},")?; }
        w!(f, ") {{\n{TAB1}fn fmt_debug(&self, f: &mut Formatter) -> FmtResult<()> {{")?;
            w0!(f, "{TAB2}f.debug_tuple(\"\")")?;
                for i in 0..arity { w0!(f, ".field(&self.{i})")?; }
                w!(f, ".finish()")?;
        w!(f, "{TAB1}}}\n}}")?;

        w!(f, "#[rustfmt::skip]")?;
        w0!(f, "impl<")?; for i in 0..arity { w0!(f, "_{i}: Display, ")?; }
        w0!(f, "> TupleDisplay for (")?; for i in 0..arity { w0!(f, "_{i},")?; }
        w!(f, ") {{\n{TAB1}fn fmt_display(&self, f: &mut Formatter) -> FmtResult<()> {{")?;
            w!(f, "{TAB2}write!(f, \"({{}}\", &self.0)?;")?;
                for i in 1..arity { w!(f, "{TAB2}write!(f, \", {{}}\", &self.{i})?;")?; }
            w!(f, "{TAB2}write!(f, \")\")")?;
        w!(f, "{TAB1}}}\n}}")?;
    }


    /* enums definitions */
    // --------------------------------------------------------------------------

    w!(f, "#[doc = crate::_TAG_CODE!()]")?;
    w!(f, "#[doc = crate::_TAG_CODEGEN_BUILD!()]")?;
    w!(f, "/// An element of a [`Tuple`].")?;
    w!(f, r#"#[doc = crate::_doc!(location: "data/list/tuple")]"#)?;
    w!(f, "#[non_exhaustive]")?;
    w!(f, "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]")?;
    w0!(f, "pub enum TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        // variants
        for i in 0..MAX_ARITY {
            w!(f, "{TAB1}/// The tuple element at index {i}.")?;
            w!(f, "{TAB1}_{i}(_{i}),")?;
        }
    w!(f, "}}")?;

    w!(f, "#[doc = crate::_TAG_CODE!()]")?;
    w!(f, "#[doc = crate::_TAG_LIFETIME!()]")?;
    w!(f, "#[doc = crate::_TAG_CODEGEN_BUILD!()]")?;
    w!(f, "/// A shared reference to an element of a [`Tuple`].")?;
    w!(f, r#"#[doc = crate::_doc!(location: "data/list/tuple")]"#)?;
    w!(f, "#[non_exhaustive]")?;
    w!(f, "#[derive(Debug, PartialEq, Eq, Hash)]")?;
    w0!(f, "pub enum TupleElementRef<'a, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        // variants
        for i in 0..MAX_ARITY {
            w!(f, "{TAB1}/// A shared reference to the tuple element at index {i}.")?;
            w!(f, "{TAB1}_{i}(&'a _{i}),")?;
        }
    w!(f, "}}")?;

    w!(f, "#[doc = crate::_TAG_CODE!()]")?;
    w!(f, "#[doc = crate::_TAG_LIFETIME!()]")?;
    w!(f, "#[doc = crate::_TAG_CODEGEN_BUILD!()]")?;
    w!(f, "/// An exclusive reference to an element of a [`Tuple`].")?;
    w!(f, r#"#[doc = crate::_doc!(location: "data/list/tuple")]"#)?;
    w!(f, "#[non_exhaustive]")?;
    w!(f, "#[derive(Debug, PartialEq, Eq, Hash)]")?;
    w0!(f, "pub enum TupleElementMut<'a, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        // variants
        for i in 0..MAX_ARITY {
            w!(f, "{TAB1}/// An exclusive reference to the tuple element at index {i}.")?;
            w!(f, "{TAB1}_{i}(&'a mut _{i}),")?;
        }
    w!(f, "}}")?;


    /* iterators definitions */
    // --------------------------------------------------------------------------

    // into
    w!(f, "#[doc = crate::_TAG_CODE!()]")?;
    w!(f, "#[doc = crate::_TAG_ITERATOR!()]")?;
    w!(f, "#[doc = crate::_TAG_CODEGEN_BUILD!()]")?;
    w!(f, "/// An iterator over elements of a [`Tuple`].")?;
    w!(f, r#"#[doc = crate::_doc!(location: "data/list/tuple")]"#)?;
    w!(f, "#[derive(Clone, Debug)]")?;
    w0!(f, "pub struct TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        // fields
        w!(f, "#[allow(clippy::type_complexity)]")?;
        w!(f, "tuple: (")?;
            for i in 0..MAX_ARITY { w0!(f, "Option<_{i}>,")?; } w!(f, "),")?;
        w!(f, "front_index: usize,")?;
        w!(f, "back_index: usize,")?;
    w!(f, "}}")?;
    // methods
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w0!(f, "> TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "{TAB1}/// Returns the remaining elements in the iterator.
    pub const fn remaining(&self) -> usize {{ self.back_index + 1 - self.front_index }}
}}")?;

    // ref
    w!(f, "#[doc = crate::_TAG_CODE!()]")?;
    w!(f, "#[doc = crate::_TAG_ITERATOR!()]")?;
    w!(f, "#[doc = crate::_TAG_LIFETIME!()]")?;
    w!(f, "#[doc = crate::_TAG_CODEGEN_BUILD!()]")?;
    w!(f, "/// An iterator over shared references to elements of a [`Tuple`].")?;
    w!(f, r#"#[doc = crate::_doc!(location: "data/list/tuple")]"#)?;
    w!(f, "#[derive(Clone, Debug)]")?;
    w!(f, "pub struct TupleIterRef<'a, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        // fields
        w!(f, "#[allow(clippy::type_complexity)]")?;
        w!(f, "tuple: (")?;
            for i in 0..MAX_ARITY { w0!(f, "&'a _{i},")?; } w!(f, "),")?;
        w!(f, "front_index: usize,")?;
        w!(f, "back_index: usize,")?;
    w!(f, "}}")?;
    // methods
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w0!(f, "> TupleIterRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "/// Returns the remaining elements in the iterator.
        pub const fn remaining(&self) -> usize {{ self.back_index + 1 - self.front_index }}
    }}")?;

    // mut
    w!(f, "#[doc = crate::_TAG_CODE!()]")?;
    w!(f, "#[doc = crate::_TAG_ITERATOR!()]")?;
    w!(f, "#[doc = crate::_TAG_LIFETIME!()]")?;
    w!(f, "#[doc = crate::_TAG_CODEGEN_BUILD!()]")?;
    w!(f, r#"#[doc = crate::_doc!(location: "data/list/tuple")]"#)?;
    w!(f, "/// An iterator over exclusive references to elements of a [`Tuple`].")?;
    w!(f, "#[derive(Debug)]")?;
    w!(f, "pub struct TupleIterMut<'a, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        // fields
        w!(f, "#[allow(clippy::type_complexity)]")?;
        w!(f, "tuple: (")?;
            for i in 0..MAX_ARITY { w0!(f, "Option<&'a mut _{i}>,")?; } w!(f, "),")?;
        w!(f, "front_index: usize,")?;
        w!(f, "back_index: usize,")?;
    w!(f, "}}")?;
    // methods
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> TupleIterMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "/// Returns the remaining elements in the iterator.
        pub const fn remaining(&self) -> usize {{ self.back_index + 1 - self.front_index }}
    }}")?;

    /* iterators traits implementations */
    // --------------------------------------------------------------------------

    // Into
    w!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w!(f, "> Iterator for TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w0!(f, "{TAB1}type Item = TupleElement<")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w0!(f, ">;")?;
        //
        w!(f, "{TAB1}fn next(&mut self) -> Option<Self::Item> {{
            #[allow(clippy::never_loop)]
            if self.front_index > self.back_index {{
                None
            }} else {{
                let result = match self.front_index {{")?;
                    for i in 0..MAX_ARITY {
                        w!(f, "{TAB5}{i} => self.tuple.{i}.take().map(TupleElement::_{i}),")?;
                    }
                    w!(f, "{TAB5}_ => None,
                }};
                self.front_index += 1;
                result
            }}
        }}
        fn size_hint(&self) -> (usize, Option<usize>) {{
            (self.remaining(), Some(self.remaining()))
        }}
        fn count(self) -> usize {{ self.remaining() }}
    }}")?;
    //
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> DoubleEndedIterator for TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "{TAB1}fn next_back(&mut self) -> Option<Self::Item> {{
            #[allow(clippy::never_loop)]
            if self.front_index > self.back_index {{
                None
            }} else {{
                let result = match self.back_index {{")?;
                    for i in 0..MAX_ARITY {
                        w!(f, "{TAB5}{i} => self.tuple.{i}.take().map(TupleElement::_{i}),")?;
                    }
                    w!(f, "{TAB5}_ => None,
                }};
                if self.back_index == 0 {{
                    self.front_index = self.back_index + 1; // Ensure iteration stops
                }} else {{
                    self.back_index -= 1;
                }}
                result
            }}
        }}
    }}")?;
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> ExactSizeIterator for TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "{TAB1}fn len(&self) -> usize {{ self.remaining() }}
    }}")?;
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> core::iter::FusedIterator for TupleIter<")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{}}")?;

    // Ref
    w0!(f, "impl<'a, ")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> Iterator for TupleIterRef<'a, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "type Item = TupleElementRef<'a, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, ">;")?;
        //
        w!(f, "{TAB1}fn next(&mut self) -> Option<Self::Item> {{
            #[allow(clippy::never_loop)]
            if self.front_index > self.back_index {{
                None
            }} else {{
                let result = match self.front_index {{")?;
                    for i in 0..MAX_ARITY {
                        w!(f, "{TAB5}{i} => Some(TupleElementRef::_{i}(self.tuple.{i})),")?;
                    }
                    w!(f, "{TAB5}_ => None,
                }};
                self.front_index += 1;
                result
            }}
        }}
        fn size_hint(&self) -> (usize, Option<usize>) {{
            (self.remaining(), Some(self.remaining()))
        }}
        fn count(self) -> usize {{ self.remaining() }}
    }}")?;
    //
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> DoubleEndedIterator for TupleIterRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "{TAB1}fn next_back(&mut self) -> Option<Self::Item> {{
            #[allow(clippy::never_loop)]
            if self.front_index > self.back_index {{
                None
            }} else {{
                let result = match self.back_index {{")?;
                    for i in 0..MAX_ARITY {
                        w!(f, "{TAB5}{i} => Some(TupleElementRef::_{i}(self.tuple.{i})),")?;
                    }
                    w!(f, "{TAB5}_ => None,
                }};
                if self.back_index == 0 {{
                    self.front_index = self.back_index + 1; // Ensure iteration stops
                }} else {{
                    self.back_index -= 1;
                }}
                result
            }}
        }}
    }}")?;
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> ExactSizeIterator for TupleIterRef<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "{TAB1}fn len(&self) -> usize {{ self.remaining() }}
    }}")?;
    // NOTE: better not to implement FusedIterator
    // w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    // w0!(f, "> core::iter::FusedIterator for TupleIterRef<'_, ")?;
    //     for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{}}")?;

    // Mut
    w0!(f, "impl<'a, ")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> Iterator for TupleIterMut<'a, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w0!(f, "type Item = TupleElementMut<'a, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, ">;")?;
        //
        w!(f, "{TAB1}fn next(&mut self) -> Option<Self::Item> {{
            #[allow(clippy::never_loop)]
            if self.front_index > self.back_index {{
                None
            }} else {{
                let result = match self.front_index {{")?;
                    for i in 0..MAX_ARITY {
                        w!(f, "{TAB5}{i} => self.tuple.{i}.take().map(TupleElementMut::_{i}),")?;
                    }
                    w!(f, "{TAB5}_ => None,
                }};
                self.front_index += 1;
                result
            }}
        }}
        fn size_hint(&self) -> (usize, Option<usize>) {{
            (self.remaining(), Some(self.remaining()))
        }}
        fn count(self) -> usize {{ self.remaining() }}
    }}")?;
    //
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> DoubleEndedIterator for TupleIterMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "{TAB1}fn next_back(&mut self) -> Option<Self::Item> {{
            #[allow(clippy::never_loop)]
            if self.front_index > self.back_index {{
                None
            }} else {{
                let result = match self.back_index {{")?;
                    for i in 0..MAX_ARITY {
                        w!(f, "{TAB5}{i} => self.tuple.{i}.take().map(TupleElementMut::_{i}),")?;
                    }
                    w!(f, "{TAB5}_ => None,
                }};
                if self.back_index == 0 {{
                    self.front_index = self.back_index + 1; // Ensure iteration stops
                }} else {{
                    self.back_index -= 1;
                }}
                result
            }}
        }}
    }}")?;
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> ExactSizeIterator for TupleIterMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w!(f, "> {{")?;
        w!(f, "{TAB1}fn len(&self) -> usize {{ self.remaining() }}
    }}")?;
    w0!(f, "impl<")?; for i in 0..MAX_ARITY { w0!(f, "_{i},")?; }
    w0!(f, "> core::iter::FusedIterator for TupleIterMut<'_, ")?;
        for i in 0..MAX_ARITY { w0!(f, "_{i},")?; } w0!(f, "> {{}}")?;

    // --------------------------------------------------------------------------

    if let Err(e) = f.flush() {
        eprintln!("Failed to write to file: {e}");
        std::process::exit(1);
    }

    // #[cfg(doc)] // format the source if we're building the docs
    // super::super::rustfmt_file(path);
    Ok(())
}
