#ifndef INSTRUCTION_CODES_H
#define INSTRUCTION_CODES_H

#define NOP_INSTR 0x00
#define MOVE_INSTR 0x01

// Not-implemented yet
#define ADD_INSTR 0x02
#define SUB_INSTR 0x03
#define MUL_INSTR 0x04
#define DIV_INSTR 0x05
#define POW_INSTR 0x06

// put something on screen
// 2 modes: stream or screen
// in stream (1 argument byte)
//   PUT <char>
// in screen (9 argument bytes)
//   PUT <char> <x> <y>
#define PUT_INSTR 0x07
// set the put mode
//   PMD <0|1>
#define PMD_INSTR 0x08

#endif
