#include "fill_screen_mode.h"

void enter_fill_screen() {
  printf("\e[?1049h");
}
void exit_fill_screen() {
  printf("\e[?1049l");
}

void fill_buffer(ScreenBuffer buffer) {

}

ScreenBuffer create_screen_buffer(int width, int height) {
  ScreenBuffer result;
  return result;
}

void put_screen_buffer(ScreenBuffer sb, char value, int x, int y) {

}

void write_screen_buffer(ScreenBuffer sb, char* value, int x, int y) {

}

