//!# Chip8 Memory
//!Implements chip8 Memory
//!```
//!use super::memory::Memory;
//!let memory = Memory::new();
//!```
use super::cfg;

pub struct Memory {
    memory: [u8; cfg::MEMORY as usize],
}

impl Memory {
    ///Initializes Memory
    pub fn new() -> Self {
        let mut memory = [0u8; cfg::MEMORY as usize];
        memory[0..80].copy_from_slice(&cfg::DEFAULT_CHAR_SET);
        Memory { memory }
    }

    ///Gets value at index
    pub fn get(&self, index: u16) -> u8 {
        self.in_bounds(index);
        self.memory[index as usize]
    }

    #[doc(hidden)]
    fn in_bounds(&self, index: u16) {
        assert!(index < cfg::MEMORY, "[Error] Memory out of bounds");
    }

    ///Sets value at index
    pub fn set(&mut self, index: u16, item: u8) {
        self.in_bounds(index);
        self.memory[index as usize] = item;
    }

    ///Gets sprite from memory
    pub fn get_splice(&self, min: u16, size: u8) -> &[u8] {
        self.in_bounds(min + size as u16);
        &self.memory[min as usize..(min + size as u16) as usize]
    }
    ///Gets oppcode
    pub fn get_opcode(&self, pc: u16) -> u16 {
        self.in_bounds(pc + 1);
        let byte1 = self.memory[pc as usize] as u16;
        let byte2 = self.memory[pc as usize + 1] as u16;

        byte1 << 8 | byte2
    }
}
