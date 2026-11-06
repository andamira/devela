//
#![doc = crate::_DOC_WORK_EXEC_PROCESS!()] // public
#![doc = crate::_doc!(modules: crate::work::exec; process)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: process)]
//

#[cfg(feature = "std")]
crate::mods_in! {
    mod _reexport_std;

    #[cfg(feature = "process")]
    mod cmd;

    #[cfg(feature = "process")]
    mod error;

    #[cfg(feature = "process")]
    mod flow;

    #[cfg(feature = "process")]
    mod output;

    #[cfg(feature = "process")]
    mod process;
}
crate::mods_out! { // _mods, _reexports, _hidden
    _mods {
        #[cfg(all(feature = "process", feature = "std"))]
        pub use super::{
            cmd::cmd,
            error::ExitStatusError,
            flow::CommandFlow,
            output::OutputExt,
            process::ProcessExt,
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
    _hidden {
        #[doc(hidden)]
        #[cfg(all(feature = "process", feature = "std"))]
        pub use super::cmd::{__cmd, __cmd_shell};
    }
}
