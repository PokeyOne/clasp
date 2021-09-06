#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "../common/file_util.h"
#include "../common/buffer.h"
#include "../common/instruction_codes.h"

#define SIGNATURE_SIZE 6 // C, L, A, S, P, <version byte>
// Currently the only version byte is 0x00, but there should be more eventually

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

int run_instruction(Buffer program, long* index, Buffer memory) {
  char instruction = program.buffer[*index];

  switch(instruction) {
  case NOP_INSTR:
    (*index)++;
    return 0;
  case MOVE_INSTR:
    // Create a new scope because variable declaration and label/scope stuff
    {
      // 0 = raw value, 1 = pointer;
      char type = program.buffer[*index + 1];
      uint32_t from = read_uint32(program, *index + 2);
      uint32_t dest = read_uint32(program, *index + 6);
      printf("Completing move instruction with type %d, from %d to %d\n", type, from, dest);

      if(type == 0) {
        write_uint32(memory, dest, from);
      } else if(type == 1) {
        uint32_t value = read_uint32(memory, from);
        write_uint32(memory, dest, value);
      } else {
        fprintf(stderr, "Invalid move instruction type, %d, expected 0, 1.\n", type);
        return 1;
      }
    }

    (*index) += 10; // Instr + Type + 4 from + 4 to
    return 0;
  default:
    fprintf(stderr, "Invalid instruction, 0x%2X. Refer to CLASM manual for valid instruction codes\n", instruction);
    return 1;
  }
}

int run_sequence(Buffer buffer, Buffer memory) {
  if(!verify_file(buffer)) {
    return 300;
  }

  long index = SIGNATURE_SIZE;

  while(index < buffer.size) {
    // TODO change run_instruction to use buffers
    int inst_status = run_instruction(buffer, &index, memory);
    if(inst_status != 0) {
      return inst_status;
    }
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

  // TODO: make the memory size variable
  // TODO: 24 is just for testing purposes
  Buffer memory = create_buffer(24);

  /*
  long file_len;
  // TODO: change this to the updated read_file_bytes definition:
  //       Buffer read_file_bytes(char* path);
  char* file_buffer_temp = read_file_bytes(argv[1], &file_len);
  // File not found returns 404;
  if(file_buffer_temp == NULL) {
    return 404;
  }
  Buffer file_buffer = create_manual_buffer(file_len, file_buffer_temp);
  */
  // TODO: Remove temporary testing bypass
  Buffer file_buffer = create_buffer(16);
  // Src:
  // MOV 1 &0
  int j = 0;
  file_buffer.buffer[j++] = 'C';
  file_buffer.buffer[j++] = 'L';
  file_buffer.buffer[j++] = 'A';
  file_buffer.buffer[j++] = 'S';
  file_buffer.buffer[j++] = 'P';
  file_buffer.buffer[j++] = 0x00;

  file_buffer.buffer[j++] = MOVE_INSTR;
  file_buffer.buffer[j++] = 0x00; // Raw move type
  file_buffer.buffer[j++] = 0x00; // Raw one value
  file_buffer.buffer[j++] = 0x00; // |
  file_buffer.buffer[j++] = 0x00; // |
  file_buffer.buffer[j++] = 0x01; // |
  file_buffer.buffer[j++] = 0x00; // Address pointing to zero
  file_buffer.buffer[j++] = 0x00; // |
  file_buffer.buffer[j++] = 0x00; // |
  file_buffer.buffer[j++] = 0x00; // |
  printf("Initialized the file buffer to %ld bytes\nRunning now...\n", file_buffer.size);

  printf("Initial buffer:\n");
  print_buffer(memory);
  printf("Program buffer:\n");
  print_buffer(file_buffer);
  int status = run_sequence(file_buffer, memory);

  printf("Buffer after running:\n");
  print_buffer(memory); // TODO: Don't print this when we get to large number

  printf("completed running\n");

  free_buffer(file_buffer);
  free_buffer(memory);
  return status;
}
