use std::convert::TryFrom;
use std::fmt;
use std::io::{self, prelude::*};

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Failed to read game file into memory")]
    LoadGameFile(#[source] io::Error),
    #[error("Failed to open game file")]
    OpenGameFile(#[source] io::Error),
}

const MEMORY_SIZE: usize = 0x1000;
const MEMORY_START: usize = 0x200; // The first 512 bytes were reserved for the CHIP-8 interpreter

#[derive(Debug)]
pub struct Memory {
    memory: Vec<u8>,
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use hexyl::{BorderStyle, Printer};

        // This is woefully inefficient, but whatever
        let mut buf: Vec<u8> = Vec::new();
        let mut printer = Printer::new(&mut buf, true, BorderStyle::Unicode, true);
        printer.display_offset(0);
        printer.print_all(&self.memory[..], None);

        f.write_str(&String::from_utf8_lossy(&buf))
    }
}

impl io::Write for Memory {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Make sure we write at most MEMORY_SIZE bytes
        let buf_len = buf.len();
        let data_length = if buf_len < MEMORY_SIZE {
            buf_len
        } else {
            MEMORY_SIZE
        };

        self.memory[0..data_length].copy_from_slice(buf);
        Ok(data_length)
    }

    fn flush(&mut self) -> io::Result<()> {
        // NOOP, since this is just a Vec
        Ok(())
    }
}

impl TryFrom<std::fs::File> for Memory {
    type Error = MemoryError;
    fn try_from(mut f: std::fs::File) -> Result<Self, Self::Error> {
        let mut memory = Memory::default();
        f.read(&mut memory.memory[MEMORY_START..])
            .map_err(|e| MemoryError::LoadGameFile(e))?;
        Ok(memory)
    }
}

impl TryFrom<&std::path::Path> for Memory {
    type Error = MemoryError;
    fn try_from(p: &std::path::Path) -> Result<Self, Self::Error> {
        let f = std::fs::File::open(p).map_err(|e| MemoryError::OpenGameFile(e))?;
        Memory::try_from(f)
    }
}

impl TryFrom<&str> for Memory {
    type Error = MemoryError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let p = std::path::Path::new(s);
        Memory::try_from(p)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![0; MEMORY_SIZE],
        }
    }

    fn mask(&self, idx: usize) -> usize {
        idx % MEMORY_SIZE
    }

    pub fn get(&self, idx: usize) -> &u8 {
        let masked_idx = self.mask(idx);
        // This is always safe because the call to `self.mask` enforces that the index is in bounds
        // by taking care of wrapping
        unsafe { self.memory.get_unchecked(masked_idx) }
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut u8 {
        let masked_idx = self.mask(idx);
        // This is always safe because the call to `self.mask` enforces that the index is in bounds
        // by taking care of wrapping
        unsafe { self.memory.get_unchecked_mut(masked_idx) }
    }

    pub fn dump(&self) {
        println!("{}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::*;
}
