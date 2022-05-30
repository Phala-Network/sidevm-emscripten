/// NOTE: These syscalls only support wasm_32 for now because they take u32 offset
use libc::c_int;
use std::os::raw::c_char;

use crate::env::EmAddrInfo;
use crate::ptr::WasmPtr;
use crate::EmEnv;

// #[no_mangle]
/// emscripten: _getenv // (name: *const char) -> *const c_char;
pub fn _getenv(_ctx: &EmEnv, _name: i32) -> u32 {
    debug!("emscripten::_getenv");
    0 // NULL
}

/// emscripten: _setenv // (name: *const char, name: *const value, overwrite: int);
pub fn _setenv(_ctx: &EmEnv, _name: c_int, _value: c_int, _overwrite: c_int) -> c_int {
    debug!("emscripten::_setenv");
    0
}

/// emscripten: _putenv // (name: *const char);
pub fn _putenv(_ctx: &EmEnv, _name: c_int) -> c_int {
    debug!("emscripten::_putenv");
    0
}

/// emscripten: _unsetenv // (name: *const char);
pub fn _unsetenv(_ctx: &EmEnv, _name: c_int) -> c_int {
    debug!("emscripten::_unsetenv");
    0
}

#[allow(clippy::cast_ptr_alignment)]
pub fn _getpwnam(_ctx: &EmEnv, name_ptr: c_int) -> c_int {
    debug!("emscripten::_getpwnam {}", name_ptr);
    0
}

#[allow(clippy::cast_ptr_alignment)]
pub fn _getgrnam(_ctx: &EmEnv, name_ptr: c_int) -> c_int {
    debug!("emscripten::_getgrnam {}", name_ptr);
    0
}

pub fn _sysconf(_ctx: &EmEnv, name: c_int) -> i32 {
    debug!("emscripten::_sysconf {}", name);
    // TODO: Implement like emscripten expects regarding memory/page size
    -1
}

// this may be a memory leak, probably not though because emscripten does the same thing
pub fn _gai_strerror(_ctx: &EmEnv, ecode: i32) -> i32 {
    debug!("emscripten::_gai_strerror({})", ecode);
    0
}

pub fn _getaddrinfo(
    _ctx: &EmEnv,
    _node_ptr: WasmPtr<c_char>,
    _service_str_ptr: WasmPtr<c_char>,
    _hints_ptr: WasmPtr<EmAddrInfo>,
    _res_val_ptr: WasmPtr<WasmPtr<EmAddrInfo>>,
) -> i32 {
    debug!("emscripten::_getaddrinfo()");
    -1
}
