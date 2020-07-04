#include <stdlib.h>

#include "memory.h"

void* reallocate(void* previous, size_t oldSize, size_t newSize) {
  if (newSize == 0) {
    free(previous);
    return NULL;
  }

  void* result = realloc(previous, newSize);
  if (result == NULL) exit(1);
  return result;
}
