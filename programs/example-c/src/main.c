#include "c_bindings.h"

void _start(void) {
    const char* msg = "Hello world from C";
    __print(msg);
    __c_exit(0);
}
