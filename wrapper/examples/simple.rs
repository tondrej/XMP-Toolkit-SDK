use libxmp::{rxmp_dump_namespaces, rxmp_free, rxmp_get_global_options,
             rxmp_get_version_info, rxmp_init, rxmp_new, rxmp_version_info};
use std::ffi::{c_char, c_uint, c_void, CStr};
use std::ptr::null_mut;

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