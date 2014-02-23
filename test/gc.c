#include <assert.h>
#include <test/setup.h>

#include <src/vm.h>
#include <src/gc.h>

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
    gc_t* gc = new_gc();
    assert(gc->liveset != NULL);
    assert(gc->liveset->size == 0);

    free_gc(gc);
}

static void test_new_gc_liveset() {
    gc_liveset_t* set = new_gc_liveset();
    assert(set->size == 0);
    assert(set->first == NULL);
    free_gc_liveset(set);
}

static void test_liveset_gc_iterator() {
    value_t* a = new_int(11);
    gc_object_t* obj = new_gc_object(a);
    gc_liveset_t* set = new_gc_liveset();
    set->first = obj;
    set->size  = 1;

    // Create a new gc_liveset iterator
    gc_liveset_iter_t* iter = new_gc_liveset_iterator(set);
    assert(next_gc_liveset_iterator(iter) == obj);
}

static void test_new_gc_object() {
    value_t* a = new_int(77);
    gc_object_t* obj = new_gc_object(a);
    assert(obj->value == a);
    assert(obj->next == NULL);
    assert(obj->marked == 0);

    free_gc_object(obj);
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
    test(new_gc_object);
    test(new_gc_liveset);

    suite("gc liveset");
    test(liveset_gc_iterator);
}