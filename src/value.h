#ifndef SRC_VALUE_H
#define SRC_VALUE_H

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
typedef struct value {
    // The current type that this value represents.
    enum type_t type;
    int marked;

    union {
        int int_value;
        struct value* vec_value[2]; // Limit the size to two.
    };

} value_t;

value_t* int_new(int);
value_t* vec_new(value_t*, value_t*);
void value_free(value_t*);

#endif