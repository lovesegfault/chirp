///! The original implementation of the CHIP-8 language includes 36 different instructions,
///! including math, graphics, and flow control functions. Super Chip-48 added an additional 10
///! instructions, for a total of 46.
///!
///! All instructions are 2 bytes long and are stored most-significant-byte first. In memory, the
///! first byte of each instruction should be located at an even addresses. If a program includes
///! sprite data, it should be padded so any instructions following it will be properly situated in
///! RAM.
///!
///! | Symbol   | Width (bits) |
///! |:--------:|:------------:|
///! | `x`, `y` | 4            |
///! | `k`...   | 4 * `k`      |
///!
///! | OpCode   | ASM                  | Op                                                                        |
///! | -------- | -------------------- | ------------------------------------------------------------------------- |
///! | `0kkk`   | `SYS addr`           | Jump to a machine code routine at `kkk` [DEPRECATED]                      |
///! | `00Ck`   | `SCD`                | Scroll down `k` lines                                                     |
///! | `00FB`   | `SCR`                | Scroll right by 4 pixel                                                   |
///! | `00FC`   | `SCL`                | Scroll left by 4 pixel                                                    |
///! | `00FD`   | `EXIT`               | Quit the emulator                                                         |
///! | `00FE`   | `LOW`                | Set CHIP-8 graphics mode                                                  |
///! | `00FF`   | `HIGH`               | Set SCHIP-8 graphics mode                                                 |
///! | `00E0`   | `CLS`                | Clear the display                                                         |
///! | `00EE`   | `RET`                | Return from a subroutine                                                  |
///! | `1kkk`   | `JP addr`            | Jump to location `kkk`                                                    |
///! | `2kkk`   | `CALL addr`          | Call subroutine at `kkk`                                                  |
///! | `3xkk`   | `SE Vx, byte`        | Skip next instruction if `Vx = kk`                                        |
///! | `4xkk`   | `SNE Vx, byte`       | Skip next instruction if `Vx != kk`                                       |
///! | `5xy0`   | `SE Vx, Vy`          | Skip next instruction if `Vx = Vy`                                        |
///! | `6xkk`   | `LD Vx, byte`        | Set Vx = kk                                                               |
///! | `7xkk`   | `ADD Vx, byte`       | Set Vx = Vx + kk                                                          |
///! | `8xy0`   | `LD Vx Vy`           | Set Vx = Vy                                                               |
///! | `8xy1`   | `OR Vx, Vy`          | Set Vx = Vx OR Vy                                                         |
///! | `8xy2`   | `AND Vx, Vy`         | Set Vx = Vx AND Vy                                                        |
///! | `8xy3`   | `XOR Vx, Vy`         | Set Vx = Vx XOR Vy                                                        |
///! | `8xy4`   | `ADD Vx, Vy`         | Set Vx = Vx + Vy, VF = carry                                              |
///! | `8xy5`   | `SUB Vx, Vy`         | Set Vx = Vx - Vy, VF = not borrow                                         |
///! | `8xy6`   | `SHR Vx{, Vy}`       | Set Vx = Vx SHR 1, VF = carry                                             |
///! | `8xy7`   | `SUBN Vx{, Vy}`      | Set Vx = Vy - Vx, set VF = not borrow                                     |
///! | `8xyE`   | `SHL Vx{, Vy}`       | Set Vx = Vx SHL 1, VF = carry                                             |
///! | `9xy0`   | `SNE Vx, Vy`         | Skip next instruction if Vx != Vy                                         |
///! | `Akkk`   | `LD I, addr`         | Set I = kkk                                                               |
///! | `Bkkk`   | `JP V0, addr`        | Jump to location kkk + V0                                                 |
///! | `Cxkk`   | `RND Vx, byte`       | Set Vx = random byte AND kk                                               |
///! | `Dxyk`   | `DRW Vx, Vy, nibble` | Display n-byte sprite starting at M[I] from (Vx, Vy), set VF = collision  |
///! | `Ex9E`   | `SKP Vx`             | Skip next instruction if key with the value of Vx is pressed.             |
///! | `ExA1`   | `SNKP Vx`            | Skip next instruction if key with the value of Vx is not pressed          |
///! | `Fx07`   | `LD Vx, DT`          | Set Vx = delay timer value                                                |
///! | `Fx0A`   | `LD Vx, K`           | Wait for a key press, store the value of the key in Vx                    |
///! | `Fx15`   | `LD DT, Vx`          | Set delay timer = Vx                                                      |
///! | `Fx18`   | `LD ST, Vx`          | Set sound timer = Vx                                                      |
///! | `Fx1E`   | `ADD I, Vx`          | Set I = I + Vx                                                            |
///! | `Fx29`   | `LD F, Vx`           | Set I = location of sprite for digit Vx                                   |
///! | `Fx33`   | `LD B, Vx`           | Store BCD representation of Vx in memory locations I, I+1, and I+2        |
///! | `Fx55`   | `LD [I], Vx`         | Store registers V0 through Vx in memory starting at location I            |
///! | `Fx65`   | `LD Vx, [I]`         | Read registers V0 through Vx from memory starting at location I           |
use bitvec::prelude::*;
use std::fmt;

pub enum Instruction {
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `00Ck`   | `SCD`                | Scroll down `k` lines                                                     |
    ScrollDown(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `00FB`   | `SCR`                | Scroll right by 4 pixel                                                   |
    ScrollRight,
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `00FC`   | `SCL`                | Scroll left by 4 pixel                                                    |
    ScrollLeft,
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `00FD`   | `EXIT`               | Quit the emulator                                                         |
    Exit,
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `00FE`   | `LOW`                | Set CHIP-8 graphics mode                                                  |
    LowRes,
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `00FF`   | `HIGH`               | Set SCHIP-8 graphics mode                                                 |
    HighRes,
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `00E0`   | `CLS`                | Clear the display                                                         |
    ClearScreen,
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `00EE`   | `RET`                | Return from a subroutine                                                  |
    Return,
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `1kkk`   | `JP addr`            | Jump to location `kkk`                                                    |
    Jump(u16),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `2kkk`   | `CALL addr`          | Call subroutine at `kkk`                                                  |
    Call(u16),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `3xkk`   | `SE Vx, byte`        | Skip next instruction if `Vx = kk`                                        |
    SkipEqualImmediate(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `4xkk`   | `SNE Vx, byte`       | Skip next instruction if `Vx != kk`                                       |
    SkipNotEqualImediate(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `5xy0`   | `SE Vx, Vy`          | Skip next instruction if `Vx = Vy`                                        |
    SkipEqual(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `6xkk`   | `LD Vx, byte`        | Set Vx = kk                                                               |
    LoadImmediate(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `7xkk`   | `ADD Vx, byte`       | Set Vx = Vx + kk                                                          |
    AddImmediate(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xy0`   | `LD Vx Vy`           | Set Vx = Vy                                                               |
    Load(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xy1`   | `OR Vx, Vy`          | Set Vx = Vx OR Vy                                                         |
    Or(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xy2`   | `AND Vx, Vy`         | Set Vx = Vx AND Vy                                                        |
    And(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xy3`   | `XOR Vx, Vy`         | Set Vx = Vx XOR Vy                                                        |
    Xor(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xy4`   | `ADD Vx, Vy`         | Set Vx = Vx + Vy, VF = carry                                              |
    Add(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xy5`   | `SUB Vx, Vy`         | Set Vx = Vx - Vy, VF = not borrow                                         |
    Sub(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xy6`   | `SHR Vx{, Vy}`       | Set Vx = Vx SHR 1, VF = carry                                             |
    ShiftRight(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xy7`   | `SUBN Vx{, Vy}`      | Set Vx = Vy - Vx, set VF = not borrow                                     |
    SubNumeric(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `8xyE`   | `SHL Vx{, Vy}`       | Set Vx = Vx SHL 1, VF = carry                                             |
    ShiftLeft(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `9xy0`   | `SNE Vx, Vy`         | Skip next instruction if Vx != Vy                                         |
    SkipNotEqual(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Akkk`   | `LD I, addr`         | Set I = kkk                                                               |
    LoadI(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Bkkk`   | `JP V0, addr`        | Jump to location kkk + V0                                                 |
    JumpImmediate(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Cxkk`   | `RND Vx, byte`       | Set Vx = random byte AND kk                                               |
    Random(u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | ----------------     | ------------------------------------------------------------------------- |
    /// | `Dxyk`   | `DRW Vx, Vy, nibble` | Display n-byte sprite starting at M[I] from (Vx, Vy), set VF = collision  |
    Draw(u8, u8, u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Ex9E`   | `SKP Vx`             | Skip next instruction if key with the value of Vx is pressed.             |
    SkipOnKey(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `ExA1`   | `SNKP Vx`            | Skip next instruction if key with the value of Vx is not pressed          |
    SkipNotOnKey(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx07`   | `LD Vx, DT`          | Set Vx = delay timer value                                                |
    LoadDTIntoV(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx0A`   | `LD Vx, K`           | Wait for a key press, store the value of the key in Vx                    |
    LoadKey(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx15`   | `LD DT, Vx`          | Set delay timer = Vx                                                      |
    LoadVIntoDT(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx18`   | `LD ST, Vx`          | Set sound timer = Vx                                                      |
    LoadVIntoST(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx1E`   | `ADD I, Vx`          | Set I = I + Vx                                                            |
    AddI(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx29`   | `LD F, Vx`           | Set I = location of sprite for digit Vx                                   |
    LoadSpriteIntoI(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx33`   | `LD B, Vx`           | Store BCD representation of Vx in memory locations I, I+1, and I+2        |
    LoadBCDIntoI(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx55`   | `LD [I], Vx`         | Store registers V0 through Vx in memory starting at location I            |
    LoadVIntoMem(u8),
    /// | OpCode   | ASM                  | Op                                                                        |
    /// | -------- | -------------------- | ------------------------------------------------------------------------- |
    /// | `Fx65`   | `LD Vx, [I]`         | Read registers V0 through Vx from memory starting at location I           |
    LoadMemIntoV(u8),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OpCode(u16);

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06X}", self.0)
    }
}

macro_rules! opcode {
    ($o:expr, $nnn:ident) => {{
        let mut opcode: u16 = 0x0;
        opcode.bits_mut::<Msb0>()[0..4].store($o as u8);
        opcode.bits_mut::<Msb0>()[4..16].store($nnn as u16);
        OpCode(opcode)
    }};
    ($o:expr, $n:ident, $nn:ident) => {{
        let mut opcode: u16 = 0x0;
        opcode.bits_mut::<Msb0>()[0..4].store($o as u8);
        opcode.bits_mut::<Msb0>()[4..8].store($n as u8);
        opcode.bits_mut::<Msb0>()[8..16].store($nn as u8);
        OpCode(opcode)
    }};
    ($om:expr, $nm:ident, $nl:ident, $ol:expr) => {{
        let mut opcode: u16 = 0x0;
        opcode.bits_mut::<Msb0>()[0..4].store($om as u8);
        opcode.bits_mut::<Msb0>()[4..8].store($nm as u8);
        opcode.bits_mut::<Msb0>()[8..12].store($nl as u8);
        opcode.bits_mut::<Msb0>()[12..16].store($ol as u8);
        OpCode(opcode)
    }};
    ($om:expr, $nm:ident, $n:ident, $nl:ident) => {{
        let mut opcode: u16 = 0x0;
        opcode.bits_mut::<Msb0>()[0..4].store($om as u8);
        opcode.bits_mut::<Msb0>()[4..8].store($nm as u8);
        opcode.bits_mut::<Msb0>()[8..12].store($n as u8);
        opcode.bits_mut::<Msb0>()[12..16].store($nl as u8);
        OpCode(opcode)
    }};
    ($om:expr, $n:ident, $ol:expr) => {{
        let mut opcode: u16 = 0x0;
        opcode.bits_mut::<Msb0>()[0..4].store($om as u8);
        opcode.bits_mut::<Msb0>()[4..8].store($n as u8);
        opcode.bits_mut::<Msb0>()[8..16].store($ol as u8);
        OpCode(opcode)
    }};
    ($oo:expr, $nn:ident) => {{
        let mut opcode: u16 = 0x0;
        opcode.bits_mut::<Msb0>()[0..8].store($oo as u8);
        opcode.bits_mut::<Msb0>()[8..16].store($nn as u8);
        OpCode(opcode)
    }};
    ($ooo:expr, $n:ident) => {{
        let mut opcode: u16 = 0x0;
        opcode.bits_mut::<Msb0>()[0..12].store($ooo as u16);
        opcode.bits_mut::<Msb0>()[12..16].store($n as u8);
        OpCode(opcode)
    }};
    ($oooo:expr) => {{
        let mut opcode: u16 = 0x0;
        opcode.bits_mut::<Msb0>()[0..16].store($oooo as u16);
        OpCode(opcode)
    }};
}

impl From<Instruction> for OpCode {
    fn from(i: Instruction) -> Self {
        use Instruction::*;
        match i {
            ScrollDown(k) => opcode!(0x00C, k),
            ScrollRight => opcode!(0x00FB),
            ScrollLeft => opcode!(0x00FC),
            Exit => opcode!(0x00FD),
            LowRes => opcode!(0x00FE),
            HighRes => opcode!(0x00FF),
            ClearScreen => opcode!(0x00E0),
            Return => opcode!(0x00E0),
            Jump(addr) => opcode!(0x1, addr),
            Call(addr) => opcode!(0x2, addr),
            SkipEqualImmediate(vx, byte) => opcode!(0x3, vx, byte),
            SkipNotEqualImediate(vx, byte) => opcode!(0x4, vx, byte),
            SkipEqual(vx, vy) => opcode!(0x5, vx, vy, 0x0),
            LoadImmediate(vx, byte) => opcode!(0x6, vx, byte),
            AddImmediate(vx, byte) => opcode!(0x7, vx, byte),
            Load(vx, vy) => opcode!(0x8, vx, vy, 0x0),
            Or(vx, vy) => opcode!(0x8, vx, vy, 0x1),
            And(vx, vy) => opcode!(0x8, vx, vy, 0x2),
            Xor(vx, vy) => opcode!(0x8, vx, vy, 0x3),
            Add(vx, vy) => opcode!(0x8, vx, vy, 0x4),
            Sub(vx, vy) => opcode!(0x8, vx, vy, 0x5),
            ShiftRight(vx, vy) => opcode!(0x8, vx, vy, 0x6),
            SubNumeric(vx, vy) => opcode!(0x8, vx, vy, 0x7),
            ShiftLeft(vx, vy) => opcode!(0x8, vx, vy, 0xE),
            SkipNotEqual(vx, vy) => opcode!(0x9, vx, vy, 0x0),
            LoadI(addr) => opcode!(0xA, addr),
            JumpImmediate(addr) => opcode!(0xB, addr),
            Random(vx, addr) => opcode!(0xC, vx, addr),
            Draw(vx, vy, nibble) => opcode!(0xD, vx, vy, nibble),
            SkipOnKey(vx) => opcode!(0xE, vx, 0x9E),
            SkipNotOnKey(vx) => opcode!(0xE, vx, 0xA1),
            LoadDTIntoV(vx) => opcode!(0xF, vx, 0x07),
            LoadKey(vx) => opcode!(0xF, vx, 0x0A),
            LoadVIntoDT(vx) => opcode!(0xF, vx, 0x15),
            LoadVIntoST(vx) => opcode!(0xf, vx, 0x18),
            AddI(vx) => opcode!(0xF, vx, 0x1E),
            LoadSpriteIntoI(vx) => opcode!(0xF, vx, 0x29),
            LoadBCDIntoI(vx) => opcode!(0xF, vx, 0x33),
            LoadVIntoMem(vx) => opcode!(0xF, vx, 0x55),
            LoadMemIntoV(vx) => opcode!(0xF, vx, 0x65),
        }
    }
}

#[cfg(test)]
mod tests {}
