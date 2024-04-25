use std::{cmp, collections::VecDeque};
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(int_code: &Vec<i32>) -> i32 {
    let mut max = 0;
    for perm in (0..=4).permutations(5) {
        let mut output = 0;
        for i in perm {
            output = solve_with_inputs(int_code.clone(), VecDeque::from([i, output]))[0];
        }
        max = cmp::max(max, output);
    }

    max
}

#[aoc(day7, part2)]
pub fn solve_part2(_input: &Vec<i32>) -> i32 {
    0
}

fn solve_with_inputs(p: Vec<i32>, mut inputs: VecDeque<i32>) -> Vec<i32> {
    let mut program = p.clone();
    let mut outputs: Vec<i32> = Vec::new();
    let mut p = 0;
    while p < program.len() {
        let mut inst = program[p];

        let op_code = inst % 100;
        inst /= 100;

        match op_code {
            99 => break,
            1 => { // Addition
                let p1: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                inst /= 10;
                let p2: usize = if inst%10 == 1 { p+2 } else { program[p+2] as usize };
                let p3: usize = program[p+3] as usize;
                program[p3] = program[p1] + program[p2];
                p += 4;
            },
            2 => { // Multiplication
                let p1: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                inst /= 10;
                let p2: usize = if inst%10 == 1 { p+2 } else { program[p+2] as usize };
                let p3: usize = program[p+3] as usize;
                program[p3] = program[p1] * program[p2];
                p += 4;
            },
            3 => { // Input
                let addr: usize = program[p+1] as usize;
                program[addr] = inputs.pop_front().unwrap();
                p += 2;
            },
            4 => { // Output
                let addr: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                outputs.push(program[addr]);
                p += 2;
            },
            5 => { // jump-if-true
                let p1: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                inst /= 10;
                let p2: usize = if inst%10 == 1 { p+2 } else { program[p+2] as usize };

                if program[p1] != 0 {
                    p = program[p2] as usize;
                } else {
                    p += 3;
                }
            },
            6 => { // jump-if-false
                let p1: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                inst /= 10;
                let p2: usize = if inst%10 == 1 { p+2 } else { program[p+2] as usize };

                if program[p1] == 0 {
                    p = program[p2] as usize;
                } else {
                    p += 3;
                }
            },
            7 => { // less than
                let p1: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                inst /= 10;
                let p2: usize = if inst%10 == 1 { p+2 } else { program[p+2] as usize };
                let p3: usize = program[p+3] as usize;

                program[p3] = if program[p1] < program[p2] { 1 } else { 0 };

                p += 4;
            },
            8 => { // equal to
                let p1: usize = if inst%10 == 1 { p+1 } else { program[p+1] as usize };
                inst /= 10;
                let p2: usize = if inst%10 == 1 { p+2 } else { program[p+2] as usize };
                let p3: usize = program[p+3] as usize;

                program[p3] = if program[p1] == program[p2] { 1 } else { 0 };

                p += 4;
            }
            _ => {}
        }
    }

    outputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")),
            43210
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part1(&input_generator("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")),
            54321
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            solve_part1(&input_generator("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")),
            65210
        )
    }
}