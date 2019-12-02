use std::fs;

#[derive(Debug)]
struct IntcodeProgram {
    current: usize,
    opcodes: Vec<usize>,
}

enum Instruction {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Stop,
}

impl Instruction {
    pub fn add(args: &[usize]) -> Instruction {
        Instruction::Add(args[0], args[1], args[2])
    }

    pub fn multiply(args: &[usize]) -> Instruction {
        Instruction::Multiply(args[0], args[1], args[2])
    }
}

impl IntcodeProgram {
    pub fn from_source(source_code: &str) -> IntcodeProgram {
        IntcodeProgram {
            current: 0,
            opcodes: source_code.trim().split(",").map(str_to_usize).collect(),
        }
    }

    fn to_source(&self) -> String {
        let opcodes: Vec<String> = self.opcodes.iter().map(ToString::to_string).collect();
        opcodes.join(",")
    }

    pub fn run(&mut self) {
        self.current = 0;

        loop {
            if self.current >= self.opcodes.len() {
                panic!("Unexpected end of opcode stream: {:?}", self);
            }

            let instruction = match self.opcodes[self.current] {
                1 => Instruction::add(&self.opcodes[self.current + 1..self.current + 4]),
                2 => Instruction::multiply(&self.opcodes[self.current + 1..self.current + 4]),
                99 => Instruction::Stop,
                _ => panic!("{} is not an instruction in {:?}", self.opcodes[self.current], self),
            };

            if !self.process(instruction) {
                break;
            }
        }
    }

    fn process(&mut self, instruction: Instruction) -> bool {
        match instruction {
            Instruction::Stop => false,
            Instruction::Add(left, right, result) => {
                self.opcodes[result] = self.opcodes[left] + self.opcodes[right];
                self.current += 4;
                true
            },
            Instruction::Multiply(left, right, result) => {
                self.opcodes[result] = self.opcodes[left] * self.opcodes[right];
                self.current += 4;
                true
            },
        }
    }
}

fn str_to_usize(input: &str) -> usize {
    input.parse().expect("Could not parse into usize")
}

fn main() {
    let input = fs::read_to_string("input/2019-day2.txt").expect("Input file could not be opened");
    let mut program = IntcodeProgram::from_source(&input);

    program.opcodes[1] = 12;
    program.opcodes[2] = 2;

    program.run();

    println!("{}", program.to_source());
}
