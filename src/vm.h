#ifndef SRC_VM_H
#define SRC_VM_H

#include <stdint.h>

// We want a limited stack size.
#define MAX_STACK_SIZE 100

#include "gc.h"
#include "value.h"

typedef struct {
    // The vm stack contains all the local variables. These are
    // akin to roots in the GC.
    struct value* stack[MAX_STACK_SIZE];

    // Keep track of how many objects are within the stack.
    int stack_size;

    struct gc* gc;
} vm_t;

// VM
vm_t* vm_new();
void vm_free(vm_t*);

void push_stack(vm_t*, value_t*);
value_t* pop_stack(vm_t*);


#endif