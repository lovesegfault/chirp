/// The register bank for a CHIP-8 CPU
// I wish there was a name for the set of registers in a CPU
pub struct Register {
    /// Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a
    /// hexadecimal digit (0 through F).
    ///
    /// The VF (`v[0xE]`) register should not be used by any program, as it is used as a flag by
    /// some instructions.
    v: [u8; 0xF],
    /// Used to store memory addresses, only the lower 12 bits are used
    i: u16,
    /// The delay timer is active whenever the delay timer register (DT) is non-zero. This timer
    /// does nothing more than subtract 1 from the value of DT at a rate of 60Hz. When DT reaches
    /// 0, it deactivates.
    dt: u8,
    ///  The sound timer is active whenever the sound timer register (ST) is non-zero. This timer
    ///  also decrements at a rate of 60Hz, however, as long as ST's value is greater than zero,
    ///  the Chip-8 buzzer will sound. When ST reaches zero, the sound timer deactivates.
    st: u8,
    /// The program counter (PC) is used to store the currently executing address
    pc: u16,
    /// The stack pointer (SP) is used to point to the topmost level of the stack.
    sp: u8,
    ///  The stack is used to store the address that the interpreter shoud return to when finished
    ///  with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.
    stack: [u16; 0xF],
}
