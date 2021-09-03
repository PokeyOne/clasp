#include "map_type.h"

// Private definitions
int map_hash(MapKey key);

Map* create_map(int key_type) {
  Map* result = (Map*) malloc(sizeof(Map));
  result->slots = (MapValue*) malloc(sizeof(MapKey) * INITIAL_MAP_SIZE);
  result->count = 0;
  result->capacity = INITIAL_MAP_SIZE;
  result->key_type = key_type;

  for(int i = 0; i < INITIAL_MAP_SIZE; i++) {
    MapHashSlot slot;
    slot.capacity = 100;
    slot.count = 0;
    slot.elements = (MapElement*) malloc(sizeof(MapElement) * 100);
    result->slots[i] = slot;
  }

  return result;
}

MapKey* create_map_key_s(char* value) {
  MapKey* result = (MapKey*) malloc(sizeof(MapKey));

  result->type = STRING_KEY_TYPE;
  result->raw_string_value = value;
  result->raw_int_value = 0;
  result->raw_pointer_value = NULL;
  result->hash = map_hash(*result, INITIAL_MAP_CAPACITY);

  return result;
}

MapKey* create_map_key_i(int value) {
  MapKey* result = (MapKey*) malloc(sizeof(MapKey));

  result->type = INTEGER_KEY_TYPE;
  result->raw_string_value = NULL;
  result->raw_int_value = value;
  result->raw_pointer_value = NULL;
  result->hash = map_hash(*result, INITIAL_MAP_CAPACITY);

  return result;
}

/**
  Compare two keys. They are expected to be the same type; if they aren't the
  same type, the return value is 0.

  if a<b then return negative number.
  if a=b then return 0.
  if a>b then return positive number.
 */
int compare_keys(MapKey a, MapKey b) {
  if(a.type != b.type) {
    fprintf(stderr, "[Map][compare_keys]The keys provided are not the same type\n");
    return 0;
  }

  switch(a.type) {
  case STRING_KEY_TYPE:
    return strcmp(a.raw_string_value, b.raw_string_value);
  case INTEGER_KEY_TYPE:
    return a - b;
  default:
    fprintf(stderr, "[Map][compare_keys]The keys provided to compare_keys are not comparable\n");
    return 0;
  }
}

int compare_map_elements(MapElement a, MapElement b) {
  return compare_keys(a.key, b.key);
}

void extend_map_slot_capacity(MapHashSlot* slot, int amount) {
  // Reallocate the memory for the elements to be longer than it was
  slot->elements = (MapElement*)
    realloc(slot->elements, sizeof(MapElement) * (slot->capacity + 20));
  slot->capacity += amount;
}

// Given a map hash slot, insert the given element into the slot in order.
void map_put_in_slot(MapHashSlot* slot, MapElement element) {
  int i = 0;

  // Find the location to insert the element
  while(i < slot->count && compare_map_elements(slot->elements[i], element) < 0) {
    i++;
  }

  // Extend the slot if needed before inserting.
  if(slot->count >= slot->capacity) {
    extend_map_slot_capacity(slot, 20);
  }

  // TODO: This special case may not be needed
  // Special Case: goes at the end
  if(i == slot->count) {
    // Set the last element to the given element and bump the count
    slot->elements[i] = element;
    slot->count++;
    return;
  }

  // Temporary storage for moving things around
  MapElement temp[] = {element, NULL};

  while(i < slot->count) {
    temp[1] = slot->elements[i];
    slot->elements[i] = temp[0];

    temp[0] = temp[1];
  }

  slot->elements[i] = temp[0];
  slot->count++;
}

void map_put(Map* map, MapKey key, MapValue value) {
  // TODO
}

MapValue map_get(Map* map, MapKey key) {
  // TODO
}

void destroy_map_hash_slot(MapHashSlot slot) {
  free(slot->elements);
}

void destroy_map(Map* map) {
  for(int i = 0; i < map->count; i++) {
    destory_map_hash_slot(map->slots[i]);
  }
  free(map->slots);
  free(map);
}

// if return -1, then error occured
int map_hash(MapKey key, int map_capacity) {
  if(key.type == STRING_KEY_TYPE) {
    int result = 0;
    int length = 0;
    for(int i = 0; key.raw_string_value[i] != '\0'; i++) {
      length++;
      result += key.raw_string_value[i];
      result %= map_capacity;
    }
    return (result + length) % map_capacity;
  } else if(key.type == INTEGER_KEY_TYPE) {
    return key.raw_int_value % map_capacity;
  } else {
    return -1;
  }
}

