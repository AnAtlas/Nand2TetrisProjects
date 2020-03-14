// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

//R0 is the mode either drawing white(0) or black(0x7FFF)
//R1 is the draw pointer

@R0
M=0

(RESETDRAW)
	@R1
	M=0
(LOOP)
	@KBD
	D=M
	@R0
	D=D-M
	@SWITCHBLACK
	D;JGT
	@KBD
	D=M
	@R0
	D=D-M
	@DRAWWHITE
	D;JEQ
	@KBD
	D=M
	@DRAWBLACK
	D;JGT
(SWITCHWHITE)
	@R0
	M=0
	@RESETDRAW
	0; JMP
(DRAWWHITE)
	@SCREEN
	D=A
	@R1
	D=D+M
	A=D
	M=0
	@END
	0; JMP
(SWITCHBLACK)
	@32767
	D=A
	@R0
	M=D
	@RESETDRAW
	0; JMP
(DRAWBLACK)
	@SCREEN
	D=A
	@R1
	D=D+M
	A=D
	M=-1
(END)
	@R1
	M=M+1
	D=M
	@8192
	D=D-A
	@LOOP
	D; JLT
	@R1
	M=0
	@LOOP
	0; JMP