#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "vm.h"


vm_t* 
new_vm() {
    vm_t* self = malloc(sizeof(vm_t));
    return self;
}

void 
free_vm(vm_t* self) {
    free(self);
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