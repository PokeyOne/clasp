#ifndef TRACKED_STRING_H
#define TRACKED_STRING_H

#include "buffer.h"

typedef struct {
  char* value;
  long length;
  long index;
} TrackedString;

TrackedString create_tracked_string(char* value, long length);

#endif
