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

#include "tracked_string.h"

TrackedString create_tracked_string(char* value, long length) {
  TrackedString result;
  result.value = value;
  result.length = length;
  result.index = 0;
  return result;
}
