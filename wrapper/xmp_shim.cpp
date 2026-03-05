#include <string>
#define TXMP_STRING_TYPE std::string
#include "XMP.incl_cpp"

extern "C" {

void xmp_init() {
    SXMPMeta::Initialize();
}

void xmp_terminate() {
    SXMPMeta::Terminate();
}

void* xmp_new() {
    return new SXMPMeta();
}

void xmp_free(void* ptr) {
    delete static_cast<SXMPMeta*>(ptr);
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