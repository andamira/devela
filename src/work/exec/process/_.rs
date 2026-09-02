// devela/src/work/exec/process/_.rs
//
#![doc = crate::_DOC_WORK_EXEC_PROCESS!()] // public
#![doc = crate::_doc!(modules: crate::work::exec; process)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: process)]
//

#[cfg(feature = "std")]
crate::mods_in! {
    mod _reexport_std;

    mod cmd; // cmd!
    mod error; // ExitStatusError
    mod flow; // CommandFlow
    mod output; // OutputExt
    mod process; // ProcessExt
}
crate::mods_out! { // _mods, _reexports, _hidden
    _mods {
        #[cfg(feature = "std")]
        pub use super::{
            cmd::cmd,
            error::*,
            flow::*,
            output::*,
            process::*,
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
    _hidden {
        #[doc(hidden)]
        #[cfg(feature = "std")]
        pub use super::cmd::__cmd_shell;
    }
}
