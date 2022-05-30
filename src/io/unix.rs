use crate::EmEnv;

/// putchar
pub fn putchar(_ctx: &EmEnv, _chr: i32) {}

/// printf
pub fn printf(_ctx: &EmEnv, memory_offset: i32, extra: i32) -> i32 {
    debug!("emscripten::printf {}, {}", memory_offset, extra);
    -1
}

/// chroot
pub fn chroot(_ctx: &EmEnv, _name_ptr: i32) -> i32 {
    debug!("emscripten::chroot");
    -1
}

/// getpwuid
#[allow(clippy::cast_ptr_alignment)]
pub fn getpwuid(_ctx: &EmEnv, uid: i32) -> i32 {
    debug!("emscripten::getpwuid {}", uid);
    0
}
