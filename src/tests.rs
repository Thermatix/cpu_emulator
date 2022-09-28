use crate::{processor, opcodes::{OpCode, NIBBLE, BYTE}};

fn make_cpu() -> processor::CPU {

   processor::CPU {
        registers: [0; 16],
        memory: [0; 0x1000],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    }
}

#[test]
fn test_add_opcode() {
    let mut cpu = make_cpu();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mut loc = 0;
    loc = cpu.add_to_mem(loc, &OpCode::add(0x0, 0x1));
    loc = cpu.add_to_mem(loc, &OpCode::add(0x0, 0x2));
    cpu.add_to_mem(loc, &OpCode::add(0x0, 0x3));

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
}

#[test]
fn test_raw_memory_copy() {
    let mut cpu = make_cpu();

    let data: [u8; 6] = [
        0x80, 0x14,
        0x80, 0x14,
        0x00, 0xEE,
    ];

    cpu.raw_copy_to_mem(0x100 , &data);

    assert_eq!(cpu.memory[0x100..0x106], data);

}

#[test]
fn test_memory_copy() {
    let mut cpu = make_cpu();

    let data: [OpCode; 3] = [
        OpCode::add(0x0, 0x1),
        OpCode::add(0x0, 0x1),
        OpCode::ret(),
    ];

    cpu.copy_to_mem(0x100, &data);

    let cpu_mem: Vec<u16> = cpu.memory[0x100..0x106].chunks(2).map(|d| (*d.first().unwrap() as u16) << BYTE | *d.last().unwrap() as u16).collect();
    let data: Vec<u16> = data.iter().map(|oc| (oc.high_byte() as u16) << BYTE | oc.low_byte() as u16).collect();

    assert_eq!(cpu_mem, data);
}

#[test]
fn test_memory_vs_raw() {
    let mut cpu = make_cpu();

    let data: [OpCode; 3] = [
        OpCode::add(0x0, 0x1),
        OpCode::add(0x0, 0x1),
        OpCode::ret(),
    ];

    cpu.copy_to_mem(0x100, &data);

    let data: [u16; 3] = [
        0x8014,
        0x8014,
        0x00EE,
    ];

    let cpu_mem: Vec<u16> = cpu.memory[0x100..0x106].chunks(2).map(|d| (*d.first().unwrap() as u16) << BYTE | *d.last().unwrap() as u16).collect();

    assert_eq!(cpu_mem, data);
}

#[test]
fn test_shift_right() {
    let mut cpu = make_cpu();

    cpu.registers[0] = 255;
    cpu.registers[1] = 0;

    cpu.add_to_mem(0, &OpCode::shift_right(0x0, 0x1));

    cpu.run();

    assert_eq!(cpu.registers[0], 127);
    assert_eq!(cpu.registers[1], 1);
}

#[test]
fn test_shift_left() {
    let mut cpu = make_cpu();

    cpu.registers[0] = 5;
    cpu.registers[1] = 0;

    cpu.add_to_mem(0, &OpCode::shift_left(0x0, 0x1));

    cpu.run();

    assert_eq!(cpu.registers[0], 10);
    assert_eq!(cpu.registers[1], 1);
}

#[test]
fn test_goto() {
    let mut cpu = make_cpu();
    
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let loc1: usize = 0x000;
    let goto: [OpCode; 1] = [
        OpCode::goto(0x1,0x0,0x0),
    ];

    let loc2: usize = 0x100;
    
    cpu.copy_to_mem(loc1, &goto);

    let stack_counter = cpu.stack_pointer;
    cpu.run();
    let opcode_length = 2;
    assert_eq!(cpu.program_counter, loc2 + opcode_length);
    assert_eq!(stack_counter, cpu.stack_pointer);
}


#[test]
fn test_call_and_return() {
    let mut cpu = make_cpu();
    
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let loc1: usize = 0x000;
    let call_function: [OpCode; 3] = [
        OpCode::call(0x1,0x0,0x0),
        OpCode::call(0x1,0x0,0x0),
        OpCode::halt(),

    ];

    let loc2: usize = 0x100;
    let add_twice: [OpCode; 3] = [
        OpCode::add(0x0, 0x1),
        OpCode::add(0x0, 0x1),
        OpCode::ret(),
    ];
    
    cpu.copy_to_mem(loc1, &call_function);
    cpu.copy_to_mem(loc2, &add_twice);

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
}

#[test]
fn test_skip_if_x_eql_nn() {
    let mut cpu = make_cpu();

    cpu.registers[0] = 5;

    let loc1: usize = 0x000;
    let skip_test: [OpCode; 3] = [
        OpCode::skip_x_eq_nn(0x0, 0x0, 0x5),
        OpCode::call(0x1,0x0,0x0),
        OpCode::halt(),
    ];
let loc2: usize = 0x100;
    let add_twice: [OpCode; 2] = [
        OpCode::add(0x0, 0x1),
        OpCode::add(0x0, 0x1),
    ];

    cpu.copy_to_mem(loc1, &skip_test);
    cpu.copy_to_mem(loc2, &add_twice);

    cpu.run();

    let opcode_length = 2;
    assert_eq!(cpu.program_counter, loc1 + (opcode_length * 3));
}
