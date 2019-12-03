#[derive(Debug)]
pub struct Computer {
    program: Vec<usize>,
    memory: Vec<usize>,
    pointer: usize,
}

impl Computer {
    pub fn init(source_code: &str) -> Computer {
        let opcodes: Vec<usize> = source_code.trim().split(",").map(str_to_usize).collect();

        Computer {
            program: opcodes.clone(),
            memory: vec![],
            pointer: 0,
        }
    }

    pub fn run(&mut self, arg1: usize, arg2: usize) -> usize {
        self.memory = self.program.clone();
        self.pointer = 0;

        if self.memory.len() < 3 {
            panic!("Invalid program: a program cannot contain less than 3 opcodes");
        }

        self.memory[1] = arg1;
        self.memory[2] = arg2;

        loop {
            if self.pointer >= self.memory.len() {
                panic!("Unexpected end of program: {:?}", self);
            }

            match self.memory[self.pointer] {
                1 => self.add(),
                2 => self.multiply(),
                99 => break, // End of program
                _ => panic!(
                    "{} is not an instruction in {:?}",
                    self.memory[self.pointer], self
                ),
            };
        }

        self.memory[0]
    }

    fn add(&mut self) {
        let _instruction = self.consume_opcode();

        let left_op_pos = self.consume_opcode();
        let right_op_pos = self.consume_opcode();
        let result_pos = self.consume_opcode();

        let result = self.memory[left_op_pos] + self.memory[right_op_pos];
        self.memory[result_pos] = result;
    }

    fn multiply(&mut self) {
        let _instruction = self.consume_opcode();

        let left_op_pos = self.consume_opcode();
        let right_op_pos = self.consume_opcode();
        let result_pos = self.consume_opcode();

        let result = self.memory[left_op_pos] * self.memory[right_op_pos];
        self.memory[result_pos] = result;
    }

    fn consume_opcode(&mut self) -> usize {
        let opcode = self.memory[self.pointer];

        self.pointer += 1;

        opcode
    }
}

fn str_to_usize(input: &str) -> usize {
    input.parse().expect("Could not parse into usize")
}
