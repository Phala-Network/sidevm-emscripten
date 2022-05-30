use super::env;
use super::env::get_emscripten_data;
use crate::storage::align_memory;
use crate::EmEnv;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::c_char;
use std::slice;
use wasmer::{GlobalInit, Memory, Module, Pages};

/// We check if a provided module is an Emscripten generated one
pub fn is_emscripten_module(module: &Module) -> bool {
    for import in module.imports().functions() {
        let name = import.name();
        let module = import.module();
        if (name == "_emscripten_memcpy_big"
            || name == "emscripten_memcpy_big"
            || name == "__map_file")
            && module == "env"
        {
            return true;
        }
    }
    false
}

pub fn get_emscripten_table_size(module: &Module) -> Result<(u32, Option<u32>), String> {
    if let Some(import) = module.imports().tables().next() {
        let ty = import.ty();
        Ok((ty.minimum, ty.maximum))
    } else {
        Err("Emscripten requires at least one imported table".to_string())
    }
}

pub fn get_emscripten_memory_size(module: &Module) -> Result<(Pages, Option<Pages>, bool), String> {
    if let Some(import) = module.imports().memories().next() {
        let ty = import.ty();
        Ok((ty.minimum, ty.maximum, ty.shared))
    } else {
        Err("Emscripten requires at least one imported memory".to_string())
    }
}

/// Reads values written by `-s EMIT_EMSCRIPTEN_METADATA=1`
/// Assumes values start from the end in this order:
/// Last export: Dynamic Base
/// Second-to-Last export: Dynamic top pointer
pub fn get_emscripten_metadata(module: &Module) -> Result<Option<(u32, u32)>, String> {
    let max_idx = match module
        .info()
        .global_initializers
        .iter()
        .map(|(k, _)| k)
        .max()
    {
        Some(x) => x,
        None => return Ok(None),
    };

    let snd_max_idx = match module
        .info()
        .global_initializers
        .iter()
        .map(|(k, _)| k)
        .filter(|k| *k != max_idx)
        .max()
    {
        Some(x) => x,
        None => return Ok(None),
    };

    if let (GlobalInit::I32Const(dynamic_base), GlobalInit::I32Const(dynamictop_ptr)) = (
        &module.info().global_initializers[max_idx],
        &module.info().global_initializers[snd_max_idx],
    ) {
        let dynamic_base = (*dynamic_base as u32).checked_sub(32).ok_or(format!(
            "emscripten unexpected dynamic_base {}",
            *dynamic_base as u32
        ))?;
        let dynamictop_ptr = (*dynamictop_ptr as u32).checked_sub(32).ok_or(format!(
            "emscripten unexpected dynamictop_ptr {}",
            *dynamictop_ptr as u32
        ))?;
        Ok(Some((
            align_memory(dynamic_base),
            align_memory(dynamictop_ptr),
        )))
    } else {
        Ok(None)
    }
}

pub unsafe fn write_to_buf(ctx: &EmEnv, string: *const c_char, buf: u32, max: u32) -> u32 {
    let buf_addr = emscripten_memory_pointer!(ctx.memory(0), buf) as *mut c_char;

    for i in 0..max {
        *buf_addr.add(i as _) = *string.add(i as _);
    }

    buf
}

/// This function expects nullbyte to be appended.
pub unsafe fn copy_cstr_into_wasm(ctx: &EmEnv, cstr: *const c_char) -> u32 {
    let s = CStr::from_ptr(cstr).to_str().unwrap();
    let cstr_len = s.len();
    let space_offset = env::call_malloc(ctx, (cstr_len as u32) + 1);
    let raw_memory = emscripten_memory_pointer!(ctx.memory(0), space_offset) as *mut c_char;
    let slice = slice::from_raw_parts_mut(raw_memory, cstr_len);

    for (byte, loc) in s.bytes().zip(slice.iter_mut()) {
        *loc = byte as _;
    }

    // TODO: Appending null byte won't work, because there is CStr::from_ptr(cstr)
    //      at the top that crashes when there is no null byte
    *raw_memory.add(cstr_len) = 0;

    space_offset
}

pub unsafe fn allocate_on_stack<'a, T: Copy>(ctx: &'a EmEnv, count: u32) -> (u32, &'a mut [T]) {
    let offset = get_emscripten_data(ctx)
        .stack_alloc_ref()
        .unwrap()
        .call(count * (size_of::<T>() as u32))
        .unwrap();

    let addr = emscripten_memory_pointer!(ctx.memory(0), offset) as *mut T;
    let slice = slice::from_raw_parts_mut(addr, count as usize);

    (offset, slice)
}

pub unsafe fn allocate_cstr_on_stack<'a>(ctx: &'a EmEnv, s: &str) -> (u32, &'a [u8]) {
    let (offset, slice) = allocate_on_stack(ctx, (s.len() + 1) as u32);

    use std::iter;
    for (byte, loc) in s.bytes().chain(iter::once(0)).zip(slice.iter_mut()) {
        *loc = byte;
    }

    (offset, slice)
}

#[allow(dead_code)] // it's used in `env/windows/mod.rs`.
pub fn read_string_from_wasm(memory: &Memory, offset: u32) -> String {
    let v: Vec<u8> = memory.view()[(offset as usize)..]
        .iter()
        .map(|cell| cell.get())
        .take_while(|&byte| byte != 0)
        .collect();
    String::from_utf8_lossy(&v).to_owned().to_string()
}
