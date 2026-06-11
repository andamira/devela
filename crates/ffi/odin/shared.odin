// Shared C ABI declarations.

package devela_ffi

Devela_Status :: i32

DEVELA_OK              :: Devela_Status(0)
DEVELA_NO_EVENT        :: Devela_Status(1)
DEVELA_ERR_NULL        :: Devela_Status(-1)
DEVELA_ERR_INVALID     :: Devela_Status(-2)
DEVELA_ERR_PANIC       :: Devela_Status(-3)
DEVELA_ERR_UNSUPPORTED :: Devela_Status(-4)

Abi_Version_Proc  :: proc "c" () -> u32
Add_I32_Proc      :: proc "c" (a, b: i32) -> i32
Bytes_Len_Proc    :: proc "c" (bytes: ^u8, len: uint, out_len: ^uint) -> Devela_Status
Error_String_Proc :: proc "c" (status: Devela_Status) -> cstring
