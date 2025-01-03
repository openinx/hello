#include "stdint.h"

void print_hello_from_rust();

static const uint32_t C_NEW = 0;
static const uint32_t C_INIT = 1;
static const uint32_t C_RUNNING = 2;
static const uint32_t C_DONE = 3;

int32_t c_str_len(char *str);

typedef struct CString{
    char *str;
    uint32_t len;
}CString;

int32_t cstring_len(CString *cstr);
