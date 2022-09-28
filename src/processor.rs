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

type DecodedOpcode = (u8, u8, u8, u8);

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

    fn decode(&mut self, code: u16) -> DecodedOpcode {
        (
            ((code & 0xF000 ) >> 12 ) as u8, // c
            ((code & 0x0F00 ) >>  8 ) as u8, // x
            ((code & 0x00F0 ) >>  4 ) as u8, // y
            ((code & 0x000F ) >>  0 ) as u8, // d
        )
    }

    pub fn run(&mut self) {
       'execution: loop {
            let code = self.read_opcode();
            let opcode = self.decode(code);
            self.program_counter += 2;

            match &opcode.into() {
                (0x0, 0x0, 0x0, 0x0) => break 'execution, // halt
                (0x0,0x0, 0x0, 0xE) => unimplemented!(), // clear
                (0x0, 0x0, 0xE, 0xE) => self.ret(), // return
                (0x1, n1, n2, n3) => self.goto(((*n1 as u16) << BYTE | (*n2 as u16) << NIBBLE) | *n3 as u16), // goto
                (0x2, n1, n2, n3) => self.call(((*n1 as u16) << BYTE | (*n2 as u16) << NIBBLE) | *n3 as u16),
                (0x3, x, n2, n3) => unimplemented!(), // skip if X equals NN
                (0x4, x, n2, n3) => unimplemented!(), // skip if X not equals NN
                (0x5, x, y, 0x0) => unimplemented!(), // skip if X equals Y
                (0x6, x, n2, n3) => unimplemented!(), // set x to NN
                (0x7, x, n2, n3) => unimplemented!(), // et x to NN
                (0x0, n1, n2, n3) => unimplemented!(), // call routine
                (0x8, x, y, 0x0) => self.set_xy(x, y),
                (0x8, x, y, 0x1) => self.or_xy(x, y),
                (0x8, x, y, 0x2) => self.and_xy(x, y),
                (0x8, x, y, 0x3) => self.xor_xy(x, y),
                (0x8, x, y, 0x4) => self.add_xy(x, y),
                (0x8, x, y, 0x5) => self.sub_xy(x, y),
                (0x8, x, y, 0x6) => self.shift_right(x, y),
                (0x8, x, y, 0x7) => self.sub_yx(x, y),
                (0x8, x, y, 0xE) => self.shift_left(x, y),
                (0x9, x, y, 0x0) => unimplemented!(), // skip if x not equal to y
                (0xA, n1, n2, n3) => unimplemented!(),
                (0xB, n1, n2, n3) => unimplemented!(),
                (0xC, n1, n2, n3) => unimplemented!(),
                (0xD, n1, n2, n3) => unimplemented!(),
                (0xE, x, 0x9, 0xE) => unimplemented!(),
                (0xF, x, 0x0, 0x7) => unimplemented!(),
                (0xF, x, 0x0, 0xA) => unimplemented!(),
                (0xF, x, 0x1, 0x5) => unimplemented!(),
                (0xF, x, 0x1, 0x8) => unimplemented!(),
                (0xF, x, 0x1, 0xE) => unimplemented!(),
                (0xF, x, 0x2, 0x9) => unimplemented!(),
                (0xF, x, 0x3, 0x3) => unimplemented!(),
                (0xF, x, 0x5, 0x5) => unimplemented!(),
                (0xF, x, 0x6, 0x5) => unimplemented!(),
                _ => todo!("opcode {:04x}", code),
            }
        }
    }

    fn set_xy(&mut self, x: &u8, y: &u8) {
        self.registers[*x as usize] = self.registers[*y as usize];
    }

    // I'm not sure if this is correct or not...
    fn or_xy(&mut self, x: &u8, y: &u8) {
        self.registers[*x as usize] = self.registers[*x as usize] | self.registers[*y as usize];
    }

    // I'm not sure if this is correct or not...
    fn and_xy(&mut self, x: &u8, y: &u8) {
        self.registers[*x as usize] = self.registers[*x as usize] & self.registers[*y as usize];
    }

    // I'm not sure if this is correct or not...
    fn xor_xy(&mut self, x: &u8, y: &u8) {
        self.registers[*x as usize] = self.registers[*x as usize] ^ self.registers[*y as usize];
    }

    fn add_xy(&mut self, x: &u8, y: &u8) {
        let arg1 = self.registers[*x as usize];
        let arg2 = self.registers[*y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[*x as usize] = val;
        
        if overflow {
            self.registers[NamedRegister::Carry as usize] = 1;
        } else {
            self.registers[NamedRegister::Carry as usize] = 0;
        }
    }

    fn  sub_xy(&mut self, x: &u8, y: &u8) {
        let arg1 = self.registers[*x as usize];
        let arg2 = self.registers[*y as usize];

        let (val, overflow) = arg1.overflowing_sub(arg2);
        self.registers[*x as usize] = val;
        
        if overflow {
            self.registers[NamedRegister::Carry as usize] = 1;
        } else {
            self.registers[NamedRegister::Carry as usize] = 0;
        }
    }

    fn  shift_right(&mut self, x: &u8, y: &u8) {
            self.registers[*y as usize] = (self.registers[*x as usize] >> 7) & 1;
            self.registers[*x as usize] = self.registers[*x as usize] >> 1;
    }

    fn  sub_yx(&mut self, x: &u8, y: &u8) {
        let arg1 = self.registers[*x as usize];
        let arg2 = self.registers[*y as usize];

        let (val, overflow) = arg2.overflowing_sub(arg1);
        self.registers[*x as usize] = val;

        if overflow {
            self.registers[NamedRegister::Carry as usize] = 1;
        } else {
            self.registers[NamedRegister::Carry as usize] = 0;
        }
    }

    fn  shift_left(&mut self, x: &u8, y: &u8) {
            self.registers[*y as usize] = self.registers[*x as usize] & 1;
            self.registers[*x as usize] = self.registers[*x as usize] << 1;
    }

    fn goto(&mut self, addr: u16) {
        self.program_counter = addr as usize;
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
