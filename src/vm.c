#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "vm.h"
#include "gc.h"

vm_t* 
new_vm() {
    vm_t* self = malloc(sizeof(vm_t));
    self->stack_size = 0;
    //self->gc = new_gc();
    return self;
}

void 
free_vm(vm_t* self) {

    // Clear the stack.
    if (self->stack_size > 0) {
        // We only need to free allocated slots.
        for (int i = 0; i < self->stack_size; i++) {
            free_value(self->stack[i]);
        }
    }

    free(self);
}

void
push_stack(vm_t* self, value_t* val) {
    // Stack Overflow
    if (self->stack_size == MAX_STACK_SIZE) {
        printf("The VM experienced a stack overflow. Exiting...");
        exit(-1);
    }

    self->stack[self->stack_size++] = val;
}


value_t*
pop_stack(vm_t* self) {
    // Should we crash?
    if (self->stack_size == 0) {
        return NULL;
    }

    value_t* val = self->stack[self->stack_size - 1];
    self->stack[self->stack_size - 1] = NULL;
    self->stack_size -= 1;

    return val;
}