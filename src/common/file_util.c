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
