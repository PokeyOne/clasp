#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include "assembly_compiler.h"
#include "compiler_flags.h"
#include "../common/fill_screen_mode.h"

CompilerFlags calculate_flags(int argc, char* argv[]) {
  printf("got %d arguments\n", argc);

  CompilerFlags result;
  result.help = false;
  result.is_asm = false;
  result.error = false;

  for(int i = 1; i < argc; i++) {
    printf("Processing argument %d: %s\n", i, argv[i]);
    if(strcmp(argv[i], "--help") == 0) {
      result.help = true;
    } else if (strcmp(argv[i], "--asm") == 0) {
      result.is_asm = true;
    } else if (i == argc-1) {
      // do nothing because the last argument should be a file path
      // TODO: Actually store this n the flags object maybe??
    } else {
      result.error = true;
      break;
    }
  }

  return result;
}

void show_help_message() {
  // TODO: Do this
}

int main(int argc, char* argv[]) {
  CompilerFlags flags = calculate_flags(argc, argv);

  printf("Flags: {help: %d, error: %d}\n", flags.help, flags.error);

  if(flags.help) {
    show_help_message();
    return 0;
  }

  if(flags.is_asm) {
    return compile_asm(argc, argv, flags);
  }

  return 0;
}
