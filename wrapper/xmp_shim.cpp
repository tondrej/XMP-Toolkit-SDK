#include <string>
#include <cstring>
#define TXMP_STRING_TYPE std::string
#include "XMP.incl_cpp"

extern "C" {

int xmp_init(void* ptr) {
    return static_cast<SXMPMeta*>(ptr)->Initialize();
}

void xmp_terminate(void* ptr) {
    static_cast<SXMPMeta*>(ptr)->Terminate();
}

void* xmp_new() {
    return new SXMPMeta();
}

void* xmp_new_from_buffer(const char* buffer, unsigned int buffer_size) {
    return new SXMPMeta(buffer, buffer_size);
}

void xmp_free(void* ptr) {
    delete static_cast<SXMPMeta*>(ptr);
}

void xmp_get_version_info(void* ptr, void* info) {
  static_cast<SXMPMeta*>(ptr)->GetVersionInfo(static_cast<XMP_VersionInfo*>(info));
}

unsigned int xmp_get_global_options(void* ptr) {
  return static_cast<SXMPMeta*>(ptr)->GetGlobalOptions();
}

void xmp_set_global_options(void* ptr, unsigned int options) {
  static_cast<SXMPMeta*>(ptr)->SetGlobalOptions(options);
}

unsigned int xmp_dump_namespaces(void* ptr, XMP_TextOutputProc out_proc, void* client_data) {
  return static_cast<SXMPMeta*>(ptr)->DumpNamespaces(out_proc, client_data);
}

int xmp_parse_from_buffer(void* ptr, const char* buffer, unsigned int buffer_size, unsigned int options) {
  static_cast<SXMPMeta*>(ptr)->ParseFromBuffer(buffer, buffer_size, options);
  return 1;
}

unsigned int xmp_get_property(void* ptr, const char* schema, const char* name, char* value, unsigned int value_size, long unsigned int* options) {
  SXMPMeta* meta = static_cast<SXMPMeta*>(ptr);

  std::string tmp;
  if (meta->GetProperty(schema, name, &tmp, options)) {
    unsigned int size_needed = tmp.length() + 1;
    if (value && value_size >= size_needed) {
      std::strcpy(value, tmp.c_str());
    }
    return size_needed;
  }
  return 0;
}

}