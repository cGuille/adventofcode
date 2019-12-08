use std::convert::TryInto;

#[derive(Debug)]
pub struct Computer {
    program: Vec<i32>,
    memory: Vec<i32>,
    pointer: usize,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    param_modes: Vec<ParameterMode>,
}

impl From<i32> for Instruction {
    fn from(mut number: i32) -> Self {
        if number < 0 {
            panic!("Invalid instruction: {}", number);
        }

        let mut digits = Vec::new();

        while number > 9 {
            digits.push(number % 10);
            number = number / 10;
        }
        digits.push(number);

        let opcode_digits = [digits.get(1).unwrap_or(&0), digits.get(0).unwrap_or(&0)];
        let opcode = Opcode::from(opcode_digits);
        let param_modes = if digits.len() < 3 {
            Vec::new()
        } else {
            digits[2..].iter().map(ParameterMode::from).collect()
        };

        Instruction {
            opcode: opcode,
            param_modes: param_modes,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    Exit,
}

impl From<[&i32;2]> for Opcode {
    fn from(digits: [&i32;2]) -> Self {
        match digits {
            [0, 1] => Opcode::Add,
            [0, 2] => Opcode::Multiply,
            [9, 9] => Opcode::Exit,
            _ => panic!("Unknown opcode {:?}", digits),
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Pointer,
    Value,
}

impl From<&i32> for ParameterMode {
    fn from(digit: &i32) -> Self {
        match digit {
            0 => ParameterMode::Pointer,
            1 => ParameterMode::Value,
            _ => panic!("Not a parameter mode {}", digit),
        }
    }
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

            let instruction = Instruction::from(self.memconsume());

            match instruction.opcode {
                Opcode::Add => self.add(instruction.param_modes),
                Opcode::Multiply => self.multiply(instruction.param_modes),
                Opcode::Exit => break,
            };
        }
    }

    fn memconsume(&mut self) -> i32 {
        let value = self.memory[self.pointer];

        self.pointer += 1;

        value
    }

    fn memconsumearg(&mut self, mode: &ParameterMode) -> i32 {
        let arg = self.memconsume();

        match mode {
            ParameterMode::Value => arg,
            ParameterMode::Pointer => {
                let pointer: usize = arg.try_into().expect("Invalid pointer");
                self.memory[pointer]
            }
        }
    }

    fn add(&mut self, param_modes: Vec<ParameterMode>) {
        let left_op = self.memconsumearg(param_modes.get(0).unwrap_or(&ParameterMode::Pointer));
        let right_op = self.memconsumearg(param_modes.get(1).unwrap_or(&ParameterMode::Pointer));

        let result_pointer: usize = self.memconsume().try_into().expect("Invalid pointer");

        self.memory[result_pointer] = left_op + right_op;
    }

    fn multiply(&mut self, param_modes: Vec<ParameterMode>) {
        let left_op = self.memconsumearg(param_modes.get(0).unwrap_or(&ParameterMode::Pointer));
        let right_op = self.memconsumearg(param_modes.get(1).unwrap_or(&ParameterMode::Pointer));

        let result_pointer: usize = self.memconsume().try_into().expect("Invalid pointer");

        self.memory[result_pointer] = left_op * right_op;
    }
}

fn str_to_i32(input: &str) -> i32 {
    input.parse().expect("Could not parse into usize")
}
