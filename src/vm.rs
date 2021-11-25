use super::chunk::*;

pub struct VM {
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ip: 0,
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) {
        self.ip = 0;
        let code = chunk.code();
        self.run(code);
    }

    fn run(&mut self, code: &[u8]) {
        loop {
            let opcode = code[self.ip].try_into().expect("Unknown opcode");
            self.ip += 1;

            match opcode {
                Opcode::Constant => {

                }
                Opcode::Return => return,
            }
        }
    }
}
