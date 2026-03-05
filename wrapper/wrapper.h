#ifdef __cplusplus
extern "C" {
#endif

void xmp_init();
void xmp_terminate();
void* xmp_new();
void xmp_free(void* ptr);
const char* xmp_get_property(void* ptr, const char* schema, const char* name);

#ifdef __cplusplus
}
#endif