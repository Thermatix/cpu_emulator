use std::convert::TryFrom;
use super::opcodes::OpCode;

back_to_enum! {
    enum Register {
        Carry = 0xF,
    }
}


pub struct CPU {
    pub registers: [u8; 16],
    pub program_counter: usize,
    pub memory: [u8; 0x1000],
}

impl CPU {
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
       'exeuction: loop {
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
                (0, 0, 0, 0) => break 'exeuction,
                (0x8, a, b, 0x4) => self.add(a, b),
                _ => todo!("opcode {:04x}", code),
            }
        }
    }

    fn add(&mut self, a: &u8, b: &u8) {
        let arg1 = self.registers[*a as usize];
        let arg2 = self.registers[*b as usize];
        println!("register: {}, {}", a, self.registers[*a as usize]);

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[*a as usize] = val;
        println!("register: {}, {}", a, self.registers[*a as usize]);
        
        if overflow {
            self.registers[Register::Carry as usize] = 1;
        } else {
            self.registers[Register::Carry as usize] = 0;
        }
    }
}
