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
