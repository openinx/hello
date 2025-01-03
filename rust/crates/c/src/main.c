#include "c.h"
#include "string.h"
#include "assert.h"

int main(){
    print_hello_from_rust();

    // Test c_str_len
    char *str = "hello world";
    assert(strlen(str) == c_str_len(str));
    assert(-1 == c_str_len(NULL));

    return 0;
}