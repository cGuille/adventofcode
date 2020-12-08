use std::collections::HashSet;
use std::convert::TryInto;
use std::num::ParseIntError;
use std::num::TryFromIntError;
use std::str::FromStr;

fn main() {
    let program: Program = include_str!("../../input/2020-day8.txt")
        .lines()
        .map(|instruction| instruction.parse().unwrap())
        .collect();

    println!("{}", BootLoaderEmulator::new(program).run().unwrap());
}

struct BootLoaderEmulator {
    instr: Program,
    instr_pointer: usize,
    acc: i64,
    visited: HashSet<usize>,
}

impl BootLoaderEmulator {
    fn new(program: Program) -> Self {
        Self {
            instr: program,
            instr_pointer: 0,
            acc: 0,
            visited: HashSet::new(),
        }
    }

    fn run(&mut self) -> Result<i64, String> {
        loop {
            if self.visited.contains(&self.instr_pointer) {
                break;
            }

            self.visited.insert(self.instr_pointer);

            match self.instr[self.instr_pointer] {
                Instruction::Acc(n) => self.exec_acc(n),
                Instruction::Jmp(delta) => self.exec_jmp(delta)?,
                Instruction::Nop(_) => self.exec_nop(),
            };
        }

        Ok(self.acc)
    }

    fn exec_acc(&mut self, n: i64) {
        self.acc += n;
        self.instr_pointer += 1;
    }

    fn exec_jmp(&mut self, delta: i64) -> Result<(), String> {
        let current: i64 = self
            .instr_pointer
            .try_into()
            .map_err(|error: TryFromIntError| error.to_string())?;

        let result = current + delta;

        self.instr_pointer = result
            .try_into()
            .map_err(|error: TryFromIntError| error.to_string())?;

        Ok(())
    }

    fn exec_nop(&mut self) {
        self.instr_pointer += 1;
    }
}

type Program = Vec<Instruction>;

#[derive(Debug)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Debug)]
enum InstructionParsingError {
    NoInstructionName,
    MissingArg,
    InvalidInstruction(String),
    InvalidArg(String, ParseIntError),
}

impl FromStr for Instruction {
    type Err = InstructionParsingError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(" ");
        let name = parts
            .next()
            .ok_or_else(|| InstructionParsingError::NoInstructionName)?;
        let arg = parts
            .next()
            .ok_or_else(|| InstructionParsingError::MissingArg)?;

        let arg: i64 = arg
            .parse()
            .map_err(|error| InstructionParsingError::InvalidArg(arg.to_string(), error))?;

        match name {
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            "nop" => Ok(Instruction::Nop(arg)),
            _ => Err(InstructionParsingError::InvalidInstruction(
                name.to_string(),
            )),
        }
    }
}
