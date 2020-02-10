use crate::instructions::Instruction;
use bitvec::prelude::*;
use std::{
    fmt,
    ops::{Index, IndexMut, Range},
};

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct OpCode(u16);

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06X}", self.0)
    }
}

impl Index<Range<usize>> for OpCode {
    type Output = BitSlice<Lsb0, u16>;
    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.bits()[range]
    }
}

impl IndexMut<Range<usize>> for OpCode {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.bits_mut()[range]
    }
}

impl OpCode {
    pub fn bits(&self) -> &BitSlice<Lsb0, u16> {
        self.0.bits::<Lsb0>()
    }

    pub fn bits_mut(&mut self) -> &mut BitSlice<Lsb0, u16> {
        self.0.bits_mut::<Lsb0>()
    }

    pub fn oooo(oooo: u16) -> Self {
        let mut opcode = Self::default();
        opcode[0..16].store(oooo);
        opcode
    }
    pub fn oook(ooo: u16, k: u8) -> Self {
        let mut opcode = Self::default();
        opcode[4..16].store(ooo);
        opcode[0..4].store(k);
        opcode
    }
    pub fn oxoo(o: u8, x: u8, oo: u8) -> Self {
        let mut opcode = Self::default();
        opcode[8..16].store(oo);
        opcode[4..8].store(x);
        opcode[0..4].store(o);
        opcode
    }
    pub fn oxyo(om: u8, x: u8, y: u8, ol: u8) -> Self {
        let mut opcode = Self::default();
        opcode[12..16].store(om);
        opcode[8..12].store(x);
        opcode[4..8].store(y);
        opcode[0..4].store(ol);
        opcode
    }
    pub fn okkk(o: u8, kkk: u16) -> Self {
        let mut opcode = Self::default();
        opcode[12..16].store(o);
        opcode[0..12].store(kkk);
        opcode
    }
    pub fn oxkk(o: u8, x: u8, kk: u8) -> Self {
        let mut opcode = Self::default();
        opcode[12..16].store(o);
        opcode[8..12].store(x);
        opcode[0..8].store(kk);
        opcode
    }
    pub fn oxyk(o: u8, x: u8, y: u8, k: u8) -> Self {
        let mut opcode = Self::default();
        opcode[12..16].store(o);
        opcode[8..12].store(x);
        opcode[4..8].store(y);
        opcode[0..4].store(k);
        opcode
    }
}

impl From<Instruction> for OpCode {
    fn from(i: Instruction) -> Self {
        use Instruction::*;
        match i {
            ScrollDown(k) => OpCode::oook(0x00C, k),
            ScrollRight => OpCode::oooo(0x00FB),
            ScrollLeft => OpCode::oooo(0x00FC),
            Exit => OpCode::oooo(0x00FD),
            LowRes => OpCode::oooo(0x00FE),
            HighRes => OpCode::oooo(0x00FF),
            ClearScreen => OpCode::oooo(0x00E0),
            Return => OpCode::oooo(0x00EE),
            Jump(addr) => OpCode::okkk(0x1, addr),
            Call(addr) => OpCode::okkk(0x2, addr),
            SkipEqualImmediate(vx, byte) => OpCode::oxkk(0x3, vx, byte),
            SkipNotEqualImediate(vx, byte) => OpCode::oxkk(0x4, vx, byte),
            SkipEqual(vx, vy) => OpCode::oxyo(0x5, vx, vy, 0x0),
            LoadImmediate(vx, byte) => OpCode::oxkk(0x6, vx, byte),
            AddImmediate(vx, byte) => OpCode::oxkk(0x7, vx, byte),
            Load(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0x0),
            Or(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0x1),
            And(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0x2),
            Xor(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0x3),
            Add(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0x4),
            Sub(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0x5),
            ShiftRight(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0x6),
            SubNumeric(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0x7),
            ShiftLeft(vx, vy) => OpCode::oxyo(0x8, vx, vy, 0xE),
            SkipNotEqual(vx, vy) => OpCode::oxyo(0x9, vx, vy, 0x0),
            LoadI(addr) => OpCode::okkk(0xA, addr),
            JumpImmediate(addr) => OpCode::okkk(0xB, addr),
            Random(vx, addr) => OpCode::oxkk(0xC, vx, addr),
            Draw(vx, vy, nibble) => OpCode::oxyk(0xD, vx, vy, nibble),
            SkipOnKey(vx) => OpCode::oxoo(0xE, vx, 0x9E),
            SkipNotOnKey(vx) => OpCode::oxoo(0xE, vx, 0xA1),
            LoadDTIntoV(vx) => OpCode::oxoo(0xF, vx, 0x07),
            LoadKey(vx) => OpCode::oxoo(0xF, vx, 0x0A),
            LoadVIntoDT(vx) => OpCode::oxoo(0xF, vx, 0x15),
            LoadVIntoST(vx) => OpCode::oxoo(0xf, vx, 0x18),
            AddI(vx) => OpCode::oxoo(0xF, vx, 0x1E),
            LoadSpriteIntoI(vx) => OpCode::oxoo(0xF, vx, 0x29),
            LoadBCDIntoI(vx) => OpCode::oxoo(0xF, vx, 0x33),
            LoadVIntoMem(vx) => OpCode::oxoo(0xF, vx, 0x55),
            LoadMemIntoV(vx) => OpCode::oxoo(0xF, vx, 0x65),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::Instruction, opcode::OpCode};

    #[test]
    fn test_scroll_down() {
        let int = Instruction::ScrollDown(0xA);
        let op: OpCode = int.into();
        let expected = OpCode(0x00CA);
        assert_eq!(expected, op);
    }

    #[test]
    fn test_scroll_right() {
        let int = Instruction::ScrollRight;
        let op: OpCode = int.into();
        let expected = OpCode(0x00FB);
        assert_eq!(expected, op);
    }

    #[test]
    fn test_scroll_left() {
        let int = Instruction::ScrollLeft;
        let op: OpCode = int.into();
        let expected = OpCode(0x00FC);
        assert_eq!(expected, op);
    }

    #[test]
    fn test_exit() {
        let int = Instruction::Exit;
        let op: OpCode = int.into();
        let expected = OpCode(0x00FD);
        assert_eq!(expected, op);
    }

    #[test]
    fn test_low_res() {
        let int = Instruction::LowRes;
        let op: OpCode = int.into();
        let expected = OpCode(0x00FE);
        assert_eq!(expected, op);
    }

    #[test]
    fn test_high_res() {
        let int = Instruction::HighRes;
        let op: OpCode = int.into();
        let expected = OpCode(0x00FF);
        assert_eq!(expected, op);
    }

    #[test]
    fn test_clear_screen() {
        let int = Instruction::ClearScreen;
        let op: OpCode = int.into();
        let expected = OpCode(0x00E0);
        assert_eq!(expected, op);
    }

    #[test]
    fn test_return() {
        let int = Instruction::Return;
        let op: OpCode = int.into();
        let expected = OpCode(0x00EE);
        assert_eq!(expected, op);
    }

    #[test]
    fn test_jump() {
        let int = Instruction::Jump(0x0ABC);
        let op: OpCode = int.into();
        let expected = OpCode(0x1ABC);
        assert_eq!(expected, op);
    }
}
