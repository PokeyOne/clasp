#ifndef MAP_TYPE_H
#define MAP_TYPE_H

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STRING_KEY_TYPE 0
#define INTEGER_KEY_TYPE 1
#define MAP_SIZE 300

typedef struct {
  int type; // STRING_KEY_TYPE || INTEGER_KEY_TYPE
  char* raw_string_value;
  int raw_int_value;
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
  int count; // Number of total items inserted
  int capacity; // Number of hash slots
  int key_type;
} Map;

MapKey* create_map_key_s(char* value);
MapKey* create_map_key_i(int value);

Map* create_map();
/**
  Put a key-value pair into the map. Returns 0 if successful, 1 if failed.
 */
int map_put(Map* map, MapKey key, MapValue value);
MapValue map_get(Map* map, MapKey key);
void destroy_map(Map* map);

#endif
