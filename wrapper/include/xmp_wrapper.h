#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct rxmp_handle {
  uint8_t _private[0];
} rxmp_handle;

struct rxmp_handle *rxmp_new(void);

void rxmp_free(struct rxmp_handle *handle);

void rxmp_init(struct rxmp_handle *handle);

void rxmp_terminate(struct rxmp_handle *handle);

char *rxmp_get_property(struct rxmp_handle *handle, const char *schema, const char *name);

void rxmp_string_free(char *str);
