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
;; Couple of no-op instructions that do nothing
nop
nop

;; copy the value stored at 0x03E8 to 0x03F0
mov  0x03E8  0x03F0
;; put the raw value 255 into the location 0x03F8
mov (0x00FF) 0x03F8

;; The following lines output single ascii characters to the standard out.
;; These are the characters for a hello world message. In future there will be the
;; ability to just write 'H' and have it converted to ascii automatically.
outr (0x48)
outr (0x65)
outr (0x6C)
outr (0x6C)
outr (0x6F)
outr (0x20)
outr (0x77)
outr (0x6F)
outr (0x72)
outr (0x6C)
outr (0x64)
;; These two lines show that you can output a memory address as well
mov (0x0A) 0x0400
outr 0x0400

;; The following are just some examples of adding and subtracting. There is
;; also multiplying, dividing, and powers that work in a similar fashion.
mov (0x01) 0x03F0
add (0x01) (0x02) 0x03F8
add 0x03F0 0x03F8 0x03F0
mov (0x05) 0x0400
sub (0x05) (0x04) 0x0408
sub 0x0400 0x0408 0x0400

;; The end instruction is required to indicate exit from the program.
end
```

After the above code runs, the memory would look like the following:
```text
!!!!THIS IS WRONG AND NEEDS TO BE UPDATED!!!!!
00 00 00 04  00 00 00 07 | ____ ____
00 00 00 0B  00 01 4C C0 | ____ __L_
68 65 6C 6C  6F 00 00 00 | HELL O___
...
00 00 00 00  00 00 00 00 | ____ ____
```

A **couple things to note** about this syntax is that a raw number with nothing
around it is treated like and address, while a number surrounded by round
brackets is treated like a raw constant value.

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

```
// TODO: The section below needs to be written.
```

NOP, MOV, ADD, SUB, MUL, DIV, POW, PUT, JMP
