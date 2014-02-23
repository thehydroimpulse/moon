#include <stdlib.h>
#include "value.h"



value_t*
int_new(int a) {
    value_t* self = malloc(sizeof(value_t));
    self->type = INT;
    self->int_value = a;
    self->marked = 0;
    return self;
}

value_t* 
vec_new(value_t* a, value_t* b) {
    value_t* self = malloc(sizeof(value_t));
    self->type = VEC;
    self->marked = 0;

    if (a != NULL) {
        self->vec_value[0] = a;
    }

    if (b != NULL) {
        self->vec_value[1] = b;
    }

    return self;
}


void
value_free(value_t* self) {
    if (self->type == VEC) {
        for (int i = 0; i < 2; i++) {
            if (self->vec_value[i] != NULL) {
                value_free(self->vec_value[i]);
                self->vec_value[i] = NULL;
            }
        }
    }

    free(self);
}