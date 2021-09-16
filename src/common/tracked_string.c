#include "tracked_string.h"

TrackedString create_tracked_string(char* value, long length) {
  TrackedString result;
  result.value = value;
  result.length = length;
  result.index = 0;
  return result;
}
