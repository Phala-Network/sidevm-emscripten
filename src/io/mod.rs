#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::*;

use crate::EmEnv;

/// getprotobyname
pub fn getprotobyname(_ctx: &EmEnv, _name_ptr: i32) -> i32 {
    debug!("emscripten::getprotobyname");
    0 // NULL
}

/// getprotobynumber
pub fn getprotobynumber(_ctx: &EmEnv, _one: i32) -> i32 {
    debug!("emscripten::getprotobynumber");
    0 // NULL
}

/// sigdelset
pub fn sigdelset(_ctx: &EmEnv, _set: i32, _signum: i32) -> i32 {
    debug!("emscripten::sigdelset");
    0
}

/// sigfillset
pub fn sigfillset(_ctx: &EmEnv, _set: i32) -> i32 {
    debug!("emscripten::sigfillset");
    0
}

/// tzset
pub fn tzset(_ctx: &EmEnv) {
    debug!("emscripten::tzset - stub");
    //unimplemented!("emscripten::tzset - stub")
}

/// strptime
pub fn strptime(_ctx: &EmEnv, _one: i32, _two: i32, _three: i32) -> i32 {
    debug!("emscripten::strptime");
    0 // NULL
}
