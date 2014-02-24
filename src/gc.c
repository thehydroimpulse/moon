#include <stdlib.h>
#include <stdio.h>

#include "gc.h"
#include "value.h"


gc_t* gc_new() {
    gc_t* gc = malloc(sizeof(gc_t));
    gc->roots_size = 0;
    gc->objects = gc_list_new();
    return gc;
}

void gc_free(gc_t* self) {
    gc_list_free(self->objects);
    free(self);
}

gc_node_t* 
gc_node_new(value_t* val) {
    gc_node_t* self = malloc(sizeof(gc_node_t));
    self->value = val;
    self->next = NULL;
    self->prev = NULL;
    return self;
}

// FIXME(TheHydroImpulse): Maybe this should also free the value itself?
void
gc_node_free(gc_node_t* self) {
    free(self);
}

gc_list_t*
gc_list_new() {
    gc_list_t* list = malloc(sizeof(gc_list_t));
    list->first = NULL;
    list->size = 0;
    return list;
}

void
gc_list_free(gc_list_t* self) {
    if (self->first) {
        gc_node_t* node = self->first;
        do {
            if (node == NULL) break;
            gc_node_t* n = node;
            node = node->next;
            free(n);
        } while(node != NULL);
    }
    free(self);
}

void
gc_list_append(gc_list_t* self, value_t* value) {
    gc_node_t* n = gc_node_new(value);

    if (self->first) {
        gc_node_t* node = self->first;
        while (node->next != NULL && (node = node->next)) {}

        n->prev = node;
        node->next = n;
    } else {
        self->first = n;
    }

    self->size++;
}

void 
gc_mark_value(gc_t* self, value_t* value) {
    if (value->type == INT) {
        printf("[marking value] type: int val: [%i]\n", value->int_value);
    } else {
        printf("[marking value] type: vec \n");
    }

    value->marked = 1;

    // Vectors are the only object type that has references.
    if (value->type == VEC) {
        for (int k = 0; k < 2; k++) {
            if (value->vec_value[k] != NULL) {
                gc_mark_value(self, value->vec_value[k]);
            }
        }
    }
}

// Algorithm:
//  - Go through each root
//  - Iteratively find each reference and mark each value.
//
void
gc_mark(gc_t* self) {
    // Loop through each roots
    for (int i = 0 ; i < self->roots_size; i++) {
        if (self->roots[i]->type == VEC) {
            for (int k = 0; k < 2; k++) {
                if (self->roots[i]->vec_value[k] != NULL) {
                    gc_mark_value(self, self->roots[i]->vec_value[k]);
                }
            }
        }
    }
}

// Algorithm:
//  - Go through each object and free unmarked objects.
//  - Go through each, again, and unmark them.
void
gc_sweep(gc_t* self, gc_node_t* node) {
    if (node == NULL) return;

    if (node->value->type == VEC) {
        printf("[sweeping] vec\n");
    }

    // Unreachable:
    if (!node->value->marked) {
        printf("[gc] freeing memory. (type: %i)\n", node->value->type);
        gc_node_t* next = node->next;
        gc_node_t* prev = node->prev;
        value_free(node->value);
        gc_node_free(node);

        next->prev = prev;
        if (prev != NULL) {
            prev->next = next;
        }
    }

    gc_sweep(self, node->next);
}

void
gc_register_roots(gc_t* self, value_t* value) {
    if (self->roots_size == MAX_STACK_SIZE) {
        return;
    }

    self->roots[self->roots_size++] = value;
}

//
//  1(0) 1(1) N(2) 1(3)
//  1(0) 1(1) 1(3) 
void
gc_unregister_roots(gc_t* self, value_t* value) {
    for (int i = 0; i < self->roots_size; i++) {
        if (self->roots[i] == value) {

            self->roots[i] = NULL;
            self->roots_size--;

            for (int k = 0; k < MAX_STACK_SIZE; k++) {
                if (!self->roots[k]) {
                    self->roots[k] = self->roots[k+1];
                    k++;
                } else {
                    self->roots[k] = self->roots[k];
                }
            }
        }
    }
}