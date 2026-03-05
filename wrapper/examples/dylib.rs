use std::ffi::c_uint;
use libloading::{Library, Symbol};
use libxmp::{rxmp_handle, rxmp_version_info};

type RxmpNew = unsafe extern "C" fn() -> *mut rxmp_handle;
type RxmpInit = unsafe extern "C" fn(*mut rxmp_handle) -> bool;
type RxmpGetVersionInfo = unsafe extern "C" fn(*mut rxmp_handle, *mut rxmp_version_info);
type RxmpGetGlobalOptions = unsafe extern "C" fn(*mut rxmp_handle) -> c_uint;
type RxmpSetGlobalOptions = unsafe extern "C" fn(*mut rxmp_handle, c_uint);
type RxmpFree = unsafe extern "C" fn(*mut rxmp_handle);

fn main() {
    unsafe {
        let lib = Library::new("libxmp.dll").unwrap();

        let rxmp_new: Symbol<RxmpNew> = lib.get(b"rxmp_new").unwrap();
        let rxmp_init: Symbol<RxmpInit> = lib.get(b"rxmp_init").unwrap();
        let rxmp_get_version_info: Symbol<RxmpGetVersionInfo> = lib.get(b"rxmp_get_version_info").unwrap();
        let rxmp_get_global_options: Symbol<RxmpGetGlobalOptions> = lib.get(b"rxmp_get_global_options").unwrap();
        let rxmp_set_global_options: Symbol<RxmpSetGlobalOptions> = lib.get(b"rxmp_set_global_options").unwrap();
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

        rxmp_free(handle);
        println!("rxmp_free()");
    }
}