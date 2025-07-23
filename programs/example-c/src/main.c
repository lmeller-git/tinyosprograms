#include "../includes/libtinyos.h"

void _start(void) {
    const char* msg = "Hello world from C\n";
    __print(msg);
    __c_exit(0);
}
