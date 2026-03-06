use libloading::{Library, Symbol};
use libxmp::{rxmp_handle, rxmp_version_info, xmp_text_output_proc};
use std::ffi::{c_char, c_uint, c_void, CStr};
use std::ptr::null_mut;

type RxmpNew = unsafe extern "C" fn() -> *mut rxmp_handle;
type RxmpInit = unsafe extern "C" fn(*mut rxmp_handle) -> bool;
type RxmpGetVersionInfo = unsafe extern "C" fn(*mut rxmp_handle, *mut rxmp_version_info);
type RxmpGetGlobalOptions = unsafe extern "C" fn(*mut rxmp_handle) -> c_uint;
type RxmpSetGlobalOptions = unsafe extern "C" fn(*mut rxmp_handle, c_uint);
type RxmpDumpNamespaces = unsafe extern "C" fn(*mut rxmp_handle, xmp_text_output_proc, *mut c_void) -> c_uint;
type RxmpFree = unsafe extern "C" fn(*mut rxmp_handle);

unsafe extern "C" fn text_output_proc(client_data: *mut c_void, buffer: *const c_char, buffer_size: c_uint) -> c_uint {
    let c_str = CStr::from_ptr(buffer);
    match c_str.to_str() {
        Ok(str) => {
            print!("{}", str);
            0
        },
        _ => c_uint::MAX
    }
}

fn main() {
    unsafe {
        let lib = Library::new("libxmp.dll").unwrap();

        let rxmp_new: Symbol<RxmpNew> = lib.get(b"rxmp_new").unwrap();
        let rxmp_init: Symbol<RxmpInit> = lib.get(b"rxmp_init").unwrap();
        let rxmp_get_version_info: Symbol<RxmpGetVersionInfo> = lib.get(b"rxmp_get_version_info").unwrap();
        let rxmp_get_global_options: Symbol<RxmpGetGlobalOptions> = lib.get(b"rxmp_get_global_options").unwrap();
        let rxmp_set_global_options: Symbol<RxmpSetGlobalOptions> = lib.get(b"rxmp_set_global_options").unwrap();
        let rxmp_dump_namespaces: Symbol<RxmpDumpNamespaces> = lib.get(b"rxmp_dump_namespaces").unwrap();
        let rxmp_free: Symbol<RxmpFree> = lib.get(b"rxmp_free").unwrap();

        let handle = rxmp_new();
        let init_result = rxmp_init(handle);
        println!("rxmp_init(): {}", init_result);

        let mut version_info = rxmp_version_info::default();
        rxmp_get_version_info(handle, &mut version_info as *mut rxmp_version_info);

        println!(
            "XMP version: {}.{}.{}.{}: {}, flags: {}",
            version_info.major,
            version_info.minor,
            version_info.micro,
            version_info.build,
            version_info.get_message(),
            version_info.flags
        );

        let global_options = rxmp_get_global_options(handle);
        println!("global_options: {}", global_options);

        // rxmp_set_global_options(handle, global_options);

        let dump_result = rxmp_dump_namespaces(handle, Some(text_output_proc), null_mut());
        println!("dump_result: {}", dump_result);

        rxmp_free(handle);
        println!("rxmp_free()");
    }
}