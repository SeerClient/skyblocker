//! SEER DIAGNOSTIC (not a port file): on a stack overflow, print the top of the
//! overflowing stack as `module+offset` lines to stderr, then let the process
//! die as it would have. Costs nothing until the exception fires.

#![allow(non_snake_case)]

use std::ffi::c_void;

#[repr(C)]
struct ExceptionRecord {
    code: u32,
    flags: u32,
    record: *mut ExceptionRecord,
    address: *mut c_void,
}

#[repr(C)]
struct ExceptionPointers {
    record: *mut ExceptionRecord,
    context: *mut c_void,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: unsafe extern "system" fn(*mut ExceptionPointers) -> i32) -> *mut c_void;
    fn RtlCaptureStackBackTrace(skip: u32, count: u32, frames: *mut *mut c_void, hash: *mut u32) -> u16;
    fn GetModuleHandleExW(flags: u32, addr: *const c_void, module: *mut *mut c_void) -> i32;
    fn GetModuleFileNameA(module: *mut c_void, name: *mut u8, size: u32) -> u32;
    fn GetStdHandle(which: u32) -> *mut c_void;
    fn WriteFile(h: *mut c_void, buf: *const u8, len: u32, written: *mut u32, ov: *mut c_void) -> i32;
}

static mut FRAMES: [*mut c_void; 96] = [std::ptr::null_mut(); 96];
static mut NAME: [u8; 260] = [0; 260];
static mut LINE: [u8; 400] = [0; 400];
static FIRED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

unsafe fn put(bytes: &[u8]) {
    let mut w = 0u32;
    unsafe { WriteFile(GetStdHandle(-12i32 as u32), bytes.as_ptr(), bytes.len() as u32, &mut w, std::ptr::null_mut()) };
}

unsafe fn hex(mut v: u64, out: &mut [u8], at: &mut usize) {
    let mut tmp = [0u8; 16];
    let mut n = 0;
    loop {
        let d = (v & 0xf) as u8;
        tmp[n] = if d < 10 { b'0' + d } else { b'a' + d - 10 };
        n += 1;
        v >>= 4;
        if v == 0 {
            break;
        }
    }
    while n > 0 {
        n -= 1;
        out[*at] = tmp[n];
        *at += 1;
    }
}

unsafe extern "system" fn handler(info: *mut ExceptionPointers) -> i32 {
    unsafe {
        let code = (*(*info).record).code;
        if code != 0xC00000FD || FIRED.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return 0;
        }
        put(b"\n=== SEER STACK OVERFLOW TRACE (module+offset) ===\n");
        let frames = &raw mut FRAMES;
        let n = RtlCaptureStackBackTrace(0, 96, (*frames).as_mut_ptr(), std::ptr::null_mut());
        for i in 0..n as usize {
            let addr = (*frames)[i];
            let mut module: *mut c_void = std::ptr::null_mut();
            let line = &mut *(&raw mut LINE);
            let mut at = 0usize;
            if GetModuleHandleExW(0x4 | 0x2, addr, &mut module) != 0 {
                let name = &mut *(&raw mut NAME);
                let len = GetModuleFileNameA(module, name.as_mut_ptr(), 260) as usize;
                let start = name[..len].iter().rposition(|&c| c == b'\\').map(|p| p + 1).unwrap_or(0);
                for &c in &name[start..len] {
                    line[at] = c;
                    at += 1;
                }
                line[at] = b'+';
                at += 1;
                hex(addr as u64 - module as u64, line, &mut at);
            } else {
                hex(addr as u64, line, &mut at);
            }
            line[at] = b'\n';
            at += 1;
            put(&line[..at]);
        }
        put(b"=== END TRACE ===\n");
        0
    }
}

pub fn install() {
    unsafe { AddVectoredExceptionHandler(1, handler) };
}
