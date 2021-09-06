#include "file_util.h"

Buffer read_file_bytes(char* path) {
  // Open file and check that it openned
  FILE* file = fopen(path, "rb");
  if(file == NULL) {
    Buffer err_buff;
    err_buff.buffer = NULL;
    err_buff.size = 0;
    return err_buff;
  }

  // Get the file length
  long file_length = 0;
  fseek(file, 0, SEEK_END);
  file_length = ftell(file);
  rewind(file);

  // Read actual file into the buffer
  Buffer buffer = create_buffer(file_length);
  fread(buffer.buffer, file_length, 1, file);
  fclose(file);

  // Return the buffer
  return buffer;
}
