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
typedef unsigned int (* xmp_text_output_proc) (void* client_data, const char* buffer, unsigned int buffer_size);
unsigned int xmp_dump_namespaces(xmp_text_output_proc out_proc, void* client_data);
const char* xmp_get_property(void* handle, const char* schema, const char* name);

#ifdef __cplusplus
}
#endif