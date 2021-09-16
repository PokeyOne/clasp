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

#ifndef FILL_SCREEN_MODE_H
#define FILL_SCREEN_MODE_H

#include <stdio.h>

typedef struct {
  char* buffer;
  int width;
  int height;
} ScreenBuffer;

void enter_fill_screen();
void exit_fill_screen();
void fill_buffer(ScreenBuffer buffer);

ScreenBuffer create_screen_buffer(int width, int height);
void put_screen_buffer(ScreenBuffer sb, char value, int x, int y);
void write_screen_buffer(ScreenBuffer sb, char* value, int x, int y);

#endif
