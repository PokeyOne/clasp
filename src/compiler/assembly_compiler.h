#ifndef ASSEMBLY_COMPILER_H
#define ASSEMBLY_COMPILER_H

#include <ctype.h>

#include "../common/file_util.h"
#include "../common/buffer.h"
#include "../common/tracked_string.h"
#include "../common/instruction_codes.h"

#include "compiler_flags.h"

int compile_asm(int argc, char* argv[], CompilerFlags flags);

#endif
