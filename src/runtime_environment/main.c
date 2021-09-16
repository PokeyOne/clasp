#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "../common/file_util.h"
#include "../common/buffer.h"
#include "../common/instruction_codes.h"
#include "../common/memory.h"

#define SIGNATURE_SIZE 6 // C, L, A, S, P, <version byte>
// Currently the only version byte is 0x00, but there should be more eventually

#define REGISTERS_MEMLOC_START 0xffffff98
#define GA_LOC 0xFFFFFFFC
#define GB_LOC 0xFFFFFFF8
#define GC_LOC 0xFFFFFFF4
#define GD_LOC 0xFFFFFFF0
#define GE_LOC 0xFFFFFFFC
#define GF_LOC 0xFFFFFFF8
#define GG_LOC 0xFFFFFFF4
#define GH_LOC 0xFFFFFFF0
#define GI_LOC 0xFFFFFFFC
#define GJ_LOC 0xFFFFFFF8
#define GK_LOC 0xFFFFFFF4
#define GL_LOC 0xFFFFFFF0
#define GM_LOC 0xFFFFFFFC
#define GN_LOC 0xFFFFFFF8
#define GO_LOC 0xFFFFFFF4
#define GP_LOC 0xFFFFFFF0
#define GQ_LOC 0xFFFFFFFC
#define GR_LOC 0xFFFFFFF8
#define GS_LOC 0xFFFFFFF4
#define GT_LOC 0xFFFFFFF0
#define GU_LOC 0xFFFFFFFC
#define GV_LOC 0xFFFFFFF8
#define GW_LOC 0xFFFFFFF4
#define GX_LOC 0xFFFFFFF0
#define GY_LOC 0xFFFFFFFC
#define GZ_LOC 0xFFFFFFF8
#define OUT_LOC 0xFFFFFFF4

uint32_t ga_r = 0;
uint32_t gb_r = 0;
uint32_t gc_r = 0;
uint32_t gd_r = 0;
uint32_t ge_r = 0;
uint32_t gf_r = 0;
uint32_t gg_r = 0;
uint32_t gh_r = 0;
uint32_t gi_r = 0;
uint32_t gj_r = 0;
uint32_t gk_r = 0;
uint32_t gl_r = 0;
uint32_t gm_r = 0;
uint32_t gn_r = 0;
uint32_t go_r = 0;
uint32_t gp_r = 0;
uint32_t gq_r = 0;
uint32_t gr_r = 0;
uint32_t gs_r = 0;
uint32_t gt_r = 0;
uint32_t gu_r = 0;
uint32_t gv_r = 0;
uint32_t gw_r = 0;
uint32_t gx_r = 0;
uint32_t gy_r = 0;
uint32_t gz_r = 0;

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
