use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub extern "C" fn rxmp_init() {
    unsafe { xmp_init() }
}

#[no_mangle]
pub extern "C" fn rxmp_terminate() {
    unsafe { xmp_terminate() }
}

#[no_mangle]
pub extern "C" fn rxmp_new() -> *mut c_void {
    unsafe { xmp_new() }
}

#[no_mangle]
pub extern "C" fn rxmp_free(ptr: *mut c_void) {
    unsafe { xmp_free(ptr) }
}

#[no_mangle]
pub extern "C" fn rxmp_get_property(
    ptr: *mut c_void,
    schema: *const c_char,
    name: *const c_char,
) -> *mut c_char {
    unsafe {
        let result = xmp_get_property(ptr, schema, name);
        if result.is_null() {
            return std::ptr::null_mut();
        }

        let c_str = CStr::from_ptr(result);
        CString::new(c_str.to_bytes())
            .unwrap()
            .into_raw()
    }
}

#[no_mangle]
pub extern "C" fn rxmp_string_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
    }
}