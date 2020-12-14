use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

fn main() -> Result<(), String> {
    let instructions = include_str!("../../input/2020-day14.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let mut vm = VirtualMachine::new();

    vm.run(&instructions);

    println!("{}", vm.mem.iter().map(|(_addr, val)| val).sum::<u64>());

    Ok(())
}

#[derive(Debug, Default)]
struct VirtualMachine {
    mask: Option<Mask>,
    mem: HashMap<u64, u64>,
}

impl VirtualMachine {
    fn new() -> Self {
        VirtualMachine { mask: None, mem: HashMap::new() }
    }

    fn run(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::MaskWith(mask) => self.mask = Some(mask.clone()),
                Instruction::Set(addr, val) => {
                    let val = self.mask.as_ref().map_or(*val, |mask| mask.apply(*val));
                    self.mem.insert(*addr, val);
                },
            };
        }
    }
}

#[derive(Debug)]
enum Instruction {
    MaskWith(Mask),
    Set(u64, u64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.starts_with("mask") {
            parse_mask(input)
        } else if input.starts_with("mem") {
            parse_set(input)
        } else {
            Err(format!("Invalid instruction: {}", input))
        }
    }
}

fn parse_mask(input: &str) -> Result<Instruction, String> {
    let right_op = input
        .split("=")
        .skip(1)
        .next()
        .ok_or(format!("Missing right operand in instruction: {}", input))?
        .trim();

    right_op.parse().map(|mask| Instruction::MaskWith(mask))
}

fn parse_set(input: &str) -> Result<Instruction, String> {
    let mut operands = input.split("=");

    let left = operands.next()
        .ok_or(format!("Missing left operand in instruction: {}", input))?
        .trim();

    let addr = left
        .split("[").skip(1).next().ok_or(format!("Invalid address in: {}", input))?
        .split("]").next().ok_or(format!("Invalid address in: {}", input))?
        .parse().map_err(|error| format!("Invalid address in {}: {}", input, error))?;

    let right = operands.next()
        .ok_or(format!("Missing right operand in instruction: {}", input))?
        .trim();

    let val = right.parse().map_err(|error| format!("Invalid value in {}: {}", input, error))?;

    Ok(Instruction::Set(addr, val))
}

#[derive(Clone, Debug)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
}

impl Mask {
    fn new(masked_bits: HashMap<u32, bool>) -> Self {
        let mut and_mask = u64::MAX;
        let mut or_mask = 0;

        // TODO: I don't think this will work lol
        // Later: OMFG IT WORKED???????????? I shall leave this comment for posterity.
        for (index, val) in masked_bits.iter() {
            match val {
                true => or_mask += 2u64.pow(*index),
                false => and_mask -= 2u64.pow(*index),
            }
        }

        Mask { and_mask, or_mask }
    }

    fn apply(&self, n: u64) -> u64 {
        let n = n & self.and_mask;
        let n = n | self.or_mask;

        n
    }
}

impl FromStr for Mask {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let masked_bits: HashMap<u32, bool> = input
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(index, c)| {
                match c {
                    'X' => None,
                    '0' => Some(Ok((u32::try_from(index).unwrap(), false))),
                    '1' => Some(Ok((u32::try_from(index).unwrap(), true))),
                    _ => return Some(Err(format!("Invalid mask input: {}", input))),
                }
            })
            .collect::<Result<HashMap<_, _>, String>>()?;

        Ok(Self::new(masked_bits))
    }
}
