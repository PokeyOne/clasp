/* buffer.c -- Buffer type and utility functions */

/* Copyright (c) 2021 Mateo Carreras

   This file is part of The CLASP Programming Language.

   CLASP is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, in version 3 of the License.

   CLASP is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY of FITNESS FOR A PARTICULAR PURPOSE. See the
   GNU General Public License for more details.

   You should have recieved a copy of the GNU General Public License
   along with Bash. If not, see <http://www.gnu.org/licenses/>
*/

#include "buffer.h"

Buffer create_buffer(long size) {
  Buffer buffer;
  buffer.size = size;
  buffer.buffer = (byte*) malloc(sizeof(byte) * size);

  return buffer;
}

Buffer create_manual_buffer(long size, byte* data) {
  Buffer buffer;
  buffer.size = size;
  buffer.buffer = data;

  return buffer;
}

void free_buffer(Buffer buffer) {
  free(buffer.buffer);
}

void print_buffer(Buffer buffer) {
  for(int i = 0; i < buffer.size; i++) {
    printf("%02X ", buffer.buffer[i]);
    if((i+1) % 8 == 0) {
      printf("| ");
      for(int j = i - 7; j <= i; j++) {
        if(isalnum(buffer.buffer[j])) {
          printf("%c", buffer.buffer[j]);
        } else {
          printf("_");
        }
        if((j+1) % 4 == 0) {
          printf(" ");
        }
      }
      printf("\n");
    } else if((i+1) % 4 == 0) {
      printf(" ");
    }
  }

  printf("\n");
}

uint32_t read_uint32(Buffer buffer, long index) {
  uint32_t value = 0;

  value += (uint32_t)(buffer.buffer[index+3]) << 0;
  value += (uint32_t)(buffer.buffer[index+2]) << 8;
  value += (uint32_t)(buffer.buffer[index+1]) << 16;
  value += (uint32_t)(buffer.buffer[index+0]) << 24;

  return value;
}

void write_uint32(Buffer memory, memloc_t index, uint32_t value) {
  memory.buffer[index + 0] = (value >>  0) & 0xFF;
  memory.buffer[index + 1] = (value >>  8) & 0xFF;
  memory.buffer[index + 2] = (value >> 16) & 0xFF;
  memory.buffer[index + 3] = (value >> 24) & 0xFF;
}
