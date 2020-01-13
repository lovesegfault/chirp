use chirp::memory;

use std::convert::TryFrom;

fn main() {
    let memory = memory::Memory::default();
    memory.dump();
    let memory = memory::Memory::try_from("../games/tetris.ch8").unwrap();
    memory.dump();

    println!("Chirp!")
}
