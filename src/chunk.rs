use super::value::Value;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Constant,
    Negate,
    Return,
}

pub type Constant = u8;

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_opcode(&mut self, opcode: Opcode, line: u32) {
        self.code.push(opcode.into());
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> Constant {
        self.constants.push(value);
        (self.constants.len() - 1)
            .try_into()
            .expect("Too many constants")
    }

    pub fn write_constant(&mut self, constant: Constant, line: u32) {
        self.code.push(constant);
        self.lines.push(line);
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn constants(&self) -> &[Value] {
        &self.constants
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {0} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{0:0>1$} ", offset, 4);

        let line = self.lines[offset];
        if offset > 0 && line == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:>4} ", line);
        }

        if let Ok(opcode) = self.code[offset].try_into() {
            match opcode {
                Opcode::Constant => self.constant_instruction("OP_CONSTANT", offset),
                Opcode::Negate => Self::simple_instruction("OP_NEGATE", offset),
                Opcode::Return => Self::simple_instruction("OP_RETURN", offset),
            }
        } else {
            println!("Unknown opcode {0}", self.code[offset]);
            offset + 1
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!(
            "{:<16} {:>4} '{}'",
            name, constant, self.constants[constant as usize]
        );
        offset + 2
    }
}
