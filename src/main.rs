#![allow(dead_code)]

mod chunk;
mod value;
mod vm;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    let mut constant = chunk.add_constant(1.2);
    chunk.write_opcode(Opcode::Constant, 123);
    chunk.write_constant(constant, 123);

    constant = chunk.add_constant(3.4);
    chunk.write_opcode(Opcode::Constant, 123);
    chunk.write_constant(constant, 123);

    chunk.write_opcode(Opcode::Add, 123);

    constant = chunk.add_constant(5.6);
    chunk.write_opcode(Opcode::Constant, 123);
    chunk.write_constant(constant, 123);

    chunk.write_opcode(Opcode::Divide, 123);

    chunk.write_opcode(Opcode::Negate, 123);
    chunk.write_opcode(Opcode::Return, 123);
    //chunk.disassemble("test chunk");

    let mut vm = vm::VM::new();
    vm.interpret(&chunk);
}
