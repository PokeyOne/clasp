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
