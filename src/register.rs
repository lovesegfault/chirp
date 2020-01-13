/// The register bank for a CHIP-8 CPU
// XXX: I wish there was a name for the set of registers in a CPU
#[derive(Default)]
pub struct Register {
    /// The general purpose registers
    ///
    /// CHIP-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a
    /// hexadecimal digit (0 through F).
    ///
    /// The VF (`v[0xE]`) register should not be used by any program, as it is used as carry
    /// (when using arithmetic instructions) and collision detector (when drawing sprites).
    pub(crate) v: [u8; 0xF],
    /// The address register
    ///
    /// As the memory is 4Kb, the interpreter uses only its 12 lower bits. The remaining 4 could be
    /// set using the `LOAD FONT` instruction as the font was located at address 0x1FAE
    pub(crate) i: u16,
    /// The delay timer
    ///
    /// It is active whenever the delay timer register (DT) is non-zero. This timer
    /// does nothing more than subtract 1 from the value of DT at a rate of 60Hz. When DT reaches
    /// 0, it deactivates.
    pub(crate) dt: u8,
    /// The sound timer
    ///
    /// It is active whenever the sound timer register (ST) is non-zero. This timer
    /// also decrements at a rate of 60Hz, however, as long as ST's value is greater than zero,
    /// the CHIP-8 buzzer will sound. When ST reaches zero, the sound timer deactivates.
    pub(crate) st: u8,
    /// The program counter (PC) is used to store the currently executing address
    pub(crate) pc: u16,
    /// The stack pointer (SP) is used to point to the topmost level of the stack.
    pub(crate) sp: u8,
    /// The stack is used to store the address that the interpreter shoud return to when finished
    /// with a subroutine. CHIP-8 allows for up to 16 levels of nested subroutines.
    ///
    /// NB: This may not apply to the original CHIP-8 as no documentation was found on the stack.
    stack: [u16; 0xF],
}
