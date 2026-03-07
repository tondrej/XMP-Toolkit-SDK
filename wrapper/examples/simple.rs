use libxmp::{rxmp_dump_namespaces, rxmp_free, rxmp_get_global_options, rxmp_get_property, rxmp_get_version_info, rxmp_handle, rxmp_init, rxmp_new, rxmp_parse_from_buffer, rxmp_version_info};
use std::ffi::{c_char, c_uint, c_void, CStr, CString};
use std::ptr::null_mut;

const XMP_PACKET: &str = "<?xpacket begin='þÿ' id='W5M0MpCehiHzreSzNTczkc9d'?>
<x:xmpmeta xmlns:x=\"adobe:ns:meta/\">
  <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">
    <rdf:Description xmlns:dc=\"http://purl.org/dc/elements/1.1/\" rdf:about=\"\">
      <dc:creator>
        <rdf:Seq>
          <rdf:li>dc creator</rdf:li>
        </rdf:Seq>
      </dc:creator>
    </rdf:Description>
    <rdf:Description xmlns:xmp=\"http://ns.adobe.com/xap/1.0/\" rdf:about=\"\" xmp:CreatorTool=\"test creator\" xmp:CreateDate=\"2025-11-11T11:11:11+01:00\" xmp:ModifyDate=\"2025-11-11T11:11:11+01:00\"/>
    <rdf:Description xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\" rdf:about=\"\" pdf:Producer=\"test producer\"/>
    <rdf:Description xmlns:pdfaid=\"http://www.aiim.org/pdfa/ns/id/\" rdf:about=\"\" pdfaid:part=\"1\" pdfaid:conformance=\"B\"/>
  </rdf:RDF>
</x:xmpmeta>
<?xpacket end='w'?>";

unsafe extern "C" fn text_output_proc(_client_data: *mut c_void, buffer: *const c_char, _buffer_size: c_uint) -> c_uint {
    unsafe {
        match CStr::from_ptr(buffer).to_str() {
            Ok(str) => {
                print!("{}", str);
                0
            },
            _ => c_uint::MAX
        }
    }
}

fn show_property(handle: *mut rxmp_handle, schema: &str, name: &str) {
    let c_schema = CString::new(schema).unwrap();
    let c_name = CString::new(name).unwrap();
    let value_size = rxmp_get_property(handle, c_schema.as_ptr(), c_name.as_ptr(), null_mut(), 0, null_mut()) as usize;
    let c_value = CString::new(vec![32u8; value_size]).unwrap();
    rxmp_get_property(handle, c_schema.as_ptr(), c_name.as_ptr(), c_value.as_ref().as_ptr(), value_size as u32, null_mut());
    println!("rxmp_get_property(\"{}\", \"{}\"): \"{}\"", schema, name, c_value.to_str().unwrap());
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

    let c_str = CString::new(XMP_PACKET).unwrap();
    let parse_result = rxmp_parse_from_buffer(handle, c_str.as_ptr(), c_str.as_bytes().len() as u32, 0);
    println!("rxmp_parse_from_buffer(): {}", parse_result);

    show_property(handle, "http://ns.adobe.com/xap/1.0/", "CreatorTool");
    show_property(handle, "http://ns.adobe.com/pdf/1.3/", "Producer");

    rxmp_free(handle);
    println!("rxmp_free()");
}