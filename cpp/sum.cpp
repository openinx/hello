#include <cassert>

int sum(int a, int b) {
    return a + b;
}

int main() {
    assert(sum(1, 1) == 2);
    assert(sum(1, 2) == 3);
    assert(sum(1, 3) == 4);
    return 0;
}