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

struct vec {

};

// A value that can be of any type. We'll use C's unions to allow
// us to represent multiple different types.
struct value {
    // The current type that this value represents.
    enum type_t type;

    union {
        int int_type;
    };
};

struct vm {

};


typedef struct vm vm_t;
typedef struct value value_t;
typedef struct vec vec_t;

vm_t* new_vm();
void free_vm(vm_t*);

#endif