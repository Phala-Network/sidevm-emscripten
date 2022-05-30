use crate::varargs::VarArgs;
use crate::EmEnv;

pub fn execvp(_ctx: &EmEnv, _command_name_offset: u32, _argv_offset: u32) -> i32 {
    debug!("emscripten::execvp");
    -1
}

/// execl
pub fn execl(_ctx: &EmEnv, _path_ptr: i32, _arg0_ptr: i32, _varargs: VarArgs) -> i32 {
    debug!("emscripten::execl");
    -1
}

/// execle
pub fn execle(_ctx: &EmEnv, _path_ptr: i32, _arg0_ptr: i32, _varargs: VarArgs) -> i32 {
    debug!("emscripten::execle");
    -1
}
