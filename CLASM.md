# CLASM
CLASM is the assembly language used to create machine code for the CLASP runner.
The clasp runner is essentially a virtual machine that read byte-code
executables. Most commonly said executables are created by compiling clasp
source code, but they can also be compiled from CLASM.

## Basic Syntax

As far as assembly languages go this is a pretty standard and simple one. It
essentially just follows a pattern of each line having a command and then
arguments.

This is what an example file may look like:
```asm
;; This would be a comment
MOV 4 &0
MOV 7 &4
ADD &0 &4 &8
MOV &8 ga
MUL ga 4 ga
POW ga 3 ga
MOV ga &12

;; Move "hello" to location 0x10 or 16
MOV 'h' &0x10 1
MOV 'e' &0x11 1
MOV 'l' &0x12 1
MOV &6 &0x13 1
MOV 'o' &0x14 1
MOV 0 &0x15 1 ;; string terminator

;; print out location 0x04
OUT &4
```

After the above code runs, the memory would look like the following:
```text
00 00 00 04  00 00 00 07 | ____ ____
00 00 00 0B  00 01 4C C0 | ____ __L_
68 65 6C 6C  6F 00 00 00 | HELL O___
...
00 00 00 00  00 00 00 00 | ____ ____
```

## Registers

|----------|--------------------|
| register | description        |
|----------|--------------------|
| ga       | General register A |
| gb       | General register B |
| gc       | General register C |
| gd       | General register D |
| ge       | General register E |
| gf       | General register F |
| gg       | General register G |
| gh       | General register H |
| gi       | General register I |
| gj       | General register J |
| gk       | General register K |
| gl       | General register L |
| gm       | General register M |
| gn       | General register N |
| go       | General register O |
| gp       | General register P |
| gq       | General register Q |
| gr       | General register R |
| gs       | General register S |
| gt       | General register T |
| gu       | General register U |
| gv       | General register V |
| gw       | General register W |
| gx       | General register X |
| gy       | General register Y |
| gz       | General register Z |
|----------|--------------------|
| pf       | print format: 0 = 1-byte ascii, 1 = 4-byte ascii. Starts on 0 |
|----------|--------------------|

## Instructions

Below is a table with all the instructions, but below that is a more detailed
description of each.

NOP, MOV, ADD, SUB, MUL, DIV, POW, PUT, JMP
