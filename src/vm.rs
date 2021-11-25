use super::chunk::*;
use super::value::Value;

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = Self {
            ip: 0,
            stack: Vec::new(),
        };

        vm.stack.reserve(255);
        vm
    }

    pub fn interpret(&mut self, chunk: &Chunk) {
        self.ip = 0;
        self.run(chunk);
    }

    fn run(&mut self, chunk: &Chunk) {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            self.trace(chunk);

            match self.read_opcode(chunk) {
                Opcode::Constant => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                }
                Opcode::Negate => {
                    let value = self.stack.pop().unwrap_or_default();
                    self.stack.push(-value);
                }
                Opcode::Return => {
                    println!("{}", self.stack.pop().unwrap_or_default());
                    return
                }
            }
        }
    }

    #[cfg(feature = "debug_trace_execution")]
    fn trace(&self, chunk: &Chunk) {
        print!("          ");
        for value in &self.stack {
            print!("[ {} ]", value);
        }
        println!();

        chunk.disassemble_instruction(self.ip);
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code()[self.ip];
        self.ip += 1;

        byte
    }

    fn read_opcode(&mut self, chunk: &Chunk) -> Opcode {
        self.read_byte(chunk).try_into().expect("Unknown opcode")
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        chunk.constants()[self.read_byte(chunk) as usize]
    }
}
