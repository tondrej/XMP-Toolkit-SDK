use std::ffi::{c_uint, c_void, CStr, CString};
use std::os::raw::c_char;
use std::ptr::null;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
pub struct rxmp_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rxmp_version_info {
    /// The primary release number, the "1" in version "1.2.3".
    pub major: u8,
    /// The secondary release number, the "2" in version "1.2.3".
    pub minor: u8,
    /// The tertiary release number, the "3" in version "1.2.3".
    pub micro: u8,
    /// A 0/1 boolean value, true if this is a debug build.
    pub is_debug: bool,
    /// A rolling build number, monotonically increasing in a release.
    pub build: u32,
    /// Individual feature implementation flags.
    pub flags: u32,
    /// A comprehensive version information string.
    pub message: *const c_char,
}

impl rxmp_version_info {
    pub fn default() -> Self {
        rxmp_version_info {
            major: 0,
            minor: 0,
            micro: 0,
            is_debug: false,
            build: 0,
            flags: 0,
            message: null(),
        }
    }

    pub fn get_message(&self) -> String {
        unsafe { CStr::from_ptr(self.message).to_str().unwrap_or("").to_owned() }
    }
}

fn ffi_guard<T>(f: impl FnOnce() -> T) -> Option<T> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).ok()
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_new() -> *mut rxmp_handle {
    ffi_guard(|| unsafe {
        return xmp_new() as *mut rxmp_handle;
    })
    .unwrap_or(std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_new_from_buffer(buffer: *const c_char) -> *mut rxmp_handle {
    ffi_guard(|| unsafe {
        let c_str = CStr::from_ptr(buffer);
        return xmp_new_from_buffer(c_str.as_ptr(), c_str.to_bytes().len() as u32) as *mut rxmp_handle;
    })
    .unwrap_or(std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_free(handle: *mut rxmp_handle) {
    let _ = ffi_guard(|| {
        if handle.is_null() {
            return;
        }
        unsafe {
            xmp_free(handle as *mut c_void);
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_init(handle: *mut rxmp_handle) -> bool {
    ffi_guard(|| {
        if handle.is_null() {
            return 0;
        }
        unsafe { xmp_init(handle as *mut c_void) }
    })
    .unwrap_or(0)
        != 0
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_terminate(handle: *mut rxmp_handle) {
    let _ = ffi_guard(|| {
        if handle.is_null() {
            return;
        }
        unsafe { xmp_terminate(handle as *mut c_void) }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_get_version_info(handle: *mut rxmp_handle, info: *mut rxmp_version_info) {
    let _ = ffi_guard(|| {
        if handle.is_null() {
            return;
        }
        unsafe { xmp_get_version_info(handle as *mut c_void, info as *mut c_void) }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_get_global_options(handle: *mut rxmp_handle) -> c_uint {
    ffi_guard(|| {
        if handle.is_null() {
            return 0;
        }
        unsafe { xmp_get_global_options(handle as *mut c_void) }
    })
    .unwrap_or(0) as c_uint
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_set_global_options(handle: *mut rxmp_handle, options: c_uint) {
    let _ = ffi_guard(|| {
        if handle.is_null() {
            return;
        }
        unsafe { xmp_set_global_options(handle as *mut c_void, options) }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_dump_namespaces(handle: *mut rxmp_handle, out_proc: xmp_text_output_proc,
  client_data: *mut c_void ) -> c_uint {
    ffi_guard(|| {
        if handle.is_null() {
            return 0;
        }
        unsafe { xmp_dump_namespaces(handle as *mut c_void, out_proc, client_data) }
    })
    .unwrap_or(c_uint::MAX)
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_get_property(
    handle: *mut rxmp_handle,
    schema: *const c_char,
    name: *const c_char,
) -> *mut c_char {
    ffi_guard(|| {
        if handle.is_null() {
            return std::ptr::null_mut();
        }
        if schema.is_null() {
            return std::ptr::null_mut();
        }
        if name.is_null() {
            return std::ptr::null_mut();
        }
        unsafe {
            let result = xmp_get_property(handle as *mut c_void, schema, name);
            if result.is_null() {
                return std::ptr::null_mut();
            }

            let c_str = CStr::from_ptr(result);
            return CString::new(c_str.to_bytes()).unwrap().into_raw();
        }
    })
    .unwrap_or(std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn rxmp_string_free(str: *mut c_char) {
    let _ = ffi_guard(|| {
        if str.is_null() {
            return;
        }
        unsafe {
            drop(CString::from_raw(str));
        }
    });
}