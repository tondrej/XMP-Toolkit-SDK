#ifdef __cplusplus
extern "C" {
#endif

int xmp_init(void* handle);
void xmp_terminate(void* handle);
void* xmp_new();
void xmp_free(void* handle);
void xmp_get_version_info(void* handle, void* info);
unsigned int xmp_get_global_options(void* handle);
void xmp_set_global_options(void* handle, unsigned int options);
const char* xmp_get_property(void* handle, const char* schema, const char* name);

#ifdef __cplusplus
}
#endif