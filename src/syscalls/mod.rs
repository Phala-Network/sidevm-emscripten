#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::*;

use crate::{
    ptr::{Array, WasmPtr},
    EmEnv, Exited, Result,
};

use super::varargs::VarArgs;

/// NOTE: TODO: These syscalls only support wasm_32 for now because they assume offsets are u32
/// Syscall list: https://www.cs.utexas.edu/~bismith/test/syscalls/syscalls32.html
use libc::{
    c_int,
    getpid,
    // ENOTTY,
};

#[allow(unused_imports)]
use std::io::Error;
use std::slice;

mod fds {
    const FD_BASE: i32 = i32::MAX - 128;

    pub const RANDOM: i32 = FD_BASE;
}

/// exit
pub fn ___syscall1(ctx: &EmEnv, _which: c_int, mut varargs: VarArgs) -> Result<()> {
    debug!("emscripten::___syscall1 (exit) {}", _which);
    let status: i32 = varargs.get(ctx);
    Err(Exited(status))
}

/// read
pub fn ___syscall3(ctx: &EmEnv, _which: i32, mut varargs: VarArgs) -> i32 {
    // -> ssize_t
    debug!("emscripten::___syscall3 (read) {}", _which);
    let fd: i32 = varargs.get(ctx);
    let buf: u32 = varargs.get(ctx);
    let count: i32 = varargs.get(ctx);
    match fd {
        fds::RANDOM => {
            use rand::RngCore;
            let buf = unsafe { slice::from_raw_parts_mut(buf as *mut u8, count as usize) };
            let mut rng = rand::thread_rng();
            rng.fill_bytes(buf);
            count
        }
        _ => -1,
    }
}

/// write
pub fn ___syscall4(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall4 (write) {}", _which);
    -1
}

/// close
pub fn ___syscall6(ctx: &EmEnv, _which: c_int, mut varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall6 (close) {}", _which);
    let fd: i32 = varargs.get(ctx);
    match fd {
        fds::RANDOM => 0,
        _ => -1,
    }
}

// chdir
pub fn ___syscall12(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall12 (chdir) {}", _which);
    -1
}

pub fn ___syscall10(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall10");
    -1
}

pub fn ___syscall14(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall14");
    -1
}

pub fn ___syscall15(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall15");
    -1
}

// getpid
pub fn ___syscall20(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall20 (getpid)");
    unsafe { getpid() }
}

pub fn ___syscall21(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall21");
    -1
}

pub fn ___syscall25(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall25");
    -1
}

pub fn ___syscall29(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall29");
    -1
}

pub fn ___syscall32(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall32");
    -1
}

pub fn ___syscall33(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall33");
    -1
}

pub fn ___syscall36(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall36");
    -1
}

// rename
pub fn ___syscall38(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> i32 {
    debug!("emscripten::___syscall38 (rename)");
    -1
}

// rmdir
pub fn ___syscall40(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall40 (rmdir)");
    -1
}

// pipe
pub fn ___syscall42(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall42 (pipe)");
    -1
}

pub fn ___syscall51(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall51");
    -1
}

pub fn ___syscall52(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall52");
    -1
}

pub fn ___syscall53(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall53");
    -1
}

pub fn ___syscall60(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall60");
    -1
}

// dup2
pub fn ___syscall63(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall63 (dup2) {}", _which);
    -1
}

// getppid
pub fn ___syscall64(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall64 (getppid)");
    1
}

pub fn ___syscall66(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall66");
    -1
}

pub fn ___syscall75(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall75");
    -1
}

pub fn ___syscall91(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall91 - stub");
    0
}

pub fn ___syscall96(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall96");
    -1
}

pub fn ___syscall97(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall97");
    -1
}

pub fn ___syscall110(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall110");
    -1
}

pub fn ___syscall121(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall121");
    -1
}

pub fn ___syscall125(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall125");
    -1
}

pub fn ___syscall133(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall133");
    -1
}

pub fn ___syscall144(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall144");
    -1
}

pub fn ___syscall147(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall147");
    -1
}

pub fn ___syscall150(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall150");
    -1
}

pub fn ___syscall151(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall151");
    -1
}

pub fn ___syscall152(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall152");
    -1
}

pub fn ___syscall153(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall153");
    -1
}

pub fn ___syscall163(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall163");
    -1
}

// getcwd
pub fn ___syscall183(ctx: &EmEnv, _which: c_int, mut varargs: VarArgs) -> Result<i32> {
    debug!("emscripten::___syscall183");
    let buf_offset: WasmPtr<libc::c_char, Array> = varargs.get(ctx);
    let size: c_int = varargs.get(ctx);
    let path_string = "/";
    let len = path_string.len();
    let memory = ctx.memory(0);

    if len + 1 > size as usize {
        // TODO: set_errno(ctx, ERANGE);
        // return NULL
        return Ok(0);
    }

    let buf_writer = buf_offset.deref(&memory, 0, len as u32).ok_or(Exited(-1))?;

    for (i, byte) in path_string.bytes().enumerate() {
        buf_writer[i].set(byte as _);
    }
    buf_writer[len].set(0);
    Ok(buf_offset.offset() as i32)
}

// mmap2
pub fn ___syscall192(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall192 (mmap2) {}", _which);
    // return ENODEV
    -19
}

/// lseek
pub fn ___syscall140(_ctx: &EmEnv, _which: i32, _varargs: VarArgs) -> i32 {
    // -> c_int
    debug!("emscripten::___syscall140 (lseek) {}", _which);
    0
}

/// readv
#[allow(clippy::cast_ptr_alignment)]
pub fn ___syscall145(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> i32 {
    // -> ssize_t
    debug!("emscripten::___syscall145 (readv) {}", _which);
    -1
}

// writev
#[allow(clippy::cast_ptr_alignment)]
pub fn ___syscall146(_ctx: &EmEnv, _which: i32, _varargs: VarArgs) -> i32 {
    // -> ssize_t
    debug!("emscripten::___syscall146 (writev) {}", _which);
    -1
}

pub fn ___syscall191(ctx: &EmEnv, _which: i32, mut varargs: VarArgs) -> i32 {
    let _resource: i32 = varargs.get(ctx);
    debug!(
        "emscripten::___syscall191 - mostly stub, resource: {}",
        _resource
    );
    0
}

pub fn ___syscall193(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall193");
    -1
}

// stat64
pub fn ___syscall195(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall195 (stat64) {}", _which);
    -1
}

// fstat64
pub fn ___syscall197(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall197 (fstat64) {}", _which);
    -1
}

pub fn ___syscall209(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall209");
    -1
}

pub fn ___syscall211(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall211");
    -1
}

pub fn ___syscall218(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall218");
    -1
}

pub fn ___syscall268(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall268");
    -1
}

pub fn ___syscall269(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall269");
    -1
}

pub fn ___syscall272(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall272");
    -1
}

pub fn ___syscall295(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall295");
    -1
}

pub fn ___syscall296(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall296");
    -1
}

pub fn ___syscall297(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall297");
    -1
}

pub fn ___syscall298(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall298");
    -1
}

pub fn ___syscall300(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall300");
    -1
}

pub fn ___syscall301(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall301");
    -1
}

pub fn ___syscall302(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall302");
    -1
}

pub fn ___syscall303(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall303");
    -1
}

pub fn ___syscall304(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall304");
    -1
}

pub fn ___syscall305(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall305");
    -1
}

pub fn ___syscall306(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall306");
    -1
}

pub fn ___syscall307(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall307");
    -1
}

pub fn ___syscall308(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall308");
    -1
}

// utimensat
pub fn ___syscall320(_ctx: &EmEnv, _which: c_int, mut _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall320 (utimensat), {}", _which);
    0
}

pub fn ___syscall331(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall331");
    -1
}

pub fn ___syscall333(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall333");
    -1
}

pub fn ___syscall334(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall334");
    -1
}

pub fn ___syscall337(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall337");
    -1
}

// prlimit64
pub fn ___syscall340(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall340 (prlimit64), {}", _which);
    // NOTE: Doesn't really matter. Wasm modules cannot exceed WASM_PAGE_SIZE anyway.
    -1
}

pub fn ___syscall345(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall345");
    -1
}
