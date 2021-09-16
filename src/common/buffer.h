#ifndef BUFFER_H
#define BUFFER_H

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <ctype.h>
#include "memory.h"

typedef uint8_t byte;

typedef struct {
  byte* buffer;
  long size;
} Buffer;

Buffer create_buffer(long size);
Buffer create_manual_buffer(long size, byte* data);
void free_buffer(Buffer buffer);
void print_buffer(Buffer buffer);

uint32_t read_uint32(Buffer buffer, long index);
void write_uint32(Buffer memory, memloc_t index, uint32_t value);

#endif
