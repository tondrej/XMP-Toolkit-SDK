use libxmp::{rxmp_free, rxmp_get_version_info, rxmp_init, rxmp_new, rxmp_version_info};

fn main() {
    let xmp = rxmp_new();
    rxmp_init(xmp);
    println!("rxmp_init()");

    let mut version_info = rxmp_version_info::default();
    rxmp_get_version_info(xmp, &mut version_info as *mut rxmp_version_info);

    println!(
        "XMP version: {}.{}.{}.{}: {}, flags: {}",
        version_info.major,
        version_info.minor,
        version_info.micro,
        version_info.build,
        version_info.get_message(),
        version_info.flags
    );

    rxmp_free(xmp);
    println!("rxmp_free()");
}
