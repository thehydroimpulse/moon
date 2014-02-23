#include <stdlib.h>

#include "gc.h"
#include "vm.h"

gc_t*
new_gc() {
    gc_t* self = malloc(sizeof(gc_t));
    self->liveset = new_gc_liveset();
    return self;
}

gc_object_t*
new_gc_object(value_t* value) {
    gc_object_t* self = malloc(sizeof(gc_object_t));
    self->marked = 0;
    self->value = value;
    self->next = NULL;
    return self;
}


gc_liveset_iter_t*
new_gc_liveset_iterator(gc_liveset_t* set) {
    gc_liveset_iter_t* self = malloc(sizeof(gc_liveset_iter_t));
    self->i = 0;
    self->liveset = set;
    self->curr = NULL;
    self->done = 0;
    return self;
}

gc_object_t*
next_gc_liveset_iterator(gc_liveset_iter_t* self) {
    if (self->i == self->liveset->size) {
        return NULL;
    }

    if (self->curr) {
        self->curr = self->curr->next;
        self->i++;
        if (self->curr == NULL) {
            self->done = 1;
            return NULL;
        }
    } else {
        self->curr = self->liveset->first;
        self->i++;
    }

    return self->curr;
}

void
free_gc_liveset_iterator(gc_liveset_iter_t* self) {
    free(self);
}


gc_liveset_t*
new_gc_liveset() {
    gc_liveset_t* self = malloc(sizeof(gc_liveset_t));
    self->size = 0;
    self->first = NULL;
    return self;
}

void
append_gc_liveset(gc_liveset_t* self, gc_object_t* object) {
    if (self->first == NULL) {
        self->first = object;
        self->size  = 1;
    } else {
        gc_liveset_iter_t* iter = new_gc_liveset_iterator(self);
        gc_object_t* obj = NULL;
        while((obj = next_gc_liveset_iterator(iter)) != NULL) {}
        if (obj != NULL) {
            obj->next = object;
            self->size++;
        }

        free_gc_liveset_iterator(iter);
    }
}

void
free_gc_liveset(gc_liveset_t* self) {
    free(self);
}

void
free_gc_object(gc_object_t* self) {
    free(self->value);
    free(self);
}

void
free_gc(gc_t* self) {
    free_gc_liveset(self->liveset);
    free(self);
}
