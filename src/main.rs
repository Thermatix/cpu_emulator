#[macro_use]
mod macros;
mod floating_point;
mod fixed_point;
mod rand;
mod processor;
mod opcodes;


fn main() {
    let mut cpu = processor::CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
    };

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

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
