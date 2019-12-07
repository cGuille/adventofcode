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

    pub fn boot(&mut self) {
        self.memory = self.program.clone();
        self.pointer = 0;
    }

    pub fn memset(&mut self, position: usize, value: usize) {
        self.memory[position] = value;
    }

    pub fn memget(&mut self, position: usize) -> usize {
        self.memory[position]
    }

    pub fn run(&mut self) {
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
