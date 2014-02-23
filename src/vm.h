#ifndef SRC_VM_H
#define SRC_VM_H

#include <stdint.h>

#define int32_t s32

// We want a limited stack size.
#define MAX_STACK_SIZE 100

// The VM only supports the following types.
enum type_t {
    // A simple, 32bit (signed) sized integer.
    INT,
    // This will be somewhat of a vector/array. This will allow
    // having references to other types and even other vectors.
    VEC
};

// A value that can be of any type. We'll use C's unions to allow
// us to represent multiple different types.
struct value {
    // The current type that this value represents.
    enum type_t type;

    union {
        int int_value;
        struct value* vec_value[2]; // Limit the size to two.
    };
};

struct vm {
    struct value* stack[MAX_STACK_SIZE];
    int stack_size;
};


typedef struct vm vm_t;
typedef struct value value_t;
typedef struct vec vec_t;

// VM
vm_t* new_vm();
void free_vm(vm_t*);

void push_stack(vm_t*, value_t*);
value_t* pop_stack(vm_t*);

// Value (int)
value_t* new_int(int);

// Value (vec)
value_t* new_vec(value_t*, value_t*);
void free_value(value_t*);

#endif