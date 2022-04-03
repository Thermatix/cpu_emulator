use crate::{processor, opcodes};

fn make_cpu() -> processor::CPU {

   processor::CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
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
    println!("loc: {}", loc);
    loc = cpu.add_to_mem(loc, &opcodes::OpCode::add(0x0, 0x1));
    println!("loc: {}", loc);
    loc = cpu.add_to_mem(loc, &opcodes::OpCode::add(0x0, 0x2));
    println!("loc: {}", loc);
    cpu.add_to_mem(loc, &opcodes::OpCode::add(0x0, 0x3));
    println!("cpu mem: {:?}", cpu.registers);

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
}

