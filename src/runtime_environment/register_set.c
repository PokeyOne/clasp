#include "register_set.h"

RegisterSet create_register_set() {
  RegisterSet value;

  // Allocate the memory for all registers simultaneously
  value.all_registers = (uint32_t*) malloc(sizeof(uint32_t) * TOTAL_REGISTER_COUNT);

  // Set pointers to subsets
  value.general_registers = value.all_registers + GENERAL_REGISTER_INDEX;
  value.matrix_registers = value.all_registers + MATRIX_REGISTER_INDEX;
  value.comparison_registers = value.all_registers + COMPARISON_REGISTER_INDEX;

  return value;
}

void destroy_register_set(RegisterSet set) {
  free(set.all_registers)
}

uint32_t read_register(RegisterSet set, memloc_t register_id) {
  return set.all_registers[register_id];
}
