#include <register_locations.h>

typedef uint64_t* register_list_t;

register_list_t rs = (register_list_t) malloc(sizeof(uint64_t) * 64);
memloc_t rs_size = 64;

memloc_t map_location_to_index(memloc_t reg_mem_loc) {
  return (MEMORY_MAX - reg_mem_loc) / 8;
}

void set_register(memloc_t location, uint32_t value) {
  memloc_t index = map_location_to_index(location);

  while(index > rs_size) {
    rs = (register_list_t) realloc(rs, sizeof(uint64_t) * (rs_size + 64));
    rs_size += 64;
  }
}

uint64_t get_register(memloc_t location) {
  return 0; // TODO
}



