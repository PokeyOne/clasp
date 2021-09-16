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

#include "assembly_compiler.h"

void skip_whitespace(TrackedString* src) {
  while(src->index < src->length && isspace(src->value[src->index])) {
    src->index += 1;
  }
}

char get_instruction_code(TrackedString* src) {
  // TODO
}

int process_statement(TrackedString* src) {
  skip_whitespace(src);

  int error;
  char instr = get_instruction_code(src);
}

int compile_asm(int argc, char* argv[], CompilerFlags flags) {
  char* file_path = argv[argc-1];
  Buffer file_contents = read_file_bytes(file_path);
  if(file_contents.buffer == NULL || file_contents.size == 0) {
    fprintf(stderr, "Invalid file given: %s\n", file_path);
    return 404;
  }

  TrackedString src = create_tracked_string_from_buffer(file_contents);

  while(src.index < src.length) {
    int status = process_statement(&src);
    if(status != 0) {
      return status;
    }
  }

  free_buffer(file_contents);

  return 0;
}
