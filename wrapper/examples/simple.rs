use libxmp::{rxmp_free, rxmp_get_global_options, rxmp_get_version_info, rxmp_init, rxmp_new, rxmp_set_global_options, rxmp_version_info};

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

    rxmp_free(handle);
    println!("rxmp_free()");
}