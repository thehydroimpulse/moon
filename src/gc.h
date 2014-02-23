#ifndef SRC_GC_H
#define SRC_GC_H

#include <stdint.h>
#include "vm.h"

// Linked-list containing all the objects.
struct gc_object {
    struct gc_object *next;

    value_t* value;
    uint8_t marked;
};

struct gc_liveset {
    uint32_t size;
    struct gc_object* first;
};

struct gc {
    struct gc_liveset* liveset;
};

struct gc_liveset_iter {
    int i;
    struct gc_object* curr;
    int done;
    struct gc_liveset* liveset;
};

typedef struct gc gc_t;
typedef struct gc_object gc_object_t;
typedef struct gc_liveset gc_liveset_t;
typedef struct gc_liveset_iter gc_liveset_iter_t;

gc_t* new_gc();
void free_gc(gc_t*);

gc_liveset_iter_t* new_gc_liveset_iterator(gc_liveset_t*);
gc_object_t* next_gc_liveset_iterator(gc_liveset_iter_t*);
void free_gc_liveset_iterator(gc_liveset_iter_t*);

gc_liveset_t* new_gc_liveset();
void append_gc_liveset(gc_liveset_t*,gc_object_t*);
gc_object_t* next_gc_liveset(gc_liveset_t*);
void free_gc_liveset(gc_liveset_t*);

gc_object_t* new_gc_object(value_t*);
void free_gc_object(gc_object_t*);

#endif