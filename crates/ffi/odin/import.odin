// Compile/link-time foreign import.

package devela_ffi

// Select imported library kind.
// false = shared library: .so/.dylib or Windows import .lib
// true  = static archive: .a/.lib
DEVELA_FFI_STATIC :: false

when ODIN_OS == .Linux {
    when DEVELA_FFI_STATIC {
        foreign import devela_ffi "../libs/libdevela_ffi.a"
    } else {
        foreign import devela_ffi "../libs/libdevela_ffi.so"
    }
} else when ODIN_OS == .Darwin {
    when DEVELA_FFI_STATIC {
        foreign import devela_ffi "../libs/libdevela_ffi.a"
    } else {
        foreign import devela_ffi "../libs/libdevela_ffi.dylib"
    }
} else when ODIN_OS == .Windows {
    foreign import devela_ffi "../libs/devela_ffi.lib"
}

foreign devela_ffi {
    devela_abi_version :: proc "c" () -> u32 ---
    devela_add_i32     :: proc "c" (a, b: i32) -> i32 ---

    devela_bytes_len :: proc "c" (
        bytes: ^u8,
        len: uint,
        out_len: ^uint,
    ) -> Devela_Status ---

    devela_error_string :: proc "c" (
        status: Devela_Status,
    ) -> cstring ---
}
