// devela::code::result::panic::define
//
//! Defines the [`define_panic_handler!`] macro.
//

/// Defines a panic handler based on the chosen strategy.
///
/// - `loop`: Enters an infinite loop, ensuring the program halts without undefined behavior.
/// - `unreachable`: optimally halts execution based on the target architecture.
///   - `wasm32`: Uses `unreachable()` to signal an unrecoverable state.
///   - `x86_64`: Uses `_mm_pause()` to reduce CPU power consumption.
///   - `aarch64`: Uses `__nop()` as a minimal halt.
///   - `riscv64`: Uses `wfi` (wait-for-interrupt) to idle the core.
///   - **Fallback:** Uses an infinite loop.
///   - (It uses `unsafe`, except for `wasm32` and the fallback.
/// - `custom`: Uses a user-provided function as the panic handler.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! define_panic_handler {
    (loop) => {
        #[panic_handler]
        fn panic(_info: &::core::panic::PanicInfo) -> ! {
            loop {}
        }
    };
    (unreachable) => {
        #[panic_handler]
        #[allow(unreachable_code, reason = "alternative architectures")]
        fn panic(_info: &::core::panic::PanicInfo) -> ! {
            // Signals an unrecoverable state to the WebAssembly runtime
            // https://doc.rust-lang.org/core/arch/wasm32/fn.unreachable.html
            #[cfg(target_arch = "wasm32")]
            ::core::arch::wasm32::unreachable();

            // Low-power pause instruction, avoids busy loops
            // https://doc.rust-lang.org/core/arch/x86_64/fn._mm_pause.html
            #[cfg(target_arch = "x86_64")]
            unsafe {
                ::core::arch::x86_64::_mm_pause();
            }

            // No-op (alternative could be `wfe` for efficiency)
            // https://doc.rust-lang.org/core/arch/aarch64/fn.__nop.html
            // https://doc.rust-lang.org/core/arch/aarch64/fn.__wfe.html (experimental)
            #[cfg(target_arch = "aarch64")]
            unsafe {
                ::core::arch::aarch64::__nop();
            }

            // Wait for interrupt (low-power equivalent)
            #[cfg(target_arch = "riscv64")]
            unsafe {
                ::core::arch::asm!("wfi");
            }

            loop {} // Always fallback to avoid UB if nothing else applies
        }
    };
    (custom, $func:path) => {
        #[panic_handler]
        fn panic(info: &::core::panic::PanicInfo) -> ! {
            $func(info)
        }
    };
}
#[doc(inline)]
pub use define_panic_handler;
