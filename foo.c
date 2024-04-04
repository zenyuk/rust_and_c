#include "stdint.h"

int32_t add(int32_t x, int32_t y) {
    return x + y;
}

/* 
 * compile with:
 * gcc -fPIC -shared -o libfoo.so foo.c
 * 
 *
 * */
