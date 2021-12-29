use super::chunk::*;
use super::compiler::compile;
use super::value::Value;

pub enum InterpretError {
    Compile,
    Runtime,
}

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::with_capacity(255),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        compile(source);
        Ok(())
    }

    fn run(&mut self, chunk: &Chunk) {
        macro_rules! binary_op {
            ($op:tt) => {
                {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a $op b);
                }
            };
        }

        loop {
            #[cfg(feature = "debug_trace_execution")]
            self.trace(chunk);

            match self.read_opcode(chunk) {
                Opcode::Constant => {
                    let constant = self.read_constant(chunk);
                    self.push(constant);
                }
                Opcode::Add => binary_op!(+),
                Opcode::Subtract => binary_op!(-),
                Opcode::Multiply => binary_op!(*),
                Opcode::Divide => binary_op!(/),
                Opcode::Negate => {
                    let top = self.top_mut();
                    *top = -*top;
                }
                Opcode::Return => {
                    println!("{}", self.pop());
                    return;
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

    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }

    fn top(&self) -> &Value {
        self.stack.last().expect("Stack underflow")
    }

    fn top_mut(&mut self) -> &mut Value {
        self.stack.last_mut().expect("Stack underflow")
    }
}
