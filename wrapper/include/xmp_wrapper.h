#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void rxmp_init(void);

void rxmp_terminate(void);

void *rxmp_new(void);

void rxmp_free(void *ptr);

char *rxmp_get_property(void *ptr, const char *schema, const char *name);

void rxmp_string_free(char *ptr);
