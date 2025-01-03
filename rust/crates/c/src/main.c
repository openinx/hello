#include "c.h"
#include "stdio.h"
#include "string.h"
#include "assert.h"

int main(){
    print_hello_from_rust();

    // Test c_str_len
    char *str = "hello world";
    assert(strlen(str) == c_str_len(str));
    assert(-1 == c_str_len(NULL));

    CString cstr = {"hello world", 11};
    assert(strlen(str) == cstring_len(&cstr));
    assert(-1 == cstring_len(NULL));
    printf("%d\n", cstring_len(&cstr));

    return 0;
}