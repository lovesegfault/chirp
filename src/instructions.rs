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
    Ret,
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x1KKK` | `JP`    | Jump to location `KKK`                  |            |
    Jp(u16),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x2KKK` | `CALL`  | Call subroutine at `KKK`                |            |
    Call(u16),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x3XKK` | `SE`    | Skip next instruction if `V[X] = KK`    |            |
    Sei(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x4XKK` | `SNE`   | Skip next instruction if `V[X] != KK`   |            |
    Snei(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x5XY0` | `SE`    | Skip next instruction if `V[X] = V[Y]`  |            |
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
    Or(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY2` | `AND`   | Set V[X] = V[X] AND V[Y]                |            |
    And(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY3` | `XOR`   | Set V[X] = V[X] XOR V[Y]                |            |
    Xor(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY4` | `ADD`   | Set V[X] = V[X] + V[Y], VF = carry      |            |
    Add(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY5` | `SUB`   | Set V[X] = V[X] - V[Y], VF = not borrow |            |
    Sub(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY6` | `SHR`   | Set V[X] = V[X] SHR 1, VF = carry       |            |
    Shr(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XY7` | `SUBN`  | Set Vx = Vy - Vx, set VF = not borrow   |            |
    Subn(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x8XYE` | `SHL`   | Set V[X] = V[X] SHL 1, VF = carry       |            |
    Shl(u8, u8),
    /// | OpCode   | Name    | Op                                      | Notes      |
    /// | -------- | ------- | --------------------------------------- | ---------- |
    /// | `0x9XY0` | `SNE`   | Skip next instruction if `V[X] != V[Y]` |            |
    Sne(u8, u8),
    LdI(u8),
    Jpi(u8),
    Rnd(u8, u8),
    Drw(u8, u8, u8),
    Skp(u8),
    Sknp(u8),
    LdVDT(u8),
    LdK(u8),
    LdDTV(u8),
    LdSTV(u8),
    AddI(u8),
}
