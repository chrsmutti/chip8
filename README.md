# Chip8

Rust implementation of a CHIP-8 VM.

### Opcodes

- **6XNN**: Sets VX to NN.
- **7XNN**: Adds NN to VX. (Carry flag is not changed)
- **8XY0**: Sets VX to the value of VY.
- **8XY1**: Sets VX to VX or VY. (Bitwise OR operation)
- **8XY2**: Sets VX to VX and VY. (Bitwise AND operation)
- **8XY3**: Sets VX to VX xor VY.
- **8XY4**: Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
- **8XY5**: VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
- **8XY6**: Shifts VY right by one and stores the result to VX (VY remains unchanged). VF is set to the value of the least significant bit of VY before the shift.
- **8XY7**: Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
- **8XYE**: Shifts VY left by one and copies the result to VX. VF is set to the value of the most significant bit of VY before the shift.
