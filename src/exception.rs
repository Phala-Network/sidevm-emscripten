use super::env;
use super::process::_abort;
use crate::process::abort_with_message;
use crate::{EmEnv, Result};

/// emscripten: ___cxa_allocate_exception
pub fn ___cxa_allocate_exception(ctx: &EmEnv, size: u32) -> u32 {
    debug!("emscripten::___cxa_allocate_exception");
    env::call_malloc(ctx, size as _)
}

pub fn ___cxa_current_primary_exception(ctx: &EmEnv) -> Result<u32> {
    debug!("emscripten::___cxa_current_primary_exception");
    abort_with_message(ctx, "emscripten::___cxa_current_primary_exception")
}

pub fn ___cxa_decrement_exception_refcount(ctx: &EmEnv, _a: u32) -> Result<()> {
    debug!("emscripten::___cxa_decrement_exception_refcount({})", _a);
    abort_with_message(ctx, "emscripten::___cxa_decrement_exception_refcount")
}

pub fn ___cxa_increment_exception_refcount(ctx: &EmEnv, _a: u32) -> Result<()> {
    debug!("emscripten::___cxa_increment_exception_refcount({})", _a);
    abort_with_message(ctx, "emscripten::___cxa_increment_exception_refcount")
}

pub fn ___cxa_rethrow_primary_exception(ctx: &EmEnv, _a: u32) -> Result<()> {
    debug!("emscripten::___cxa_rethrow_primary_exception({})", _a);
    abort_with_message(ctx, "emscripten::___cxa_rethrow_primary_exception")
}

/// emscripten: ___cxa_throw
/// TODO: We don't have support for exceptions yet
pub fn ___cxa_throw(ctx: &EmEnv, _ptr: u32, _ty: u32, _destructor: u32) -> Result<()> {
    debug!("emscripten::___cxa_throw");
    eprintln!("Throwing exceptions not yet implemented: aborting!");
    _abort(ctx)
}

pub fn ___cxa_begin_catch(_ctx: &EmEnv, _exception_object_ptr: u32) -> i32 {
    debug!("emscripten::___cxa_begin_catch");
    -1
}

pub fn ___cxa_end_catch(_ctx: &EmEnv) {
    debug!("emscripten::___cxa_end_catch");
}

pub fn ___cxa_uncaught_exception(_ctx: &EmEnv) -> i32 {
    debug!("emscripten::___cxa_uncaught_exception");
    -1
}

pub fn ___cxa_pure_virtual(ctx: &EmEnv) -> Result<()> {
    debug!("emscripten::___cxa_pure_virtual");
    abort_with_message(ctx, "Pure virtual function called!")
}
