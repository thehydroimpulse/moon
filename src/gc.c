#include "gc.h"
#include "vm.h"

gc_t*
new_gc() {
    gc_t* self = malloc(sizeof(gc_t));
    self->liveset = NULL;
    self->liveset_size = 0;
}

void
free_gc(gc_t*) {

}
