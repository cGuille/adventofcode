use std::fs;

#[derive(Debug)]
struct Computer {
    program: Vec<usize>,
    memory: Vec<usize>,
    pointer: usize,
}

impl Computer {
    fn init(source_code: &str) -> Computer {
        let opcodes: Vec<usize> = source_code.trim().split(",").map(str_to_usize).collect();

        Computer {
            program: opcodes.clone(),
            memory: vec![],
            pointer: 0,
        }
    }

    fn run(&mut self, arg1: usize, arg2: usize) -> usize {
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
                1 => {
                    let left_op_pos = self.memory[self.pointer + 1];
                    let right_op_pos = self.memory[self.pointer + 2];
                    let result_pos = self.memory[self.pointer + 3];
                    self.memory[result_pos] = self.memory[left_op_pos] + self.memory[right_op_pos];
                    self.pointer += 4;
                },
                2 => {
                    let left_op_pos = self.memory[self.pointer + 1];
                    let right_op_pos = self.memory[self.pointer + 2];
                    let result_pos = self.memory[self.pointer + 3];
                    self.memory[result_pos] = self.memory[left_op_pos] * self.memory[right_op_pos];
                    self.pointer += 4;
                },
                99 => break,// End of program
                _ => panic!("{} is not an instruction in {:?}", self.memory[self.pointer], self),
            };
        }

        self.memory[0]
    }
}

fn str_to_usize(input: &str) -> usize {
    input.parse().expect("Could not parse into usize")
}

fn main() {
    let source_code = fs::read_to_string("input/2019-day2.txt").expect("Input file could not be opened");
    let mut computer = Computer::init(&source_code);

    println!("{}", computer.run(12, 2));
}
