use std::convert::TryInto;

#[derive(Debug)]
pub struct Computer {
    program: Vec<i32>,
    memory: Vec<i32>,
    pointer: usize,
}

impl Computer {
    pub fn init(source_code: &str) -> Computer {
        let opcodes: Vec<i32> = source_code.trim().split(",").map(str_to_i32).collect();

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

    pub fn memset(&mut self, position: usize, value: i32) {
        self.memory[position] = value;
    }

    pub fn memget(&mut self, position: usize) -> i32 {
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

    fn memconsume(&mut self) -> i32 {
        let value = self.memory[self.pointer];

        self.pointer += 1;

        value
    }

    fn add(&mut self) {
        let _instruction = self.memconsume();

        let left_op_pos: usize = self.memconsume().try_into().expect("Invalid position");
        let right_op_pos: usize = self.memconsume().try_into().expect("Invalid position");
        let result_pos: usize = self.memconsume().try_into().expect("Invalid position");

        let result = self.memory[left_op_pos] + self.memory[right_op_pos];
        self.memory[result_pos] = result;
    }

    fn multiply(&mut self) {
        let _instruction = self.memconsume();

        let left_op_pos: usize = self.memconsume().try_into().expect("Invalid position");
        let right_op_pos: usize = self.memconsume().try_into().expect("Invalid position");
        let result_pos: usize = self.memconsume().try_into().expect("Invalid position");

        let result = self.memory[left_op_pos] * self.memory[right_op_pos];
        self.memory[result_pos] = result;
    }
}

fn str_to_i32(input: &str) -> i32 {
    input.parse().expect("Could not parse into usize")
}
