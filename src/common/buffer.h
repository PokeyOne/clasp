/* buffer.h -- Buffers of bytes and utility functions */

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

uint32_t read_uint32(Buffer buffer, memloc_t index);
void write_uint32(Buffer memory, memloc_t index, uint32_t value);

#endif
