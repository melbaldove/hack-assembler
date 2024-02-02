# hack-assembler

An assembler for the Hack assembly language written in Rust.

### Usage
Run with cargo:
```bash
cargo run -- <file_name>.asm
```
See the `samples` directory for some sample .asm code.
### Examples
```bash
cargo run -- samples/Max.asm
```
#### Max.asm
```asm
// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/max/Max.asm

// Computes R2 = max(R0, R1)  (R0,R1,R2 refer to RAM[0],RAM[1],RAM[2])

   // D = R0 - R1
   @R0
   D=M
   @R1
   D=D-M
   // If (D > 0) goto ITSR0
   @ITSR0
   D;JGT
   // Its R1
   @R1
   D=M
   @R2
   M=D
   @END
   0;JMP
(ITSR0)
   @R0
   D=M
   @R2
   M=D
(END)
   @END
   0;JMP
```
#### Max.hack
```
0000000000000000
1111110000010000
0000000000000001
1111010011010000
0000000000001100
1110001100000001
0000000000000001
1111110000010000
0000000000000010
1110001100001000
0000000000010000
1110101010000111
0000000000000000
1111110000010000
0000000000000010
1110001100001000
0000000000010000
1110101010000111
```

### Simulator
A CPU simulator for the Hack Computer is available [here](https://www.nand2tetris.org/software).

### Documentation
See Chapter 6 of [The Elements of Computing Systems: Building a Modern Computer from First Principles](https://www.amazon.com/Elements-Computing-Systems-Building-Principles/dp/0262640686) for the full specification of the Hack Assembly language.
