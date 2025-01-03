#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum State {
  New,
  Init,
  Running,
  Done,
} State;

typedef struct CString {
  const char *str;
  uint32_t len;
} CString;

void print_hello_from_rust(void);

int32_t state_to_i32(enum State state);

int32_t c_str_len(const char *str);

int32_t cstring_len(const struct CString *cstr);
