#ifndef SRC_GC_H
#define SRC_GC_H

#include <stdint.h>
#include "vm.h"

// Linked-list containing all the objects.
struct object {
    struct object *next;
    struct object *prev;

    value_t* val;
    uint8_t marked;
};

struct gc {
    struct object* liveset;
    uint32_t liveset_size;
};

typedef struct gc gc_t;
typedef struct object object_t;

gc_t* new_gc();
void free_gc(gc_t*);

#endif