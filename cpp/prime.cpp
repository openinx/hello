#include <iostream>

int main(int argc, char* argv[]) {
    for(int i = 0; i < 100; i++){
        printf("Hello world\n");
    }

    for(int i = 0 ; i < argc; i++){
        printf("Argument[%d] = [%s]\n", i, argv[i]);
    }
    
    return 0;
}