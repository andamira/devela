// devela/src/data/store/arena/string/_internal.rs
//
//! Defines [`__arena_string!`].
//

#[doc(hidden)]
#[macro_export]
macro_rules! __arena_string· {
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__arena_string! {
            %normalize_cursor
            [kind: $($kind)?]
            [index: $iprim + $iprim]
            $($rest)*
        }
    };
    (%normalize_index
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        $($rest:tt)*
    ) => {
        $crate::__arena_string! {
            %normalize_cursor
            [kind: $($kind)?]
            [index: $iprim + $Index]
            $($rest)*
        }
    };
    (%normalize_cursor
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident]
        $($rest:tt)*
    ) => {
        $crate::__arena_string! {
            %generate
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [cursor: $cprim + $cprim]
            $($rest)*
        }
    };
    (%normalize_cursor
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident + $Cursor:ty]
        $($rest:tt)*
    ) => {
        $crate::__arena_string! {
            %generate
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [cursor: $cprim + $Cursor]
            $($rest)*
        }
    };
    (%generate
        [kind: $($kind:ident)?]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $(#[$handle_attr:meta])* $hvis:vis $Handle:ident]
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
        $crate::handle! {
            [index: $iprim + $Index;]
            $(#[$handle_attr])*
            $hvis $Handle;
        }

        $crate::__arena_string! {
            %backend
            [kind: $($kind)?]
            [index: $iprim + $Index]
            [cursor: $cprim + $Cursor]
            [arena: $(#[$arena_attr])* $vis $Arena]
            [handle: $hvis $Handle]
            [mark: $($(#[$mark_attr])* $mvis $Mark)?]
        }
    };
    (%backend
        [kind:]
        $($rest:tt)*
    ) => {
        $crate::__arena_string! {
            %backend
            [kind: static]
            $($rest)*
        }
    };
    (%backend
        [kind: static]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
        $crate::__arena_string_impl_array! {
            [index: $iprim + $Index;]
            [cursor: $cprim + $Cursor;]
            $(#[$arena_attr])* $vis $Arena;
            $hvis $Handle;
            [mark: $($mvis $Mark)?]
        }
        $(
            $(#[$mark_attr])*
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $mvis struct $Mark($crate::MaybeNiche<$Index>);

            #[allow(dead_code)]
            impl $Mark {
                const fn new(mark: $crate::MaybeNiche<$Index>) -> Self {
                    Self(mark)
                }
            }
        )?
    };
    (%backend
        [kind: alloc]
        [index: $iprim:ident + $Index:ty]
        [cursor: $cprim:ident + $Cursor:ty]
        [arena: $(#[$arena_attr:meta])* $vis:vis $Arena:ident]
        [handle: $hvis:vis $Handle:ident]
        [mark: $($(#[$mark_attr:meta])* $mvis:vis $Mark:ident)?]
    ) => {
        $crate::__arena_string_impl_vec! {
            [index: $iprim + $Index;]
            [cursor: $cprim + $Cursor;]
            $(#[$arena_attr])* $vis $Arena;
            $hvis $Handle;
            [mark: $($mvis $Mark)?]
        }
        $(
            $(#[$mark_attr])*
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $mvis struct $Mark(usize);

            #[allow(dead_code)]
            impl $Mark {
                const fn new(mark: usize) -> Self {
                    Self(mark)
                }
            }
        )?
    };
}
pub use __arena_string· as __arena_string;
