#ifndef SRC_GC_H
#define SRC_GC_H

#include <stdint.h>
#include "value.h"
#include "vm.h"

typedef struct {
    value_t* roots[MAX_STACK_SIZE];
    int roots_size;
} gc_t;

gc_t* new_gc();
void free_gc(gc_t*);

#endif