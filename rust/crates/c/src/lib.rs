// For more details, please see: https://www.greyblake.com/blog/exposing-rust-library-to-c/

use std::ffi::{c_char, CStr};

//
// C API: 'print_hello_from_rust'
//

#[no_mangle]
pub extern "C" fn print_hello_from_rust() {
    println!("Hello from Rust");
}

//
// C API: 'state_to_i32'
//

#[repr(C)]
pub enum State {
    New,
    Init,
    Running,
    Done,
}

#[no_mangle]
pub extern "C" fn state_to_i32(state: State) -> i32 {
    match state {
        State::New => 0,
        State::Init => 1,
        State::Running => 2,
        State::Done => 4,
    }
}

//
// C API: 'c_str_len'
//

#[no_mangle]
pub extern "C" fn c_str_len(str: *const c_char) -> i32 {
    if str.is_null() {
        return -1;
    }

    let cstr = unsafe { CStr::from_ptr(str) };
    match cstr.to_str() {
        Ok(s) => s.len() as i32,
        Err(_) => -1,
    }
}

//
// C API: 'cstring_len'
//

#[repr(C)]
pub struct CString {
    str: *const c_char,
    len: u32,
}

#[no_mangle]
pub extern "C" fn cstring_len(cstr: *const CString) -> i32 {
    if cstr.is_null() {
        return -1;
    }

    unsafe { c_str_len((*cstr).str) }
}

