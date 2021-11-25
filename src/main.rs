#![allow(dead_code)]

mod chunk;
mod value;
mod vm;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_opcode(Opcode::Constant, 123);
    chunk.write_constant(constant, 123);
    chunk.write_opcode(Opcode::Negate, 123);
    chunk.write_opcode(Opcode::Return, 123);
    //chunk.disassemble("test chunk");

    let mut vm = vm::VM::new();
    vm.interpret(&chunk);
}
