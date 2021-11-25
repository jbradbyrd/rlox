mod chunk;
mod value;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_opcode(Opcode::Constant, 123);
    chunk.write_constant(constant, 123);
    chunk.write_opcode(Opcode::Return, 123);
    chunk.disassemble("test chunk");
}
