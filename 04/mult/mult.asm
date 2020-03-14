// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

@2
M=0
//For efficiency, make sure we loop n times, where n is the smaller of the two numbers
@0
D=M
@1
D=D-M
@START
D;JGT
@0
D=M
@3
M=D
@1
D=M
@0
M=D
@3
D=M
@1
M=D

//R3 will be our running total
(START)
@3
M=0
(LOOP)
    @1
    D=M
    @END
    D;JEQ
    @0
    D=M
    @3
    D=D+M
    M=D
    @1
    M=M-1
    @LOOP
    0;JMP
(END)
    @3
    D=M
    @2
    M=D