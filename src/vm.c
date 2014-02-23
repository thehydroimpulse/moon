#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "vm.h"
#include "gc.h"

vm_t* 
new_vm() {
    vm_t* self = malloc(sizeof(vm_t));
    self->stack_size = 0;
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



value_t*
new_int(int a) {
    value_t* self = malloc(sizeof(value_t));
    self->type = INT;
    self->int_value = a;
    return self;
}

value_t* 
new_vec(value_t* a, value_t* b) {
    value_t* self = malloc(sizeof(value_t));
    self->type = VEC;

    if (a != NULL) {
        self->vec_value[0] = a;
    }

    if (b != NULL) {
        self->vec_value[1] = b;
    }

    return self;
}


void
free_value(value_t* self) {
    if (self->type == VEC) {
        for (int i = 0; i < 2; i++) {
            if (self->vec_value[i] != NULL) {
                free_value(self->vec_value[i]);
                self->vec_value[i] = NULL;
            }
        }
    }

    free(self);
}