// devela::sys::os::print
//
//! OS print related macros.
//!
//! Generates the macros: `os_print`, `os_println`, `os_eprint`, `os_eprintln`.
//
// NOTE: It's necessary to duplicate the macros because the `cfg` attribute
// checks the features of the current crate, not the current library.
//
// NOTE: `linux` version has preference over `std` version.

// std, not(linux)
#[cfg(feature = "std")]
#[cfg(not(all(feature = "linux", feature = "unsafe_syscall", not(miri),
    any( // targets:
        target_arch = "x86", target_arch = "x86_64",
        target_arch = "arm", target_arch = "aarch64",
        target_arch = "riscv32", target_arch = "riscv64"
    ),
)))]
crate::items! {
    mod std;
    use std::generate_os_std_print_macros;
    generate_os_std_print_macros![];
}

// *linux, not(std)
// #[cfg(not(feature = "std"))]
#[cfg(all(feature = "linux", feature = "unsafe_syscall", not(miri),
    any( // targets:
        target_arch = "x86", target_arch = "x86_64",
        target_arch = "arm", target_arch = "aarch64",
        target_arch = "riscv32", target_arch = "riscv64"
    ),
))]
crate::items! {
    mod linux;
    use linux::generate_os_linux_print_macros;
    generate_os_linux_print_macros![];
}
