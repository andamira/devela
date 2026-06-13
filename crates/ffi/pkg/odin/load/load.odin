// Runtime symbol loading.

package devela_ffi_load

import "core:dynlib"
import "core:fmt"
import common "../common"

Devela_Status :: common.Devela_Status

DEVELA_OK              :: common.DEVELA_OK
DEVELA_NO_EVENT        :: common.DEVELA_NO_EVENT
DEVELA_ERR_NULL        :: common.DEVELA_ERR_NULL
DEVELA_ERR_INVALID     :: common.DEVELA_ERR_INVALID
DEVELA_ERR_PANIC       :: common.DEVELA_ERR_PANIC
DEVELA_ERR_UNSUPPORTED :: common.DEVELA_ERR_UNSUPPORTED

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

    lib, ok = load_fns(handle)
    if !ok {
        dynlib.unload_library(handle)
        fmt.eprintln("devela_ffi: failed to load one or more symbols")
        return {}, false
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
