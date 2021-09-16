#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "../common/file_util.h"
#include "../common/buffer.h"
#include "../common/instruction_codes.h"
#include "../common/memory.h"

#define SIGNATURE_SIZE 6 // C, L, A, S, P, <version byte>
// Currently the only version byte is 0x00, but there should be more eventually

typedef struct {
  Buffer buffer;
  memloc_t program_counter;
} Program;

int verify_file(Buffer buffer) {
  if(buffer.size < SIGNATURE_SIZE) {
    return 0;
  }

  return buffer.buffer[0] == 'C' &&
    buffer.buffer[1] == 'L' &&
    buffer.buffer[2] == 'A' &&
    buffer.buffer[3] == 'S' &&
    buffer.buffer[4] == 'P' &&
    buffer.buffer[5] == 0x00; // In the future the 6th byte will be version
}

int run_instruction(Program* program, Buffer memory) {
  program->program_counter++;
  // TODO: Implement instuctions

  return 0;
}

int run_sequence(Buffer program_buffer, Buffer memory) {
  if(!verify_file(program_buffer)) {
    return 300;
  }

  Program program;
  program.buffer = program_buffer;
  program.program_counter = SIGNATURE_SIZE;

  while(program.program_counter < program.buffer.size) {
    int inst_status = run_instruction(&program, memory);

    // TODO: If inst_status is not zero, set the "err" register to the value
  }

  return 0;
}

void show_usage() {
  printf("Command Usage:\n\nclasp [compile_program_path]\n\nex:\nclasp /path/to/file.cclasp\n");
}

int main(int argc, char* argv[]) {
  if(argc <= 1) {
    show_usage();
    return 1;
  }
  printf("Starting the runner from file %s\n", argv[1]);

  Buffer memory = create_buffer(32000);
  Buffer file_buffer = read_file_bytes(argv[1]);

  printf("Program buffer:\n");
  print_buffer(file_buffer);

  int status = run_sequence(file_buffer, memory);

  printf("completed running\n");

  free_buffer(file_buffer);
  free_buffer(memory);

  return status;
}
