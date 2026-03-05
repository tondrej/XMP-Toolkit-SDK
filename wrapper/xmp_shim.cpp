#include <string>
#define TXMP_STRING_TYPE std::string
#include "XMP.incl_cpp"

extern "C" {

void xmp_init(void* ptr) {
    static_cast<SXMPMeta*>(ptr)->Initialize();
}

void xmp_terminate(void* ptr) {
    static_cast<SXMPMeta*>(ptr)->Terminate();
}

void* xmp_new() {
    return new SXMPMeta();
}

void xmp_free(void* ptr) {
    delete static_cast<SXMPMeta*>(ptr);
}

void xmp_get_version_info(void* ptr, void* info) {
  static_cast<SXMPMeta*>(ptr)->GetVersionInfo(static_cast<XMP_VersionInfo*>(info));
}

const char* xmp_get_property(
    void* ptr,
    const char* schema,
    const char* name
) {
    static std::string value;
    SXMPMeta* meta = static_cast<SXMPMeta*>(ptr);

    std::string tmp;
    if (meta->GetProperty(schema, name, &tmp, 0)) {
        value = tmp;
        return value.c_str();
    }
    return nullptr;
}

}