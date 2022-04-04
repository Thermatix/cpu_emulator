use std::convert::TryFrom;
use super::opcodes::{BYTE, NIBBLE, OpCode};

back_to_enum! {
    enum NamedRegister {
        Carry = 0xF,
    }
}


#[derive(Debug)]
pub struct CPU {
    pub registers: [u8; 16],
    pub program_counter: usize,
    pub memory: [u8; 0x1000],
    pub stack: [u16; 16],
    pub stack_pointer: usize,
}

impl CPU {
    pub fn copy_to_mem(&mut self, loc: usize, data: &[OpCode]) {
        data.iter().fold(loc, |loc, bytes| {
            self.add_to_mem(loc, bytes)
        });
    }

    pub fn raw_copy_to_mem(&mut self, loc: usize, data: &[u8]) {
        data.chunks(2).fold(loc, |loc, bytes| {
            self.raw_add_to_mem(loc, *bytes.first().unwrap(), *bytes.last().unwrap())
        });
    }

    pub fn add_to_mem(&mut self, loc: usize, oc: &OpCode) -> usize {
        self.memory[loc] = oc.high_byte(); self.memory[loc + 1] = oc.low_byte();
        loc + 2
    }

    pub fn raw_add_to_mem(&mut self, loc: usize, high: u8, low: u8) -> usize {
        self.memory[loc] = high; self.memory[loc + 1] = low;
        loc + 2
    }
    
    fn read_opcode(&self) -> u16 {
        let pc = self.program_counter;
        let op_byte1 = self.memory[pc] as u16;
        let op_byte2 = self.memory[pc + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
       'execution: loop {
            let code = self.read_opcode();
            self.program_counter += 2;
            let opcode = 
                (
                    ((code & 0xF000 ) >> 12 ) as u8, // c
                    ((code & 0x0F00 ) >>  8 ) as u8, // x
                    ((code & 0x00F0 ) >>  4 ) as u8, // y
                    ((code & 0x000F ) >>  0 ) as u8, // d
                );

            match &opcode.into() {
                (0, 0, 0, 0) => break 'execution,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, n1, n2, n3) => self.call(((*n1 as u16) << BYTE | (*n2 as u16) << NIBBLE) | *n3 as u16),
                (0x8, a, b, 0x4) => self.add(a, b),
                _ => todo!("opcode {:04x}", code),
            }
        }
    }

    fn add(&mut self, a: &u8, b: &u8) {
        let arg1 = self.registers[*a as usize];
        let arg2 = self.registers[*b as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[*a as usize] = val;
        
        if overflow {
            self.registers[NamedRegister::Carry as usize] = 1;
        } else {
            self.registers[NamedRegister::Carry as usize] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        if self.stack_pointer > self.stack.len() {
            panic!("Stack Overflow!");
        }

        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer +=1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack Underflow!")
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;

    }
}
