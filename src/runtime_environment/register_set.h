#ifndef REGISTER_SET_H
#define REGISTER_SET_H

#include <stdint.h>
#include "../common/memory.h"

#define GENERAL_REGISTER_COUNT 26
#define GENERAL_REGISTER_INDEX 0
#define MATRIX_REGISTER_COUNT 9
#define MATRIX_REGISTER_INDEX GENERAL_REGISTER_COUNT
#define COMPARISON_REGISTER_COUNT 2
#define COMPARISON_REGISTER_INDEX (MATRIX_REGISTER_INDEX + MATRIX_REGISTER_COUNT)
#define TOTAL_REGISTER_COUNT (26 + 9 + 2)

typedef struct {
  // ga -> gz
  uint32_t* general_registers;
  // ma(x), may, maz, mb(x), mby, mbz, mc(x), mcy, mcz
  uint32_t* matrix_registers;
  // ca, cb
  uint32_t* comparison_registers;

  // All the registers.
  uint32_t* all_registers
} RegisterSet;

RegisterSet create_register_set();
void destroy_register_set(RegisterSet set);
uint32_t read_register(memloc_t register_location);

#endif
