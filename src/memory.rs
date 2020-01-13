use std::convert::TryFrom;
use std::fmt;
use std::io::{self, prelude::*};

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Failed to read game file into memory")]
    LoadFile(#[source] io::Error),
    #[error("Failed to open game file")]
    OpenFile(#[source] io::Error),
    #[error("Out of bounds memory access at position {0}")]
    OutOfBoundsAccess(usize),
}

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
        // Make sure we write at most MEMORY_SIZE - MEMORY_START bytes
        let buf_len = buf.len();
        let writeable_size = Self::MEMORY_SIZE - Self::MEMORY_START;
        let data_length = if buf_len < writeable_size {
            buf_len
        } else {
            writeable_size
        };

        self.memory[Self::MEMORY_START..data_length].copy_from_slice(buf);
        Ok(data_length)
    }

    fn flush(&mut self) -> io::Result<()> {
        // NOOP, since this is just a Vec
        Ok(())
    }
}

impl From<&[u8]> for Memory {
    fn from(buf: &[u8]) -> Self {
        let mut memory = Self::default();
        memory.write(buf);
        memory
    }
}

impl TryFrom<std::fs::File> for Memory {
    type Error = MemoryError;
    fn try_from(mut f: std::fs::File) -> Result<Self, Self::Error> {
        let mut memory = Self::default();
        f.read(&mut memory.memory[Self::MEMORY_START..])
            .map_err(MemoryError::LoadFile)?;
        Ok(memory)
    }
}

impl TryFrom<&std::path::Path> for Memory {
    type Error = MemoryError;
    fn try_from(p: &std::path::Path) -> Result<Self, Self::Error> {
        let f = std::fs::File::open(p).map_err(MemoryError::OpenFile)?;
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
    const MEMORY_SIZE: usize = 0x1000;
    const MEMORY_START: usize = 0x200; // The first 512 bytes were reserved for the CHIP-8 interpreter

    pub fn new() -> Self {
        Self {
            memory: vec![0; Self::MEMORY_SIZE],
        }
    }

    #[inline]
    fn check_idx(idx: usize) -> Result<usize, MemoryError> {
        if !(Self::MEMORY_START..Self::MEMORY_SIZE).contains(&idx) {
            Err(MemoryError::OutOfBoundsAccess(idx))
        } else {
            Ok(idx)
        }
    }

    pub fn get(&self, idx: usize) -> Result<&u8, MemoryError> {
        let idx = Self::check_idx(idx)?;
        // This is guaranteed to be safe beause of the call to check_idx
        Ok(unsafe { self.memory.get_unchecked(idx) })
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<&mut u8, MemoryError> {
        let idx = Self::check_idx(idx)?;
        // This is guaranteed to be safe beause of the call to check_idx
        Ok(unsafe { self.memory.get_unchecked_mut(idx) })
    }

    pub fn dump(&self) {
        println!("{}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::*;
}
