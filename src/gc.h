#ifndef SRC_GC_H
#define SRC_GC_H

#include <stdint.h>
#include "value.h"
#include "vm.h"

typedef struct gc_node {
    struct gc_node* next;
    value_t* value;
} gc_node_t;

typedef struct gc_list {
    gc_node_t* first;
    int size;
} gc_list_t;

typedef struct {
    value_t* roots[MAX_STACK_SIZE];
    int roots_size;
} gc_t;

gc_t* gc_new();
void gc_free(gc_t*);

gc_node_t* gc_node_new(value_t*);
void gc_node_free(gc_node_t*);

gc_list_t* gc_list_new();
void gc_list_free(gc_list_t*);
void gc_list_append(gc_list_t*,value_t*);

#endif