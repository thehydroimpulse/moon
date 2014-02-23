#include <assert.h>
#include <test/setup.h>

#include <src/vm.h>
#include <src/gc.h>
#include <src/value.h>

static void test_vm_new() {
    vm_t* vm = vm_new();
    assert(vm != NULL);
    vm_free(vm);
}

static void test_val_int_new() {
    value_t* i = int_new(5);
    assert(i->type == INT);
    assert(i->int_value == 5);
    value_free(i);
}

static void test_val_vec_new() {
    value_t* a   = int_new(1);
    value_t* b   = int_new(2);
    value_t* vec = vec_new(a, b);
    assert(vec->type == VEC);
    assert(vec->vec_value[0] == a);
    assert(vec->vec_value[1] == b);

    value_free(vec);
}

static void test_stack_push() {
    value_t* a = int_new(5);
    vm_t* vm = vm_new();
    push_stack(vm, a);
    assert(vm->stack_size == 1);

    vm_free(vm);
}

static void test_stack_pop() {
    value_t* a = int_new(10);
    vm_t* vm = vm_new();
    push_stack(vm, a);

    assert(pop_stack(vm) == a);

    vm_free(vm);
}

static void test_new_gc() {
    gc_t* gc = gc_new();
    assert(gc->roots_size == 0);
    gc_free(gc);
}

static void test_new_gc_node() {
    value_t* i = int_new(55);
    gc_node_t* node = gc_node_new(i);
    assert(node->value == i);
    assert(node->next == NULL);
    gc_node_free(node);
}

static void test_new_gc_list() {
    value_t* a = int_new(55);
    value_t* b = int_new(10);
    value_t* c = int_new(33);

    gc_list_t* list = gc_list_new();
    
    gc_list_append(list,a);
    gc_list_append(list,b);
    gc_list_append(list,c);

    assert(list->size == 3);
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
    test(new_gc);

    suite("gc node");
    test(new_gc_node);

    suite("gc list");
    test(new_gc_list);
}