//!# Chip8 CPU
//!Implements Chip8 CPU
//!```
//!use super::Cpu;
//!let cpu = Cpu::new();
//!```

use super::cfg;
pub struct Cpu {
    pub v: [u8; cfg::TOTAL_DATA_REGISTERS as usize],
    pub i: u16,
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub sp: u8,
    stack: [u16; cfg::TOTAL_STACK_SIZE as usize],
}

impl Cpu {
    ///Initializes Cpu
    pub fn new() -> Self {
        Cpu {
            v: [0; cfg::TOTAL_DATA_REGISTERS as usize],
            i: 0,
            dt: 0,
            st: 0,
            pc: cfg::MEM_START,
            sp: 0,
            stack: [0; cfg::TOTAL_STACK_SIZE as usize],
        }
    }

    ///Pushes into stack
    pub fn push(&mut self, item: u16) {
        assert!(
            self.sp < cfg::TOTAL_STACK_SIZE,
            "[Error] Invalid pointer to stack"
        );
        self.stack[self.sp as usize] = item;

        self.sp += 1;
    }
    ///Pops from stack
    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        match self.stack.get(self.sp as usize) {
            Some(val) => *val,
            None => panic!("[Error] Stack Overflow"),
        }
    }
}
