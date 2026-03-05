use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
pub struct rxmp_handle {
    _private: [u8; 0],
}

pub(crate) struct RxmpHandle {
    inner: std::ptr::NonNull<c_void>,
}

fn handle_from_ptr(ptr: *mut rxmp_handle) -> Option<&'static mut RxmpHandle> {
    if ptr.is_null() {
        return None;
    }

    unsafe {
        Some(&mut *(ptr as *mut RxmpHandle))
    }
}

fn handle_into_ptr(h: Box<RxmpHandle>) -> *mut rxmp_handle {
    Box::into_raw(h) as *mut rxmp_handle
}

fn ffi_guard<T>(f: impl FnOnce() -> T) -> Option<T> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).ok()
}

#[no_mangle]
pub extern "C" fn rxmp_new() -> *mut rxmp_handle {

    ffi_guard(|| {
        unsafe {
            let ptr = xmp_new();
            let inner = std::ptr::NonNull::new(ptr)?;
            let handle = RxmpHandle { inner };
            Some(handle_into_ptr(Box::new(handle)))
        }
    })
        .flatten()
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn rxmp_free(handle: *mut rxmp_handle) {

    let _ = ffi_guard(|| {

        if handle.is_null() {
            return;
        }

        unsafe {
            let boxed = Box::from_raw(handle as *mut RxmpHandle);

            xmp_free(boxed.inner.as_ptr());
        }

    });
}

#[no_mangle]
pub extern "C" fn rxmp_init(handle: *mut rxmp_handle) {
    unsafe { xmp_init(handle as *mut c_void) }
}

#[no_mangle]
pub extern "C" fn rxmp_terminate(handle: *mut rxmp_handle)
{
    unsafe { xmp_terminate(handle as *mut c_void) }
}

#[no_mangle]
pub extern "C" fn rxmp_get_property(
    handle: *mut rxmp_handle,
    schema: *const c_char,
    name: *const c_char,
) -> *mut c_char {
    unsafe {
        let result = xmp_get_property(handle as *mut c_void, schema, name);
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
pub extern "C" fn rxmp_string_free(str: *mut c_char) {
    if str.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(str));
    }
}