# Chip8

Rust implementation of a CHIP-8 VM, using SDL2 crate for display and keyboard events.

## Opcodes

|                    | Opcode |  Type   | Explanation                                                                                                                                                                                                                                                                                                                                                                             |
| :----------------: | :----: | :-----: | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| :white_check_mark: |  0NNN  |  Call   | Calls RCA 1802 program at address NNN. Not necessary for most ROMs.                                                                                                                                                                                                                                                                                                                     |
| :heavy_check_mark: |  00E0  | Display | Clears the screen.                                                                                                                                                                                                                                                                                                                                                                      |
| :heavy_check_mark: |  00EE  |  Flow   | Returns from a subroutine.                                                                                                                                                                                                                                                                                                                                                              |
| :heavy_check_mark: |  1NNN  |  Flow   | Jumps to address NNN.                                                                                                                                                                                                                                                                                                                                                                   |
| :heavy_check_mark: |  2NNN  |  Flow   | Calls subroutine at NNN.                                                                                                                                                                                                                                                                                                                                                                |
| :heavy_check_mark: |  3XNN  |  Cond   | Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)                                                                                                                                                                                                                                                                               |
| :heavy_check_mark: |  4XNN  |  Cond   | Skips the next instruction if VX doesn't equal NN. (Usually the next instruction is a jump to skip a code block)                                                                                                                                                                                                                                                                        |
| :heavy_check_mark: |  5XY0  |  Cond   | Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)                                                                                                                                                                                                                                                                               |
| :heavy_check_mark: |  6XNN  |  Const  | Sets VX to NN.                                                                                                                                                                                                                                                                                                                                                                          |
| :heavy_check_mark: |  7XNN  |  Const  | Adds NN to VX. (Carry flag is not changed)                                                                                                                                                                                                                                                                                                                                              |
| :heavy_check_mark: |  8XY0  | Assign  | Sets VX to the value of VY.                                                                                                                                                                                                                                                                                                                                                             |
| :heavy_check_mark: |  8XY1  |  BitOp  | Sets VX to VX or VY. (Bitwise OR operation)                                                                                                                                                                                                                                                                                                                                             |
| :heavy_check_mark: |  8XY2  |  BitOp  | Sets VX to VX and VY. (Bitwise AND operation)                                                                                                                                                                                                                                                                                                                                           |
| :heavy_check_mark: |  8XY3  |  BitOp  | Sets VX to VX xor VY.                                                                                                                                                                                                                                                                                                                                                                   |
| :heavy_check_mark: |  8XY4  |  Math   | Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.                                                                                                                                                                                                                                                                                                          |
| :heavy_check_mark: |  8XY5  |  Math   | VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.                                                                                                                                                                                                                                                                                                 |
| :heavy_check_mark: |  8XY6  |  BitOp  | Shifts VY right by one and stores the result to VX (VY remains unchanged). VF is set to the value of the least significant bit of VY before the shift.                                                                                                                                                                                                                                  |
| :heavy_check_mark: |  8XY7  |  Math   | Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.                                                                                                                                                                                                                                                                                                   |
| :heavy_check_mark: |  8XYE  |  BitOp  | Shifts VY left by one and copies the result to VX. VF is set to the value of the most significant bit of VY before the shift.                                                                                                                                                                                                                                                           |
| :heavy_check_mark: |  9XY0  |  Cond   | Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)                                                                                                                                                                                                                                                                        |
| :heavy_check_mark: |  ANNN  |   MEM   | Sets I to the address NNN.                                                                                                                                                                                                                                                                                                                                                              |
| :heavy_check_mark: |  BNNN  |  Flow   | Jumps to the address NNN plus V0.                                                                                                                                                                                                                                                                                                                                                       |
| :heavy_check_mark: |  CXNN  |  Rand   | Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.                                                                                                                                                                                                                                                                                       |
| :heavy_check_mark: |  DXYN  |  Disp   | Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen |
| :white_check_mark: |  EX9E  |  KeyOp  | Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block)                                                                                                                                                                                                                                                            |
| :white_check_mark: |  EXA1  |  KeyOp  | Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block)                                                                                                                                                                                                                                                         |
| :white_check_mark: |  FX07  |  Timer  | Sets VX to the value of the delay timer.                                                                                                                                                                                                                                                                                                                                                |
| :white_check_mark: |  FX0A  |  KeyOp  | A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)                                                                                                                                                                                                                                                                        |
| :white_check_mark: |  FX15  |  Timer  | Sets the delay timer to VX.                                                                                                                                                                                                                                                                                                                                                             |
| :white_check_mark: |  FX18  |  Sound  | Sets the sound timer to VX.                                                                                                                                                                                                                                                                                                                                                             |
| :white_check_mark: |  FX1E  |   MEM   | Adds VX to I.                                                                                                                                                                                                                                                                                                                                                                           |
| :white_check_mark: |  FX29  |   MEM   | Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.                                                                                                                                                                                                                                                            |
| :white_check_mark: |  FX33  |   BCD   | Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)       |
| :white_check_mark: |  FX55  |   MEM   | Stores V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.                                                                                                                                                                                                                            |
| :white_check_mark: |  FX65  |   MEM   | Fills V0 to VX (including VX) with values from memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.                                                                                                                                                                                                               |

> Courtesy of [Wikipedia](https://en.wikipedia.org/wiki/CHIP-8).
