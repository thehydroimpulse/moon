#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "vm.h"


vm_t* new_vm() {
    vm_t* self = malloc(sizeof(vm_t));
    return self;
}