#ifndef COMPILER_FLAGS_H
#define COMPILER_FLAGS_H

#include <stdbool.h>

typedef struct {
  bool help;
  bool is_asm;
  bool error;
} CompilerFlags;

#endif
