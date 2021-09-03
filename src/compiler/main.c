#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

typedef struct {
  bool help;
  bool error;
} CompilerFlags;

CompilerFlags* calculate_flags(int argc, char* argv[]) {
  printf("got %d arguments\n", argc);

  CompilerFlags* result = (CompilerFlags*)malloc(sizeof(CompilerFlags));
  result->help = false;
  result->error = false;

  for(int i = 1; i < argc; i++) {
    printf("Processing argument %d: %s\n", i, argv[i]);
    if(strcmp(argv[i], "--help") == 0) {
      result->help = true;
    } else {
      result->error = true;
      break;
    }
  }

  return result;
}

int main(int argc, char* argv[]) {
  CompilerFlags* flags = calculate_flags(argc, argv);

  printf("Flags: {help: %d, error: %d}\n", flags->help, flags->error);

  return 0;
}
