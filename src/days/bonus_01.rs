// https://www.reddit.com/r/adventofcode/comments/128t3c6/puzzle_implement_a_fantasy_computer_to_find_out/

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
pub struct Solution {
    program: Vec<i32>,
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let program = data
            .split(',')
            .map(|number| number.trim().parse().expect("Invalid number"))
            .collect();

        Solution { program: program }
    }

    fn part_1(&self) -> String {
        let mut registers = vec![0; 4];
        let mut instruction_pointer: usize = 0;
        let mut stack = Vec::new();

        let mut output = Vec::new();

        loop {
            let current_instruction = self.program[instruction_pointer];
            match current_instruction {
                // MOVR reg_dst, reg_src
                10 => {
                    let reg_dst = self.program[instruction_pointer + 1] as usize;
                    let reg_src = self.program[instruction_pointer + 2] as usize;
                    registers[reg_dst] = registers[reg_src];
                    instruction_pointer += 3;
                }
                // MOVV reg_dst, value
                11 => {
                    let reg_dst = self.program[instruction_pointer + 1] as usize;
                    let value = self.program[instruction_pointer + 2];
                    registers[reg_dst] = value;
                    instruction_pointer += 3;
                }
                // ADD reg_dst, reg_src
                20 => {
                    let reg_dst = self.program[instruction_pointer + 1] as usize;
                    let reg_src = self.program[instruction_pointer + 2] as usize;
                    registers[reg_dst] += registers[reg_src];
                    instruction_pointer += 3;
                }
                // SUB reg_dst, reg_src
                21 => {
                    let reg_dst = self.program[instruction_pointer + 1] as usize;
                    let reg_src = self.program[instruction_pointer + 2] as usize;
                    registers[reg_dst] -= registers[reg_src];
                    instruction_pointer += 3;
                }
                // PUSH reg_src
                30 => {
                    let reg_src = self.program[instruction_pointer + 1] as usize;
                    stack.push(registers[reg_src]);
                    instruction_pointer += 2;
                }
                // POP reg_dst
                31 => {
                    let reg_dst = self.program[instruction_pointer + 1] as usize;
                    registers[reg_dst] = stack.pop().expect("Popping an empty stack.");
                    instruction_pointer += 2;
                }
                // JP addr
                40 => {
                    let address = self.program[instruction_pointer + 1] as usize;
                    instruction_pointer = address;
                }
                // JL reg_1, reg_2, addr
                41 => {
                    let reg_1 = self.program[instruction_pointer + 1] as usize;
                    let reg_2 = self.program[instruction_pointer + 2] as usize;
                    let address = self.program[instruction_pointer + 3] as usize;
                    if registers[reg_1] < registers[reg_2] {
                        instruction_pointer = address;
                    } else {
                        instruction_pointer += 4;
                    }
                }
                // CALL addr
                42 => {
                    let address = self.program[instruction_pointer + 1] as usize;
                    stack.push(instruction_pointer as i32 + 2);
                    instruction_pointer = address;
                }
                // RET
                50 => {
                    instruction_pointer = stack.pop().expect("Popping an empty stack.") as usize;
                }
                // PRINT reg
                60 => {
                    let reg = self.program[instruction_pointer + 1] as usize;
                    println!("{}", registers[reg]);
                    output.push(registers[reg]);
                    instruction_pointer += 2;
                }
                // HALT
                255 => break,
                // INVALID
                _ => panic!(
                    "Invalid instruction {current_instruction} at address {instruction_pointer}"
                ),
            }
        }

        format!("{:?}", output)
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data: String = fs::read_to_string("data/bonus_01_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                program: vec![11, 1, 42, 60, 1, 255]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "[42]")
    }
}
