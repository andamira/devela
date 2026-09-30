//
//! Defines [`spectrum_main!`].
//

#[doc = crate::_tags!(hw code)]
/// Defines the entry point of a ZX Spectrum program.
#[doc = crate::_doc_meta!{
    location("computer/zx/spectrum", macro spectrum_main),
}]
/// This macro centralizes the Spectrum startup and entry contract,
/// keeping application code independent from its low-level details.
///
/// It currently:
/// - installs a looping panic handler;
/// - exports the program entry point without name mangling;
/// - places it in the `.text._start` linker section.
/// - initializes zeroed static storage (`.bss`);
///
/// The program body may return.
/// With the current Spectrum loader arrangement,
/// returning transfers control back to to its caller.
///
/// # Example
/// ```ignore
/// #![no_std]
/// #![no_main]
///
/// use devela_micros::spectrum_main;
///
/// spectrum_main! {
///     // Spectrum program...
/// }
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! spectrum_main· {
    ($($body:tt)*) => {
        $crate::devela::set_panic_handler! { loop }

        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".text._start")]
        pub extern "C" fn start() {
            // LLVM-Z80 prefixes external symbols with `_`,
            // so these bind to linker symbols `__bss_start` and `__bss_end`.
            unsafe extern "C" {
                static mut _bss_start: u8;
                static mut _bss_end: u8;
            }

            let mut addr = unsafe { (&raw mut _bss_start) as usize };
            let end = unsafe { (&raw mut _bss_end) as usize };

            while addr < end {
                unsafe {
                    $crate::devela::Ptr::write_volatile(
                        $crate::devela::Ptr::without_provenance_mut::<u8>(addr), 0);
                }
                addr += 1;
            }

            $($body)*
        }
    };
}
#[doc(inline)]
pub use spectrum_main· as spectrum_main;
