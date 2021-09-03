#ifndef MAP_TYPE_H
#define MAP_TYPE_H

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STRING_KEY_TYPE 0
#define OBJECT_KEY_TYPE 1
#define INTEGER_KEY_TYPE 2
#define INITIAL_MAP_SIZE 300

typedef struct {
  // 0 = string, 1 = pointer, 2 = integer;
  int type;
  char* raw_string_value;
  int raw_int_value;
  void* raw_pointer_value;
  int hash;
} MapKey;

typedef struct {
  void* pointer;
} MapValue;

typedef struct {
  MapKey key;
  MapValue value;
} MapElement;

typedef struct {
  MapElement* elements;
  int capacity;
  int count;
} MapHashSlot;

typedef struct {
  MapHashSlot* slots;
  int count;
  int capacity;
  int key_type;
} Map;

MapKey* create_map_key_s(char* value);
MapKey* create_map_key_i(int value);
MapKey* create_map_key_o(void* value);

Map* create_map();
void map_put(Map* map, MapKey key, MapValue value);
MapValue map_get(Map* map, MapKey key);
void destroy_map(Map* map);

#endif
