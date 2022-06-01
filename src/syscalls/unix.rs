use crate::varargs::VarArgs;
/// NOTE: TODO: These syscalls only support wasm_32 for now because they assume offsets are u32
/// Syscall list: https://www.cs.utexas.edu/~bismith/test/syscalls/syscalls32.html
use libc::{c_int, pid_t};

use crate::EmEnv;

/// open
pub fn ___syscall5(ctx: &EmEnv, _which: c_int, mut varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall5 (open) {}", _which);
    let pathname_addr = varargs.get_str(ctx);
    let _flags: i32 = varargs.get(ctx);
    let _mode: u32 = varargs.get(ctx);
    let path_str = unsafe { std::ffi::CStr::from_ptr(pathname_addr).to_str() };
    match path_str {
        Ok("/dev/urandom" | "/dev/random") => crate::syscalls::fds::RANDOM,
        _ => -1,
    }
}

/// link
pub fn ___syscall9(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall9 (link) {}", _which);
    -1
}

/// getrusage
pub fn ___syscall77(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall77 (getrusage) {}", _which);
    -1
}

/// symlink
pub fn ___syscall83(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall83 (symlink) {}", _which);
    -1
}

/// readlink
pub fn ___syscall85(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> i32 {
    debug!("emscripten::___syscall85 (readlink)");
    -1
}

/// ftruncate64
pub fn ___syscall194(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall194 (ftruncate64) {}", _which);
    -1
}

/// lchown
pub fn ___syscall198(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall198 (lchown) {}", _which);
    -1
}

/// getgroups
pub fn ___syscall205(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall205 (getgroups) {}", _which);
    -1
}

// chown
pub fn ___syscall212(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall212 (chown) {}", _which);
    -1
}

/// madvise
pub fn ___syscall219(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall212 (chown) {}", _which);
    -1
}

/// access
pub fn ___syscall33(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall33 (access) {}", _which);
    -1
}

/// nice
pub fn ___syscall34(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall34 (nice) {}", _which);
    -1
}

// mkdir
pub fn ___syscall39(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall39 (mkdir) {}", _which);
    -1
}

/// dup
pub fn ___syscall41(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall41 (dup) {}", _which);
    -1
}

/// getgid32
pub fn ___syscall200(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall200 (getgid32)");
    -1
}

// geteuid32
pub fn ___syscall201(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall201 (geteuid32)");
    -1
}

// getegid32
pub fn ___syscall202(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    // gid_t
    debug!("emscripten::___syscall202 (getegid32)");
    -1
}

/// fchown
pub fn ___syscall207(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall207 (fchown) {}", _which);
    -1
}

/// dup3
pub fn ___syscall330(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> pid_t {
    // Implementation based on description at https://linux.die.net/man/2/dup3
    debug!("emscripten::___syscall330 (dup3)");
    -1
}

/// ioctl
pub fn ___syscall54(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall54 (ioctl) {}", _which);
    -1
}

// socketcall
#[allow(clippy::cast_ptr_alignment)]
pub fn ___syscall102(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall102 (socketcall) {}", _which);
    -1
}

/// getpgid
pub fn ___syscall132(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall132 (getpgid)");
    -1
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct EmPollFd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

unsafe impl wasmer::ValueType for EmPollFd {}

/// poll
pub fn ___syscall168(_ctx: &EmEnv, _which: i32, _varargs: VarArgs) -> i32 {
    debug!("emscripten::___syscall168(poll)");
    -1
}

// pread
pub fn ___syscall180(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall180 (pread) {}", _which);
    -1
}

// pwrite
pub fn ___syscall181(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall181 (pwrite) {}", _which);
    -1
}

/// fchmod
pub fn ___syscall94(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall118 (fchmod) {}", _which);
    -1
}

/// wait4
#[allow(clippy::cast_ptr_alignment)]
pub fn ___syscall114(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> pid_t {
    debug!("emscripten::___syscall114 (wait4)");
    -1
}

/// fsync
pub fn ___syscall118(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall118 (fsync) {}", _which);
    -1
}

// select
#[allow(clippy::cast_ptr_alignment)]
pub fn ___syscall142(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall142 (newselect) {}", _which);
    -1
}

/// fdatasync
pub fn ___syscall148(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall148 (fdatasync) {}", _which);
    -1
}

// setpgid
pub fn ___syscall57(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall57 (setpgid) {}", _which);
    -1
}

/// uname
// NOTE: Wondering if we should return custom utsname, like Emscripten.
pub fn ___syscall122(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall122 (uname) {}", _which);
    -1
}

/// lstat64
pub fn ___syscall196(_ctx: &EmEnv, _which: i32, _varargs: VarArgs) -> i32 {
    debug!("emscripten::___syscall196 (lstat64) {}", _which);
    -1
}

// getuid
pub fn ___syscall199(_ctx: &EmEnv, _one: i32, _two: i32) -> i32 {
    debug!("emscripten::___syscall199 (getuid)");
    -1
}

// getdents
// dirent structure is
// i64, i64, u16 (280), i8, [i8; 256]
pub fn ___syscall220(_ctx: &EmEnv, _which: i32, _varargs: VarArgs) -> i32 {
    debug!("emscripten::___syscall220 (getdents)");
    -1
}

// fcntl64
pub fn ___syscall221(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall221 (fcntl64) {}", _which);
    -1
}

/// fallocate
pub fn ___syscall324(_ctx: &EmEnv, _which: c_int, _varargs: VarArgs) -> c_int {
    debug!("emscripten::___syscall324 (fallocate) {}", _which);
    -1
}
