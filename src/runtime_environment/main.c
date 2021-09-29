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

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "../common/file_util.h"
#include "../common/buffer.h"
#include "../common/instruction_codes.h"
#include "../common/memory.h"

#define SIGNATURE_SIZE 6 // C, L, A, S, P, <version byte>
#define VERSION_ALPHA_ONE 0x00
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

int is_loc_register(memloc_t location) {
  return location >= GZ_LOC && location <= GA_LOC;
}

void set_register(memloc_t location, uint32_t value) {
  switch(location) {
  case GA_LOC:
    ga_r = value;
    break;
  case GB_LOC:
    gb_r = value;
    break;
  case GC_LOC:
    gc_r = value;
    break;
  case GD_LOC:
    gd_r = value;
    break;
  case GE_LOC:
    ge_r = value;
    break;
  case GF_LOC:
    gf_r = value;
    break;
  case GG_LOC:
    gg_r = value;
    break;
  case GH_LOC:
    gh_r = value;
    break;
  case GI_LOC:
    gi_r = value;
    break;
  case GJ_LOC:
    gj_r = value;
    break;
  case GK_LOC:
    gk_r = value;
    break;
  case GL_LOC:
    gl_r = value;
    break;
  case GM_LOC:
    gm_r = value;
    break;
  case GN_LOC:
    gn_r = value;
    break;
  case GO_LOC:
    go_r = value;
    break;
  case GP_LOC:
    gp_r = value;
    break;
  case GQ_LOC:
    gq_r = value;
    break;
  case GR_LOC:
    gr_r = value;
    break;
  case GS_LOC:
    gs_r = value;
    break;
  case GT_LOC:
    gt_r = value;
    break;
  case GU_LOC:
    gu_r = value;
    break;
  case GV_LOC:
    gv_r = value;
    break;
  case GW_LOC:
    gw_r = value;
    break;
  case GX_LOC:
    gx_r = value;
    break;
  case GY_LOC:
    gy_r = value;
    break;
  case GZ_LOC:
    gz_r = value;
    break;
  }
}

int run_instruction(Program* program, Buffer memory) {
  byte inst_code = program->buffer.buffer[program->program_counter];

  switch(inst_code) {
  case NOP_INST:
    program->program_counter++;
    break;
  case MOV_INST:
    program->program_counter++;
    uint32_t location = read_uint32(program->buffer, program->program_counter);
    uint32_t destination = read_uint32(program->buffer, program->program_counter + 4);
    program->program_counter += 8;

    // TODO: going to need to handle locations that are special io registers
    //       but that might be something for a memory controller.

    // TODO: make move size settable on the mvs register

    // TODO: finish this implementation
    return 1;
  case SET_INST:
    program->program_counter++;
    uint32_t cv = read_uint32(program->buffer, program->program_counter);
    uint32_t destination = read_uint32(program->buffer, program->program_counter + 4);
    program->program_counter += 8;

    if(destination == OUT_LOC) {
      printf("%c", (char)cv);
      break;
    }

    if(is_loc_register((memloc_t)destination)) {
      set_register(destination, cv);
      break;
    }

    write_uint32(memory, (memloc_t) destination, cv);
    break;
  default:
    program->program_counter++;
    return 1;
  }

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
