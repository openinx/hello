#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CString {
  const char *str;
  uint32_t len;
} CString;

void print_hello_from_rust(void);

int32_t c_str_len(const char *str);

int32_t cstring_len(const struct CString *cstr);
