#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct rxmp_handle {
  uint8_t _private[0];
} rxmp_handle;

typedef struct rxmp_version_info {
  /**
   * The primary release number, the "1" in version "1.2.3".
   */
  uint8_t major;
  /**
   * The secondary release number, the "2" in version "1.2.3".
   */
  uint8_t minor;
  /**
   * The tertiary release number, the "3" in version "1.2.3".
   */
  uint8_t micro;
  /**
   * A 0/1 boolean value, true if this is a debug build.
   */
  bool is_debug;
  /**
   * A rolling build number, monotonically increasing in a release.
   */
  uint32_t build;
  /**
   * Individual feature implementation flags.
   */
  uint32_t flags;
  /**
   * A comprehensive version information string.
   */
  const char *message;
} rxmp_version_info;

struct rxmp_handle *rxmp_new(void);

struct rxmp_handle *rxmp_new_from_buffer(const char *buffer);

void rxmp_free(struct rxmp_handle *handle);

bool rxmp_init(struct rxmp_handle *handle);

void rxmp_terminate(struct rxmp_handle *handle);

void rxmp_get_version_info(struct rxmp_handle *handle, struct rxmp_version_info *info);

unsigned int rxmp_get_global_options(struct rxmp_handle *handle);

void rxmp_set_global_options(struct rxmp_handle *handle, unsigned int options);

unsigned int rxmp_dump_namespaces(struct rxmp_handle *handle,
                                  xmp_text_output_proc out_proc,
                                  void *client_data);

bool rxmp_parse_from_buffer(struct rxmp_handle *handle,
                            const char *buffer,
                            uint32_t buffer_size,
                            uint32_t options);

uint32_t rxmp_get_property(struct rxmp_handle *handle,
                           const char *schema,
                           const char *name,
                           const char *value,
                           uint32_t value_size,
                           uint64_t *options);

void rxmp_string_free(char *str);
