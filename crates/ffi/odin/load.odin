// Runtime symbol loading.

package devela_ffi

import "core:dynlib"
import "core:fmt"

Library :: struct {
    handle: dynlib.Library,

    abi_version:  Abi_Version_Proc,
    add_i32:      Add_I32_Proc,
    bytes_len:    Bytes_Len_Proc,
    error_string: Error_String_Proc,
}

library_path :: proc() -> string {
    when ODIN_OS == .Linux {
        return "../../libs/libdevela_ffi.so"
    } else when ODIN_OS == .Darwin {
        return "../../libs/libdevela_ffi.dylib"
    } else when ODIN_OS == .Windows {
        return "../../libs/devela_ffi.dll"
    } else {
        return ""
    }
}

load :: proc() -> (lib: Library, ok: bool) {
    path := library_path()
    if path == "" {
        return {}, false
    }

    handle, ok1 := dynlib.load_library(path)
    if !ok1 {
        fmt.eprintln("devela_ffi: failed to load library: ", path)
        return {}, false
    }

    raw_abi_version, ok2  := dynlib.symbol_address(handle, "devela_abi_version")
    raw_add_i32, ok3      := dynlib.symbol_address(handle, "devela_add_i32")
    raw_bytes_len, ok4    := dynlib.symbol_address(handle, "devela_bytes_len")
    raw_error_string, ok5 := dynlib.symbol_address(handle, "devela_error_string")

    if !(ok2 && ok3 && ok4 && ok5) {
        dynlib.unload_library(handle)
        fmt.eprintln("devela_ffi: failed to load one or more symbols")
        return {}, false
    }

    lib = Library {
        handle       = handle,
        abi_version  = cast(Abi_Version_Proc)raw_abi_version,
        add_i32      = cast(Add_I32_Proc)raw_add_i32,
        bytes_len    = cast(Bytes_Len_Proc)raw_bytes_len,
        error_string = cast(Error_String_Proc)raw_error_string,
    }

    return lib, true
}

unload :: proc(lib: ^Library) -> bool {
    if lib.handle == nil {
        return true
    }

    ok := dynlib.unload_library(lib.handle)
    lib^ = {}
    return ok
}
