#include <stdlib.h>

#include "gc.h"


gc_t* new_gc() {
    gc_t* gc = malloc(sizeof(gc_t));
    gc->roots_size = 0;
    return gc;
}

void free_gc(gc_t* self) {
    free(self);
}