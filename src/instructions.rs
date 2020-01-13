/// The original implementation of the CHIP-8 language includes 36 different instructions,
/// including math, graphics, and flow control functions. Super Chip-48 added an additional 10
/// instructions, for a total of 46.
///
/// All instructions are 2 bytes long and are stored most-significant-byte first. In memory, the
/// first byte of each instruction should be located at an even addresses. If a program includes
/// sprite data, it should be padded so any instructions following it will be properly situated in
/// RAM.
///
/// | Symbol | Width (bits)  | Addressing Mode | Notes      |
/// |:------:|:-------------:|:---------------:|:----------:|
/// | MMM    | 12            | Memory          |            |
/// | X, Y   | 4             | Register        |            |
/// | K      | 4             | Immediate       | SCHIP only |
/// | KK     | 8             | Immediate       |            |
/// | KKK    | 12            | Immediate       |            |
pub enum Instruction {
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------  | --------------------------------------- | ---------- |
    /// | `0x0KKK` | `SYS`   | Jump to a machine code routine at `KKK` | Deprecated |
    Sys(u16),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------  | --------------------------------------- | ---------- |
    /// | `0x00CK` | `SCD`   | Scroll down `K` lines                   | SCHIP only |
    Scd(u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------  | --------------------------------------- | ---------- |
    /// | `0x00FB` | `SCR`   | Scroll right by 4 pixel                 | SCHIP only |
    Scr,
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------  | --------------------------------------- | ---------- |
    /// | `0x00FC` | `SCL`   | Scroll left by 4 pixel                  | SCHIP only |
    Scl,
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x00FD` | `EXIT`  | Quit the emulator                       | SCHIP only |
    Exit,
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x00FE` | `LOW`   | Set CHIP-8 graphics mode                | SCHIP only |
    Low,
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x00FF` | `HIGH`   | Set SCHIP-8 graphics mode              | SCHIP only |
    High,
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x00E0` | `CLS`   | Clear the display                       |            |
    Cls,
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x00EE` | `RET`   | Return from a subroutine                |            |
    // The interpreter sets the program counter to the address at the top of the stack, then
    // subtracts 1 from the stack pointer
    Ret,
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x1KKK` | `JP`    | Jump to location `KKK`                  |            |
    // The interpreter sets the program counter to KKK
    Jp(u16),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x2KKK` | `CALL`  | Call subroutine at `KKK`                |            |
    // The interpreter increments the stack pointer, then puts the current PC on the top of the
    // stack. The PC is then set to KKK
    Call(u16),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x3XKK` | `SE`    | Skip next instruction if `V[X] = KK`    |            |
    // The interpreter compares register V[X] to KK, and if they are equal, increments the program
    // counter by 2
    Sei(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x4XKK` | `SNE`   | Skip next instruction if `V[X] != KK`   |            |
    // The interpreter compares register V[X] to KK, and if they are not equal, increments the
    // program counter by 2
    Sne(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x5XY0` | `SE`    | Skip next instruction if `V[X] = V[Y]`  |            |
    // The interpreter compares register V[X] to register V[Y], and if they are equal, increments
    // the program counter by 2.
    Se(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x6XKK` | `LD`    | Set V[X] = KK                           |            |
    Ldi(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x7XKK` | `ADD`   | Set V[X] = V[X] + KK                    |            |
    Addi(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY0` | `LD`    | Set V[X] = V[Y]                         |            |
    Ld(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY1` | `OR`    | Set V[X] = V[X] OR V[Y]                 |            |
    // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR
    // compares the corresponding bits from two values, and if either bit is 1, then the same bit
    // in the result is also 1. Otherwise, it is 0.
    Or(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY2` | `AND`   | Set V[X] = V[X] AND V[Y]                |            |
    // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise
    // AND compares the corrseponding bits from two values, and if both bits are 1, then the same
    // bit in the result is also 1. Otherwise, it is 0.
    And(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY3` | `XOR`   | Set V[X] = V[X] XOR V[Y]                |            |
    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    // An exclusive OR compares the corrseponding bits from two values, and if the bits are not
    // both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    Xor(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY4` | `ADD`   | Set V[X] = V[X] + V[Y], VF = carry      |            |
    // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., >
    // 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored
    // in Vx.
    Add(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY5` | `SUB`   | Set V[X] = V[X] - V[Y], VF = not borrow |            |
    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results
    // stored in Vx.
    Sub(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY6` | `SHR`   | Set V[X] = V[X] SHR 1, VF = carry       |            |
    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is
    // divided by 2.
    Shr(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY7` | `SUBN`  | Set Vx = Vy - Vx, set VF = not borrow   |            |
    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results
    // stored in Vx.
    Subn(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XYE` | `SHL`   | Set V[X] = V[X] SHL 1, VF = carry       |            |
    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is
    // multiplied by 2.
    Shl(u8, u8),
}
