#ifndef TEST_SETUP_H
#define TEST_SETUP_H

#include <stdio.h>

/*
 * Test the given `fn`.
 */

#define test(fn) \
  printf("    \e[92m✓ \e[90m%s\e[0m\n", #fn); \
  test_##fn();

/*
 * Test suite title.
 */

#define suite(title) \
  printf("\n  \e[36m%s\e[0m\n", title)

/*
 * Report sizeof.
 */

#define size(type) \
  printf("\n  \e[90m%s: %ld bytes\e[0m\n", #type, sizeof(type));


#endif