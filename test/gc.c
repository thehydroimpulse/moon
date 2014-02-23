#include <assert.h>
#include <test/setup.h>
#include <src/vm.h>

static void test_vm_new() {
    vm_t* vm = new_vm();
    assert(vm != NULL);
    free_vm(vm);
}

static void test_val_int_new() {
    value_t* i = new_int(5);
    assert(i->type == INT);
    assert(i->int_value == 5);
    free_value(i);
}

static void test_val_vec_new() {
    value_t* a   = new_int(1);
    value_t* b   = new_int(2);
    value_t* vec = new_vec(a, b);
    assert(vec->type == VEC);
    assert(vec->vec_value[0] == a);
    assert(vec->vec_value[1] == b);

    free_value(vec);
}

static void test_stack_push() {
    value_t* a = new_int(5);
    vm_t* vm = new_vm();
    push_stack(vm, a);
    assert(vm->stack_size == 1);

    free_vm(vm);
}

static void test_stack_pop() {
    value_t* a = new_int(10);
    vm_t* vm = new_vm();
    push_stack(vm, a);

    assert(pop_stack(vm) == a);

    free_vm(vm);
}

static void test_gc_new() {

}

static void test_gc_mark() {

}

static void test_gc_sweep() {

}

int main() {
    suite("vm");
    test(vm_new);

    suite("rvalues");
    test(val_int_new);
    test(val_vec_new);

    suite("stack");
    test(stack_push);
    test(stack_pop);

    suite("gc");
    test(gc_new);
    test(gc_mark);
    test(gc_sweep);
}