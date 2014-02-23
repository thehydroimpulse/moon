cmake_minimum_required(VERSION 2.6)

project(gc)
add_definitions(-std=c99)
add_executable(gc src/main.c ${SOURCES})

set (SOURCES src/vm.c src/gc.c src/value.c)

include_directories(.)

enable_testing()
add_executable(testgc test/gc.c ${SOURCES})
add_test(testgc testgc)