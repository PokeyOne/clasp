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

#ifndef REGISTER_LOCATIONS_H
#define REGISTER_LOCATIONS_H

#include "../common/memory.h"

#define MEMORY_MAX 0xFFFFFFFFFFFFFFFF
#define GA_LOC (MEMORY_MAX - 7)
#define GB_LOC (GA_LOC - 8)
#define GC_LOC (GB_LOC - 8)
#define GD_LOC (GC_LOC - 8)
#define GE_LOC (GD_LOC - 8)
#define GF_LOC (GE_LOC - 8)
#define GG_LOC (GF_LOC - 8)
#define GH_LOC (GG_LOC - 8)
#define GI_LOC (GH_LOC - 8)
#define GJ_LOC (GI_LOC - 8)
#define GK_LOC (GJ_LOC - 8)
#define GL_LOC (GK_LOC - 8)
#define GM_LOC (GL_LOC - 8)
#define GN_LOC (GM_LOC - 8)
#define GO_LOC (GN_LOC - 8)
#define GP_LOC (GO_LOC - 8)
#define GQ_LOC (GP_LOC - 8)
#define GR_LOC (GQ_LOC - 8)
#define GS_LOC (GR_LOC - 8)
#define GT_LOC (GS_LOC - 8)
#define GU_LOC (GT_LOC - 8)
#define GV_LOC (GU_LOC - 8)
#define GW_LOC (GV_LOC - 8)
#define GX_LOC (GW_LOC - 8)
#define GY_LOC (GX_LOC - 8)
#define GZ_LOC (GY_LOC - 8)
#define OUT_LOC (GZ_LOC - 8)

void set_register(memloc_t location, uint32_t value);
uint64_t get_register(memloc_t location);

#endif
